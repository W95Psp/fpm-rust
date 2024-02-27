use std::{hash::Hasher, io};

pub trait TryHash {
    type Error;
    fn try_hash<H: Hasher>(&self, state: &mut H) -> Result<(), Self::Error>;
    fn direct_hash(&self) -> Result<u64, Self::Error> {
        hash::<Self>(&self)
    }
}

pub fn hash<T: TryHash + ?Sized>(x: &T) -> Result<u64, T::Error> {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    x.try_hash(&mut hasher)?;
    Ok(hasher.finish())
}

pub struct HashWriter<'a, T: Hasher>(pub &'a mut T);

impl<'a, T: Hasher> io::Write for HashWriter<'a, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write(buf).map(|_| ())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
