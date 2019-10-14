extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Clone`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ident, $u:ident) => {
        TokenStream::from(quote! {
            impl<'a> $imp<#$u> for &'a #$t {
                type Output = <#$t as $imp<#$u>>::Output;

                #[inline]
                fn $method(self, other: #$u) -> <#$t as $imp<#$u>>::Output {
                    $imp::$method(self.clone(), other)
                }
            }

            impl<'a> $imp<&'a #$u> for #$t {
                type Output = <#$t as $imp<#$u>>::Output;

                #[inline]
                fn $method(self, other: &'a #$u) -> <#$t as $imp<#$u>>::Output {
                    $imp::$method(self, other.clone())
                }
            }

            impl<'a, 'b> $imp<&'a #$u> for &'b #$t {
                type Output = <#$t as $imp<#$u>>::Output;

                #[inline]
                fn $method(self, other: &'a #$u) -> <#$t as $imp<#$u>>::Output {
                    $imp::$method(self.clone(), other.clone())
                }
            }
        })
    };
}

macro_rules! ident {
    ($pat: literal, $name: ident) => {
        Ident::new(&format!($pat, $name), Span::call_site())
    };
}

#[proc_macro_derive(Formattable)]
pub fn derive_formattable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let set_str_fn = ident!("mclBn{}_setStr", name);
    let get_str_fn = ident!("mclBn{}_getStr", name);

    let inner_t = ident!("MclBn{}", name);

    let expanded = quote! {
        impl #name {
            pub fn from_str(buffer: &str, io_mode: Base) -> Self {
                let mut result = Self::default();
                result.set_str(buffer, io_mode);
                result
            }
        }

        impl Formattable for #name {
            fn set_str(&mut self, buffer: &str, io_mode: Base) {
                let err = unsafe {
                    #set_str_fn(
                        &mut self.inner as *mut #inner_t,
                        buffer.as_ptr() as *const std::os::raw::c_char,
                        buffer.len() as libc::size_t,
                        io_mode as libc::c_int,
                    )
                };
                assert_eq!(err, 0);
            }

            fn get_str(&self, io_mode: Base) -> String {
                let len = 2048;
                let mut buf = vec![0u8; len];
                let bytes = unsafe {
                    #get_str_fn(
                        buf.as_mut_ptr() as *mut std::os::raw::c_char,
                        len as libc::size_t,
                        &self.inner as *const #inner_t,
                        io_mode as libc::c_int,
                    )
                };
                assert_ne!(bytes, 0);
                String::from_utf8_lossy(&buf[..bytes]).into_owned()
            }

        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Random)]
pub fn derive_from_csprng(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let inner_t = ident!("MclBn{}", name);
    let from_csprng_fn = ident!("mclBn{}_setByCSPRNG", name);

    let expanded = quote! {
        impl Random for #name {
            fn set_by_csprng(&mut self) {
                unsafe { #from_csprng_fn(&mut self.inner as *mut #inner_t) };
            }
        }

        impl #name {
            pub fn from_csprng() -> Self {
                let mut result = #name::default();
                result.set_by_csprng();
                result
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Object)]
pub fn derive_mcl_object(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let tests_name = ident!("{}_tests", name);

    let clear_fn = ident!("mclBn{}_clear", name);
    let eq_fn = ident!("mclBn{}_isEqual", name);
    let ser_fn = ident!("mclBn{}_serialize", name);
    let de_fn = ident!("mclBn{}_deserialize", name);

    let inner_t = ident!("MclBn{}", name);
    let visitor_struct = ident!("{}Visitor", name);

    let expanded = quote! {
        impl #name {
            pub fn clear(&mut self) {
                unsafe {
                    #clear_fn(&mut self.inner);
                }
            }
        }

        impl RawSerializable for #name {
            fn serialize_raw(&self) -> Result<Vec<u8>, ()> {
                let mut buf = vec![0; 2048];
                let bytes = unsafe {
                    #ser_fn(
                        buf.as_mut_ptr() as *mut std::os::raw::c_void,
                        buf.len() as libc::size_t,
                        &self.inner as *const #inner_t,
                    )
                };
                match bytes {
                    0 => Err(()),
                    _ => Ok(buf[..bytes].to_vec())
                }
            }

            fn deserialize_raw(&mut self, bytes: &[u8]) -> Result<usize, ()> {
                let copied = unsafe {
                    #de_fn(
                        &mut self.inner as *mut #inner_t,
                        bytes.as_ptr() as *const std::os::raw::c_void,
                        bytes.len()
                    )
                };
                match copied {
                    0 => Err(()),
                    _ => Ok(copied)
                }
            }
        }

        #[cfg(feature="serde_lib")]
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
                S::Error: serde::ser::Error,
            {
                use serde::ser::Error;
                let buf = self.serialize_raw().map_err(|_| S::Error::custom("Couldn't serialize MCL object"))?;
                serializer.serialize_bytes(&buf)
            }
        }

        #[cfg(feature="serde_lib")]
        struct #visitor_struct;

        #[cfg(feature="serde_lib")]
        impl<'de> serde::de::Visitor<'de> for #visitor_struct {
            type Value = #name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "bytes serializing the mcl object")
            }

            fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let mut val = #name::default();
                val.deserialize_raw(bytes).map_err(|_| E::custom("Invalid MCL object to deserialize"))?;
                Ok(val)
            }
        }

        #[cfg(feature="serde_lib")]
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>
            {
                deserializer.deserialize_bytes(#visitor_struct)
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                unsafe {
                    #eq_fn(&self.inner, &other.inner) == 1
                }
            }
        }

        #[cfg(test)]
        mod #tests_name {
            use super::#name;
            use crate::traits::RawSerializable;
            use crate::init::{ init_curve, Curve };

            #[test]
            fn test_deserialize_empty_vec() {
                init_curve(Curve::Bls12_381);
                let mut x = #name::default();
                let deserialized =  x.deserialize_raw(&[]);
                assert!(deserialized.is_err())
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(ScalarPoint)]
pub fn derive_scalar_group(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let add_fn = ident!("mclBn{}_add", name);
    let sub_fn = ident!("mclBn{}_sub", name);
    let mul_fn = ident!("mclBn{}_mul", name);
    let div_fn = ident!("mclBn{}_div", name);
    let neg_fn = ident!("mclBn{}_neg", name);
    let inv_fn = ident!("mclBn{}_inv", name);
    let sqr_fn = ident!("mclBn{}_sqr", name);

    let expanded = quote! {
        impl Add for #name {
            type Output = #name;

            #[inline]
            fn add(self, other: Self) -> Self {
                let mut result = Self::default();
                unsafe {
                    #add_fn(&mut result.inner, &self.inner, &other.inner);
                };
                result
            }
        }

        impl Sub for #name {
            type Output = #name;

            #[inline]
            fn sub(self, other: Self) -> Self {
                 let mut result = Self::default();
                 unsafe {
                     #sub_fn(&mut result.inner, &self.inner, &other.inner);
                 };
                 result
            }
        }

        impl Mul for #name {
            type Output = #name;

            #[inline]
            fn mul(self, other: Self) -> Self {
                 let mut result = Self::default();
                 unsafe {
                     #mul_fn(&mut result.inner, &self.inner, &other.inner);
                 };
                 result
            }
        }

        impl Div for #name {
            type Output = #name;

            #[inline]
            fn div(self, other: Self) -> Self {
                 let mut result = Self::default();
                 unsafe {
                     #div_fn(&mut result.inner, &self.inner, &other.inner);
                 };
                 result
            }
        }

        impl #name {
            pub fn neg(&self) -> Self {
                let mut result = Self::default();
                unsafe {
                    #neg_fn(&mut result.inner, &self.inner);
                };
                result
            }

            pub fn inv(&self) -> Self {
                let mut result = Self::default();
                unsafe {
                    #inv_fn(&mut result.inner, &self.inner);
                };
                result
            }

            pub fn sqr(&self) -> Self {
                let mut result = Self::default();
                unsafe {
                    #sqr_fn(&mut result.inner, &self.inner);
                };
                result
            }
        }

    };
    let mut result = TokenStream::from(expanded);
    result.extend(forward_ref_binop! { impl Add, add for name, name });
    result.extend(forward_ref_binop! { impl Sub, sub for name, name });
    result.extend(forward_ref_binop! { impl Mul, mul for name, name });
    result.extend(forward_ref_binop! { impl Div, div for name, name });

    result
}

#[proc_macro_derive(AdditivePoint)]
pub fn derive_additivte_group(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let add_fn = ident!("mclBn{}_add", name);
    let sub_fn = ident!("mclBn{}_sub", name);
    let neg_fn = ident!("mclBn{}_neg", name);
    let dbl_fn = ident!("mclBn{}_dbl", name);
    let mul_fn = ident!("mclBn{}_mul", name);

    let hnm_fn = ident!("mclBn{}_hashAndMapTo", name);

    let inner_t = ident!("MclBn{}", name);

    let expanded = quote! {
        impl Add for #name {
            type Output = #name;

            #[inline]
            fn add(self, other: Self) -> Self {
                let mut result = Self::default();
                unsafe {
                    #add_fn(&mut result.inner, &self.inner, &other.inner);
                };
                result
            }
        }

        impl Sub for #name {
            type Output = #name;

            #[inline]
            fn sub(self, other: Self) -> Self {
                 let mut result = Self::default();
                 unsafe {
                     #sub_fn(&mut result.inner, &self.inner, &other.inner);
                 };
                 result
            }
        }

        impl #name {
            pub fn neg(&self) -> Self {
                let mut result = Self::default();
                unsafe {
                    #neg_fn(&mut result.inner, &self.inner);
                };
                result
            }

            pub fn dbl(&self) -> Self {
                let mut result = Self::default();
                unsafe {
                    #dbl_fn(&mut result.inner, &self.inner);
                };
                result
            }

            pub fn hash_and_map(buf: &[u8]) -> Result<Self, i32> {
                let mut result = Self::default();
                let err = unsafe {
                    #hnm_fn(
                        &mut result.inner as *mut #inner_t,
                        buf.as_ptr() as *const std::os::raw::c_void,
                        buf.len(),
                    )
                };
                match err {
                    0 => Ok(result),
                    n => Err(n),
                }
            }

        }

        impl Mul<Fr> for #name {
            type Output = #name;

            #[inline]
            fn mul(self, other: Fr) -> Self {
                let mut result = Self::default();
                unsafe {
                    #mul_fn(&mut result.inner, &self.inner, &other.inner);
                };
                result
            }
        }


    };
    let fr_ident = Ident::new("Fr", Span::call_site());
    let mut result = TokenStream::from(expanded);
    result.extend(forward_ref_binop! { impl Add, add for name, name });
    result.extend(forward_ref_binop! { impl Sub, sub for name, name });
    result.extend(forward_ref_binop! { impl Mul, mul for name, fr_ident });
    result
}

#[proc_macro_derive(MultiplicativePoint)]
pub fn derive_multiplicative_group(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let mul_fn = ident!("mclBn{}_mul", name);

    let expanded = quote! {
        impl Mul for #name {
            type Output = #name;

            #[inline]
            fn mul(self, other: Self) -> Self {
                 let mut result = Self::default();
                 unsafe {
                     #mul_fn(&mut result.inner, &self.inner, &other.inner);
                 };
                 result
            }
        }

        impl #name {
            pub fn pow(&self, a: &Fr) -> Self {
                let mut result = MclBnGT::default();
                unsafe {
                    mclBnGT_pow(
                        &mut result as *mut MclBnGT,
                        &self.inner as *const MclBnGT,
                        &a.inner as *const MclBnFr,
                    );
                }
                GT { inner: result }
            }
        }
    };

    let mut result = TokenStream::from(expanded);
    result.extend(forward_ref_binop! { impl Mul, mul for name, name });
    result
}
