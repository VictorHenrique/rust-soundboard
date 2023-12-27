pub trait Frame<T> {
    fn read(buffer: &[u8], frame_size: usize) -> T;
}


