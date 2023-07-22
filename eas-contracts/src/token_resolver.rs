pub use token_resolver::*;
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
pub mod token_resolver {
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
                        name: ::std::borrow::ToOwned::to_owned("targetToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC20"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("targetAmount"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
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
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TOKENRESOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01@`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa\re8\x03\x80a\re\x839\x81\x01`@\x81\x90Ra\x000\x91a\0\x9FV[`\x01`\x80R`\0`\xA0\x81\x90R`\xC0R\x82`\x01`\x01`\xA0\x1B\x03\x81\x16a\0gW`@QcA\xBC\x07\xFF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xE0R\x91\x90\x91\x16a\x01\0Ra\x01 RPa\0\xE2V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x9CW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\xB4W`\0\x80\xFD[\x83Qa\0\xBF\x81a\0\x87V[` \x85\x01Q\x90\x93Pa\0\xD0\x81a\0\x87V[\x80\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x0C4a\x011`\09`\0a\x05\n\x01R`\0a\x04\xE6\x01R`\0a\x04`\x01R`\0a\x01\x99\x01R`\0a\x01p\x01R`\0a\x01G\x01Ra\x0C4`\0\xF3\xFE`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\xCEF\xE0F\x11a\0CW\x80c\xCEF\xE0F\x14a\x01\x06W\x80c\xE4\x96\x17\xE1\x14a\x01\x1AW\x80c\xE6\x0C5\x05\x14a\x01-W`\0\x80\xFD[\x80cT\xFDMP\x14a\0\xA5W\x80c\x88\xE5\xB2\xD9\x14a\0\xD0W\x80c\x91\xDB\x0B~\x14a\0\xF3W`\0\x80\xFD[6a\0\xA0W`@Q\x7F\x15t\xF9\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\xFD[4\x80\x15a\0\xB1W`\0\x80\xFD[Pa\0\xBAa\x01@V[`@Qa\0\xC7\x91\x90a\t_V[`@Q\x80\x91\x03\x90\xF3[a\0\xE3a\0\xDE6`\x04a\t\xFCV[a\x01\xE3V[`@Q\x90\x15\x15\x81R` \x01a\0\xC7V[a\0\xE3a\x01\x016`\x04a\t\xFCV[a\x02\xABV[4\x80\x15a\x01\x12W`\0\x80\xFD[P`\0a\0\xE3V[a\0\xE3a\x01(6`\x04a\nhV[a\x03cV[a\0\xE3a\x01;6`\x04a\nhV[a\x03vV[``a\x01k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x8AV[a\x01\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x8AV[a\x01\xBD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x8AV[`@Q` \x01a\x01\xCF\x93\x92\x91\x90a\n\xABV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x01\xEDa\x04HV[\x834`\0[\x82\x81\x10\x15a\x02\x9BW`\0\x86\x86\x83\x81\x81\x10a\x02\x0EWa\x02\x0Ea\x0B!V[\x90P` \x02\x015\x90P\x82\x81\x11\x15a\x02QW`@Q\x7F\x11\x01\x12\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02~\x89\x89\x84\x81\x81\x10a\x02fWa\x02fa\x0B!V[\x90P` \x02\x81\x01\x90a\x02x\x91\x90a\x0BPV[P`\x01\x90V[a\x02\x8FW`\0\x94PPPPPa\x02\xA3V[\x90\x91\x03\x90`\x01\x01a\x01\xF2V[P`\x01\x92PPP[\x94\x93PPPPV[`\0a\x02\xB5a\x04HV[\x834`\0[\x82\x81\x10\x15a\x02\x9BW`\0\x86\x86\x83\x81\x81\x10a\x02\xD6Wa\x02\xD6a\x0B!V[\x90P` \x02\x015\x90P\x82\x81\x11\x15a\x03\x19W`@Q\x7F\x11\x01\x12\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03F\x89\x89\x84\x81\x81\x10a\x03.Wa\x03.a\x0B!V[\x90P` \x02\x81\x01\x90a\x03@\x91\x90a\x0BPV[\x82a\x04\xB9V[a\x03WW`\0\x94PPPPPa\x02\xA3V[\x90\x91\x03\x90`\x01\x01a\x02\xBAV[`\0a\x03ma\x04HV[`\x01[\x92\x91PPV[`\0a\x03\x80a\x04HV[a\x03p\x824a\x04\xB9V[```\0a\x03\x97\x83a\x057V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xB7Wa\x03\xB7a\x0B\x8EV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x03\xE1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x03\xEBWP\x93\x92PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xB7W`@Q\x7FL\xA8\x88g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\x05.a\x04\xCFa\x01\0\x85\x01`\xE0\x86\x01a\x0B\xBDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x900\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\x19V[P`\x01\x92\x91PPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x05\x80Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x05\xACWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x05\xCAWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x05\xE2Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x05\xF6Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x06\x08W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x03pW`\x01\x01\x92\x91PPV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x06\xAE\x90\x85\x90a\x06\xB4V[PPPPV[`\0a\x07\x16\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\xCA\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x07\xC5W\x80\x80` \x01\x90Q\x81\x01\x90a\x074\x91\x90a\x0B\xF3V[a\x07\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\x02\xA3\x84\x84`\0\x85\x85`\0\x80\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x87`@Qa\x07\xFE\x91\x90a\x0C\x15V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08;W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08@V[``\x91P[P\x91P\x91Pa\x08Q\x87\x83\x83\x87a\x08\\V[\x97\x96PPPPPPPV[``\x83\x15a\x08\xF2W\x82Q`\0\x03a\x08\xEBWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a\x08\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x07\xBCV[P\x81a\x02\xA3V[a\x02\xA3\x83\x83\x81Q\x15a\t\x07W\x81Q\x80\x83` \x01\xFD[\x80`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xBC\x91\x90a\t_V[`\0[\x83\x81\x10\x15a\tVW\x81\x81\x01Q\x83\x82\x01R` \x01a\t>V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\t~\x81`@\x85\x01` \x87\x01a\t;V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\t\xC2W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xDAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\t\xF5W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\n\x12W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n*W`\0\x80\xFD[a\n6\x88\x83\x89\x01a\t\xB0V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\nOW`\0\x80\xFD[Pa\n\\\x87\x82\x88\x01a\t\xB0V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a\nzW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x91W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\n\xA4W`\0\x80\xFD[\x93\x92PPPV[`\0\x84Qa\n\xBD\x81\x84` \x89\x01a\t;V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\n\xF9\x81`\x01\x85\x01` \x8A\x01a\t;V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x0B\x14\x81`\x02\x84\x01` \x88\x01a\t;V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xC1\x836\x03\x01\x81\x12a\x0B\x84W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\xCFW`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\xA4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\x05W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\n\xA4W`\0\x80\xFD[`\0\x82Qa\x0B\x84\x81\x84` \x87\x01a\t;V\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The bytecode of the contract.
    pub static TOKENRESOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\xCEF\xE0F\x11a\0CW\x80c\xCEF\xE0F\x14a\x01\x06W\x80c\xE4\x96\x17\xE1\x14a\x01\x1AW\x80c\xE6\x0C5\x05\x14a\x01-W`\0\x80\xFD[\x80cT\xFDMP\x14a\0\xA5W\x80c\x88\xE5\xB2\xD9\x14a\0\xD0W\x80c\x91\xDB\x0B~\x14a\0\xF3W`\0\x80\xFD[6a\0\xA0W`@Q\x7F\x15t\xF9\xF3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\xFD[4\x80\x15a\0\xB1W`\0\x80\xFD[Pa\0\xBAa\x01@V[`@Qa\0\xC7\x91\x90a\t_V[`@Q\x80\x91\x03\x90\xF3[a\0\xE3a\0\xDE6`\x04a\t\xFCV[a\x01\xE3V[`@Q\x90\x15\x15\x81R` \x01a\0\xC7V[a\0\xE3a\x01\x016`\x04a\t\xFCV[a\x02\xABV[4\x80\x15a\x01\x12W`\0\x80\xFD[P`\0a\0\xE3V[a\0\xE3a\x01(6`\x04a\nhV[a\x03cV[a\0\xE3a\x01;6`\x04a\nhV[a\x03vV[``a\x01k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x8AV[a\x01\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x8AV[a\x01\xBD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03\x8AV[`@Q` \x01a\x01\xCF\x93\x92\x91\x90a\n\xABV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`\0a\x01\xEDa\x04HV[\x834`\0[\x82\x81\x10\x15a\x02\x9BW`\0\x86\x86\x83\x81\x81\x10a\x02\x0EWa\x02\x0Ea\x0B!V[\x90P` \x02\x015\x90P\x82\x81\x11\x15a\x02QW`@Q\x7F\x11\x01\x12\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02~\x89\x89\x84\x81\x81\x10a\x02fWa\x02fa\x0B!V[\x90P` \x02\x81\x01\x90a\x02x\x91\x90a\x0BPV[P`\x01\x90V[a\x02\x8FW`\0\x94PPPPPa\x02\xA3V[\x90\x91\x03\x90`\x01\x01a\x01\xF2V[P`\x01\x92PPP[\x94\x93PPPPV[`\0a\x02\xB5a\x04HV[\x834`\0[\x82\x81\x10\x15a\x02\x9BW`\0\x86\x86\x83\x81\x81\x10a\x02\xD6Wa\x02\xD6a\x0B!V[\x90P` \x02\x015\x90P\x82\x81\x11\x15a\x03\x19W`@Q\x7F\x11\x01\x12\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03F\x89\x89\x84\x81\x81\x10a\x03.Wa\x03.a\x0B!V[\x90P` \x02\x81\x01\x90a\x03@\x91\x90a\x0BPV[\x82a\x04\xB9V[a\x03WW`\0\x94PPPPPa\x02\xA3V[\x90\x91\x03\x90`\x01\x01a\x02\xBAV[`\0a\x03ma\x04HV[`\x01[\x92\x91PPV[`\0a\x03\x80a\x04HV[a\x03p\x824a\x04\xB9V[```\0a\x03\x97\x83a\x057V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xB7Wa\x03\xB7a\x0B\x8EV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x03\xE1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x03\xEBWP\x93\x92PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xB7W`@Q\x7FL\xA8\x88g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\0a\x05.a\x04\xCFa\x01\0\x85\x01`\xE0\x86\x01a\x0B\xBDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x900\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\x19V[P`\x01\x92\x91PPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x05\x80Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x05\xACWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x05\xCAWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x05\xE2Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x05\xF6Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x06\x08W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x03pW`\x01\x01\x92\x91PPV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x06\xAE\x90\x85\x90a\x06\xB4V[PPPPV[`\0a\x07\x16\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x07\xCA\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x07\xC5W\x80\x80` \x01\x90Q\x81\x01\x90a\x074\x91\x90a\x0B\xF3V[a\x07\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\x02\xA3\x84\x84`\0\x85\x85`\0\x80\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x87`@Qa\x07\xFE\x91\x90a\x0C\x15V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08;W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08@V[``\x91P[P\x91P\x91Pa\x08Q\x87\x83\x83\x87a\x08\\V[\x97\x96PPPPPPPV[``\x83\x15a\x08\xF2W\x82Q`\0\x03a\x08\xEBWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a\x08\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x07\xBCV[P\x81a\x02\xA3V[a\x02\xA3\x83\x83\x81Q\x15a\t\x07W\x81Q\x80\x83` \x01\xFD[\x80`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xBC\x91\x90a\t_V[`\0[\x83\x81\x10\x15a\tVW\x81\x81\x01Q\x83\x82\x01R` \x01a\t>V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\t~\x81`@\x85\x01` \x87\x01a\t;V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\t\xC2W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xDAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\t\xF5W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\n\x12W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n*W`\0\x80\xFD[a\n6\x88\x83\x89\x01a\t\xB0V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\nOW`\0\x80\xFD[Pa\n\\\x87\x82\x88\x01a\t\xB0V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a\nzW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x91W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\n\xA4W`\0\x80\xFD[\x93\x92PPPV[`\0\x84Qa\n\xBD\x81\x84` \x89\x01a\t;V[\x80\x83\x01\x90P\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x82R\x85Qa\n\xF9\x81`\x01\x85\x01` \x8A\x01a\t;V[`\x01\x92\x01\x91\x82\x01R\x83Qa\x0B\x14\x81`\x02\x84\x01` \x88\x01a\t;V[\x01`\x02\x01\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xC1\x836\x03\x01\x81\x12a\x0B\x84W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\xCFW`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\xA4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\x05W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\n\xA4W`\0\x80\xFD[`\0\x82Qa\x0B\x84\x81\x84` \x87\x01a\t;V\xFE\xA1dsolcC\0\x08\x13\0\n";
    /// The deployed bytecode of the contract.
    pub static TOKENRESOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TokenResolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TokenResolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TokenResolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TokenResolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TokenResolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TokenResolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TokenResolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TOKENRESOLVER_ABI.clone(),
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
                TOKENRESOLVER_ABI.clone(),
                TOKENRESOLVER_BYTECODE.clone().into(),
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
    for TokenResolver<M> {
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TokenResolverErrors {
        AccessDenied(AccessDenied),
        InsufficientValue(InsufficientValue),
        InvalidEAS(InvalidEAS),
        NotPayable(NotPayable),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for TokenResolverErrors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TokenResolverErrors {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for TokenResolverErrors {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for TokenResolverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessDenied(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEAS(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for TokenResolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessDenied> for TokenResolverErrors {
        fn from(value: AccessDenied) -> Self {
            Self::AccessDenied(value)
        }
    }
    impl ::core::convert::From<InsufficientValue> for TokenResolverErrors {
        fn from(value: InsufficientValue) -> Self {
            Self::InsufficientValue(value)
        }
    }
    impl ::core::convert::From<InvalidEAS> for TokenResolverErrors {
        fn from(value: InvalidEAS) -> Self {
            Self::InvalidEAS(value)
        }
    }
    impl ::core::convert::From<NotPayable> for TokenResolverErrors {
        fn from(value: NotPayable) -> Self {
            Self::NotPayable(value)
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
    pub enum TokenResolverCalls {
        Attest(AttestCall),
        IsPayable(IsPayableCall),
        MultiAttest(MultiAttestCall),
        MultiRevoke(MultiRevokeCall),
        Revoke(RevokeCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for TokenResolverCalls {
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
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TokenResolverCalls {
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
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for TokenResolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Attest(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiAttest(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiRevoke(element) => ::core::fmt::Display::fmt(element, f),
                Self::Revoke(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestCall> for TokenResolverCalls {
        fn from(value: AttestCall) -> Self {
            Self::Attest(value)
        }
    }
    impl ::core::convert::From<IsPayableCall> for TokenResolverCalls {
        fn from(value: IsPayableCall) -> Self {
            Self::IsPayable(value)
        }
    }
    impl ::core::convert::From<MultiAttestCall> for TokenResolverCalls {
        fn from(value: MultiAttestCall) -> Self {
            Self::MultiAttest(value)
        }
    }
    impl ::core::convert::From<MultiRevokeCall> for TokenResolverCalls {
        fn from(value: MultiRevokeCall) -> Self {
            Self::MultiRevoke(value)
        }
    }
    impl ::core::convert::From<RevokeCall> for TokenResolverCalls {
        fn from(value: RevokeCall) -> Self {
            Self::Revoke(value)
        }
    }
    impl ::core::convert::From<VersionCall> for TokenResolverCalls {
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
