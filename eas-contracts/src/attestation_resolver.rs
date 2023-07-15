pub use attestation_resolver::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod attestation_resolver {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("eas"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IEAS"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("attest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("attest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Attestation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPayable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPayable"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multiAttest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multiAttest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestations"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Attestation[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multiRevoke"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multiRevoke"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestations"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Attestation[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revoke"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revoke"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Attestation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("toBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccessDenied"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AccessDenied"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InsufficientValue"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidEAS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidEAS"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotPayable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotPayable"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATTESTATIONRESOLVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa\x0B\xE38\x03\x80a\x0B\xE3\x839\x81\x01`@\x81\x90Ra\x000\x91a\0yV[`\x01`\x80R`\0`\xA0\x81\x90R`\xC0R\x80`\x01`\x01`\xA0\x1B\x03\x81\x16a\0gW`@QcA\xBC\x07\xFF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\xE0RPa\0\xA9V[`\0` \x82\x84\x03\x12\x15a\0\x8BW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xA2W`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\n\xFAa\0\xE9`\09`\0\x81\x81a\x04\xAC\x01Ra\x05\x1E\x01R`\0a\x01\xD2\x01R`\0a\x01\xA9\x01R`\0a\x01\x80\x01Ra\n\xFA`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80c\x99\xDD\x9B\xD7\x11a\0NW\x80c\x99\xDD\x9B\xD7\x14a\x01\x11W\x80c\xCEF\xE0F\x14a\x01?W\x80c\xE4\x96\x17\xE1\x14a\x01SW\x80c\xE6\x0C5\x05\x14a\x01fW`\0\x80\xFD[\x80cT\xFDMP\x14a\0\xB0W\x80c\x88\xE5\xB2\xD9\x14a\0\xDBW\x80c\x91\xDB\x0B~\x14a\0\xFEW`\0\x80\xFD[6a\0\xABW`@Q\x7F\x15t\xF9\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\xFD[4\x80\x15a\0\xBCW`\0\x80\xFD[Pa\0\xC5a\x01yV[`@Qa\0\xD2\x91\x90a\x07<V[`@Q\x80\x91\x03\x90\xF3[a\0\xEEa\0\xE96`\x04a\x07\xD9V[a\x02\x1CV[`@Q\x90\x15\x15\x81R` \x01a\0\xD2V[a\0\xEEa\x01\x0C6`\x04a\x07\xD9V[a\x02\xE4V[4\x80\x15a\x01\x1DW`\0\x80\xFD[Pa\x011a\x01,6`\x04a\x08tV[a\x03\x9CV[`@Q\x90\x81R` \x01a\0\xD2V[4\x80\x15a\x01KW`\0\x80\xFD[P`\0a\0\xEEV[a\0\xEEa\x01a6`\x04a\tGV[a\x03\xAFV[a\0\xEEa\x01t6`\x04a\tGV[a\x03\xC2V[``a\x01\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xD6V[a\x01\xCD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xD6V[a\x01\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xD6V[`@Q` \x01a\x02\x08\x93\x92\x91\x90a\t\x83V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x02&a\x04\x94V[\x834`\0[\x82\x81\x10\x15a\x02\xD4W`\0\x86\x86\x83\x81\x81\x10a\x02GWa\x02Ga\t\xF9V[\x90P` \x02\x015\x90P\x82\x81\x11\x15a\x02\x8AW`@Q\x7F\x11\x01\x12\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\xB7\x89\x89\x84\x81\x81\x10a\x02\x9FWa\x02\x9Fa\t\xF9V[\x90P` \x02\x81\x01\x90a\x02\xB1\x91\x90a\n(V[P`\x01\x90V[a\x02\xC8W`\0\x94PPPPPa\x02\xDCV[\x90\x91\x03\x90`\x01\x01a\x02+V[P`\x01\x92PPP[\x94\x93PPPPV[`\0a\x02\xEEa\x04\x94V[\x834`\0[\x82\x81\x10\x15a\x02\xD4W`\0\x86\x86\x83\x81\x81\x10a\x03\x0FWa\x03\x0Fa\t\xF9V[\x90P` \x02\x015\x90P\x82\x81\x11\x15a\x03RW`@Q\x7F\x11\x01\x12\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x7F\x89\x89\x84\x81\x81\x10a\x03gWa\x03ga\t\xF9V[\x90P` \x02\x81\x01\x90a\x03y\x91\x90a\n(V[\x82a\x05\x05V[a\x03\x90W`\0\x94PPPPPa\x02\xDCV[\x90\x91\x03\x90`\x01\x01a\x02\xF3V[`\0a\x03\xA8\x83\x83a\x05\xEDV[\x93\x92PPPV[`\0a\x03\xB9a\x04\x94V[`\x01[\x92\x91PPV[`\0a\x03\xCCa\x04\x94V[a\x03\xBC\x824a\x05\x05V[```\0a\x03\xE3\x83a\x066V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x03Wa\x04\x03a\x08EV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x04-W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x047WP\x93\x92PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05\x03W`@Q\x7FL\xA8\x88g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xE3\x0B\xB5ca\x05\x8Ea\x05Ta\x01 \x87\x01\x87a\nfV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x92Pa\x05\xED\x91PPV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xAC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA8\x91\x90a\n\xCBV[`\0\x81` \x01\x83Q\x10\x15a\x06-W`@Q\x7F\xB4\x12\x0F\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x01` \x01Q\x90V[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x06\x7FWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x06\xABWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x06\xC9Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x06\xE1Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x06\xF5Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x07\x07W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x03\xBCW`\x01\x01\x92\x91PPV[`\0[\x83\x81\x10\x15a\x073W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\x1BV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x07[\x81`@\x85\x01` \x87\x01a\x07\x18V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x07\x9FW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xB7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x07\xD2W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x07\xEFW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x07W`\0\x80\xFD[a\x08\x13\x88\x83\x89\x01a\x07\x8DV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x08,W`\0\x80\xFD[Pa\x089\x87\x82\x88\x01a\x07\x8DV[\x95\x98\x94\x97P\x95PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x87W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x9FW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x08\xB3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x08\xC5Wa\x08\xC5a\x08EV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\t\x0BWa\t\x0Ba\x08EV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\t$W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x93\x82\x01\x84\x01R\x98\x96\x90\x91\x015\x96PPPPPPV[`\0` \x82\x84\x03\x12\x15a\tYW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tpW`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x03\xA8W`\0\x80\xFD[`\0\x84Qa\t\x95\x81\x84` \x89\x01a\x07\x18V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\t\xD1\x81`\x01\x85\x01` \x8A\x01a\x07\x18V[`\x01\x92\x01\x91\x82\x01R\x83Qa\t\xEC\x81`\x02\x84\x01` \x88\x01a\x07\x18V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xC1\x836\x03\x01\x81\x12a\n\\W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\n\x9BW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xB6W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x07\xD2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\n\xDDW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xA8W`\0\x80\xFD\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The bytecode of the contract.
    pub static ATTESTATIONRESOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80c\x99\xDD\x9B\xD7\x11a\0NW\x80c\x99\xDD\x9B\xD7\x14a\x01\x11W\x80c\xCEF\xE0F\x14a\x01?W\x80c\xE4\x96\x17\xE1\x14a\x01SW\x80c\xE6\x0C5\x05\x14a\x01fW`\0\x80\xFD[\x80cT\xFDMP\x14a\0\xB0W\x80c\x88\xE5\xB2\xD9\x14a\0\xDBW\x80c\x91\xDB\x0B~\x14a\0\xFEW`\0\x80\xFD[6a\0\xABW`@Q\x7F\x15t\xF9\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\xFD[4\x80\x15a\0\xBCW`\0\x80\xFD[Pa\0\xC5a\x01yV[`@Qa\0\xD2\x91\x90a\x07<V[`@Q\x80\x91\x03\x90\xF3[a\0\xEEa\0\xE96`\x04a\x07\xD9V[a\x02\x1CV[`@Q\x90\x15\x15\x81R` \x01a\0\xD2V[a\0\xEEa\x01\x0C6`\x04a\x07\xD9V[a\x02\xE4V[4\x80\x15a\x01\x1DW`\0\x80\xFD[Pa\x011a\x01,6`\x04a\x08tV[a\x03\x9CV[`@Q\x90\x81R` \x01a\0\xD2V[4\x80\x15a\x01KW`\0\x80\xFD[P`\0a\0\xEEV[a\0\xEEa\x01a6`\x04a\tGV[a\x03\xAFV[a\0\xEEa\x01t6`\x04a\tGV[a\x03\xC2V[``a\x01\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xD6V[a\x01\xCD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xD6V[a\x01\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\xD6V[`@Q` \x01a\x02\x08\x93\x92\x91\x90a\t\x83V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x02&a\x04\x94V[\x834`\0[\x82\x81\x10\x15a\x02\xD4W`\0\x86\x86\x83\x81\x81\x10a\x02GWa\x02Ga\t\xF9V[\x90P` \x02\x015\x90P\x82\x81\x11\x15a\x02\x8AW`@Q\x7F\x11\x01\x12\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\xB7\x89\x89\x84\x81\x81\x10a\x02\x9FWa\x02\x9Fa\t\xF9V[\x90P` \x02\x81\x01\x90a\x02\xB1\x91\x90a\n(V[P`\x01\x90V[a\x02\xC8W`\0\x94PPPPPa\x02\xDCV[\x90\x91\x03\x90`\x01\x01a\x02+V[P`\x01\x92PPP[\x94\x93PPPPV[`\0a\x02\xEEa\x04\x94V[\x834`\0[\x82\x81\x10\x15a\x02\xD4W`\0\x86\x86\x83\x81\x81\x10a\x03\x0FWa\x03\x0Fa\t\xF9V[\x90P` \x02\x015\x90P\x82\x81\x11\x15a\x03RW`@Q\x7F\x11\x01\x12\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x7F\x89\x89\x84\x81\x81\x10a\x03gWa\x03ga\t\xF9V[\x90P` \x02\x81\x01\x90a\x03y\x91\x90a\n(V[\x82a\x05\x05V[a\x03\x90W`\0\x94PPPPPa\x02\xDCV[\x90\x91\x03\x90`\x01\x01a\x02\xF3V[`\0a\x03\xA8\x83\x83a\x05\xEDV[\x93\x92PPPV[`\0a\x03\xB9a\x04\x94V[`\x01[\x92\x91PPV[`\0a\x03\xCCa\x04\x94V[a\x03\xBC\x824a\x05\x05V[```\0a\x03\xE3\x83a\x066V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x03Wa\x04\x03a\x08EV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x04-W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x047WP\x93\x92PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05\x03W`@Q\x7FL\xA8\x88g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xE3\x0B\xB5ca\x05\x8Ea\x05Ta\x01 \x87\x01\x87a\nfV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x92Pa\x05\xED\x91PPV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xAC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA8\x91\x90a\n\xCBV[`\0\x81` \x01\x83Q\x10\x15a\x06-W`@Q\x7F\xB4\x12\x0F\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x01` \x01Q\x90V[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x06\x7FWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x06\xABWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x06\xC9Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x06\xE1Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x06\xF5Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x07\x07W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x03\xBCW`\x01\x01\x92\x91PPV[`\0[\x83\x81\x10\x15a\x073W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\x1BV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x07[\x81`@\x85\x01` \x87\x01a\x07\x18V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x07\x9FW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xB7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x07\xD2W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x07\xEFW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x07W`\0\x80\xFD[a\x08\x13\x88\x83\x89\x01a\x07\x8DV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x08,W`\0\x80\xFD[Pa\x089\x87\x82\x88\x01a\x07\x8DV[\x95\x98\x94\x97P\x95PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x87W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x9FW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x08\xB3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x08\xC5Wa\x08\xC5a\x08EV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\t\x0BWa\t\x0Ba\x08EV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\t$W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x93\x82\x01\x84\x01R\x98\x96\x90\x91\x015\x96PPPPPPV[`\0` \x82\x84\x03\x12\x15a\tYW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tpW`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x03\xA8W`\0\x80\xFD[`\0\x84Qa\t\x95\x81\x84` \x89\x01a\x07\x18V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\t\xD1\x81`\x01\x85\x01` \x8A\x01a\x07\x18V[`\x01\x92\x01\x91\x82\x01R\x83Qa\t\xEC\x81`\x02\x84\x01` \x88\x01a\x07\x18V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xC1\x836\x03\x01\x81\x12a\n\\W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\n\x9BW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xB6W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x07\xD2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\n\xDDW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xA8W`\0\x80\xFD\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The deployed bytecode of the contract.
    pub static ATTESTATIONRESOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AttestationResolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AttestationResolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AttestationResolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AttestationResolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AttestationResolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AttestationResolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AttestationResolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ATTESTATIONRESOLVER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ATTESTATIONRESOLVER_ABI.clone(),
                ATTESTATIONRESOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `attest` (0xe60c3505) function
        pub fn attest(
            &self,
            attestation: Attestation,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([230, 12, 53, 5], (attestation,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPayable` (0xce46e046) function
        pub fn is_payable(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([206, 70, 224, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiAttest` (0x91db0b7e) function
        pub fn multi_attest(
            &self,
            attestations: ::std::vec::Vec<Attestation>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 219, 11, 126], (attestations, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiRevoke` (0x88e5b2d9) function
        pub fn multi_revoke(
            &self,
            attestations: ::std::vec::Vec<Attestation>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([136, 229, 178, 217], (attestations, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revoke` (0xe49617e1) function
        pub fn revoke(
            &self,
            attestation: Attestation,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 150, 23, 225], (attestation,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toBytes32` (0x99dd9bd7) function
        pub fn to_bytes_32(
            &self,
            data: ::ethers::core::types::Bytes,
            start: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([153, 221, 155, 215], (data, start))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AttestationResolver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccessDenied` with signature `AccessDenied()` and selector `0x4ca88867`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AccessDenied", abi = "AccessDenied()")]
    pub struct AccessDenied;
    ///Custom Error type `InsufficientValue` with signature `InsufficientValue()` and selector `0x11011294`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InsufficientValue", abi = "InsufficientValue()")]
    pub struct InsufficientValue;
    ///Custom Error type `InvalidEAS` with signature `InvalidEAS()` and selector `0x83780ffe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidEAS", abi = "InvalidEAS()")]
    pub struct InvalidEAS;
    ///Custom Error type `NotPayable` with signature `NotPayable()` and selector `0x1574f9f3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotPayable", abi = "NotPayable()")]
    pub struct NotPayable;
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AttestationResolverErrors {
        AccessDenied(AccessDenied),
        InsufficientValue(InsufficientValue),
        InvalidEAS(InvalidEAS),
        NotPayable(NotPayable),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationResolverErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <AccessDenied as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AccessDenied(decoded));
            }
            if let Ok(decoded)
                = <InsufficientValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InsufficientValue(decoded));
            }
            if let Ok(decoded)
                = <InvalidEAS as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidEAS(decoded));
            }
            if let Ok(decoded)
                = <NotPayable as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotPayable(decoded));
            }
            if let Ok(decoded)
                = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationResolverErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessDenied(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEAS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotPayable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AttestationResolverErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessDenied as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InsufficientValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEAS as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotPayable as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AttestationResolverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessDenied(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEAS(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AttestationResolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessDenied> for AttestationResolverErrors {
        fn from(value: AccessDenied) -> Self {
            Self::AccessDenied(value)
        }
    }
    impl ::core::convert::From<InsufficientValue> for AttestationResolverErrors {
        fn from(value: InsufficientValue) -> Self {
            Self::InsufficientValue(value)
        }
    }
    impl ::core::convert::From<InvalidEAS> for AttestationResolverErrors {
        fn from(value: InvalidEAS) -> Self {
            Self::InvalidEAS(value)
        }
    }
    impl ::core::convert::From<NotPayable> for AttestationResolverErrors {
        fn from(value: NotPayable) -> Self {
            Self::NotPayable(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for AttestationResolverErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    ///Container type for all input parameters for the `attest` function with signature `attest((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes))` and selector `0xe60c3505`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "attest",
        abi = "attest((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes))"
    )]
    pub struct AttestCall {
        pub attestation: Attestation,
    }
    ///Container type for all input parameters for the `isPayable` function with signature `isPayable()` and selector `0xce46e046`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isPayable", abi = "isPayable()")]
    pub struct IsPayableCall;
    ///Container type for all input parameters for the `multiAttest` function with signature `multiAttest((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes)[],uint256[])` and selector `0x91db0b7e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "multiAttest",
        abi = "multiAttest((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes)[],uint256[])"
    )]
    pub struct MultiAttestCall {
        pub attestations: ::std::vec::Vec<Attestation>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `multiRevoke` function with signature `multiRevoke((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes)[],uint256[])` and selector `0x88e5b2d9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "multiRevoke",
        abi = "multiRevoke((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes)[],uint256[])"
    )]
    pub struct MultiRevokeCall {
        pub attestations: ::std::vec::Vec<Attestation>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `revoke` function with signature `revoke((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes))` and selector `0xe49617e1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "revoke",
        abi = "revoke((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes))"
    )]
    pub struct RevokeCall {
        pub attestation: Attestation,
    }
    ///Container type for all input parameters for the `toBytes32` function with signature `toBytes32(bytes,uint256)` and selector `0x99dd9bd7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "toBytes32", abi = "toBytes32(bytes,uint256)")]
    pub struct ToBytes32Call {
        pub data: ::ethers::core::types::Bytes,
        pub start: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AttestationResolverCalls {
        Attest(AttestCall),
        IsPayable(IsPayableCall),
        MultiAttest(MultiAttestCall),
        MultiRevoke(MultiRevokeCall),
        Revoke(RevokeCall),
        ToBytes32(ToBytes32Call),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationResolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AttestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Attest(decoded));
            }
            if let Ok(decoded)
                = <IsPayableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPayable(decoded));
            }
            if let Ok(decoded)
                = <MultiAttestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MultiAttest(decoded));
            }
            if let Ok(decoded)
                = <MultiRevokeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MultiRevoke(decoded));
            }
            if let Ok(decoded)
                = <RevokeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Revoke(decoded));
            }
            if let Ok(decoded)
                = <ToBytes32Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ToBytes32(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationResolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Attest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsPayable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiAttest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiRevoke(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Revoke(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ToBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AttestationResolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Attest(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiAttest(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiRevoke(element) => ::core::fmt::Display::fmt(element, f),
                Self::Revoke(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestCall> for AttestationResolverCalls {
        fn from(value: AttestCall) -> Self {
            Self::Attest(value)
        }
    }
    impl ::core::convert::From<IsPayableCall> for AttestationResolverCalls {
        fn from(value: IsPayableCall) -> Self {
            Self::IsPayable(value)
        }
    }
    impl ::core::convert::From<MultiAttestCall> for AttestationResolverCalls {
        fn from(value: MultiAttestCall) -> Self {
            Self::MultiAttest(value)
        }
    }
    impl ::core::convert::From<MultiRevokeCall> for AttestationResolverCalls {
        fn from(value: MultiRevokeCall) -> Self {
            Self::MultiRevoke(value)
        }
    }
    impl ::core::convert::From<RevokeCall> for AttestationResolverCalls {
        fn from(value: RevokeCall) -> Self {
            Self::Revoke(value)
        }
    }
    impl ::core::convert::From<ToBytes32Call> for AttestationResolverCalls {
        fn from(value: ToBytes32Call) -> Self {
            Self::ToBytes32(value)
        }
    }
    impl ::core::convert::From<VersionCall> for AttestationResolverCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `attest` function with signature `attest((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes))` and selector `0xe60c3505`
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
    pub struct AttestReturn(pub bool);
    ///Container type for all return fields from the `isPayable` function with signature `isPayable()` and selector `0xce46e046`
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
    pub struct IsPayableReturn(pub bool);
    ///Container type for all return fields from the `multiAttest` function with signature `multiAttest((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes)[],uint256[])` and selector `0x91db0b7e`
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
    pub struct MultiAttestReturn(pub bool);
    ///Container type for all return fields from the `multiRevoke` function with signature `multiRevoke((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes)[],uint256[])` and selector `0x88e5b2d9`
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
    pub struct MultiRevokeReturn(pub bool);
    ///Container type for all return fields from the `revoke` function with signature `revoke((bytes32,bytes32,uint64,uint64,uint64,bytes32,address,address,bool,bytes))` and selector `0xe49617e1`
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
    pub struct RevokeReturn(pub bool);
    ///Container type for all return fields from the `toBytes32` function with signature `toBytes32(bytes,uint256)` and selector `0x99dd9bd7`
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
    pub struct ToBytes32Return(pub [u8; 32]);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::std::string::String);
}
