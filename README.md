# EAS-RS

Rust utilities for the ethereum attestation service.

# Repository Structure

- `eas-contracts`: Foundry generated bindings for the current version of the EAS contracts.
- `src`: EAS-RS library.

# How To Use

Add this to your crate dependencies:

```
eas-rs = { git = "https://github.com/brech1/eas-rs" }
```

Then you can import:

```rust
use eas::{
    eas::*,
    eas_contracts::{
        eas::AttestedFilter,
        value_resolver::{Attestation, AttestationRequest, AttestationRequestData},
    },
    schema_registry::SchemaRegistryContract,
};
```

You should then instantiate a local signer with a wallet:

```rust
let wallet = MnemonicBuilder::<English>::default()
    .phrase(mnemonic)
    .build();

let signer = SignerMiddleware::new(provider, wallet.with_chain_id(CHAIN_ID));
```

You can then attest:

```rust
let eas = Eas::new(
    signer.clone(),
    Some(Address::from_str(EAS_CONTRACT_ADDRESS).unwrap()),
);

let att_data: Bytes = Bytes::from(data_bytes);

let att_object = AttestationRequestData {
    recipient: Address::zero(),
    expiration_time: 0,
    revocable: false,
    ref_uid: [0u8; 32],
    data: att_data,
    value: U256::zero(),
};

let att = AttestationRequest {
    schema: SCHEMA_ID,
    data: att_object,
};

eas.attest(att).await.unwrap();
```

Keep in mind the above snippets are just illustrative, for a full implementation you should check out [this example project](https://github.com/brech1/zk-attestation) which includes:

- Deploying on a local testnet.
- Submitting schema.
- Attesting.
- Fetching attestations with filter.

## Contributing

Contributions are welcome!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
