//! Bindings for NiPoPow

use super::block_header::BlockId;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

use crate::{
    block_header::{BlockHeader, BlockHeaders},
    error_conversion::to_js,
};

/// A structure representing NiPoPow proof.
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct NipopowProof(ergo_lib::ergo_nipopow::NipopowProof);

#[wasm_bindgen]
impl NipopowProof {
    /// Implementation of the ≥ algorithm from [`KMZ17`], see Algorithm 4
    ///
    /// [`KMZ17`]: https://fc20.ifca.ai/preproceedings/74.pdf
    pub fn is_better_than(&self, that: &NipopowProof) -> Result<bool, JsValue> {
        self.0.is_better_than(&that.0).map_err(to_js)
    }

    /// JSON representation as text
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(to_js)
    }

    /// Get suffix head
    pub fn suffix_head(&self) -> PoPowHeader {
        self.0.suffix_head.clone().into()
    }

    /// Parse from JSON
    /// supports Ergo Node/Explorer API and box values and token amount encoded as strings
    pub fn from_json(json: &str) -> Result<NipopowProof, JsValue> {
        serde_json::from_str(json).map(Self).map_err(to_js)
    }
}

/// A verifier for PoPoW proofs. During its lifetime, it processes many proofs with the aim of
/// deducing at any given point what is the best (sub)chain rooted at the specified genesis.
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct NipopowVerifier(ergo_lib::ergo_nipopow::NipopowVerifier);

#[wasm_bindgen]
impl NipopowVerifier {
    /// Create new instance
    #[wasm_bindgen(constructor)]
    pub fn new(genesis_block_id: BlockId) -> NipopowVerifier {
        ergo_lib::ergo_nipopow::NipopowVerifier::new(genesis_block_id.0).into()
    }

    /// Return best proof
    pub fn best_proof(&self) -> Option<NipopowProof> {
        self.0.best_proof().map(NipopowProof)
    }

    /// Returns chain of `BlockHeader`s from the best proof.
    pub fn best_chain(&self) -> BlockHeaders {
        BlockHeaders(self.0.best_chain().into_iter().map(|h| h.into()).collect())
    }

    /// Process given proof
    pub fn process(&mut self, new_proof: NipopowProof) -> Result<(), JsValue> {
        self.0.process(new_proof.0).map_err(to_js)
    }
}
/// PoPowHeader structure. Represents the block header and unpacked interlinks
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct PoPowHeader(ergo_lib::ergo_nipopow::PoPowHeader);

#[wasm_bindgen]
impl PoPowHeader {
    /// Returns block header
    pub fn header(&self) -> BlockHeader {
        self.0.header.clone().into()
    }
    /// Returns interlinks for PoPowHeader
    pub fn interlinks(&self) -> Result<JsValue, JsError> {
        serde_json::to_string(&self.0.interlinks)
            .map(Into::into)
            .map_err(Into::into)
    }
    /// Returns interlinks proof [`crate::batchmerkleproof::BatchMerkleProof`]
    pub fn interlinks_proof(&self) -> crate::batchmerkleproof::BatchMerkleProof {
        crate::batchmerkleproof::BatchMerkleProof(self.0.interlinks_proof.clone())
    }
    /// Validates interlinks merkle root with compact merkle multiproof. See [`PoPowHeader::interlinks_proof`] for BatchMerkleProof access
    pub fn check_interlinks_proof(&self) -> bool {
        self.0.check_interlinks_proof()
    }

    /// Returns block height for Header
    pub fn height(&self) -> u32 {
        self.0.header.height
    }
    /// Returns Block ID for Header
    pub fn id(&self) -> crate::block_header::BlockId {
        self.0.header.id.clone().into()
    }
}
