use route::policy;


simple_error!(
    ProofError, "error indicating a bad routing proof",
    Root => "root hash does not match",
    Path => "invalid path traversal",
);


/// hash of a child node
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum NodeHash {
    /// hash of left child node
    Left([u8;32]),
    /// hash of right child node
    Right([u8;32]),
}


/// attempt to recover the root hash of a proof.
///
/// Requires the traversal key in order to perform proper checks &
/// the leaf hash in order to seed the path.  Returns a `ProofError`
/// if traversal was not performed correctly.
///
/// *note*: returns the value of the leaf hash in the event of an
/// empty path.
///
pub fn recover_root(key: &[u8;32], leaf: &[u8;32], path: &[NodeHash]) -> Result<[u8;32],ProofError> {
    let mut base_hash = leaf.to_owned();
    // iteratively verify proof (reverse of construction order).
    for node_hash in path.iter().rev() {
        match *node_hash {
            NodeHash::Left(ref left_hash) => {
                // calculate new base w/ node hash as left child.
                base_hash = policy::hash_pair(left_hash,&base_hash);
                // if policy would have turned left, then path is bad.
                if policy::turn_left(key,&base_hash) {
                    return Err(ProofError::Path);
                }
            },
            NodeHash::Right(ref right_hash) => {
                // calculate new base w/ node hash as right child.
                base_hash = policy::hash_pair(&base_hash,right_hash);
                // if policy would have turned right, then path is bad.
                if policy::turn_right(key,&base_hash) {
                    return Err(ProofError::Path);
                }
            },
        }
    }
    Ok(base_hash)
}

