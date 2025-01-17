//! Ergo Merkle Tree and Merkle verification tools

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
// Clippy exclusions
#![allow(clippy::unit_arg)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::panic)]

use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2b;
use ergo_chain_types::Digest32;

// Constants for hashing
/// Hash size for all nodes in [`crate::MerkleTree`], [`crate::MerkleProof`] and [`crate::BatchMerkleProof`]
pub const HASH_SIZE: usize = 32;
pub(crate) const INTERNAL_PREFIX: u8 = 1;
pub(crate) const LEAF_PREFIX: u8 = 0;

#[cfg(feature = "json")]
pub(crate) mod json;

// Unwrap is safe here since the hash is guaranteed to be 32 bytes
#[allow(clippy::unwrap_used)]
// Generates a hash of data prefixed with `prefix`
pub(crate) fn prefixed_hash(prefix: u8, data: &[u8]) -> Digest32 {
    let mut hasher = VarBlake2b::new(HASH_SIZE).unwrap();
    hasher.update(&[prefix]);
    hasher.update(data);
    let hash: Box<[u8; HASH_SIZE]> = hasher.finalize_boxed().try_into().unwrap();
    Digest32::from(hash)
}

#[allow(clippy::unwrap_used)]
// Generates a hash of data prefixed with `prefix`, allows for an optional second hash
pub(crate) fn prefixed_hash2<'a>(
    prefix: u8,
    data: impl Into<Option<&'a [u8]>>,
    data2: impl Into<Option<&'a [u8]>>,
) -> Digest32 {
    let mut hasher = VarBlake2b::new(HASH_SIZE).unwrap();
    hasher.update(&[prefix]);

    if let Some(data) = data.into() {
        hasher.update(data);
    }
    if let Some(data2) = data2.into() {
        hasher.update(data2);
    };
    let hash: Box<[u8; HASH_SIZE]> = hasher.finalize_boxed().try_into().unwrap();
    Digest32::from(hash)
}

mod batchmerkleproof;
mod merkleproof;
mod merkletree;

pub use batchmerkleproof::BatchMerkleProof;
pub use merkleproof::*;
pub use merkletree::*;
