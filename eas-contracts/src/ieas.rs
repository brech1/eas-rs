pub use ieas::*;
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
pub mod ieas {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("attest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("attest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("request"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AttestationRequest",
                                        ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("attestByDelegation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("attestByDelegation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegatedRequest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DelegatedAttestationRequest",
                                        ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAttestation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAttestation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRevokeOffchain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRevokeOffchain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("revoker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSchemaRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSchemaRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISchemaRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTimestamp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isAttestationValid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isAttestationValid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                    name: ::std::borrow::ToOwned::to_owned("multiRequests"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MultiAttestationRequest[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multiAttestByDelegation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "multiAttestByDelegation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "multiDelegatedRequests",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MultiDelegatedAttestationRequest[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                                    name: ::std::borrow::ToOwned::to_owned("multiRequests"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MultiRevocationRequest[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multiRevokeByDelegation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "multiRevokeByDelegation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "multiDelegatedRequests",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MultiDelegatedRevocationRequest[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multiRevokeOffchain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "multiRevokeOffchain",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multiTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multiTimestamp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::borrow::ToOwned::to_owned("request"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct RevocationRequest"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeByDelegation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeByDelegation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegatedRequest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DelegatedRevocationRequest",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeOffchain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeOffchain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("timestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("timestamp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Attested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Attested"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("attester"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("uid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("schema"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Revoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Revoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("attester"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("uid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("schema"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RevokedOffchain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RevokedOffchain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("revoker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Timestamped"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Timestamped"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IEAS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IEAS<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IEAS<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IEAS<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IEAS<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IEAS<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IEAS)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IEAS<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IEAS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `attest` (0xf17325e7) function
        pub fn attest(
            &self,
            request: AttestationRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([241, 115, 37, 231], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `attestByDelegation` (0xe13458fc) function
        pub fn attest_by_delegation(
            &self,
            delegated_request: DelegatedAttestationRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([225, 52, 88, 252], (delegated_request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAttestation` (0xa3112a64) function
        pub fn get_attestation(
            &self,
            uid: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Attestation> {
            self.0
                .method_hash([163, 17, 42, 100], uid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRevokeOffchain` (0xb469318d) function
        pub fn get_revoke_offchain(
            &self,
            revoker: ::ethers::core::types::Address,
            data: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([180, 105, 49, 141], (revoker, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSchemaRegistry` (0xf10b5cc8) function
        pub fn get_schema_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([241, 11, 92, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTimestamp` (0xd45c4435) function
        pub fn get_timestamp(
            &self,
            data: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([212, 92, 68, 53], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAttestationValid` (0xe30bb563) function
        pub fn is_attestation_valid(
            &self,
            uid: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([227, 11, 181, 99], uid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiAttest` (0x44adc90e) function
        pub fn multi_attest(
            &self,
            multi_requests: ::std::vec::Vec<MultiAttestationRequest>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([68, 173, 201, 14], multi_requests)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiAttestByDelegation` (0x831e05a1) function
        pub fn multi_attest_by_delegation(
            &self,
            multi_delegated_requests: ::std::vec::Vec<MultiDelegatedAttestationRequest>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([131, 30, 5, 161], multi_delegated_requests)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiRevoke` (0x4cb7e9e5) function
        pub fn multi_revoke(
            &self,
            multi_requests: ::std::vec::Vec<MultiRevocationRequest>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 183, 233, 229], multi_requests)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiRevokeByDelegation` (0xe45d03f9) function
        pub fn multi_revoke_by_delegation(
            &self,
            multi_delegated_requests: ::std::vec::Vec<MultiDelegatedRevocationRequest>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 93, 3, 249], multi_delegated_requests)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiRevokeOffchain` (0x13893f61) function
        pub fn multi_revoke_offchain(
            &self,
            data: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([19, 137, 63, 97], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiTimestamp` (0xe71ff365) function
        pub fn multi_timestamp(
            &self,
            data: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([231, 31, 243, 101], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revoke` (0x46926267) function
        pub fn revoke(
            &self,
            request: RevocationRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 146, 98, 103], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeByDelegation` (0xe57a6b1b) function
        pub fn revoke_by_delegation(
            &self,
            delegated_request: DelegatedRevocationRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 122, 107, 27], (delegated_request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeOffchain` (0xcf190f34) function
        pub fn revoke_offchain(
            &self,
            data: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([207, 25, 15, 52], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timestamp` (0x4d003070) function
        pub fn timestamp(
            &self,
            data: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([77, 0, 48, 112], data)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Attested` event
        pub fn attested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AttestedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Revoked` event
        pub fn revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RevokedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RevokedOffchain` event
        pub fn revoked_offchain_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RevokedOffchainFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Timestamped` event
        pub fn timestamped_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TimestampedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IEASEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IEAS<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Attested", abi = "Attested(address,address,bytes32,bytes32)")]
    pub struct AttestedFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub attester: ::ethers::core::types::Address,
        pub uid: [u8; 32],
        #[ethevent(indexed)]
        pub schema: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Revoked", abi = "Revoked(address,address,bytes32,bytes32)")]
    pub struct RevokedFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub attester: ::ethers::core::types::Address,
        pub uid: [u8; 32],
        #[ethevent(indexed)]
        pub schema: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RevokedOffchain",
        abi = "RevokedOffchain(address,bytes32,uint64)"
    )]
    pub struct RevokedOffchainFilter {
        #[ethevent(indexed)]
        pub revoker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub data: [u8; 32],
        #[ethevent(indexed)]
        pub timestamp: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Timestamped", abi = "Timestamped(bytes32,uint64)")]
    pub struct TimestampedFilter {
        #[ethevent(indexed)]
        pub data: [u8; 32],
        #[ethevent(indexed)]
        pub timestamp: u64,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IEASEvents {
        AttestedFilter(AttestedFilter),
        RevokedFilter(RevokedFilter),
        RevokedOffchainFilter(RevokedOffchainFilter),
        TimestampedFilter(TimestampedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IEASEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AttestedFilter::decode_log(log) {
                return Ok(IEASEvents::AttestedFilter(decoded));
            }
            if let Ok(decoded) = RevokedFilter::decode_log(log) {
                return Ok(IEASEvents::RevokedFilter(decoded));
            }
            if let Ok(decoded) = RevokedOffchainFilter::decode_log(log) {
                return Ok(IEASEvents::RevokedOffchainFilter(decoded));
            }
            if let Ok(decoded) = TimestampedFilter::decode_log(log) {
                return Ok(IEASEvents::TimestampedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IEASEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokedOffchainFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TimestampedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestedFilter> for IEASEvents {
        fn from(value: AttestedFilter) -> Self {
            Self::AttestedFilter(value)
        }
    }
    impl ::core::convert::From<RevokedFilter> for IEASEvents {
        fn from(value: RevokedFilter) -> Self {
            Self::RevokedFilter(value)
        }
    }
    impl ::core::convert::From<RevokedOffchainFilter> for IEASEvents {
        fn from(value: RevokedOffchainFilter) -> Self {
            Self::RevokedOffchainFilter(value)
        }
    }
    impl ::core::convert::From<TimestampedFilter> for IEASEvents {
        fn from(value: TimestampedFilter) -> Self {
            Self::TimestampedFilter(value)
        }
    }
    ///Container type for all input parameters for the `attest` function with signature `attest((bytes32,(address,uint64,bool,bytes32,bytes,uint256)))` and selector `0xf17325e7`
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
        abi = "attest((bytes32,(address,uint64,bool,bytes32,bytes,uint256)))"
    )]
    pub struct AttestCall {
        pub request: AttestationRequest,
    }
    ///Container type for all input parameters for the `attestByDelegation` function with signature `attestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address))` and selector `0xe13458fc`
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
        name = "attestByDelegation",
        abi = "attestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address))"
    )]
    pub struct AttestByDelegationCall {
        pub delegated_request: DelegatedAttestationRequest,
    }
    ///Container type for all input parameters for the `getAttestation` function with signature `getAttestation(bytes32)` and selector `0xa3112a64`
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
    #[ethcall(name = "getAttestation", abi = "getAttestation(bytes32)")]
    pub struct GetAttestationCall {
        pub uid: [u8; 32],
    }
    ///Container type for all input parameters for the `getRevokeOffchain` function with signature `getRevokeOffchain(address,bytes32)` and selector `0xb469318d`
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
    #[ethcall(name = "getRevokeOffchain", abi = "getRevokeOffchain(address,bytes32)")]
    pub struct GetRevokeOffchainCall {
        pub revoker: ::ethers::core::types::Address,
        pub data: [u8; 32],
    }
    ///Container type for all input parameters for the `getSchemaRegistry` function with signature `getSchemaRegistry()` and selector `0xf10b5cc8`
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
    #[ethcall(name = "getSchemaRegistry", abi = "getSchemaRegistry()")]
    pub struct GetSchemaRegistryCall;
    ///Container type for all input parameters for the `getTimestamp` function with signature `getTimestamp(bytes32)` and selector `0xd45c4435`
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
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(bytes32)")]
    pub struct GetTimestampCall {
        pub data: [u8; 32],
    }
    ///Container type for all input parameters for the `isAttestationValid` function with signature `isAttestationValid(bytes32)` and selector `0xe30bb563`
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
    #[ethcall(name = "isAttestationValid", abi = "isAttestationValid(bytes32)")]
    pub struct IsAttestationValidCall {
        pub uid: [u8; 32],
    }
    ///Container type for all input parameters for the `multiAttest` function with signature `multiAttest((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[])[])` and selector `0x44adc90e`
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
        abi = "multiAttest((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[])[])"
    )]
    pub struct MultiAttestCall {
        pub multi_requests: ::std::vec::Vec<MultiAttestationRequest>,
    }
    ///Container type for all input parameters for the `multiAttestByDelegation` function with signature `multiAttestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address)[])` and selector `0x831e05a1`
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
        name = "multiAttestByDelegation",
        abi = "multiAttestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address)[])"
    )]
    pub struct MultiAttestByDelegationCall {
        pub multi_delegated_requests: ::std::vec::Vec<MultiDelegatedAttestationRequest>,
    }
    ///Container type for all input parameters for the `multiRevoke` function with signature `multiRevoke((bytes32,(bytes32,uint256)[])[])` and selector `0x4cb7e9e5`
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
        abi = "multiRevoke((bytes32,(bytes32,uint256)[])[])"
    )]
    pub struct MultiRevokeCall {
        pub multi_requests: ::std::vec::Vec<MultiRevocationRequest>,
    }
    ///Container type for all input parameters for the `multiRevokeByDelegation` function with signature `multiRevokeByDelegation((bytes32,(bytes32,uint256)[],(uint8,bytes32,bytes32)[],address)[])` and selector `0xe45d03f9`
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
        name = "multiRevokeByDelegation",
        abi = "multiRevokeByDelegation((bytes32,(bytes32,uint256)[],(uint8,bytes32,bytes32)[],address)[])"
    )]
    pub struct MultiRevokeByDelegationCall {
        pub multi_delegated_requests: ::std::vec::Vec<MultiDelegatedRevocationRequest>,
    }
    ///Container type for all input parameters for the `multiRevokeOffchain` function with signature `multiRevokeOffchain(bytes32[])` and selector `0x13893f61`
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
    #[ethcall(name = "multiRevokeOffchain", abi = "multiRevokeOffchain(bytes32[])")]
    pub struct MultiRevokeOffchainCall {
        pub data: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `multiTimestamp` function with signature `multiTimestamp(bytes32[])` and selector `0xe71ff365`
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
    #[ethcall(name = "multiTimestamp", abi = "multiTimestamp(bytes32[])")]
    pub struct MultiTimestampCall {
        pub data: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `revoke` function with signature `revoke((bytes32,(bytes32,uint256)))` and selector `0x46926267`
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
    #[ethcall(name = "revoke", abi = "revoke((bytes32,(bytes32,uint256)))")]
    pub struct RevokeCall {
        pub request: RevocationRequest,
    }
    ///Container type for all input parameters for the `revokeByDelegation` function with signature `revokeByDelegation((bytes32,(bytes32,uint256),(uint8,bytes32,bytes32),address))` and selector `0xe57a6b1b`
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
        name = "revokeByDelegation",
        abi = "revokeByDelegation((bytes32,(bytes32,uint256),(uint8,bytes32,bytes32),address))"
    )]
    pub struct RevokeByDelegationCall {
        pub delegated_request: DelegatedRevocationRequest,
    }
    ///Container type for all input parameters for the `revokeOffchain` function with signature `revokeOffchain(bytes32)` and selector `0xcf190f34`
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
    #[ethcall(name = "revokeOffchain", abi = "revokeOffchain(bytes32)")]
    pub struct RevokeOffchainCall {
        pub data: [u8; 32],
    }
    ///Container type for all input parameters for the `timestamp` function with signature `timestamp(bytes32)` and selector `0x4d003070`
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
    #[ethcall(name = "timestamp", abi = "timestamp(bytes32)")]
    pub struct TimestampCall {
        pub data: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IEASCalls {
        Attest(AttestCall),
        AttestByDelegation(AttestByDelegationCall),
        GetAttestation(GetAttestationCall),
        GetRevokeOffchain(GetRevokeOffchainCall),
        GetSchemaRegistry(GetSchemaRegistryCall),
        GetTimestamp(GetTimestampCall),
        IsAttestationValid(IsAttestationValidCall),
        MultiAttest(MultiAttestCall),
        MultiAttestByDelegation(MultiAttestByDelegationCall),
        MultiRevoke(MultiRevokeCall),
        MultiRevokeByDelegation(MultiRevokeByDelegationCall),
        MultiRevokeOffchain(MultiRevokeOffchainCall),
        MultiTimestamp(MultiTimestampCall),
        Revoke(RevokeCall),
        RevokeByDelegation(RevokeByDelegationCall),
        RevokeOffchain(RevokeOffchainCall),
        Timestamp(TimestampCall),
    }
    impl ::ethers::core::abi::AbiDecode for IEASCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AttestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Attest(decoded));
            }
            if let Ok(decoded)
                = <AttestByDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AttestByDelegation(decoded));
            }
            if let Ok(decoded)
                = <GetAttestationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAttestation(decoded));
            }
            if let Ok(decoded)
                = <GetRevokeOffchainCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRevokeOffchain(decoded));
            }
            if let Ok(decoded)
                = <GetSchemaRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSchemaRegistry(decoded));
            }
            if let Ok(decoded)
                = <GetTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTimestamp(decoded));
            }
            if let Ok(decoded)
                = <IsAttestationValidCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAttestationValid(decoded));
            }
            if let Ok(decoded)
                = <MultiAttestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MultiAttest(decoded));
            }
            if let Ok(decoded)
                = <MultiAttestByDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MultiAttestByDelegation(decoded));
            }
            if let Ok(decoded)
                = <MultiRevokeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MultiRevoke(decoded));
            }
            if let Ok(decoded)
                = <MultiRevokeByDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MultiRevokeByDelegation(decoded));
            }
            if let Ok(decoded)
                = <MultiRevokeOffchainCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MultiRevokeOffchain(decoded));
            }
            if let Ok(decoded)
                = <MultiTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MultiTimestamp(decoded));
            }
            if let Ok(decoded)
                = <RevokeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Revoke(decoded));
            }
            if let Ok(decoded)
                = <RevokeByDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevokeByDelegation(decoded));
            }
            if let Ok(decoded)
                = <RevokeOffchainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeOffchain(decoded));
            }
            if let Ok(decoded)
                = <TimestampCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Timestamp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IEASCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Attest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AttestByDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAttestation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRevokeOffchain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSchemaRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAttestationValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiAttest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiAttestByDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiRevoke(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiRevokeByDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiRevokeOffchain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Revoke(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeByDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeOffchain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Timestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IEASCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Attest(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestByDelegation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAttestation(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRevokeOffchain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSchemaRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAttestationValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MultiAttest(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiAttestByDelegation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MultiRevoke(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiRevokeByDelegation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MultiRevokeOffchain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MultiTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Revoke(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeByDelegation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeOffchain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Timestamp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestCall> for IEASCalls {
        fn from(value: AttestCall) -> Self {
            Self::Attest(value)
        }
    }
    impl ::core::convert::From<AttestByDelegationCall> for IEASCalls {
        fn from(value: AttestByDelegationCall) -> Self {
            Self::AttestByDelegation(value)
        }
    }
    impl ::core::convert::From<GetAttestationCall> for IEASCalls {
        fn from(value: GetAttestationCall) -> Self {
            Self::GetAttestation(value)
        }
    }
    impl ::core::convert::From<GetRevokeOffchainCall> for IEASCalls {
        fn from(value: GetRevokeOffchainCall) -> Self {
            Self::GetRevokeOffchain(value)
        }
    }
    impl ::core::convert::From<GetSchemaRegistryCall> for IEASCalls {
        fn from(value: GetSchemaRegistryCall) -> Self {
            Self::GetSchemaRegistry(value)
        }
    }
    impl ::core::convert::From<GetTimestampCall> for IEASCalls {
        fn from(value: GetTimestampCall) -> Self {
            Self::GetTimestamp(value)
        }
    }
    impl ::core::convert::From<IsAttestationValidCall> for IEASCalls {
        fn from(value: IsAttestationValidCall) -> Self {
            Self::IsAttestationValid(value)
        }
    }
    impl ::core::convert::From<MultiAttestCall> for IEASCalls {
        fn from(value: MultiAttestCall) -> Self {
            Self::MultiAttest(value)
        }
    }
    impl ::core::convert::From<MultiAttestByDelegationCall> for IEASCalls {
        fn from(value: MultiAttestByDelegationCall) -> Self {
            Self::MultiAttestByDelegation(value)
        }
    }
    impl ::core::convert::From<MultiRevokeCall> for IEASCalls {
        fn from(value: MultiRevokeCall) -> Self {
            Self::MultiRevoke(value)
        }
    }
    impl ::core::convert::From<MultiRevokeByDelegationCall> for IEASCalls {
        fn from(value: MultiRevokeByDelegationCall) -> Self {
            Self::MultiRevokeByDelegation(value)
        }
    }
    impl ::core::convert::From<MultiRevokeOffchainCall> for IEASCalls {
        fn from(value: MultiRevokeOffchainCall) -> Self {
            Self::MultiRevokeOffchain(value)
        }
    }
    impl ::core::convert::From<MultiTimestampCall> for IEASCalls {
        fn from(value: MultiTimestampCall) -> Self {
            Self::MultiTimestamp(value)
        }
    }
    impl ::core::convert::From<RevokeCall> for IEASCalls {
        fn from(value: RevokeCall) -> Self {
            Self::Revoke(value)
        }
    }
    impl ::core::convert::From<RevokeByDelegationCall> for IEASCalls {
        fn from(value: RevokeByDelegationCall) -> Self {
            Self::RevokeByDelegation(value)
        }
    }
    impl ::core::convert::From<RevokeOffchainCall> for IEASCalls {
        fn from(value: RevokeOffchainCall) -> Self {
            Self::RevokeOffchain(value)
        }
    }
    impl ::core::convert::From<TimestampCall> for IEASCalls {
        fn from(value: TimestampCall) -> Self {
            Self::Timestamp(value)
        }
    }
    ///Container type for all return fields from the `attest` function with signature `attest((bytes32,(address,uint64,bool,bytes32,bytes,uint256)))` and selector `0xf17325e7`
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
    pub struct AttestReturn(pub [u8; 32]);
    ///Container type for all return fields from the `attestByDelegation` function with signature `attestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address))` and selector `0xe13458fc`
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
    pub struct AttestByDelegationReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getAttestation` function with signature `getAttestation(bytes32)` and selector `0xa3112a64`
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
    pub struct GetAttestationReturn(pub Attestation);
    ///Container type for all return fields from the `getRevokeOffchain` function with signature `getRevokeOffchain(address,bytes32)` and selector `0xb469318d`
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
    pub struct GetRevokeOffchainReturn(pub u64);
    ///Container type for all return fields from the `getSchemaRegistry` function with signature `getSchemaRegistry()` and selector `0xf10b5cc8`
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
    pub struct GetSchemaRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getTimestamp` function with signature `getTimestamp(bytes32)` and selector `0xd45c4435`
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
    pub struct GetTimestampReturn(pub u64);
    ///Container type for all return fields from the `isAttestationValid` function with signature `isAttestationValid(bytes32)` and selector `0xe30bb563`
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
    pub struct IsAttestationValidReturn(pub bool);
    ///Container type for all return fields from the `multiAttest` function with signature `multiAttest((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[])[])` and selector `0x44adc90e`
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
    pub struct MultiAttestReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `multiAttestByDelegation` function with signature `multiAttestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address)[])` and selector `0x831e05a1`
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
    pub struct MultiAttestByDelegationReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `multiRevokeOffchain` function with signature `multiRevokeOffchain(bytes32[])` and selector `0x13893f61`
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
    pub struct MultiRevokeOffchainReturn(pub u64);
    ///Container type for all return fields from the `multiTimestamp` function with signature `multiTimestamp(bytes32[])` and selector `0xe71ff365`
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
    pub struct MultiTimestampReturn(pub u64);
    ///Container type for all return fields from the `revokeOffchain` function with signature `revokeOffchain(bytes32)` and selector `0xcf190f34`
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
    pub struct RevokeOffchainReturn(pub u64);
    ///Container type for all return fields from the `timestamp` function with signature `timestamp(bytes32)` and selector `0x4d003070`
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
    pub struct TimestampReturn(pub u64);
}
