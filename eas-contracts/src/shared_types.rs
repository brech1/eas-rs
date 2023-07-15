///`Attestation(bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Attestation {
    pub uid: [u8; 32],
    pub schema: [u8; 32],
    pub time: u64,
    pub expiration_time: u64,
    pub revocation_time: u64,
    pub ref_uid: [u8; 32],
    pub recipient: ::ethers::core::types::Address,
    pub attester: ::ethers::core::types::Address,
    pub revocable: bool,
    pub data: ::ethers::core::types::Bytes,
}
///`AttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct AttestationRequest {
    pub schema: [u8; 32],
    pub data: AttestationRequestData,
}
///`AttestationRequestData(address,uint64,bool,bytes32,bytes,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct AttestationRequestData {
    pub recipient: ::ethers::core::types::Address,
    pub expiration_time: u64,
    pub revocable: bool,
    pub ref_uid: [u8; 32],
    pub data: ::ethers::core::types::Bytes,
    pub value: ::ethers::core::types::U256,
}
///`DelegatedAttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DelegatedAttestationRequest {
    pub schema: [u8; 32],
    pub data: AttestationRequestData,
    pub signature: Eip712Signature,
    pub attester: ::ethers::core::types::Address,
}
///`DelegatedProxyAttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DelegatedProxyAttestationRequest {
    pub schema: [u8; 32],
    pub data: AttestationRequestData,
    pub signature: Eip712Signature,
    pub attester: ::ethers::core::types::Address,
    pub deadline: u64,
}
///`DelegatedProxyRevocationRequest(bytes32,(bytes32,uint256),(uint8,bytes32,bytes32),address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DelegatedProxyRevocationRequest {
    pub schema: [u8; 32],
    pub data: RevocationRequestData,
    pub signature: Eip712Signature,
    pub revoker: ::ethers::core::types::Address,
    pub deadline: u64,
}
///`DelegatedRevocationRequest(bytes32,(bytes32,uint256),(uint8,bytes32,bytes32),address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DelegatedRevocationRequest {
    pub schema: [u8; 32],
    pub data: RevocationRequestData,
    pub signature: Eip712Signature,
    pub revoker: ::ethers::core::types::Address,
}
///`Eip712Signature(uint8,bytes32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Eip712Signature {
    pub v: u8,
    pub r: [u8; 32],
    pub s: [u8; 32],
}
///`MultiAttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256)[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MultiAttestationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<AttestationRequestData>,
}
///`MultiDelegatedAttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MultiDelegatedAttestationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<AttestationRequestData>,
    pub signatures: ::std::vec::Vec<Eip712Signature>,
    pub attester: ::ethers::core::types::Address,
}
///`MultiDelegatedProxyAttestationRequest(bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MultiDelegatedProxyAttestationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<AttestationRequestData>,
    pub signatures: ::std::vec::Vec<Eip712Signature>,
    pub attester: ::ethers::core::types::Address,
    pub deadline: u64,
}
///`MultiDelegatedProxyRevocationRequest(bytes32,(bytes32,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MultiDelegatedProxyRevocationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<RevocationRequestData>,
    pub signatures: ::std::vec::Vec<Eip712Signature>,
    pub revoker: ::ethers::core::types::Address,
    pub deadline: u64,
}
///`MultiDelegatedRevocationRequest(bytes32,(bytes32,uint256)[],(uint8,bytes32,bytes32)[],address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MultiDelegatedRevocationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<RevocationRequestData>,
    pub signatures: ::std::vec::Vec<Eip712Signature>,
    pub revoker: ::ethers::core::types::Address,
}
///`MultiRevocationRequest(bytes32,(bytes32,uint256)[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MultiRevocationRequest {
    pub schema: [u8; 32],
    pub data: ::std::vec::Vec<RevocationRequestData>,
}
///`RevocationRequest(bytes32,(bytes32,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct RevocationRequest {
    pub schema: [u8; 32],
    pub data: RevocationRequestData,
}
///`RevocationRequestData(bytes32,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct RevocationRequestData {
    pub uid: [u8; 32],
    pub value: ::ethers::core::types::U256,
}
///`SchemaRecord(bytes32,address,bool,string)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SchemaRecord {
    pub uid: [u8; 32],
    pub resolver: ::ethers::core::types::Address,
    pub revocable: bool,
    pub schema: ::std::string::String,
}
