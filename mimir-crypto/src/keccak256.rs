//! `keccak-256` hashing algorithm.
//!
//! this module exposes a minimal wrapper around the 256 bit
//! variant of keccak from the `tiny-keccak` crate.  This wrapper
//! is specifically intended to be used for easy interop
//! via this crate's `Hasher` trait.  If you are just looking
//! for a good `keccak` implementation, prefer using `tiny-keccak`
//! directly.
use tiny_keccak::Keccak;
use ::Hasher;


/// context object for `keccak-256` hashing algorithm.
#[derive(Clone)]
pub struct Keccak256 {
    inner: Keccak
}


impl Hasher for Keccak256 {

    type Out = [u8;32];

    fn absorb(&mut self, bytes: &[u8]) {
        self.inner.update(bytes);
    }

    fn finish(self) -> Self::Out {
        let mut buff = [0u8;32];
        self.inner.finalize(&mut buff);
        buff
    }
}



impl Keccak256 {

    /// absorb bytes-like value into hasher
    pub fn absorb(&mut self, bytes: &[u8]) {
        <Self as Hasher>::absorb(self,bytes)
    }

    /// consume hasher, returning output
    pub fn finish(self) -> [u8;32] {
        <Self as Hasher>::finish(self)
    }

    /// hash a bytes-like object 
    pub fn hash(bytes: &[u8]) -> [u8;32] {
        <Self as Hasher>::hash(bytes)
    }
}



impl Default for Keccak256 {

   fn default() -> Self {
       let inner = Keccak::new_keccak256();
       Keccak256 { inner }
   }
}
