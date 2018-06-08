use traits::Hasher;
use std::sync::Arc;
use std::rc::Rc;
use std::mem;


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
