pub trait Memory {
    fn read_address<T>(&self, offset: usize) -> &mut T;
}

