use eas_contracts::{
    eas::EAS,
    value_resolver::{
        AttestationRequest, AttestationRequestData, DelegatedAttestationRequest,
        MultiAttestationRequest, MultiDelegatedAttestationRequest, MultiDelegatedRevocationRequest,
        MultiRevocationRequest,
    },
};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::Address,
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
    eas: EAS<Signer>,
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
            eas: EAS::new(address, signer_arc),
        }
    }

    /// Deploys the EAS.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the EAS was successfully deployed, else an Err.
    pub async fn deploy(&self, schema_registry: Address) -> FmtResult {
        let deployment = EAS::deploy(self.signer.clone(), schema_registry).unwrap();
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

    /// Performs multiple attestations with the provided request.
    ///
    /// # Parameters
    ///
    /// * `requests`: The vector of MultiAttestationRequest to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the multiple attestations were successful, else an Err.
    pub async fn multi_attest(&self, requests: Vec<MultiAttestationRequest>) -> FmtResult {
        let call = self.eas.multi_attest(requests);
        let res = call.send().await.unwrap();

        info!("Multi attestation result: {:?}", res);
        Ok(())
    }

    /// Performs multiple attestations by delegation with the provided request.
    ///
    /// # Parameters
    ///
    /// * `multi_delegated_requests`: The vector of MultiDelegatedAttestationRequest to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the multiple attestations by delegation were successful, else an Err.
    pub async fn multi_attest_by_delegation(
        &self,
        multi_delegated_requests: Vec<MultiDelegatedAttestationRequest>,
    ) -> FmtResult {
        let call = self
            .eas
            .multi_attest_by_delegation(multi_delegated_requests);
        let res = call.send().await.unwrap();

        info!("Multi attestation by delegation result: {:?}", res);
        Ok(())
    }

    /// Revokes multiple attestations with the provided request.
    ///
    /// # Parameters
    ///
    /// * `multi_requests`: The vector of MultiRevocationRequest to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the multiple revocations were successful, else an Err.
    pub async fn multi_revoke(&self, multi_requests: Vec<MultiRevocationRequest>) -> FmtResult {
        let call = self.eas.multi_revoke(multi_requests);
        let res = call.send().await.unwrap();

        info!("Multi revocation result: {:?}", res);
        Ok(())
    }

    /// Revokes multiple attestations by delegation with the provided request.
    ///
    /// # Parameters
    ///
    /// * `multi_delegated_requests`: The vector of MultiDelegatedRevocationRequest to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the multiple revocations by delegation were successful, else an Err.
    pub async fn multi_revoke_by_delegation(
        &self,
        multi_delegated_requests: Vec<MultiDelegatedRevocationRequest>,
    ) -> FmtResult {
        let call = self
            .eas
            .multi_revoke_by_delegation(multi_delegated_requests);
        let res = call.send().await.unwrap();

        info!("Multi revocation by delegation result: {:?}", res);
        Ok(())
    }

    /// Revokes multiple attestations off-chain with the provided data.
    ///
    /// # Parameters
    ///
    /// * `data`: The vector of 32-byte arrays to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the multiple off-chain revocations were successful, else an Err.
    pub async fn multi_revoke_offchain(&self, data: Vec<[u8; 32]>) -> FmtResult {
        let call = self.eas.multi_revoke_offchain(data);
        let res = call.send().await.unwrap();

        info!("Multi off-chain revocation result: {:?}", res);
        Ok(())
    }

    /// Constructs and returns the attestation data.
    ///
    /// # Parameters
    ///
    /// * `recipient`: The recipient of the attestation.
    /// * `expiration_time`: The expiration time of the attestation.
    /// * `revocable`: Whether the attestation is revocable.
    /// * `ref_uid`: The reference UID of the attestation.
    /// * `data`: The data of the attestation.
    /// * `value`: The value of the attestation.
    ///
    /// # Returns
    ///
    /// An instance of AttestationRequestData.
    pub fn get_attestation_data(
        recipient: ::ethers::core::types::Address,
        expiration_time: u64,
        revocable: bool,
        ref_uid: [u8; 32],
        data: ::ethers::core::types::Bytes,
        value: ::ethers::core::types::U256,
    ) -> AttestationRequestData {
        AttestationRequestData {
            recipient,
            expiration_time,
            revocable,
            ref_uid,
            data,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eas::*;
    use eas_contracts::value_resolver::{
        AttestationRequest, DelegatedAttestationRequest, Eip712Signature, RevocationRequestData,
    };
    use ethers::{
        middleware::SignerMiddleware,
        providers::{Http, Provider},
        signers::{coins_bip39::English, MnemonicBuilder, Signer},
        types::{Address, U256},
        utils::Anvil,
    };
    use std::convert::TryFrom;

    const TEST_MNEMONIC: &'static str =
        "test test test test test test test test test test test junk";
    const TEST_NODE_URL: &'static str = "http://localhost:8545";

    async fn setup_eas() -> Eas {
        let provider = Provider::<Http>::try_from(TEST_NODE_URL).unwrap();
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(TEST_MNEMONIC)
            .build()
            .unwrap();
        let signer = SignerMiddleware::new(provider, wallet.with_chain_id(31337u64));

        Eas::new(signer)
    }

    #[tokio::test]
    async fn test_eas_new() {
        setup_eas().await;
    }

    #[tokio::test]
    async fn test_eas_deploy() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let schema_registry = Address::from([0x42; 20]);

        assert!(eas.deploy(schema_registry).await.is_ok());
    }

    #[tokio::test]
    async fn test_eas_attest() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = Eas::get_attestation_data(
            Address::zero(),
            0,
            false,
            [0u8; 32],
            [0u8; 32].into(),
            0.into(),
        );

        let attestation_request = AttestationRequest {
            schema: [0; 32],
            data,
        };

        assert!(eas.attest(attestation_request).await.is_ok());
    }

    #[tokio::test]
    async fn test_eas_attest_by_delegation() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let schema_registry: ethers::types::H160 = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = Eas::get_attestation_data(
            Address::zero(),
            0,
            false,
            [0u8; 32],
            [0u8; 32].into(),
            0.into(),
        );

        let delegated_attestation_request = DelegatedAttestationRequest {
            schema: [0; 32],
            data,
            signature: Eip712Signature::default(),
            attester: Address::zero(),
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

    #[tokio::test]
    async fn test_eas_multi_attest() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let schema_registry: ethers::types::H160 = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = Eas::get_attestation_data(
            Address::zero(),
            0,
            false,
            [0u8; 32],
            [0u8; 32].into(),
            0.into(),
        );

        let multi_attestation_request = MultiAttestationRequest {
            schema: [0; 32],
            data: vec![data],
        };

        assert!(eas
            .multi_attest(vec![multi_attestation_request])
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_eas_multi_attest_by_delegation() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let schema_registry: ethers::types::H160 = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = Eas::get_attestation_data(
            Address::zero(),
            0,
            false,
            [0u8; 32],
            [0u8; 32].into(),
            0.into(),
        );

        let multi_delegated_attestation_request = MultiDelegatedAttestationRequest {
            schema: [0; 32],
            data: vec![data],
            signatures: vec![Eip712Signature::default()],
            attester: Address::zero(),
        };

        assert!(eas
            .multi_attest_by_delegation(vec![multi_delegated_attestation_request])
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_eas_multi_revoke() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let schema_registry: ethers::types::H160 = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let revocation_request_data = RevocationRequestData {
            uid: [0; 32],
            value: U256::default(),
        };

        let multi_revocation_request = MultiRevocationRequest {
            schema: [0; 32],
            data: vec![revocation_request_data],
        };

        assert!(eas
            .multi_revoke(vec![multi_revocation_request])
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_eas_multi_revoke_by_delegation() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let schema_registry: ethers::types::H160 = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let revocation_request_data = RevocationRequestData {
            uid: [0; 32],
            value: U256::default(),
        };

        let multi_delegated_revocation_request = MultiDelegatedRevocationRequest {
            schema: [0; 32],
            data: vec![revocation_request_data],
            signatures: vec![Eip712Signature::default()],
            revoker: Address::zero(),
        };

        assert!(eas
            .multi_revoke_by_delegation(vec![multi_delegated_revocation_request])
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_eas_multi_revoke_offchain() {
        let _anvil = Anvil::new().spawn();
        let eas = setup_eas().await;
        let schema_registry: ethers::types::H160 = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = [0u8; 32]; // This is a placeholder. Replace with actual data.

        assert!(eas.multi_revoke_offchain(vec![data]).await.is_ok());
    }
}
