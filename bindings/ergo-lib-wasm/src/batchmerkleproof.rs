//! BatchMerkleProof (compact Merkle multi-proofs)
use ergo_lib::ergo_merkle_tree;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// BatchMerkleProof type to validate root hash for multiple nodes
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct BatchMerkleProof(pub(crate) ergo_merkle_tree::BatchMerkleProof);

#[wasm_bindgen]
impl BatchMerkleProof {
    /// Creates a new [`BatchMerkleProof`] from json representation
    pub fn from_json(json: &JsValue) -> Result<BatchMerkleProof, String> {
        json.into_serde().map_err(|err| err.to_string())
    }
    /// Converts [`BatchMerkleProof`] to json representation
    pub fn to_json(&self) -> Result<JsValue, String> {
        JsValue::from_serde(&self).map_err(|err| err.to_string())
    }

    /// Calculates root hash for [`BatchMerkleProof`] and compares it against expected root hash
    pub fn valid(&self, expected_root: &[u8]) -> bool {
        self.0.valid(expected_root)
    }
}
