use super::Serializer;
use crate::error::{Error, Result};
use serde::ser::Serialize;
use std::io::Write;

pub struct SeqSeralizer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    must_close_tag: bool,
}

impl<'ser, W: 'ser + Write> SeqSeralizer<'ser, W> {
    pub fn new(ser: &'ser mut Serializer<W>, must_close_tag: bool) -> Self {
        SeqSeralizer {
            ser,
            must_close_tag,
        }
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeSeq for SeqSeralizer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}
