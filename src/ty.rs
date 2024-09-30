pub enum IoType {
    FileSystem,
    Socket,
    Device,
}

pub trait IoDescriptor<T = IoType> {
    fn io_type(&self) -> T;
}
