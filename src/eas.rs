use eas_contracts::{
    eas,
    value_resolver::{AttestationRequest, DelegatedAttestationRequest},
};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};
use log::info;
use std::fmt::Result as FmtResult;
use std::sync::Arc;

// Signer type alias.
pub type Signer = SignerMiddleware<Provider<Http>, LocalWallet>;

/// The Eas struct encapsulates a signer and an instance of an EAS.
pub struct Eas {
    /// The signer used by the Eas instance.
    signer: Arc<Signer>,
    /// The EAS contract.
    eas: eas::EAS<Signer>,
}

// Implementation block for the Eas struct.
impl Eas {
    /// Constructs a new Eas instance.
    ///
    /// # Parameters
    ///
    /// * `signer`: An instance of the Signer.
    ///
    /// # Returns
    ///
    /// A new Eas instance.
    pub fn new(signer: Signer) -> Self {
        let address = signer.address();
        let signer_arc = Arc::new(signer);

        Self {
            signer: signer_arc.clone(),
            eas: eas::EAS::new(address, signer_arc),
        }
    }

    /// Deploys the EAS.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the EAS was successfully deployed, else an Err.
    pub async fn deploy(&self) -> FmtResult {
        let deployment = eas::EAS::deploy(self.signer.clone(), ()).unwrap();
        let address = deployment.send().await.unwrap().address();

        info!("EAS deployed at: {:?}", address);
        Ok(())
    }

    /// Returns the signer used by the Eas instance.
    ///
    /// # Returns
    ///
    /// An Arc pointing to the Signer instance.
    pub fn signer(&self) -> Arc<Signer> {
        self.signer.clone()
    }

    /// Performs an attestation with the provided request.
    ///
    /// # Parameters
    ///
    /// * `request`: The AttestationRequest to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the attestation was successful, else an Err.
    pub async fn attest(&self, request: AttestationRequest) -> FmtResult {
        let call = self.eas.attest(request);
        let res = call.send().await.unwrap();

        info!("Attestation result: {:?}", res);
        Ok(())
    }

    /// Performs an attestation by delegation with the provided request.
    ///
    /// # Parameters
    ///
    /// * `delegated_request`: The DelegatedAttestationRequest to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the attestation by delegation was successful, else an Err.
    pub async fn attest_by_delegation(
        &self,
        delegated_request: DelegatedAttestationRequest,
    ) -> FmtResult {
        let call = self.eas.attest_by_delegation(delegated_request);
        let res = call.send().await.unwrap();

        info!("Attestation by delegation result: {:?}", res);
        Ok(())
    }

    /// Retrieves the attestation type hash.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the attestation type hash was successfully retrieved, else an Err.
    pub async fn get_attest_type_hash(&self) -> FmtResult {
        let result = self.eas.get_attest_type_hash().call().await.unwrap();

        info!("Attestation type hash: {:?}", result);
        Ok(())
    }

    /// Retrieves an attestation for the provided UID.
    ///
    /// # Parameters
    ///
    /// * `uid`: The UID for which to retrieve the attestation.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the attestation was successfully retrieved, else an Err.
    pub async fn get_attestation(&self, uid: [u8; 32]) -> FmtResult {
        let result = self.eas.get_attestation(uid).call().await.unwrap();

        info!("Attestation: {:?}", result);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::eas::*;
    use eas_contracts::value_resolver::{AttestationRequest, DelegatedAttestationRequest};
    use ethers::{
        middleware::SignerMiddleware,
        providers::{Http, Provider},
        signers::LocalWallet,
        utils::Anvil,
    };
    use std::convert::TryFrom;
    use std::sync::Arc;

    const TEST_MNEMONIC: &'static str =
        "test test test test test test test test test test test junk";
    const TEST_NODE_URL: &'static str = "http://localhost:8545";

    async fn setup_eas() -> Eas {
        let wallet = LocalWallet::try_from(TEST_MNEMONIC).unwrap();
        let provider = Provider::<Http>::try_from(TEST_NODE_URL).unwrap();
        let signer = SignerMiddleware::new(provider, wallet);
        Eas::new(signer)
    }

    #[tokio::test]
    async fn test_eas_new() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        assert_eq!(2, Arc::strong_count(&eas.signer()));
    }

    #[tokio::test]
    async fn test_eas_deploy() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        assert!(eas.deploy().await.is_ok());
    }

    #[tokio::test]
    async fn test_eas_attest() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let attestation_request = AttestationRequest {
            // Replace with your actual fields
            field1: "test data".into(),
            field2: 42,
        };
        assert!(eas.attest(attestation_request).await.is_ok());
    }

    #[tokio::test]
    async fn test_eas_attest_by_delegation() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let delegated_attestation_request = DelegatedAttestationRequest {
            // Replace with your actual fields
            field1: "test data".into(),
            field2: 42,
        };
        assert!(eas
            .attest_by_delegation(delegated_attestation_request)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_eas_get_attest_type_hash() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        assert!(eas.get_attest_type_hash().await.is_ok());
    }

    #[tokio::test]
    async fn test_eas_get_attestation() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let uid = [0u8; 32]; // This is a placeholder. Replace with actual uid.
        assert!(eas.get_attestation(uid).await.is_ok());
    }
}
