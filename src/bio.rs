use core::error::Error;

use crate::Buffer;

pub trait Bio:Sync+Sync {
    type Err: Error;
    fn acquire(&self) -> Result<(), Self::Err>;
    fn release(&self) -> Result<(), Self::Err>;
}

pub trait BlockIO: Bio {
    fn read_block(&self, buffer: &mut impl Buffer) -> Result<(), Self::Err>;
    fn read_blocks(&self,buffers:&mut [impl Buffer]) ->Result<(),Self::Err>;
    fn write_block(&self, buffer: &impl Buffer) -> Result<(), Self::Err>;
    fn write_blocks(&self, buffer: &[impl Buffer]) -> Result<(), Self::Err>;
}

pub trait CharIO: Bio {
    fn read_char(&self, buffer: &mut impl Buffer) -> Result<(), Self::Err>;
    fn read_chars(&self, buffer: &mut [impl Buffer]) -> Result<(), Self::Err>;
    fn write_char(&self, buffer: &impl Buffer) -> Result<(), Self::Err>;
    fn write_chars(&self, buffer: &[impl Buffer]) -> Result<(), Self::Err>;
}
