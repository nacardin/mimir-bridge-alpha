
/// a cryptographic hasher
pub trait Hasher: Default {

    /// final output of hasher
    type Out;

    /// absorb bytes-like value into hasher
    fn absorb(&mut self, bytes: &[u8]);

    /// consume hasher, returning output
    fn finish(self) -> Self::Out;

    /// hash a bytes-like object 
    fn hash(bytes: &[u8]) -> Self::Out {
        let mut hasher = Self::default();
        hasher.absorb(bytes);
        hasher.finish()
    }
}

