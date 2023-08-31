use crate::eas::Signer;
use alloy_sol_types::SolType;
use eas_contracts::{schema_registry::SchemaRegistry, value_resolver::SchemaRecord};
use ethers::{prelude::ContractError, providers::ProviderError, types::Address};
use log::info;
use std::sync::Arc;

pub struct SchemaField<T: SolType> {
    _name: String,
    _sol_type: T,
}

/// The SchemaRegistryContract struct encapsulates a signer and an instance of a Schema Registry.
pub struct SchemaRegistryContract {
    /// The signer used by the SchemaRegistryContract instance.
    signer: Arc<Signer>,
    /// The Schema Registry contract.
    schema_registry: SchemaRegistry<Signer>,
}

/// Implementation block for the SchemaRegistry
impl SchemaRegistryContract {
    /// Constructs a new SchemaRegistryContract instance.
    /// If no contract address is provided, the zero address is used.
    /// The address will be updated when the SchemaRegistry is deployed.
    ///
    /// # Parameters
    ///
    /// * `signer`: An instance of the Signer.
    /// * `sr_address`: The address of the SchemaRegistry contract.
    ///
    /// # Returns
    ///
    /// A new SchemaRegistryContract instance.
    pub fn new(signer: Signer, sr_address: Option<Address>) -> Self {
        let signer_arc = Arc::new(signer);

        Self {
            signer: signer_arc.clone(),
            schema_registry: SchemaRegistry::new(sr_address.unwrap_or(Address::zero()), signer_arc),
        }
    }

    /// Returns the signer used by the SchemaRegistry instance.
    ///
    /// # Returns
    ///
    /// An Arc pointing to the Signer instance.
    pub fn signer(&self) -> Arc<Signer> {
        self.signer.clone()
    }

    /// Returns the SchemaRegistryContract instance.
    ///
    /// # Returns
    ///
    /// An instance of SchemaRegistryContract.
    pub fn contract(&self) -> SchemaRegistry<Signer> {
        self.schema_registry.clone()
    }

    /// Deploys the Schema Registry.
    /// This function will update the SchemaRegistryContract instance with the deployed contract address.
    ///
    /// # Returns
    ///
    /// A Result which is an Ok if the registry was successfully deployed, else an Err.
    pub async fn deploy(&mut self, schema_registry: Address) -> Result<(), ContractError<Signer>> {
        let deployment = SchemaRegistry::deploy(self.signer.clone(), schema_registry)?;
        let address = deployment.send().await?.address();

        // Update the Schema Registry instance with the deployed contract address.
        self.schema_registry = SchemaRegistry::new(address, self.signer.clone());

        info!("Schema Registry deployed at: {:?}", address);
        Ok(())
    }

    /// Registers a new schema on the Schema Registry.
    ///
    /// # Parameters
    ///
    /// * `schema`: The definition or representation of the schema.
    /// * `resolver`: The address of the resolver contract.
    /// * `revocable`: A flag indicating whether the schema can be revoked.
    ///
    /// # Returns
    ///
    /// A Result containing the unique identifier (ID) of the registered schema if successful, or a ContractError if the operation fails.
    pub async fn register_schema(
        &self,
        schema: String,
        resolver: Address,
        revocable: bool,
    ) -> Result<[u8; 32], ContractError<Signer>> {
        let register_call = self.schema_registry.register(schema, resolver, revocable);
        let tx = register_call.send().await?;

        match tx.await? {
            Some(receipt) => {
                if receipt.status == Some(1.into()) {
                    let schema_id = receipt
                        .logs
                        .first()
                        .ok_or(ContractError::ProviderError {
                            e: ProviderError::CustomError(
                                "No logs found in the receipt".to_string(),
                            ),
                        })?
                        .topics
                        .get(1)
                        .ok_or(ContractError::ProviderError {
                            e: ProviderError::CustomError("Missing expected log topic".to_string()),
                        })?
                        .as_fixed_bytes();

                    Ok(*schema_id)
                } else {
                    Err(ContractError::ProviderError {
                        e: ProviderError::CustomError("Transaction failed".to_string()),
                    })
                }
            }
            None => Err(ContractError::ProviderError {
                e: ProviderError::CustomError("No receipt found for the transaction".to_string()),
            }),
        }
    }

    /// Retrieves the details of a schema given its unique identifier.
    ///
    /// # Parameters
    ///
    /// * `schema_id`: The unique identifier of the schema to be retrieved.
    ///
    /// # Returns
    ///
    /// A Result containing the details of the requested schema if found, or a ContractError if the operation fails.
    pub async fn get_schema(
        &self,
        schema_id: [u8; 32],
    ) -> Result<SchemaRecord, ContractError<Signer>> {
        self.schema_registry.get_schema(schema_id).call().await
    }
}
