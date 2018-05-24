//! useful crypto traits.
//!
mod hashable;
mod hasher;
mod signer;


pub use self::hashable::Hashable;
pub use self::hasher::Hasher;
pub use self::signer::Signer;


/*
use std::sync::Arc;
use std::rc::Rc;
use std::mem;


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


/// a type which can be hashed
///
/// Provides a standardized mechanism by which types may describe
/// exactly how they should be hashed.  Useful for complex compound
/// types which need to be hashed in a very specific way.
///
pub trait Hashable {

    /// absorb `self` into provided hasher
    fn absorb_with<H: Hasher>(&self, hasher: &mut H);

    /// hash `self` with specified hasher type
    fn hash<H: Hasher>(&self) -> H::Out {
        let mut hasher: H = Default::default();
        self.absorb_with(&mut hasher);
        hasher.finish()
    }
}


impl Hashable for [u8] {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) { hasher.absorb(self); }
}


impl Hashable for u8 {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) { hasher.absorb(&[*self]); }
}

impl Hashable for u16 {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) {
        let byte_repr: [u8;2] = unsafe { mem::transmute(*self) };
        hasher.absorb(&byte_repr);
    }
}

impl Hashable for u32 {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) {
        let byte_repr: [u8;4] = unsafe { mem::transmute(*self) };
        hasher.absorb(&byte_repr);
    }
}

impl Hashable for u64 {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) {
        let byte_repr: [u8;8] = unsafe { mem::transmute(*self) };
        hasher.absorb(&byte_repr);
    }
}

impl Hashable for u128 {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) {
        let byte_repr: [u8;16] = unsafe { mem::transmute(*self) };
        hasher.absorb(&byte_repr);
    }
}

impl Hashable for str {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) { hasher.absorb(self.as_ref()); }
}


impl<'a,T> Hashable for &'a T where T: Hashable + ?Sized {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) { <T as Hashable>::absorb_with(self,hasher) }
}


impl<'a,T> Hashable for &'a mut T where T: Hashable + ?Sized {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) { <T as Hashable>::absorb_with(self,hasher) }
}


impl<T> Hashable for Arc<T> where T: Hashable + ?Sized {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) { <T as Hashable>::absorb_with(self,hasher) }
}


impl<T> Hashable for Rc<T> where T: Hashable + ?Sized {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) { <T as Hashable>::absorb_with(self,hasher) }
}


impl<T> Hashable for Box<T> where T: Hashable + ?Sized {

    fn absorb_with<H: Hasher>(&self, hasher: &mut H) { <T as Hashable>::absorb_with(self,hasher) }
}


/// a cryptographic signature generator.
pub trait Signer {

    /// type of input to be signed.
    type Msg: ?Sized;

    /// type of signature produced.
    type Sig;

    /// public identity of signer
    ///
    /// this may be a public key or some similar 
    /// abstraction such as an ethereum address.
    type Pub;

    /// perform the signing operation.
    fn sign(&self, msg: &Self::Msg) -> Self::Sig;

    /// get copy of public identity
    fn identify(&self) -> Self::Pub;
}


impl<'a,T> Signer for &'a T where T: Signer + ?Sized {

    type Msg = T::Msg;

    type Sig = T::Sig;

    type Pub = T::Pub;

    fn sign(&self, msg: &Self::Msg) -> Self::Sig {
        <T as Signer>::sign(self,msg)
    }

    fn identify(&self) -> Self::Pub {
        <T as Signer>::identify(self)
    }
}


impl<'a,T> Signer for &'a mut T where T: Signer + ?Sized {

    type Msg = T::Msg;

    type Sig = T::Sig;

    type Pub = T::Pub;

    fn sign(&self, msg: &Self::Msg) -> Self::Sig {
        <T as Signer>::sign(self,msg)
    }

    fn identify(&self) -> Self::Pub {
        <T as Signer>::identify(self)
    }
}


impl<T> Signer for Arc<T> where T: Signer + ?Sized {

    type Msg = T::Msg;

    type Sig = T::Sig;

    type Pub = T::Pub;

    fn sign(&self, msg: &Self::Msg) -> Self::Sig {
        <T as Signer>::sign(self.as_ref(),msg)
    }

    fn identify(&self) -> Self::Pub {
        <T as Signer>::identify(self)
    }
}


impl<T> Signer for Rc<T> where T: Signer + ?Sized {

    type Msg = T::Msg;

    type Sig = T::Sig;

    type Pub = T::Pub;

    fn sign(&self, msg: &Self::Msg) -> Self::Sig {
        <T as Signer>::sign(self.as_ref(),msg)
    }

    fn identify(&self) -> Self::Pub {
        <T as Signer>::identify(self)
    }
}


impl<T> Signer for Box<T> where T: Signer + ?Sized {

    type Msg = T::Msg;

    type Sig = T::Sig;

    type Pub = T::Pub;

    fn sign(&self, msg: &Self::Msg) -> Self::Sig {
        <T as Signer>::sign(self.as_ref(),msg)
    }

    fn identify(&self) -> Self::Pub {
        <T as Signer>::identify(self)
    }
}
*/
