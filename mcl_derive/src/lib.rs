extern crate proc_macro;

use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span};
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Object)]
pub fn derive_mcl_object(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let clear_fn = Ident::new(&format!("mclBn{}_clear", name), Span::call_site());
    let eq_fn = Ident::new(&format!("mclBn{}_isEqual", name), Span::call_site());
    let ser_fn = Ident::new(&format!("mclBn{}_serialize", name), Span::call_site());
    let de_fn = Ident::new(&format!("mclBn{}_deserialize", name), Span::call_site());

    let inner_t = Ident::new(&format!("MclBn{}", name), Span::call_site());
    let visitor_struct = Ident::new(&format!("{}Visitor", name), Span::call_site());

    let expanded = quote! {
        impl #name {
            pub fn clear(&mut self) {
                unsafe {
                    #clear_fn(&mut self.inner);
                }
            }

            pub fn serialize_raw(&self) -> Vec<u8> {
                let mut buf = vec![0; 2048];
                let bytes = unsafe {
                    #ser_fn(
                        buf.as_mut_ptr() as *mut c_void,
                        buf.len() as size_t,
                        &self.inner as *const #inner_t,
                    )
                };
                buf[..bytes].to_vec()
            }

            pub fn deserialize_raw(bytes: &[u8]) -> Result<Self, ()> {
                let mut result = Self::default();
                let err = unsafe {
                    #de_fn(
                        &mut result.inner as *mut #inner_t,
                        bytes.as_ptr() as *const c_void,
                        bytes.len()
                    )
                };
                match err {
                    0 => Err(()),
                    _ => Ok(result)
                }
            }
        }

        #[cfg(feature="serde_lib")]
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let buf = self.serialize_raw();
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
                #name::deserialize_raw(bytes).map_err(|_| E::custom("Invalid MCL object to deserialize"))
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
    };
    TokenStream::from(expanded)
}


#[proc_macro_derive(ScalarGroup)]
pub fn derive_scalar_group(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let add_fn = Ident::new(&format!("mclBn{}_add", name), Span::call_site());
    let sub_fn = Ident::new(&format!("mclBn{}_sub", name), Span::call_site());
    let mul_fn = Ident::new(&format!("mclBn{}_mul", name), Span::call_site());
    let div_fn = Ident::new(&format!("mclBn{}_div", name), Span::call_site());
    let neg_fn = Ident::new(&format!("mclBn{}_neg", name), Span::call_site());
    let inv_fn = Ident::new(&format!("mclBn{}_inv", name), Span::call_site());
    let sqr_fn = Ident::new(&format!("mclBn{}_sqr", name), Span::call_site());

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
        forward_ref_binop! { impl Add, add for #name, #name }

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
        forward_ref_binop! { impl Sub, sub for #name, #name }

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
        forward_ref_binop! { impl Mul, mul for #name, #name }

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
        forward_ref_binop! { impl Div, div for #name, #name }

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

    TokenStream::from(expanded)
}

#[proc_macro_derive(AdditiveGroup)]
pub fn derive_additivte_group(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let add_fn = Ident::new(&format!("mclBn{}_add", name), Span::call_site());
    let sub_fn = Ident::new(&format!("mclBn{}_sub", name), Span::call_site());
    let neg_fn = Ident::new(&format!("mclBn{}_neg", name), Span::call_site());
    let dbl_fn = Ident::new(&format!("mclBn{}_dbl", name), Span::call_site());
    let mul_fn = Ident::new(&format!("mclBn{}_mul", name), Span::call_site());

    let hnm_fn = Ident::new(&format!("mclBn{}_hashAndMapTo", name), Span::call_site());

    let inner_t = Ident::new(&format!("MclBn{}", name), Span::call_site());

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
        forward_ref_binop! { impl Add, add for #name, #name }

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
        forward_ref_binop! { impl Sub, sub for #name, #name }

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
                        buf.as_ptr() as *const c_void,
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
        forward_ref_binop! { impl Mul, mul for #name, Fr }


    };
    TokenStream::from(expanded)
}


#[proc_macro_derive(MultiplicativeGroup)]
pub fn derive_multiplicative_group(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let mul_fn = Ident::new(&format!("mclBn{}_mul", name), Span::call_site());

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
        forward_ref_binop! { impl Mul, mul for #name, #name }

    };
    
    TokenStream::from(expanded)
}
