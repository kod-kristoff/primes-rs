use std::io;
use byteorder::{BigEndian, ByteOrder};


type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

pub struct IntSet<D>(fst::raw::Fst<D>);

impl<D: AsRef<[u8]>> IntSet<D> {
    pub fn new(data: D) -> Result<IntSet<D>> {
        let raw_fst = fst::raw::Fst::new(data)?;
        Ok(Self(raw_fst))
    }

    pub fn contains(&self, n: u32) -> bool {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, n);
        self.0.contains_key(buf)
    }
}

pub struct IntSetBuilder<W: io::Write>(fst::SetBuilder<W>);

impl<W: io::Write> IntSetBuilder<W> {
    pub fn new(write: W) -> Result<IntSetBuilder<W>> {
        log::trace!("IntSetBuilder::new");
        Ok(Self(fst::SetBuilder::new(write)?))
    }

    pub fn insert(&mut self, num: u32) -> Result<()> {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, num);
        self.0.insert(buf)?;
        Ok(())
    }

    pub fn finish(self) -> Result<()> {
        self.0.finish()?;
        Ok(())
    }

    pub fn bytes_written(&self) -> u64 {
        self.0.bytes_written()
    }
}
