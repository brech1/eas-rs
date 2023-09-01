use eas_contracts::{
    eas::EAS,
    value_resolver::{
        AttestationRequest, AttestationRequestData, DelegatedAttestationRequest,
        DelegatedRevocationRequest, MultiAttestationRequest, MultiDelegatedAttestationRequest,
        MultiDelegatedRevocationRequest, MultiRevocationRequest, RevocationRequest,
    },
};
use ethers::{
    middleware::SignerMiddleware,
    prelude::ContractError,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::{Address, Bytes, U256},
};
use log::info;
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
    /// If no EAS contract address is provided, the zero address is used.
    /// The address will be updated when the EAS is deployed.
    ///
    /// # Parameters
    ///
    /// * `signer`: An instance of the Signer.
    /// * `eas_address`: The address of the EAS contract.
    ///
    /// # Returns
    ///
    /// A new Eas instance.
    pub fn new(signer: Signer, eas_address: Option<Address>) -> Self {
        let signer_arc = Arc::new(signer);

        Self {
            signer: signer_arc.clone(),
            eas: EAS::new(eas_address.unwrap_or(Address::zero()), signer_arc),
        }
    }

    /// Returns the signer used by the Eas instance.
    ///
    /// # Returns
    ///
    /// An Arc pointing to the Signer instance.
    pub fn signer(&self) -> Arc<Signer> {
        self.signer.clone()
    }

    /// Returns the EAS contract.
    ///
    /// # Returns
    ///
    /// An instance of the EAS contract.
    pub fn eas(&self) -> EAS<Signer> {
        self.eas.clone()
    }

    /// Deploys the EAS.
    /// This function will update the EAS instance with the deployed contract address.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the EAS was successfully deployed, else an Err.
    pub async fn deploy(
        &mut self,
        schema_registry: Address,
    ) -> Result<Address, ContractError<Signer>> {
        let deployment = EAS::deploy(self.signer.clone(), schema_registry)?;
        let address = deployment.send().await?.address();

        // Update the EAS instance with the deployed contract address.
        self.eas = EAS::new(address, self.signer.clone());

        Ok(address)
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
    pub async fn attest(&self, request: AttestationRequest) -> Result<(), ContractError<Signer>> {
        let call = self.eas.attest(request);
        let res = call.send().await?;

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
    ) -> Result<(), ContractError<Signer>> {
        let call = self.eas.attest_by_delegation(delegated_request);
        let res = call.send().await?;

        info!("Attestation by delegation result: {:?}", res);
        Ok(())
    }

    /// Retrieves the attestation type hash.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the attestation type hash was successfully retrieved, else an Err.
    pub async fn get_attest_type_hash(&self) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_attest_type_hash();
        let res = call.send().await?;

        info!("Attestation type hash: {:?}", res);
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
    pub async fn get_attestation(&self, uid: [u8; 32]) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_attestation(uid);
        let res = call.send().await?;

        info!("Attestation: {:?}", res);
        Ok(())
    }

    /// Retrieves the domain separator of the EAS contract.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the domain separator was successfully retrieved, else an Err.
    pub async fn get_domain_separator(&self) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_domain_separator();
        let res = call.send().await?;

        info!("Domain separator: {:?}", res);
        Ok(())
    }

    /// Retrieves the name of the EAS contract.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the name was successfully retrieved, else an Err.
    pub async fn get_name(&self) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_name();
        let res = call.send().await?;

        info!("Name: {:?}", res);
        Ok(())
    }

    /// Retrieves the nonce of the provided account.
    ///
    /// # Parameters
    ///
    /// * `account`: The account for which to retrieve the nonce.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the nonce was successfully retrieved, else an Err.
    pub async fn get_nonce(
        &self,
        account: ::ethers::core::types::Address,
    ) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_nonce(account);
        let res = call.send().await?;

        info!("Nonce: {:?}", res);
        Ok(())
    }

    /// Retrieves the offchain revocation for the provided revoker and data.
    ///
    /// # Parameters
    ///
    /// * `revoker`: The revoker for which to retrieve the offchain revocation.
    /// * `data`: The data for which to retrieve the offchain revocation.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the offchain revocation was successfully retrieved, else an Err.
    pub async fn get_revoke_offchain(
        &self,
        revoker: ::ethers::core::types::Address,
        data: [u8; 32],
    ) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_revoke_offchain(revoker, data);
        let res = call.send().await?;

        info!("Revoke offchain: {:?}", res);
        Ok(())
    }

    /// Retrieves the revocation type hash of the EAS contract.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the revocation type hash was successfully retrieved, else an Err.
    pub async fn get_revoke_type_hash(&self) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_revoke_type_hash();
        let res = call.send().await?;

        info!("Revoke type hash: {:?}", res);
        Ok(())
    }

    /// Retrieves the schema registry of the EAS contract.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the schema registry was successfully retrieved, else an Err.
    pub async fn get_schema_registry(&self) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_schema_registry();
        let res = call.send().await?;

        info!("Schema registry: {:?}", res);
        Ok(())
    }

    /// Retrieves the timestamp for the provided data.
    ///
    /// # Parameters
    ///
    /// * `data`: The data for which to retrieve the timestamp.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the timestamp was successfully retrieved, else an Err.
    pub async fn get_timestamp(&self, data: [u8; 32]) -> Result<(), ContractError<Signer>> {
        let call = self.eas.get_timestamp(data);
        let res = call.send().await?;

        info!("Timestamp: {:?}", res);
        Ok(())
    }

    /// Checks if the attestation for the provided UID is valid.
    ///
    /// # Parameters
    ///
    /// * `uid`: The UID for which to check the attestation.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the attestation was successfully checked, else an Err.
    pub async fn is_attestation_valid(&self, uid: [u8; 32]) -> Result<(), ContractError<Signer>> {
        let call = self.eas.is_attestation_valid(uid);
        let res = call.send().await?;

        info!("Is attestation valid: {:?}", res);
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
    pub async fn multi_attest(
        &self,
        requests: Vec<MultiAttestationRequest>,
    ) -> Result<(), ContractError<Signer>> {
        let call = self.eas.multi_attest(requests);
        let res = call.send().await?;

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
    ) -> Result<(), ContractError<Signer>> {
        let call = self
            .eas
            .multi_attest_by_delegation(multi_delegated_requests);
        let res = call.send().await?;

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
    pub async fn multi_revoke(
        &self,
        multi_requests: Vec<MultiRevocationRequest>,
    ) -> Result<(), ContractError<Signer>> {
        let call = self.eas.multi_revoke(multi_requests);
        let res = call.send().await?;

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
    ) -> Result<(), ContractError<Signer>> {
        let call = self
            .eas
            .multi_revoke_by_delegation(multi_delegated_requests);
        let res = call.send().await?;

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
    pub async fn multi_revoke_offchain(
        &self,
        data: Vec<[u8; 32]>,
    ) -> Result<(), ContractError<Signer>> {
        let call = self.eas.multi_revoke_offchain(data);
        let res = call.send().await?;

        info!("Multi off-chain revocation result: {:?}", res);
        Ok(())
    }

    /// Performs multiple timestamp actions with the provided data.
    ///
    /// # Parameters
    ///
    /// * `data`: The vector of 32-byte arrays to be timestamped.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the multiple timestamps were successful, else an Err.
    pub async fn multi_timestamp(&self, data: Vec<[u8; 32]>) -> Result<(), ContractError<Signer>> {
        let call = self.eas.multi_timestamp(data);
        let res = call.send().await?;

        info!("Multi timestamp result: {:?}", res);
        Ok(())
    }

    /// Revokes an attestation with the provided request.
    ///
    /// # Parameters
    ///
    /// * `request`: The RevocationRequest to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the revocation was successful, else an Err.
    pub async fn revoke(&self, request: RevocationRequest) -> Result<(), ContractError<Signer>> {
        let call = self.eas.revoke(request);
        let res = call.send().await?;

        info!("Revoke result: {:?}", res);
        Ok(())
    }

    /// Revokes an attestation by delegation with the provided request.
    ///
    /// # Parameters
    ///
    /// * `delegated_request`: The DelegatedRevocationRequest to be sent.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the revocation by delegation was successful, else an Err.
    pub async fn revoke_by_delegation(
        &self,
        delegated_request: DelegatedRevocationRequest,
    ) -> Result<(), ContractError<Signer>> {
        let call = self.eas.revoke_by_delegation(delegated_request);
        let res = call.send().await?;

        info!("Revoke by delegation result: {:?}", res);
        Ok(())
    }

    /// Revokes an attestation off-chain with the provided data.
    ///
    /// # Parameters
    ///
    /// * `data`: The 32-byte array to be revoked off-chain.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the off-chain revocation was successful, else an Err.
    pub async fn revoke_offchain(&self, data: [u8; 32]) -> Result<(), ContractError<Signer>> {
        let call = self.eas.revoke_offchain(data);
        let res = call.send().await?;

        info!("Revoke offchain result: {:?}", res);
        Ok(())
    }

    /// Performs a timestamp action with the provided data.
    ///
    /// # Parameters
    ///
    /// * `data`: The 32-byte array to be timestamped.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the timestamp was successful, else an Err.
    pub async fn timestamp(&self, data: [u8; 32]) -> Result<(), ContractError<Signer>> {
        let call = self.eas.timestamp(data);
        let res = call.send().await?;

        info!("Timestamp result: {:?}", res);
        Ok(())
    }

    /// Retrieves the version of the EAS contract.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the version was successfully retrieved, else an Err.
    pub async fn version(&self) -> Result<(), ContractError<Signer>> {
        let call = self.eas.version();
        let res = call.send().await?;

        info!("Version: {:?}", res);
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
        recipient: Address,
        expiration_time: u64,
        revocable: bool,
        ref_uid: [u8; 32],
        data: Bytes,
        value: U256,
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

    async fn setup_eas(node_url: &str) -> Eas {
        let provider = Provider::<Http>::try_from(node_url).unwrap();
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(TEST_MNEMONIC)
            .build()
            .unwrap();
        let signer = SignerMiddleware::new(provider, wallet.with_chain_id(31337u64));

        Eas::new(signer, None)
    }

    #[tokio::test]
    async fn test_eas_new() {
        let anvil = Anvil::new().spawn();
        setup_eas(anvil.endpoint().as_str()).await;
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_deploy() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);

        let res = eas.deploy(schema_registry).await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_attest() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
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

        let res = eas.attest(attestation_request).await;
        res.unwrap();

        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_attest_by_delegation() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
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

        let res = eas
            .attest_by_delegation(delegated_attestation_request)
            .await;

        res.unwrap();
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_attest_type_hash() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let res = eas.get_attest_type_hash().await;
        res.unwrap();

        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_attestation() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let uid = [0u8; 32];

        let res = eas.get_attestation(uid).await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_domain_separator() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let res = eas.get_domain_separator().await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_name() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let _res = eas.get_name().await.unwrap();

        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_nonce() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let account = Address::zero();

        let res = eas.get_nonce(account).await;

        res.unwrap();

        // assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_revoke_offchain() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let revoker = Address::zero();
        let data = [0u8; 32];

        let res = eas.get_revoke_offchain(revoker, data).await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_revoke_type_hash() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let res = eas.get_revoke_type_hash().await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_schema_registry() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let res = eas.get_schema_registry().await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_get_timestamp() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = [0u8; 32];

        let res = eas.get_timestamp(data).await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_is_attestation_valid() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let uid = [0u8; 32];

        let res = eas.is_attestation_valid(uid).await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_multi_attest() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
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

        let multi_attestation_request = MultiAttestationRequest {
            schema: [0; 32],
            data: vec![data],
        };

        let res = eas.multi_attest(vec![multi_attestation_request]).await;
        res.unwrap();

        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_multi_attest_by_delegation() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
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

        let multi_delegated_attestation_request = MultiDelegatedAttestationRequest {
            schema: [0; 32],
            data: vec![data],
            signatures: vec![Eip712Signature::default()],
            attester: Address::zero(),
        };

        let res = eas
            .multi_attest_by_delegation(vec![multi_delegated_attestation_request])
            .await;
        res.unwrap();

        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_multi_revoke() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let revocation_request_data = RevocationRequestData {
            uid: [0; 32],
            value: U256::default(),
        };

        let multi_revocation_request = MultiRevocationRequest {
            schema: [0; 32],
            data: vec![revocation_request_data],
        };

        let res = eas.multi_revoke(vec![multi_revocation_request]).await;
        res.unwrap();

        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_multi_revoke_by_delegation() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
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

        let res = eas
            .multi_revoke_by_delegation(vec![multi_delegated_revocation_request])
            .await;
        res.unwrap();

        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_multi_revoke_offchain() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = [0u8; 32];

        assert!(eas.multi_revoke_offchain(vec![data]).await.is_ok());
        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_multi_timestamp() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = vec![[0; 32]; 10];
        let res = eas.multi_timestamp(data).await;
        res.unwrap();

        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_revoke() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let request = RevocationRequest::default();

        let res = eas.revoke(request).await;
        res.unwrap();

        drop(anvil);
    }

    #[ignore]
    #[tokio::test]
    async fn test_eas_revoke_by_delegation() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let delegated_request = DelegatedRevocationRequest::default();
        let res = eas.revoke_by_delegation(delegated_request).await;
        res.unwrap();

        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_revoke_offchain() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = [0; 32];
        let res = eas.revoke_offchain(data).await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_timestamp() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let data = [0; 32];
        let res = eas.timestamp(data).await;

        assert!(res.is_ok());
        drop(anvil);
    }

    #[tokio::test]
    async fn test_eas_version() {
        let anvil = Anvil::new().spawn();
        let mut eas = setup_eas(anvil.endpoint().as_str()).await;
        let schema_registry = Address::from([0x42; 20]);
        eas.deploy(schema_registry).await.unwrap();

        let res = eas.version().await;

        assert!(res.is_ok());
        drop(anvil);
    }
}
