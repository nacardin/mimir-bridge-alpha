//! misc helpers.
//!
use traits::Hasher;

/// fake `Hasher` implementer which records all bytes
/// observed to a vector for later use.
#[derive(Default,Debug,Clone)]
pub struct HashVoyeur {
    /// internal byte collector
    pub inner: Vec<u8>
}


impl Hasher for HashVoyeur {
    
    /// final output of hasher
    type Out = Vec<u8>;

    /// absorb bytes-like value into hasher
    fn absorb(&mut self, bytes: &[u8]) {
        self.inner.extend_from_slice(bytes);
    }

    /// consume hasher, returning output
    fn finish(self) -> Self::Out {
        let HashVoyeur { inner } = self;
        inner
    }

    /// hash a bytes-like object 
    fn hash(bytes: &[u8]) -> Self::Out {
        let mut hasher = Self::default();
        hasher.absorb(bytes);
        hasher.finish()
    }
}


impl AsRef<[u8]> for HashVoyeur {

    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}


impl AsMut<[u8]> for HashVoyeur {

    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}


