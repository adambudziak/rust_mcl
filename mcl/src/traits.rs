use crate::common::Base;

/// An interface for using the custom MCL serialization format.
/// The result types are used to wrap the result type of
/// `mclBn<name>_serialize` and `mclBn<name>_deserialize`.
///
/// See `mcl/api.md` to see the details of the serialization format.
///
/// When the feature `serde_lib` is enabled, then this trait MCL objects
/// deriving from this trait using the `RawSerializable` macro also implement
/// [`serde::Serialize`] and [`serde::Deserialize`].
pub trait RawSerializable {
    /// Serialize the object into a vector of bytes.
    ///
    /// The resulting vector is truncated to the number of bytes
    /// used in the serialization process. It shouldn't be further
    /// manipulated.
    ///
    /// # Errors
    /// Returns `Err(())` when the `ffi` serialization function returns
    /// `0` corresponding to the number of bytes copied.
    ///
    fn serialize_raw(&self) -> Result<Vec<u8>, ()>;
    /// Deserialize the object from a vector of bytes in place.
    ///
    /// If ok, this function returns the number of bytes consumed.
    ///
    /// # Errors
    /// Returns `Err(())` when the `ffi` deserialization function returns
    /// `0` corresponding to the number of bytes copied.
    ///
    /// # Safety
    /// This function is fairly safe, trying to deserialize an empty vector
    /// doesn't trigger an UB and generally the degenerate cases yield
    /// `Error`s as they should.
    fn deserialize_raw(&mut self, bytes: &[u8]) -> Result<usize, ()>;
}

pub trait Formattable {
    fn set_str(&mut self, buffer: &str, io_mode: Base);
    fn get_str(&self, io_mode: Base) -> String;
}

pub trait Random {
    fn set_by_csprng(&mut self);
}

