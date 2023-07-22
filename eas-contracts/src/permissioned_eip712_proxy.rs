pub use permissioned_eip712_proxy::*;
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
pub mod permissioned_eip712_proxy {
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
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DelegatedProxyAttestationRequest",
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
                    ::std::borrow::ToOwned::to_owned("getAttestTypeHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAttestTypeHash"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getAttester"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAttester"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDomainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDomainSeparator"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEAS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEAS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IEAS"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getName"),
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
                (
                    ::std::borrow::ToOwned::to_owned("getRevokeTypeHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRevokeTypeHash"),
                            inputs: ::std::vec![],
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
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MultiDelegatedProxyAttestationRequest[]",
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
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MultiDelegatedProxyRevocationRequest[]",
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DelegatedProxyRevocationRequest",
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
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
                    ::std::borrow::ToOwned::to_owned("DeadlineExpired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DeadlineExpired"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidLength"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSignature"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotFound"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UsedSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UsedSignature"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PERMISSIONEDEIP712PROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xC0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0-|8\x03\x80b\0-|\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01\xDCV[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rd\x03\x02\xE3\x12\xE3`\xDC\x1B` \x80\x83\x01\x91\x82R`\0`\x80\x81\x90R`\x01`\xA0R`\xC0R\x83Q\x90\x84\x01 \x82Q\x90\x91 a\x01@\x82\x90Ra\x01`\x81\x90RFa\x01\0R\x84\x92\x84\x92\x83\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fb\0\0\xF6\x81\x84\x84`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\xE0R0a\x01 Ra\x01\x80RPPP`\x01`\x01`\xA0\x1B\x03\x83\x16\x90Pb\0\x01/W`@QcA\xBC\x07\xFF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x01\xA0R`\0b\0\x01K\x82\x82b\0\x03aV[PPPb\0\x01hb\0\x01bb\0\x01p` \x1B` \x1CV[b\0\x01tV[PPb\0\x04-V[3\x90V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01\xF0W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\x08W`\0\x80\xFD[` \x84\x81\x01Q\x91\x93P\x90`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02(W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12b\0\x02=W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x02RWb\0\x02Rb\0\x01\xC6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x02}Wb\0\x02}b\0\x01\xC6V[\x81`@R\x82\x81R\x89\x86\x84\x87\x01\x01\x11\x15b\0\x02\x96W`\0\x80\xFD[`\0\x93P[\x82\x84\x10\x15b\0\x02\xBAW\x84\x84\x01\x86\x01Q\x81\x85\x01\x87\x01R\x92\x85\x01\x92b\0\x02\x9BV[`\0\x86\x84\x83\x01\x01R\x80\x96PPPPPPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02\xE7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x03\x08WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x03\\W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x037WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x03XW\x82\x81U`\x01\x01b\0\x03CV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03}Wb\0\x03}b\0\x01\xC6V[b\0\x03\x95\x81b\0\x03\x8E\x84Tb\0\x02\xD2V[\x84b\0\x03\x0EV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03\xCDW`\0\x84\x15b\0\x03\xB4WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x03XV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03\xFEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03\xDDV[P\x85\x82\x10\x15b\0\x04\x1DW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa(\xB7b\0\x04\xC5`\09`\0\x81\x81a\x01\xFA\x01R\x81\x81a\x08a\x01R\x81\x81a\x08\xE6\x01R\x81\x81a\x0E\x06\x01Ra\x0F\xE1\x01R`\0a\x11_\x01R`\0a\x11\xAE\x01R`\0a\x11\x89\x01R`\0a\x10\xE2\x01R`\0a\x11\x0C\x01R`\0a\x116\x01R`\0a\x04g\x01R`\0a\x04>\x01R`\0a\x04\x15\x01Ra(\xB7`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xDDW`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x7FW\x80c\xA6\xD4\xDB\xC7\x11a\0YW\x80c\xA6\xD4\xDB\xC7\x14a\x02~W\x80c\xB80\x10\xD3\x14a\x02\x91W\x80c\xED$\x91\x1D\x14a\x02\xC4W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xD9W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x02\x1EW\x80c\x8D\xA5\xCB[\x14a\x023W\x80c\x95A\x15%\x14a\x02^W`\0\x80\xFD[\x80c\x17\xD7\xDE|\x11a\0\xBBW\x80c\x17\xD7\xDE|\x14a\x01\xA1W\x80c<\x04'\x15\x14a\x01\xC3W\x80cT\xFDMP\x14a\x01\xD6W\x80ce\xC4\x0B\x9C\x14a\x01\xEBW`\0\x80\xFD[\x80c\x0E\xAB\xF6`\x14a\0\xE2W\x80c\x10\xD76\xD5\x14a\0\xF7W\x80c\x12\xB1\x1A\x17\x14a\x01dW[`\0\x80\xFD[a\0\xF5a\0\xF06`\x04a\x1BfV[a\x02\xF9V[\0[4\x80\x15a\x01\x03W`\0\x80\xFD[Pa\x01:a\x01\x126`\x04a\x1B\xA8V[`\0\x90\x81R`\x01` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01pW`\0\x80\xFD[P\x7FA \xD3\xB2\x83\x06fkqH&\xAD|\xB7\x07D\xD9e\x8A\xD3\xE6\xCD\x874\x11\xBE\xDA\xDC\xF5Z\xFD\xA7[`@Q\x90\x81R` \x01a\x01[V[4\x80\x15a\x01\xADW`\0\x80\xFD[Pa\x01\xB6a\x03XV[`@Qa\x01[\x91\x90a\x1C/V[a\x01\x93a\x01\xD16`\x04a\x1CBV[a\x03\xEAV[4\x80\x15a\x01\xE2W`\0\x80\xFD[Pa\x01\xB6a\x04\x0EV[4\x80\x15a\x01\xF7W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01:V[4\x80\x15a\x02*W`\0\x80\xFD[Pa\0\xF5a\x04\xB1V[4\x80\x15a\x02?W`\0\x80\xFD[P`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01:V[a\x02qa\x02l6`\x04a\x1BfV[a\x04\xC5V[`@Qa\x01[\x91\x90a\x1C}V[a\0\xF5a\x02\x8C6`\x04a\x1C\xC1V[a\x05\x01V[4\x80\x15a\x02\x9DW`\0\x80\xFD[P\x7F\x96\xBD\xBE\xA8\xFA(\x0F\x8A\r\x085X~\x1F\xBB\x14p\xE8\x1D\x05\xC4E\x14\x15\x84C4\x0C\xEA@\xA0]a\x01\x93V[4\x80\x15a\x02\xD0W`\0\x80\xFD[Pa\x01\x93a\x05 V[4\x80\x15a\x02\xE5W`\0\x80\xFD[Pa\0\xF5a\x02\xF46`\x04a\x1D\x03V[a\x05/V[`\0[\x81\x81\x10\x15a\x03IWa\x03A\x83\x83\x83\x81\x81\x10a\x03\x19Wa\x03\x19a\x1D\x1EV[\x90P` \x02\x81\x01\x90a\x03+\x91\x90a\x1DMV[a\x03<\x90`\x80\x81\x01\x90``\x01a\x1D\x03V[a\x05\xE8V[`\x01\x01a\x02\xFCV[Pa\x03T\x82\x82a\x06<V[PPV[```\0\x80Ta\x03g\x90a\x1D\x8BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x93\x90a\x1D\x8BV[\x80\x15a\x03\xE0W\x80`\x1F\x10a\x03\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xE0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x03\xFFa\x03<`\xC0\x84\x01`\xA0\x85\x01a\x1D\x03V[a\x04\x08\x82a\x08\xCFV[\x92\x91PPV[``a\x049\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x1AV[a\x04b\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x1AV[a\x04\x8B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x1AV[`@Q` \x01a\x04\x9D\x93\x92\x91\x90a\x1D\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\x04\xB9a\n\xD8V[a\x04\xC3`\0a\x0BYV[V[```\0[\x82\x81\x10\x15a\x04\xEFWa\x04\xE7\x84\x84\x83\x81\x81\x10a\x03\x19Wa\x03\x19a\x1D\x1EV[`\x01\x01a\x04\xCAV[Pa\x04\xFA\x83\x83a\x0B\xD0V[\x93\x92PPPV[a\x05\x14a\x03<`\xE0\x83\x01`\xC0\x84\x01a\x1D\x03V[a\x05\x1D\x81a\x0F\xC8V[PV[`\0a\x05*a\x10\xC8V[\x90P\x90V[a\x057a\n\xD8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x05\xDFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\x1D\x81a\x0BYV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x91\x16\x14a\x05\x1DW`@Q\x7FL\xA8\x88g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06WWa\x06Wa\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x9DW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06uW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x08#W`\0\x84\x84\x83\x81\x81\x10a\x06\xBFWa\x06\xBFa\x1D\x1EV[\x90P` \x02\x81\x01\x90a\x06\xD1\x91\x90a\x1DMV[a\x06\xDA\x90a \x83V[` \x81\x01Q\x80Q\x91\x92P\x90\x15\x80a\x06\xF7WP\x81`@\x01QQ\x81Q\x14\x15[\x15a\x07.W`@Q\x7F\x94}Z\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81Q\x81\x10\x15a\x07\xD9W`\0\x82\x82\x81Q\x81\x10a\x07NWa\x07Na\x1D\x1EV[` \x02` \x01\x01Q\x90Pa\x07\xD0`@Q\x80`\xA0\x01`@R\x80\x86`\0\x01Q\x81R` \x01\x83\x81R` \x01\x86`@\x01Q\x85\x81Q\x81\x10a\x07\x8CWa\x07\x8Ca\x1D\x1EV[` \x02` \x01\x01Q\x81R` \x01\x86``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x80\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPa\x11\xFCV[P`\x01\x01a\x071V[P`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x82\x81RP\x84\x84\x81Q\x81\x10a\x08\x05Wa\x08\x05a\x1D\x1EV[` \x02` \x01\x01\x81\x90RPPPa\x08\x1C\x81`\x01\x01\x90V[\x90Pa\x06\xA3V[P`@Q\x7FL\xB7\xE9\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cL\xB7\xE9\xE5\x904\x90a\x08\x98\x90\x85\x90`\x04\x01a!~V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x08\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xC5W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0a\x08\xE2a\x08\xDD\x83a#qV[a\x14/V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF1s%\xE74`@Q\x80`@\x01`@R\x80\x87`\0\x015\x81R` \x01\x87\x80` \x01\x90a\tF\x91\x90a#\xEAV[a\tO\x90a$\x1EV[\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tn\x91\x90a$\x9DV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\t\x8CW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xB1\x91\x90a$\xCAV[\x90Pa\t\xC3`\xC0\x84\x01`\xA0\x85\x01a\x1D\x03V[`\0\x82\x81R`\x01` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x92\x91PPV[```\0a\n'\x83a\x15\xF7V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nGWa\nGa\x1ENV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\nqW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\n{WP\x93\x92PPPV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\xC3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xD6V[`\x03\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xEDWa\x0B\xEDa\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C3W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\x0BW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0E\x01W6\x85\x85\x83\x81\x81\x10a\x0CTWa\x0CTa\x1D\x1EV[\x90P` \x02\x81\x01\x90a\x0Cf\x91\x90a\x1DMV[\x90P6`\0a\x0Cx` \x84\x01\x84a$\xE3V[\x90\x92P\x90P\x80\x15\x80a\x0C\x98WPa\x0C\x92`@\x84\x01\x84a%KV[\x82\x14\x15\x90P[\x15a\x0C\xCFW`@Q\x7F\x94}Z\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\r\xB0Wa\r\xA8`@Q\x80`\xA0\x01`@R\x80\x86`\0\x015\x81R` \x01\x85\x85\x85\x81\x81\x10a\r\x04Wa\r\x04a\x1D\x1EV[\x90P` \x02\x81\x01\x90a\r\x16\x91\x90a#\xEAV[a\r\x1F\x90a$\x1EV[\x81R` \x01a\r1`@\x88\x01\x88a%KV[\x85\x81\x81\x10a\rAWa\rAa\x1D\x1EV[\x90P``\x02\x01\x806\x03\x81\x01\x90a\rW\x91\x90a%\xB2V[\x81R` \x01a\rl`\x80\x88\x01``\x89\x01a\x1D\x03V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01a\r\x97`\xA0\x88\x01`\x80\x89\x01a%\xCEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Ra\x14/V[`\x01\x01a\x0C\xD2V[P`@\x80Q\x80\x82\x01\x90\x91R\x835\x81R` \x81\x01a\r\xCD\x83\x85a%\xE9V[\x81RP\x85\x85\x81Q\x81\x10a\r\xE2Wa\r\xE2a\x1D\x1EV[` \x02` \x01\x01\x81\x90RPPPPa\r\xFA\x81`\x01\x01\x90V[\x90Pa\x0C9V[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cD\xAD\xC9\x0E4\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E^\x91\x90a&]V[`\0`@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x0E|W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0E\xC3\x91\x90\x81\x01\x90a'PV[\x90P`\0\x80[\x85\x81\x10\x15a\x0F\xBDW6\x87\x87\x83\x81\x81\x10a\x0E\xE4Wa\x0E\xE4a\x1D\x1EV[\x90P` \x02\x81\x01\x90a\x0E\xF6\x91\x90a\x1DMV[\x90P6`\0a\x0F\x08` \x84\x01\x84a$\xE3V[\x91P\x91P`\0[\x81\x81\x10\x15a\x0F\xA8Wa\x0F'`\x80\x85\x01``\x86\x01a\x1D\x03V[`\x01`\0\x89\x89\x81Q\x81\x10a\x0F=Wa\x0F=a\x1D\x1EV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x85`\x01\x01\x95Pa\x0F\xA1\x81`\x01\x01\x90V[\x90Pa\x0F\x0FV[PPPPa\x0F\xB6\x81`\x01\x01\x90V[\x90Pa\x0E\xC9V[P\x90\x95\x94PPPPPV[a\x0F\xDFa\x0F\xDA6\x83\x90\x03\x83\x01\x83a'\xE1V[a\x11\xFCV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\x92bg4`@Q\x80`@\x01`@R\x80\x85`\0\x015\x81R` \x01\x85` \x01\x806\x03\x81\x01\x90a\x10E\x91\x90a(MV[\x90R`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R\x81Q`\x04\x82\x01R` \x91\x82\x01Q\x80Q`$\x83\x01R\x90\x91\x01Q`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x10\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xC0W=`\0\x80>=`\0\xFD[PPPPPPV[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x11.WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x11XWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\x80\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\x121WPBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x80\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15[\x15a\x12hW`@Q\x7F\x1A\xB7\xDAk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01Q\x80Q`\0\x90\x81R`\x01\x90\x92R`@\x90\x91 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x12\xCDW`@Q\x7F\xC5r;Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x13\x1CW`@Q\x7FL\xA8\x88g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x83\x01Qa\x13*\x81a\x16\xD9V[\x83Q\x83Q`\x80\x86\x01Q`@Q`\0\x93a\x13\xA6\x93a\x13\x8B\x93\x7F\x96\xBD\xBE\xA8\xFA(\x0F\x8A\r\x085X~\x1F\xBB\x14p\xE8\x1D\x05\xC4E\x14\x15\x84C4\x0C\xEA@\xA0]\x93` \x01\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x17\xE7V[\x90P\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13\xDB\x82\x84`\0\x01Q\x85` \x01Q\x86`@\x01Qa\x18PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x14(W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\x80\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\x14dWPBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x80\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15[\x15a\x14\x9BW`@Q\x7F\x1A\xB7\xDAk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`@\x82\x01Qa\x14\xAE\x81a\x16\xD9V[`\0a\x15o\x7FA \xD3\xB2\x83\x06fkqH&\xAD|\xB7\x07D\xD9e\x8A\xD3\xE6\xCD\x874\x11\xBE\xDA\xDC\xF5Z\xFD\xA7`\0\x1B\x85`\0\x01Q\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q\x88``\x01Q\x89`\x80\x01Q\x80Q\x90` \x01 \x8B`\x80\x01Q`@Q` \x01a\x13\x8B\x98\x97\x96\x95\x94\x93\x92\x91\x90\x97\x88R` \x88\x01\x96\x90\x96Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x90\x94\x16`@\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16``\x87\x01R\x90\x15\x15`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01\x91\x90\x91R\x16`\xE0\x82\x01Ra\x01\0\x01\x90V[\x90P\x83``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15\xA4\x82\x84`\0\x01Q\x85` \x01Q\x86`@\x01Qa\x18PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15\xF1W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x16@Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x16lWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x16\x8AWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x16\xA2Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x16\xB6Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x16\xC8W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x04\x08W`\x01\x01\x92\x91PPV[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q\x90Q`\xF8\x94\x90\x94\x1B\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x84\x01\x92\x90\x92R`!\x83\x01R`A\x82\x01R`\0\x90`a\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\x02\x81`@Qa\x17I\x91\x90a(iV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x15a\x17\x93W`@Q\x7F\xCC\xE9\xA8$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x02\x82`@Qa\x17\xA5\x91\x90a(iV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T\x91\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x92\x16\x91\x90\x91\x17\x90UPPV[`\0a\x04\x08a\x17\xF4a\x10\xC8V[\x83`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80`\0a\x18a\x87\x87\x87\x87a\x18xV[\x91P\x91Pa\x18n\x81a\x19gV[P\x95\x94PPPPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x18\xAFWP`\0\x90P`\x03a\x19^V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x19\x03W=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x19WW`\0`\x01\x92P\x92PPa\x19^V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a\x19{Wa\x19{a({V[\x03a\x19\x83WPV[`\x01\x81`\x04\x81\x11\x15a\x19\x97Wa\x19\x97a({V[\x03a\x19\xFEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xD6V[`\x02\x81`\x04\x81\x11\x15a\x1A\x12Wa\x1A\x12a({V[\x03a\x1AyW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\xD6V[`\x03\x81`\x04\x81\x11\x15a\x1A\x8DWa\x1A\x8Da({V[\x03a\x05\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xD6V[`\0\x80\x83`\x1F\x84\x01\x12a\x1B,W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1BDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1B_W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x1ByW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\x90W`\0\x80\xFD[a\x1B\x9C\x85\x82\x86\x01a\x1B\x1AV[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x1B\xBAW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x1B\xDCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1B\xC4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1B\xFD\x81` \x86\x01` \x86\x01a\x1B\xC1V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\xFA` \x83\x01\x84a\x1B\xE5V[`\0` \x82\x84\x03\x12\x15a\x1CTW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1CkW`\0\x80\xFD[\x82\x01`\xE0\x81\x85\x03\x12\x15a\x04\xFAW`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1C\xB5W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1C\x99V[P\x90\x96\x95PPPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15a\x1C\xD4W`\0\x80\xFD[P\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xFEW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1D\x15W`\0\x80\xFD[a\x04\xFA\x82a\x1C\xDAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x836\x03\x01\x81\x12a\x1D\x81W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1D\x9FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1C\xD4W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0\x84Qa\x1D\xEA\x81\x84` \x89\x01a\x1B\xC1V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x1E&\x81`\x01\x85\x01` \x8A\x01a\x1B\xC1V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x1EA\x81`\x02\x84\x01` \x88\x01a\x1B\xC1V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\xA0Wa\x1E\xA0a\x1ENV[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\xA0Wa\x1E\xA0a\x1ENV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\x10Wa\x1F\x10a\x1ENV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F2Wa\x1F2a\x1ENV[P`\x05\x1B` \x01\x90V[`\0`@\x82\x84\x03\x12\x15a\x1FNW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1FqWa\x1Fqa\x1ENV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x1F\x9DW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1F\xC0Wa\x1F\xC0a\x1ENV[`@R\x90P\x80\x825`\xFF\x81\x16\x81\x14a\x1F\xD7W`\0\x80\xFD[\x80\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01RP\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a \x07W`\0\x80\xFD[\x815` a \x1Ca \x17\x83a\x1F\x18V[a\x1E\xC9V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a ;W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a ^Wa Q\x89\x82a\x1F\x8BV[\x84R\x92\x84\x01\x92\x81\x01a ?V[P\x90\x97\x96PPPPPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xFEW`\0\x80\xFD[`\0`\xA0\x826\x03\x12\x15a \x95W`\0\x80\xFD[a \x9Da\x1E}V[\x825\x81R` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xBDW`\0\x80\xFD[\x90\x85\x01\x906`\x1F\x83\x01\x12a \xD0W`\0\x80\xFD[\x815a \xDEa \x17\x82a\x1F\x18V[\x81\x81R`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x906\x83\x11\x15a \xFDW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a!&Wa!\x146\x86a\x1F<V[\x82R\x85\x82\x01\x91P`@\x85\x01\x94Pa!\x02V[\x80\x86\x88\x01RPPP`@\x86\x015\x92P\x80\x83\x11\x15a!BW`\0\x80\xFD[PPa!P6\x82\x86\x01a\x1F\xF6V[`@\x83\x01RPa!b``\x84\x01a\x1C\xDAV[``\x82\x01Ra!s`\x80\x84\x01a kV[`\x80\x82\x01R\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\">W\x89\x84\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x01\x86R\x82Q\x80Q\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\")Wa\"\x15\x82\x85Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8A\x01\x90a!\xF8V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a!\xA6V[P\x91\x99\x98PPPPPPPPPV[`\0\x82`\x1F\x83\x01\x12a\"^W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"xWa\"xa\x1ENV[a\"\xA9` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x1E\xC9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\"\xBEW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\"\xEDW`\0\x80\xFD[a\"\xF5a\x1E\xA6V[\x90Pa#\0\x82a\x1C\xDAV[\x81Ra#\x0E` \x83\x01a kV[` \x82\x01R`@\x82\x015\x80\x15\x15\x81\x14a#&W`\0\x80\xFD[`@\x82\x01R``\x82\x81\x015\x90\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#OW`\0\x80\xFD[a#[\x84\x82\x85\x01a\"MV[`\x80\x83\x01RP`\xA0\x82\x015`\xA0\x82\x01R\x92\x91PPV[`\0`\xE0\x826\x03\x12\x15a#\x83W`\0\x80\xFD[a#\x8Ba\x1E}V[\x825\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xA9W`\0\x80\xFD[a#\xB56\x82\x86\x01a\"\xDBV[` \x83\x01RPa#\xC86`@\x85\x01a\x1F\x8BV[`@\x82\x01Ra#\xD9`\xA0\x84\x01a\x1C\xDAV[``\x82\x01Ra!s`\xC0\x84\x01a kV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFA\x836\x03\x01\x81\x12a\x1D\x81W`\0\x80\xFD[`\0a\x04\x086\x83a\"\xDBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q\x15\x15`@\x83\x01R``\x81\x01Q``\x83\x01R`\0`\x80\x82\x01Q`\xC0`\x80\x85\x01Ra$\x89`\xC0\x85\x01\x82a\x1B\xE5V[`\xA0\x93\x84\x01Q\x94\x90\x93\x01\x93\x90\x93RP\x91\x90PV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01Ra$\xC2``\x84\x01\x82a$*V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a$\xDCW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a%\x18W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%3W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x1B_W`\0\x80\xFD[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a%\x80W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\x9BW`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a\x1B_W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a%\xC4W`\0\x80\xFD[a\x04\xFA\x83\x83a\x1F\x8BV[`\0` \x82\x84\x03\x12\x15a%\xE0W`\0\x80\xFD[a\x04\xFA\x82a kV[`\0a%\xF7a \x17\x84a\x1F\x18V[\x80\x84\x82R` \x80\x83\x01\x92P\x85`\x05\x1B\x85\x016\x81\x11\x15a&\x15W`\0\x80\xFD[\x85[\x81\x81\x10\x15a&QW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&7W`\0\x80\x81\xFD[a&C6\x82\x8A\x01a\"\xDBV[\x86RP\x93\x82\x01\x93\x82\x01a&\x17V[P\x91\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a'@W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8B\x85\x03\x01\x87R\x82Q\x80Q\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a'*W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x85\x03\x01\x83Ra'\x18\x84\x86Qa$*V[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a&\xDEV[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a&\x84V[P\x91\x9A\x99PPPPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a'cW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'zW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a'\x8BW`\0\x80\xFD[\x80Qa'\x99a \x17\x82a\x1F\x18V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a'\xB8W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a'\xD6W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a'\xBDV[\x97\x96PPPPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15a'\xF4W`\0\x80\xFD[a'\xFCa\x1E}V[\x825\x81Ra(\r\x84` \x85\x01a\x1F<V[` \x82\x01Ra(\x1F\x84``\x85\x01a\x1F\x8BV[`@\x82\x01Ra(0`\xC0\x84\x01a\x1C\xDAV[``\x82\x01Ra(A`\xE0\x84\x01a kV[`\x80\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a(_W`\0\x80\xFD[a\x04\xFA\x83\x83a\x1F<V[`\0\x82Qa\x1D\x81\x81\x84` \x87\x01a\x1B\xC1V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The bytecode of the contract.
    pub static PERMISSIONEDEIP712PROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xDDW`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x7FW\x80c\xA6\xD4\xDB\xC7\x11a\0YW\x80c\xA6\xD4\xDB\xC7\x14a\x02~W\x80c\xB80\x10\xD3\x14a\x02\x91W\x80c\xED$\x91\x1D\x14a\x02\xC4W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xD9W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x02\x1EW\x80c\x8D\xA5\xCB[\x14a\x023W\x80c\x95A\x15%\x14a\x02^W`\0\x80\xFD[\x80c\x17\xD7\xDE|\x11a\0\xBBW\x80c\x17\xD7\xDE|\x14a\x01\xA1W\x80c<\x04'\x15\x14a\x01\xC3W\x80cT\xFDMP\x14a\x01\xD6W\x80ce\xC4\x0B\x9C\x14a\x01\xEBW`\0\x80\xFD[\x80c\x0E\xAB\xF6`\x14a\0\xE2W\x80c\x10\xD76\xD5\x14a\0\xF7W\x80c\x12\xB1\x1A\x17\x14a\x01dW[`\0\x80\xFD[a\0\xF5a\0\xF06`\x04a\x1BfV[a\x02\xF9V[\0[4\x80\x15a\x01\x03W`\0\x80\xFD[Pa\x01:a\x01\x126`\x04a\x1B\xA8V[`\0\x90\x81R`\x01` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01pW`\0\x80\xFD[P\x7FA \xD3\xB2\x83\x06fkqH&\xAD|\xB7\x07D\xD9e\x8A\xD3\xE6\xCD\x874\x11\xBE\xDA\xDC\xF5Z\xFD\xA7[`@Q\x90\x81R` \x01a\x01[V[4\x80\x15a\x01\xADW`\0\x80\xFD[Pa\x01\xB6a\x03XV[`@Qa\x01[\x91\x90a\x1C/V[a\x01\x93a\x01\xD16`\x04a\x1CBV[a\x03\xEAV[4\x80\x15a\x01\xE2W`\0\x80\xFD[Pa\x01\xB6a\x04\x0EV[4\x80\x15a\x01\xF7W`\0\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01:V[4\x80\x15a\x02*W`\0\x80\xFD[Pa\0\xF5a\x04\xB1V[4\x80\x15a\x02?W`\0\x80\xFD[P`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01:V[a\x02qa\x02l6`\x04a\x1BfV[a\x04\xC5V[`@Qa\x01[\x91\x90a\x1C}V[a\0\xF5a\x02\x8C6`\x04a\x1C\xC1V[a\x05\x01V[4\x80\x15a\x02\x9DW`\0\x80\xFD[P\x7F\x96\xBD\xBE\xA8\xFA(\x0F\x8A\r\x085X~\x1F\xBB\x14p\xE8\x1D\x05\xC4E\x14\x15\x84C4\x0C\xEA@\xA0]a\x01\x93V[4\x80\x15a\x02\xD0W`\0\x80\xFD[Pa\x01\x93a\x05 V[4\x80\x15a\x02\xE5W`\0\x80\xFD[Pa\0\xF5a\x02\xF46`\x04a\x1D\x03V[a\x05/V[`\0[\x81\x81\x10\x15a\x03IWa\x03A\x83\x83\x83\x81\x81\x10a\x03\x19Wa\x03\x19a\x1D\x1EV[\x90P` \x02\x81\x01\x90a\x03+\x91\x90a\x1DMV[a\x03<\x90`\x80\x81\x01\x90``\x01a\x1D\x03V[a\x05\xE8V[`\x01\x01a\x02\xFCV[Pa\x03T\x82\x82a\x06<V[PPV[```\0\x80Ta\x03g\x90a\x1D\x8BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x93\x90a\x1D\x8BV[\x80\x15a\x03\xE0W\x80`\x1F\x10a\x03\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xE0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x03\xFFa\x03<`\xC0\x84\x01`\xA0\x85\x01a\x1D\x03V[a\x04\x08\x82a\x08\xCFV[\x92\x91PPV[``a\x049\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x1AV[a\x04b\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x1AV[a\x04\x8B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x1AV[`@Q` \x01a\x04\x9D\x93\x92\x91\x90a\x1D\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\x04\xB9a\n\xD8V[a\x04\xC3`\0a\x0BYV[V[```\0[\x82\x81\x10\x15a\x04\xEFWa\x04\xE7\x84\x84\x83\x81\x81\x10a\x03\x19Wa\x03\x19a\x1D\x1EV[`\x01\x01a\x04\xCAV[Pa\x04\xFA\x83\x83a\x0B\xD0V[\x93\x92PPPV[a\x05\x14a\x03<`\xE0\x83\x01`\xC0\x84\x01a\x1D\x03V[a\x05\x1D\x81a\x0F\xC8V[PV[`\0a\x05*a\x10\xC8V[\x90P\x90V[a\x057a\n\xD8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x05\xDFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x05\x1D\x81a\x0BYV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x91\x16\x14a\x05\x1DW`@Q\x7FL\xA8\x88g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06WWa\x06Wa\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x9DW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06uW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x08#W`\0\x84\x84\x83\x81\x81\x10a\x06\xBFWa\x06\xBFa\x1D\x1EV[\x90P` \x02\x81\x01\x90a\x06\xD1\x91\x90a\x1DMV[a\x06\xDA\x90a \x83V[` \x81\x01Q\x80Q\x91\x92P\x90\x15\x80a\x06\xF7WP\x81`@\x01QQ\x81Q\x14\x15[\x15a\x07.W`@Q\x7F\x94}Z\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81Q\x81\x10\x15a\x07\xD9W`\0\x82\x82\x81Q\x81\x10a\x07NWa\x07Na\x1D\x1EV[` \x02` \x01\x01Q\x90Pa\x07\xD0`@Q\x80`\xA0\x01`@R\x80\x86`\0\x01Q\x81R` \x01\x83\x81R` \x01\x86`@\x01Q\x85\x81Q\x81\x10a\x07\x8CWa\x07\x8Ca\x1D\x1EV[` \x02` \x01\x01Q\x81R` \x01\x86``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x80\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPa\x11\xFCV[P`\x01\x01a\x071V[P`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x82\x81RP\x84\x84\x81Q\x81\x10a\x08\x05Wa\x08\x05a\x1D\x1EV[` \x02` \x01\x01\x81\x90RPPPa\x08\x1C\x81`\x01\x01\x90V[\x90Pa\x06\xA3V[P`@Q\x7FL\xB7\xE9\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cL\xB7\xE9\xE5\x904\x90a\x08\x98\x90\x85\x90`\x04\x01a!~V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x08\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xC5W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0a\x08\xE2a\x08\xDD\x83a#qV[a\x14/V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF1s%\xE74`@Q\x80`@\x01`@R\x80\x87`\0\x015\x81R` \x01\x87\x80` \x01\x90a\tF\x91\x90a#\xEAV[a\tO\x90a$\x1EV[\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tn\x91\x90a$\x9DV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\t\x8CW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xB1\x91\x90a$\xCAV[\x90Pa\t\xC3`\xC0\x84\x01`\xA0\x85\x01a\x1D\x03V[`\0\x82\x81R`\x01` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x92\x91PPV[```\0a\n'\x83a\x15\xF7V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nGWa\nGa\x1ENV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\nqW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\n{WP\x93\x92PPPV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\xC3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xD6V[`\x03\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xEDWa\x0B\xEDa\x1ENV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C3W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\x0BW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0E\x01W6\x85\x85\x83\x81\x81\x10a\x0CTWa\x0CTa\x1D\x1EV[\x90P` \x02\x81\x01\x90a\x0Cf\x91\x90a\x1DMV[\x90P6`\0a\x0Cx` \x84\x01\x84a$\xE3V[\x90\x92P\x90P\x80\x15\x80a\x0C\x98WPa\x0C\x92`@\x84\x01\x84a%KV[\x82\x14\x15\x90P[\x15a\x0C\xCFW`@Q\x7F\x94}Z\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\r\xB0Wa\r\xA8`@Q\x80`\xA0\x01`@R\x80\x86`\0\x015\x81R` \x01\x85\x85\x85\x81\x81\x10a\r\x04Wa\r\x04a\x1D\x1EV[\x90P` \x02\x81\x01\x90a\r\x16\x91\x90a#\xEAV[a\r\x1F\x90a$\x1EV[\x81R` \x01a\r1`@\x88\x01\x88a%KV[\x85\x81\x81\x10a\rAWa\rAa\x1D\x1EV[\x90P``\x02\x01\x806\x03\x81\x01\x90a\rW\x91\x90a%\xB2V[\x81R` \x01a\rl`\x80\x88\x01``\x89\x01a\x1D\x03V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01a\r\x97`\xA0\x88\x01`\x80\x89\x01a%\xCEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Ra\x14/V[`\x01\x01a\x0C\xD2V[P`@\x80Q\x80\x82\x01\x90\x91R\x835\x81R` \x81\x01a\r\xCD\x83\x85a%\xE9V[\x81RP\x85\x85\x81Q\x81\x10a\r\xE2Wa\r\xE2a\x1D\x1EV[` \x02` \x01\x01\x81\x90RPPPPa\r\xFA\x81`\x01\x01\x90V[\x90Pa\x0C9V[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cD\xAD\xC9\x0E4\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E^\x91\x90a&]V[`\0`@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x0E|W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0E\xC3\x91\x90\x81\x01\x90a'PV[\x90P`\0\x80[\x85\x81\x10\x15a\x0F\xBDW6\x87\x87\x83\x81\x81\x10a\x0E\xE4Wa\x0E\xE4a\x1D\x1EV[\x90P` \x02\x81\x01\x90a\x0E\xF6\x91\x90a\x1DMV[\x90P6`\0a\x0F\x08` \x84\x01\x84a$\xE3V[\x91P\x91P`\0[\x81\x81\x10\x15a\x0F\xA8Wa\x0F'`\x80\x85\x01``\x86\x01a\x1D\x03V[`\x01`\0\x89\x89\x81Q\x81\x10a\x0F=Wa\x0F=a\x1D\x1EV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x85`\x01\x01\x95Pa\x0F\xA1\x81`\x01\x01\x90V[\x90Pa\x0F\x0FV[PPPPa\x0F\xB6\x81`\x01\x01\x90V[\x90Pa\x0E\xC9V[P\x90\x95\x94PPPPPV[a\x0F\xDFa\x0F\xDA6\x83\x90\x03\x83\x01\x83a'\xE1V[a\x11\xFCV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\x92bg4`@Q\x80`@\x01`@R\x80\x85`\0\x015\x81R` \x01\x85` \x01\x806\x03\x81\x01\x90a\x10E\x91\x90a(MV[\x90R`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R\x81Q`\x04\x82\x01R` \x91\x82\x01Q\x80Q`$\x83\x01R\x90\x91\x01Q`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x10\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xC0W=`\0\x80>=`\0\xFD[PPPPPPV[`\x000s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x11.WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x11XWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\x80\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\x121WPBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x80\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15[\x15a\x12hW`@Q\x7F\x1A\xB7\xDAk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x82\x01Q\x80Q`\0\x90\x81R`\x01\x90\x92R`@\x90\x91 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x12\xCDW`@Q\x7F\xC5r;Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14a\x13\x1CW`@Q\x7FL\xA8\x88g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x83\x01Qa\x13*\x81a\x16\xD9V[\x83Q\x83Q`\x80\x86\x01Q`@Q`\0\x93a\x13\xA6\x93a\x13\x8B\x93\x7F\x96\xBD\xBE\xA8\xFA(\x0F\x8A\r\x085X~\x1F\xBB\x14p\xE8\x1D\x05\xC4E\x14\x15\x84C4\x0C\xEA@\xA0]\x93` \x01\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x17\xE7V[\x90P\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13\xDB\x82\x84`\0\x01Q\x85` \x01Q\x86`@\x01Qa\x18PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x14(W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\x80\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\x14dWPBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x80\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15[\x15a\x14\x9BW`@Q\x7F\x1A\xB7\xDAk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`@\x82\x01Qa\x14\xAE\x81a\x16\xD9V[`\0a\x15o\x7FA \xD3\xB2\x83\x06fkqH&\xAD|\xB7\x07D\xD9e\x8A\xD3\xE6\xCD\x874\x11\xBE\xDA\xDC\xF5Z\xFD\xA7`\0\x1B\x85`\0\x01Q\x85`\0\x01Q\x86` \x01Q\x87`@\x01Q\x88``\x01Q\x89`\x80\x01Q\x80Q\x90` \x01 \x8B`\x80\x01Q`@Q` \x01a\x13\x8B\x98\x97\x96\x95\x94\x93\x92\x91\x90\x97\x88R` \x88\x01\x96\x90\x96Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x90\x94\x16`@\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16``\x87\x01R\x90\x15\x15`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01\x91\x90\x91R\x16`\xE0\x82\x01Ra\x01\0\x01\x90V[\x90P\x83``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15\xA4\x82\x84`\0\x01Q\x85` \x01Q\x86`@\x01Qa\x18PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15\xF1W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x16@Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x16lWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x16\x8AWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x16\xA2Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x16\xB6Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x16\xC8W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x04\x08W`\x01\x01\x92\x91PPV[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q\x90Q`\xF8\x94\x90\x94\x1B\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x84\x01\x92\x90\x92R`!\x83\x01R`A\x82\x01R`\0\x90`a\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\x02\x81`@Qa\x17I\x91\x90a(iV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x15a\x17\x93W`@Q\x7F\xCC\xE9\xA8$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x02\x82`@Qa\x17\xA5\x91\x90a(iV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T\x91\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x92\x16\x91\x90\x91\x17\x90UPPV[`\0a\x04\x08a\x17\xF4a\x10\xC8V[\x83`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80`\0a\x18a\x87\x87\x87\x87a\x18xV[\x91P\x91Pa\x18n\x81a\x19gV[P\x95\x94PPPPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x18\xAFWP`\0\x90P`\x03a\x19^V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x19\x03W=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x19WW`\0`\x01\x92P\x92PPa\x19^V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a\x19{Wa\x19{a({V[\x03a\x19\x83WPV[`\x01\x81`\x04\x81\x11\x15a\x19\x97Wa\x19\x97a({V[\x03a\x19\xFEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xD6V[`\x02\x81`\x04\x81\x11\x15a\x1A\x12Wa\x1A\x12a({V[\x03a\x1AyW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\xD6V[`\x03\x81`\x04\x81\x11\x15a\x1A\x8DWa\x1A\x8Da({V[\x03a\x05\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xD6V[`\0\x80\x83`\x1F\x84\x01\x12a\x1B,W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1BDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1B_W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x1ByW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\x90W`\0\x80\xFD[a\x1B\x9C\x85\x82\x86\x01a\x1B\x1AV[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x1B\xBAW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x1B\xDCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1B\xC4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1B\xFD\x81` \x86\x01` \x86\x01a\x1B\xC1V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\xFA` \x83\x01\x84a\x1B\xE5V[`\0` \x82\x84\x03\x12\x15a\x1CTW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1CkW`\0\x80\xFD[\x82\x01`\xE0\x81\x85\x03\x12\x15a\x04\xFAW`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1C\xB5W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1C\x99V[P\x90\x96\x95PPPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15a\x1C\xD4W`\0\x80\xFD[P\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xFEW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1D\x15W`\0\x80\xFD[a\x04\xFA\x82a\x1C\xDAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x836\x03\x01\x81\x12a\x1D\x81W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1D\x9FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1C\xD4W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0\x84Qa\x1D\xEA\x81\x84` \x89\x01a\x1B\xC1V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\x1E&\x81`\x01\x85\x01` \x8A\x01a\x1B\xC1V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x1EA\x81`\x02\x84\x01` \x88\x01a\x1B\xC1V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\xA0Wa\x1E\xA0a\x1ENV[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\xA0Wa\x1E\xA0a\x1ENV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\x10Wa\x1F\x10a\x1ENV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F2Wa\x1F2a\x1ENV[P`\x05\x1B` \x01\x90V[`\0`@\x82\x84\x03\x12\x15a\x1FNW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1FqWa\x1Fqa\x1ENV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x1F\x9DW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1F\xC0Wa\x1F\xC0a\x1ENV[`@R\x90P\x80\x825`\xFF\x81\x16\x81\x14a\x1F\xD7W`\0\x80\xFD[\x80\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01RP\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a \x07W`\0\x80\xFD[\x815` a \x1Ca \x17\x83a\x1F\x18V[a\x1E\xC9V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a ;W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a ^Wa Q\x89\x82a\x1F\x8BV[\x84R\x92\x84\x01\x92\x81\x01a ?V[P\x90\x97\x96PPPPPPPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xFEW`\0\x80\xFD[`\0`\xA0\x826\x03\x12\x15a \x95W`\0\x80\xFD[a \x9Da\x1E}V[\x825\x81R` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \xBDW`\0\x80\xFD[\x90\x85\x01\x906`\x1F\x83\x01\x12a \xD0W`\0\x80\xFD[\x815a \xDEa \x17\x82a\x1F\x18V[\x81\x81R`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x906\x83\x11\x15a \xFDW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a!&Wa!\x146\x86a\x1F<V[\x82R\x85\x82\x01\x91P`@\x85\x01\x94Pa!\x02V[\x80\x86\x88\x01RPPP`@\x86\x015\x92P\x80\x83\x11\x15a!BW`\0\x80\xFD[PPa!P6\x82\x86\x01a\x1F\xF6V[`@\x83\x01RPa!b``\x84\x01a\x1C\xDAV[``\x82\x01Ra!s`\x80\x84\x01a kV[`\x80\x82\x01R\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\">W\x89\x84\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x01\x86R\x82Q\x80Q\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\")Wa\"\x15\x82\x85Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8A\x01\x90a!\xF8V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a!\xA6V[P\x91\x99\x98PPPPPPPPPV[`\0\x82`\x1F\x83\x01\x12a\"^W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"xWa\"xa\x1ENV[a\"\xA9` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x1E\xC9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\"\xBEW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\"\xEDW`\0\x80\xFD[a\"\xF5a\x1E\xA6V[\x90Pa#\0\x82a\x1C\xDAV[\x81Ra#\x0E` \x83\x01a kV[` \x82\x01R`@\x82\x015\x80\x15\x15\x81\x14a#&W`\0\x80\xFD[`@\x82\x01R``\x82\x81\x015\x90\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#OW`\0\x80\xFD[a#[\x84\x82\x85\x01a\"MV[`\x80\x83\x01RP`\xA0\x82\x015`\xA0\x82\x01R\x92\x91PPV[`\0`\xE0\x826\x03\x12\x15a#\x83W`\0\x80\xFD[a#\x8Ba\x1E}V[\x825\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xA9W`\0\x80\xFD[a#\xB56\x82\x86\x01a\"\xDBV[` \x83\x01RPa#\xC86`@\x85\x01a\x1F\x8BV[`@\x82\x01Ra#\xD9`\xA0\x84\x01a\x1C\xDAV[``\x82\x01Ra!s`\xC0\x84\x01a kV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFA\x836\x03\x01\x81\x12a\x1D\x81W`\0\x80\xFD[`\0a\x04\x086\x83a\"\xDBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q\x15\x15`@\x83\x01R``\x81\x01Q``\x83\x01R`\0`\x80\x82\x01Q`\xC0`\x80\x85\x01Ra$\x89`\xC0\x85\x01\x82a\x1B\xE5V[`\xA0\x93\x84\x01Q\x94\x90\x93\x01\x93\x90\x93RP\x91\x90PV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01Ra$\xC2``\x84\x01\x82a$*V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a$\xDCW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a%\x18W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%3W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x1B_W`\0\x80\xFD[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a%\x80W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\x9BW`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a\x1B_W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a%\xC4W`\0\x80\xFD[a\x04\xFA\x83\x83a\x1F\x8BV[`\0` \x82\x84\x03\x12\x15a%\xE0W`\0\x80\xFD[a\x04\xFA\x82a kV[`\0a%\xF7a \x17\x84a\x1F\x18V[\x80\x84\x82R` \x80\x83\x01\x92P\x85`\x05\x1B\x85\x016\x81\x11\x15a&\x15W`\0\x80\xFD[\x85[\x81\x81\x10\x15a&QW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&7W`\0\x80\x81\xFD[a&C6\x82\x8A\x01a\"\xDBV[\x86RP\x93\x82\x01\x93\x82\x01a&\x17V[P\x91\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a'@W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8B\x85\x03\x01\x87R\x82Q\x80Q\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a'*W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x85\x03\x01\x83Ra'\x18\x84\x86Qa$*V[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a&\xDEV[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a&\x84V[P\x91\x9A\x99PPPPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a'cW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'zW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a'\x8BW`\0\x80\xFD[\x80Qa'\x99a \x17\x82a\x1F\x18V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a'\xB8W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a'\xD6W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a'\xBDV[\x97\x96PPPPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15a'\xF4W`\0\x80\xFD[a'\xFCa\x1E}V[\x825\x81Ra(\r\x84` \x85\x01a\x1F<V[` \x82\x01Ra(\x1F\x84``\x85\x01a\x1F\x8BV[`@\x82\x01Ra(0`\xC0\x84\x01a\x1C\xDAV[``\x82\x01Ra(A`\xE0\x84\x01a kV[`\x80\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a(_W`\0\x80\xFD[a\x04\xFA\x83\x83a\x1F<V[`\0\x82Qa\x1D\x81\x81\x84` \x87\x01a\x1B\xC1V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The deployed bytecode of the contract.
    pub static PERMISSIONEDEIP712PROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PermissionedEIP712Proxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PermissionedEIP712Proxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PermissionedEIP712Proxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PermissionedEIP712Proxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PermissionedEIP712Proxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PermissionedEIP712Proxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PermissionedEIP712Proxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PERMISSIONEDEIP712PROXY_ABI.clone(),
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
                PERMISSIONEDEIP712PROXY_ABI.clone(),
                PERMISSIONEDEIP712PROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `attestByDelegation` (0x3c042715) function
        pub fn attest_by_delegation(
            &self,
            delegated_request: DelegatedProxyAttestationRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([60, 4, 39, 21], (delegated_request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAttestTypeHash` (0x12b11a17) function
        pub fn get_attest_type_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([18, 177, 26, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAttester` (0x10d736d5) function
        pub fn get_attester(
            &self,
            uid: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([16, 215, 54, 213], uid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDomainSeparator` (0xed24911d) function
        pub fn get_domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([237, 36, 145, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEAS` (0x65c40b9c) function
        pub fn get_eas(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([101, 196, 11, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getName` (0x17d7de7c) function
        pub fn get_name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([23, 215, 222, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRevokeTypeHash` (0xb83010d3) function
        pub fn get_revoke_type_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([184, 48, 16, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiAttestByDelegation` (0x95411525) function
        pub fn multi_attest_by_delegation(
            &self,
            multi_delegated_requests: ::std::vec::Vec<
                MultiDelegatedProxyAttestationRequest,
            >,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([149, 65, 21, 37], multi_delegated_requests)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiRevokeByDelegation` (0x0eabf660) function
        pub fn multi_revoke_by_delegation(
            &self,
            multi_delegated_requests: ::std::vec::Vec<
                MultiDelegatedProxyRevocationRequest,
            >,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 171, 246, 96], multi_delegated_requests)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeByDelegation` (0xa6d4dbc7) function
        pub fn revoke_by_delegation(
            &self,
            delegated_request: DelegatedProxyRevocationRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 212, 219, 199], (delegated_request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PermissionedEIP712Proxy<M> {
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
    ///Custom Error type `DeadlineExpired` with signature `DeadlineExpired()` and selector `0x1ab7da6b`
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
    #[etherror(name = "DeadlineExpired", abi = "DeadlineExpired()")]
    pub struct DeadlineExpired;
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
    ///Custom Error type `InvalidLength` with signature `InvalidLength()` and selector `0x947d5a84`
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
    #[etherror(name = "InvalidLength", abi = "InvalidLength()")]
    pub struct InvalidLength;
    ///Custom Error type `InvalidSignature` with signature `InvalidSignature()` and selector `0x8baa579f`
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
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature()")]
    pub struct InvalidSignature;
    ///Custom Error type `NotFound` with signature `NotFound()` and selector `0xc5723b51`
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
    #[etherror(name = "NotFound", abi = "NotFound()")]
    pub struct NotFound;
    ///Custom Error type `UsedSignature` with signature `UsedSignature()` and selector `0xcce9a824`
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
    #[etherror(name = "UsedSignature", abi = "UsedSignature()")]
    pub struct UsedSignature;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PermissionedEIP712ProxyErrors {
        AccessDenied(AccessDenied),
        DeadlineExpired(DeadlineExpired),
        InvalidEAS(InvalidEAS),
        InvalidLength(InvalidLength),
        InvalidSignature(InvalidSignature),
        NotFound(NotFound),
        UsedSignature(UsedSignature),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PermissionedEIP712ProxyErrors {
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
                = <DeadlineExpired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeadlineExpired(decoded));
            }
            if let Ok(decoded)
                = <InvalidEAS as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidEAS(decoded));
            }
            if let Ok(decoded)
                = <InvalidLength as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidLength(decoded));
            }
            if let Ok(decoded)
                = <InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSignature(decoded));
            }
            if let Ok(decoded)
                = <NotFound as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotFound(decoded));
            }
            if let Ok(decoded)
                = <UsedSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UsedSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PermissionedEIP712ProxyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessDenied(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeadlineExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEAS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsedSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PermissionedEIP712ProxyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessDenied as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <DeadlineExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEAS as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotFound as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UsedSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PermissionedEIP712ProxyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessDenied(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeadlineExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEAS(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PermissionedEIP712ProxyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessDenied> for PermissionedEIP712ProxyErrors {
        fn from(value: AccessDenied) -> Self {
            Self::AccessDenied(value)
        }
    }
    impl ::core::convert::From<DeadlineExpired> for PermissionedEIP712ProxyErrors {
        fn from(value: DeadlineExpired) -> Self {
            Self::DeadlineExpired(value)
        }
    }
    impl ::core::convert::From<InvalidEAS> for PermissionedEIP712ProxyErrors {
        fn from(value: InvalidEAS) -> Self {
            Self::InvalidEAS(value)
        }
    }
    impl ::core::convert::From<InvalidLength> for PermissionedEIP712ProxyErrors {
        fn from(value: InvalidLength) -> Self {
            Self::InvalidLength(value)
        }
    }
    impl ::core::convert::From<InvalidSignature> for PermissionedEIP712ProxyErrors {
        fn from(value: InvalidSignature) -> Self {
            Self::InvalidSignature(value)
        }
    }
    impl ::core::convert::From<NotFound> for PermissionedEIP712ProxyErrors {
        fn from(value: NotFound) -> Self {
            Self::NotFound(value)
        }
    }
    impl ::core::convert::From<UsedSignature> for PermissionedEIP712ProxyErrors {
        fn from(value: UsedSignature) -> Self {
            Self::UsedSignature(value)
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `attestByDelegation` function with signature `attestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address,uint64))` and selector `0x3c042715`
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
        abi = "attestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address,uint64))"
    )]
    pub struct AttestByDelegationCall {
        pub delegated_request: DelegatedProxyAttestationRequest,
    }
    ///Container type for all input parameters for the `getAttestTypeHash` function with signature `getAttestTypeHash()` and selector `0x12b11a17`
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
    #[ethcall(name = "getAttestTypeHash", abi = "getAttestTypeHash()")]
    pub struct GetAttestTypeHashCall;
    ///Container type for all input parameters for the `getAttester` function with signature `getAttester(bytes32)` and selector `0x10d736d5`
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
    #[ethcall(name = "getAttester", abi = "getAttester(bytes32)")]
    pub struct GetAttesterCall {
        pub uid: [u8; 32],
    }
    ///Container type for all input parameters for the `getDomainSeparator` function with signature `getDomainSeparator()` and selector `0xed24911d`
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
    #[ethcall(name = "getDomainSeparator", abi = "getDomainSeparator()")]
    pub struct GetDomainSeparatorCall;
    ///Container type for all input parameters for the `getEAS` function with signature `getEAS()` and selector `0x65c40b9c`
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
    #[ethcall(name = "getEAS", abi = "getEAS()")]
    pub struct GetEASCall;
    ///Container type for all input parameters for the `getName` function with signature `getName()` and selector `0x17d7de7c`
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
    #[ethcall(name = "getName", abi = "getName()")]
    pub struct GetNameCall;
    ///Container type for all input parameters for the `getRevokeTypeHash` function with signature `getRevokeTypeHash()` and selector `0xb83010d3`
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
    #[ethcall(name = "getRevokeTypeHash", abi = "getRevokeTypeHash()")]
    pub struct GetRevokeTypeHashCall;
    ///Container type for all input parameters for the `multiAttestByDelegation` function with signature `multiAttestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)[])` and selector `0x95411525`
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
        abi = "multiAttestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)[])"
    )]
    pub struct MultiAttestByDelegationCall {
        pub multi_delegated_requests: ::std::vec::Vec<
            MultiDelegatedProxyAttestationRequest,
        >,
    }
    ///Container type for all input parameters for the `multiRevokeByDelegation` function with signature `multiRevokeByDelegation((bytes32,(bytes32,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)[])` and selector `0x0eabf660`
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
        abi = "multiRevokeByDelegation((bytes32,(bytes32,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)[])"
    )]
    pub struct MultiRevokeByDelegationCall {
        pub multi_delegated_requests: ::std::vec::Vec<
            MultiDelegatedProxyRevocationRequest,
        >,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `revokeByDelegation` function with signature `revokeByDelegation((bytes32,(bytes32,uint256),(uint8,bytes32,bytes32),address,uint64))` and selector `0xa6d4dbc7`
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
        abi = "revokeByDelegation((bytes32,(bytes32,uint256),(uint8,bytes32,bytes32),address,uint64))"
    )]
    pub struct RevokeByDelegationCall {
        pub delegated_request: DelegatedProxyRevocationRequest,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum PermissionedEIP712ProxyCalls {
        AttestByDelegation(AttestByDelegationCall),
        GetAttestTypeHash(GetAttestTypeHashCall),
        GetAttester(GetAttesterCall),
        GetDomainSeparator(GetDomainSeparatorCall),
        GetEAS(GetEASCall),
        GetName(GetNameCall),
        GetRevokeTypeHash(GetRevokeTypeHashCall),
        MultiAttestByDelegation(MultiAttestByDelegationCall),
        MultiRevokeByDelegation(MultiRevokeByDelegationCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        RevokeByDelegation(RevokeByDelegationCall),
        TransferOwnership(TransferOwnershipCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for PermissionedEIP712ProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AttestByDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AttestByDelegation(decoded));
            }
            if let Ok(decoded)
                = <GetAttestTypeHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAttestTypeHash(decoded));
            }
            if let Ok(decoded)
                = <GetAttesterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAttester(decoded));
            }
            if let Ok(decoded)
                = <GetDomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetDomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <GetEASCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetEAS(decoded));
            }
            if let Ok(decoded)
                = <GetNameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetName(decoded));
            }
            if let Ok(decoded)
                = <GetRevokeTypeHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRevokeTypeHash(decoded));
            }
            if let Ok(decoded)
                = <MultiAttestByDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MultiAttestByDelegation(decoded));
            }
            if let Ok(decoded)
                = <MultiRevokeByDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MultiRevokeByDelegation(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <RevokeByDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevokeByDelegation(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PermissionedEIP712ProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AttestByDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAttestTypeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAttester(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEAS(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetName(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRevokeTypeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiAttestByDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiRevokeByDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeByDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PermissionedEIP712ProxyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestByDelegation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAttestTypeHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAttester(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDomainSeparator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetEAS(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetName(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRevokeTypeHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiAttestByDelegation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MultiRevokeByDelegation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeByDelegation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestByDelegationCall> for PermissionedEIP712ProxyCalls {
        fn from(value: AttestByDelegationCall) -> Self {
            Self::AttestByDelegation(value)
        }
    }
    impl ::core::convert::From<GetAttestTypeHashCall> for PermissionedEIP712ProxyCalls {
        fn from(value: GetAttestTypeHashCall) -> Self {
            Self::GetAttestTypeHash(value)
        }
    }
    impl ::core::convert::From<GetAttesterCall> for PermissionedEIP712ProxyCalls {
        fn from(value: GetAttesterCall) -> Self {
            Self::GetAttester(value)
        }
    }
    impl ::core::convert::From<GetDomainSeparatorCall> for PermissionedEIP712ProxyCalls {
        fn from(value: GetDomainSeparatorCall) -> Self {
            Self::GetDomainSeparator(value)
        }
    }
    impl ::core::convert::From<GetEASCall> for PermissionedEIP712ProxyCalls {
        fn from(value: GetEASCall) -> Self {
            Self::GetEAS(value)
        }
    }
    impl ::core::convert::From<GetNameCall> for PermissionedEIP712ProxyCalls {
        fn from(value: GetNameCall) -> Self {
            Self::GetName(value)
        }
    }
    impl ::core::convert::From<GetRevokeTypeHashCall> for PermissionedEIP712ProxyCalls {
        fn from(value: GetRevokeTypeHashCall) -> Self {
            Self::GetRevokeTypeHash(value)
        }
    }
    impl ::core::convert::From<MultiAttestByDelegationCall>
    for PermissionedEIP712ProxyCalls {
        fn from(value: MultiAttestByDelegationCall) -> Self {
            Self::MultiAttestByDelegation(value)
        }
    }
    impl ::core::convert::From<MultiRevokeByDelegationCall>
    for PermissionedEIP712ProxyCalls {
        fn from(value: MultiRevokeByDelegationCall) -> Self {
            Self::MultiRevokeByDelegation(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PermissionedEIP712ProxyCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PermissionedEIP712ProxyCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RevokeByDelegationCall> for PermissionedEIP712ProxyCalls {
        fn from(value: RevokeByDelegationCall) -> Self {
            Self::RevokeByDelegation(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PermissionedEIP712ProxyCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<VersionCall> for PermissionedEIP712ProxyCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `attestByDelegation` function with signature `attestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256),(uint8,bytes32,bytes32),address,uint64))` and selector `0x3c042715`
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
    ///Container type for all return fields from the `getAttestTypeHash` function with signature `getAttestTypeHash()` and selector `0x12b11a17`
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
    pub struct GetAttestTypeHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getAttester` function with signature `getAttester(bytes32)` and selector `0x10d736d5`
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
    pub struct GetAttesterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDomainSeparator` function with signature `getDomainSeparator()` and selector `0xed24911d`
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
    pub struct GetDomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getEAS` function with signature `getEAS()` and selector `0x65c40b9c`
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
    pub struct GetEASReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getName` function with signature `getName()` and selector `0x17d7de7c`
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
    pub struct GetNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getRevokeTypeHash` function with signature `getRevokeTypeHash()` and selector `0xb83010d3`
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
    pub struct GetRevokeTypeHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `multiAttestByDelegation` function with signature `multiAttestByDelegation((bytes32,(address,uint64,bool,bytes32,bytes,uint256)[],(uint8,bytes32,bytes32)[],address,uint64)[])` and selector `0x95411525`
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
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
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
