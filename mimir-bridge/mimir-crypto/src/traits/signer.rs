use std::sync::Arc;
use std::rc::Rc;


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

