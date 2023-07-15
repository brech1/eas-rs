use std::sync::Arc;

pub use eas_contracts;

use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

/// Signer type alias.
pub type Signer = SignerMiddleware<Provider<Http>, LocalWallet>;

/// EAS struct.
pub struct Eas {
    signer: Arc<Signer>,
}

impl Eas {
    /// Creates new EAS instance.
    pub fn new(signer: Signer) -> Self {
        Self {
            signer: Arc::new(signer),
        }
    }

    /// Deploys contracts
    pub async fn deploy() {}

    /// Gets signer
    pub fn signer(&self) -> Arc<Signer> {
        self.signer.clone()
    }

    pub fn create_signer() -> Signer {
        todo!()
    }
}
