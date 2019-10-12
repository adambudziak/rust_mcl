use crate::common::Base;

pub trait RawSerializable {
    fn serialize_raw(&self) -> Vec<u8>;
    fn deserialize_raw(&mut self, bytes: &[u8]) -> Result<usize, ()>;
}

pub trait Formattable {
    fn set_str(&mut self, buffer: &str, io_mode: Base);
    fn get_str(&self, io_mode: Base) -> String;
}

pub trait Random {
    fn set_by_csprng(&mut self);
}

