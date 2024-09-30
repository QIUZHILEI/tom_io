use core::error::Error;

pub enum Status {}

pub trait Aio {
    type Err: Error;
    fn status(&self) -> Status;
}

pub trait BlockIO: Aio {
    fn read_block(&self, buf: &mut dyn Buffer) -> Result<(), Self::Err>;
    fn write_block(&self, buffer: &dyn Buffer) -> Result<(), Self::Err>;
}

pub trait CharIO: Aio {
    fn read_char(&self, buffer: &mut dyn Buffer) -> Result<(), Self::Err>;
    fn write_char(&self, buffer: &dyn Buffer) -> Result<(), Self::Err>;
}
