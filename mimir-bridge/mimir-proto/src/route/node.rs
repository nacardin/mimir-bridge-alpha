use mimir_util::types::Either;
use mimir_crypto::Keccak256;


/// branch node in routing tree.
///
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Branch<T> {
    hash: [u8;32],
    inner: Either<Box<Pair<Branch<T>>>,Pair<Leaf<T>>>
}


impl<T> Branch<T> where T: AsRef<[u8]> {

    /// build branch from pair of leaf values
    pub fn from_values(left: T, right: T) -> Self {
        let (left,right) = (Leaf::new(left),Leaf::new(right));
        let mut hasher = Keccak256::default();
        hasher.absorb(&left.hash);
        hasher.absorb(&right.hash);
        let hash = hasher.finish();
        let inner = Either::B(Pair { left, right });
        Branch { hash, inner }
    }

}


impl<T> Branch<T> {

    /// build branch from pair of sub-branches
    pub fn from_branches(left: Branch<T>, right: Branch<T>) -> Self {
        let mut hasher = Keccak256::default();
        hasher.absorb(&left.hash);
        hasher.absorb(&right.hash);
        let hash = hasher.finish();
        let inner = Either::A(Box::new(Pair { left, right }));
        Branch { hash, inner }
    }

    /// get reference to lefthand child node
    pub fn left(&self) -> Either<&Branch<T>,&Leaf<T>> {
        match self.inner {
            Either::A(ref pair) => Either::A(&pair.left),
            Either::B(ref pair) => Either::B(&pair.left),
        }
    }

    /// get reference to righthand child node
    pub fn right(&self) -> Either<&Branch<T>,&Leaf<T>> {
        match self.inner {
            Either::A(ref pair) => Either::A(&pair.right),
            Either::B(ref pair) => Either::B(&pair.right),
        }
    }
}


/// pair of values with "handedness".
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
struct Pair<T> {
    left: T,
    right: T, 
}


/// leaf of routing tree.
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Leaf<T> {
    /// `keccak-256` hash of leaf address
    pub hash: [u8;32],

    /// item of this leaf (final value of routing)
    pub item: T,
}


impl<T> Leaf<T> where T: AsRef<[u8]> {

    /// generate new leaf around specified item
    pub fn new(item: T) -> Self {
        let hash = Keccak256::hash(item.as_ref());
        Leaf { hash, item }
    }
}


