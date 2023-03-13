use derive_more::Constructor;
use serde::{Deserialize, Serialize};

// Contructor generates a new instance of the struct using a (new) fn
#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        Self.0
    }
}
