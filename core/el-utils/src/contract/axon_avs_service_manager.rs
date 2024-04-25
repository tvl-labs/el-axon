pub use axon_avs_service_manager::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod axon_avs_service_manager {
    const _: () = {
        ::core::include_bytes!("/home/gao/dev/el-axon/avs-contract/AxonAVSServiceManager.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_avsDirectory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IAVSDirectory"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_registryCoordinator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IRegistryCoordinator",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stakeRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStakeRegistry"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_axonAVSTaskManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IAxonAVSTaskManager",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("avsDirectory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("avsDirectory"),
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
                    ::std::borrow::ToOwned::to_owned("axonAVSTaskManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("axonAVSTaskManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IAxonAVSTaskManager",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperatorFromAVS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deregisterOperatorFromAVS",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("getOperatorRestakedStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOperatorRestakedStrategies",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRestakeableStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRestakeableStrategies",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialOwner"),
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
                    ::std::borrow::ToOwned::to_owned("payForRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payForRange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rangePayments"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPaymentCoordinator.RangePayment[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("registerOperatorToAVS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerOperatorToAVS",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorSignature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithSaltAndExpiry",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("updateAVSMetadataURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateAVSMetadataURI",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_metadataURI"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static AXONAVSSERVICEMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1C(8\x03\x80b\0\x1C(\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01ZV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x80R`\0`\xA0\x81\x90R\x81\x85\x16`\xC0R\x90\x83\x16`\xE0R\x84\x90\x84\x84b\0\0db\0\0\x7FV[PPPP`\x01`\x01`\xA0\x1B\x03\x16a\x01\0RPb\0\x01\xC2\x91PPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01?W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01WW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01qW`\0\x80\xFD[\x84Qb\0\x01~\x81b\0\x01AV[` \x86\x01Q\x90\x94Pb\0\x01\x91\x81b\0\x01AV[`@\x86\x01Q\x90\x93Pb\0\x01\xA4\x81b\0\x01AV[``\x86\x01Q\x90\x92Pb\0\x01\xB7\x81b\0\x01AV[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x19\xB9b\0\x02o`\09`\0a\x01\x88\x01R`\0\x81\x81a\x06Z\x01R\x81\x81a\x07\xB6\x01R\x81\x81a\x08M\x01R\x81\x81a\x0C\xD9\x01R\x81\x81a\x0E]\x01Ra\x0E\xFC\x01R`\0\x81\x81a\x04\x85\x01R\x81\x81a\x05\x14\x01R\x81\x81a\x05\x94\x01R\x81\x81a\tO\x01R\x81\x81a\t\xEE\x01R\x81\x81a\x0C\x17\x01Ra\r\xB8\x01R`\0\x81\x81a\x03\x15\x01Ra\x03\xF3\x01R`\0\x81\x81`\xF9\x01R\x81\x81a\t\xAC\x01R\x81\x81a\nJ\x01Ra\n\xC9\x01Ra\x19\xB9`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\xA3d\xF4\xDA\x11a\0qW\x80c\xA3d\xF4\xDA\x14a\x01]W\x80c\xA9\x8F\xB3U\x14a\x01pW\x80c\xBC\xFB\x0C\x88\x14a\x01\x83W\x80c\xC4\xD6m\xE8\x14a\x01\xAAW\x80c\xE4\x81\xAF\x9D\x14a\x01\xBDW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xC5W`\0\x80\xFD[\x80c\x1BDU\x16\x14a\0\xB9W\x80c3\xCF\xB7\xB7\x14a\0\xCEW\x80ck:\xA7.\x14a\0\xF7W\x80cqP\x18\xA6\x14a\x011W\x80c\x8D\xA5\xCB[\x14a\x019W\x80c\x99&\xEE}\x14a\x01JW[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\x12_V[a\x01\xD8V[\0[a\0\xE1a\0\xDC6`\x04a\x12\xE9V[a\x04`V[`@Qa\0\xEE\x91\x90a\x13\rV[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\0\xCCa\t0V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01\x19V[a\0\xCCa\x01X6`\x04a\x14\x0FV[a\tDV[a\0\xCCa\x01k6`\x04a\x12\xE9V[a\t\xE3V[a\0\xCCa\x01~6`\x04a\x14\xBAV[a\n\xAAV[a\x01\x19\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xCCa\x01\xB86`\x04a\x12\xE9V[a\n\xFEV[a\0\xE1a\x0C\x11V[a\0\xCCa\x01\xD36`\x04a\x12\xE9V[a\x0F\xDBV[a\x01\xE0a\x10TV[`\0[\x81\x81\x10\x15a\x03\xDBW\x82\x82\x82\x81\x81\x10a\x01\xFDWa\x01\xFDa\x15\x0BV[\x90P` \x02\x81\x01\x90a\x02\x0F\x91\x90a\x15!V[a\x02 \x90`@\x81\x01\x90` \x01a\x12\xE9V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86\x86\x86\x81\x81\x10a\x02BWa\x02Ba\x15\x0BV[\x90P` \x02\x81\x01\x90a\x02T\x91\x90a\x15!V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x93\x90\x92\x16`$\x84\x01R\x015`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCF\x91\x90a\x15QV[P\x82\x82\x82\x81\x81\x10a\x02\xE2Wa\x02\xE2a\x15\x0BV[\x90P` \x02\x81\x01\x90a\x02\xF4\x91\x90a\x15!V[a\x03\x05\x90`@\x81\x01\x90` \x01a\x12\xE9V[`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85\x85\x81\x81\x10a\x03FWa\x03Fa\x15\x0BV[\x90P` \x02\x81\x01\x90a\x03X\x91\x90a\x15!V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x04\x84\x01R\x015`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCA\x91\x90a\x15QV[Pa\x03\xD4\x81a\x15\x89V[\x90Pa\x01\xE3V[P`@Qc\r\xA2*\x8B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x1BDU\x16\x90a\x04*\x90\x85\x90\x85\x90`\x04\x01a\x16=V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04XW=`\0\x80>=`\0\xFD[PPPPPPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF0\x91\x90a\x17KV[`@Qc\x87\x1E\xF0I`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\x1E\xF0I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x7F\x91\x90a\x17dV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15\x80a\x06\x19WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x14\x91\x90a\x17\x8DV[`\xFF\x16\x15[\x15a\x065WPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x92\x91PPV[`\0a\x06I\x82`\x01`\x01`\xC0\x1B\x03\x16a\x10\xAEV[\x90P`\0\x80[\x82Q\x81\x10\x15a\x07\x1FW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c<\xA5\xA5\xF5\x84\x83\x81Q\x81\x10a\x06\x99Wa\x06\x99a\x15\x0BV[\x01` \x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x01\x91\x90a\x17KV[a\x07\x0B\x90\x83a\x17\xB0V[\x91P\x80a\x07\x17\x81a\x15\x89V[\x91PPa\x06OV[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07;Wa\x07;a\x13ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07dW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x84Q\x81\x10\x15a\t#W`\0\x85\x82\x81Q\x81\x10a\x07\x88Wa\x07\x88a\x15\x0BV[\x01` \x01Q`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08!\x91\x90a\x17KV[\x90P`\0[\x81\x81\x10\x15a\t\rW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBF\x91\x90a\x17\xC8V[`\0\x01Q\x86\x86\x81Q\x81\x10a\x08\xD5Wa\x08\xD5a\x15\x0BV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\x08\xF7\x81a\x15\x89V[\x95PP\x80\x80a\t\x05\x90a\x15\x89V[\x91PPa\x08&V[PPP\x80\x80a\t\x1B\x90a\x15\x89V[\x91PPa\x07kV[P\x90\x97\x96PPPPPPPV[a\t8a\x10TV[a\tB`\0a\x11qV[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a\x18'V[`@Q\x80\x91\x03\x90\xFD[`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\x04*\x90\x85\x90\x85\x90`\x04\x01a\x18\xECV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a\x18'V[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x8FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xA3W=`\0\x80>=`\0\xFD[PPPPPV[a\n\xB2a\x10TV[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\nu\x90\x84\x90`\x04\x01a\x197V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0B\x1EWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0B8WP0;\x15\x80\x15a\x0B8WP`\0T`\xFF\x16`\x01\x14[a\x0B\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\t\x8CV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0B\xBEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0B\xC7\x82a\x11\xC3V[\x80\x15a\x0C\rW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x97\x91\x90a\x17\x8DV[`\xFF\x16\x90P\x80a\x0C\xB5WPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`\0\x80[\x82\x81\x10\x15a\rjW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rL\x91\x90a\x17KV[a\rV\x90\x83a\x17\xB0V[\x91P\x80a\rb\x81a\x15\x89V[\x91PPa\x0C\xB9V[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x86Wa\r\x86a\x13ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xAFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E8\x91\x90a\x17\x8DV[`\xFF\x16\x81\x10\x15a\x0F\xD1W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD0\x91\x90a\x17KV[\x90P`\0[\x81\x81\x10\x15a\x0F\xBCW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fn\x91\x90a\x17\xC8V[`\0\x01Q\x85\x85\x81Q\x81\x10a\x0F\x84Wa\x0F\x84a\x15\x0BV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\x0F\xA6\x81a\x15\x89V[\x94PP\x80\x80a\x0F\xB4\x90a\x15\x89V[\x91PPa\x0E\xD5V[PP\x80\x80a\x0F\xC9\x90a\x15\x89V[\x91PPa\r\xB6V[P\x90\x94\x93PPPPV[a\x0F\xE3a\x10TV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\t\x8CV[a\x10Q\x81a\x11qV[PV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\tBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\x8CV[```\0\x80a\x10\xBC\x84a\x12.V[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xD8Wa\x10\xD8a\x13ZV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x11\x02W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x11\x1AWPa\x01\0\x81\x10[\x15a\x0F\xD1W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x11aW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x11CWa\x11Ca\x15\x0BV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x11j\x81a\x15\x89V[\x90Pa\x11\tV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x10HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\t\x8CV[`\0\x80[\x82\x15a\x12YWa\x12C`\x01\x84a\x19JV[\x90\x92\x16\x91\x80a\x12Q\x81a\x19aV[\x91PPa\x122V[\x92\x91PPV[`\0\x80` \x83\x85\x03\x12\x15a\x12rW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\x8AW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x12\x9EW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12\xADW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x12\xC2W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10QW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12\xFBW`\0\x80\xFD[\x815a\x13\x06\x81a\x12\xD4V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x13NW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x13)V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\x93Wa\x13\x93a\x13ZV[`@R\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x13\xB4Wa\x13\xB4a\x13ZV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x13\xDCWa\x13\xDCa\x13ZV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x13\xF5W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\"W`\0\x80\xFD[\x825a\x14-\x81a\x12\xD4V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14JW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x14^W`\0\x80\xFD[a\x14fa\x13pV[\x825\x82\x81\x11\x15a\x14uW`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x14\x88W`\0\x80\xFD[a\x14\x97\x87\x835` \x85\x01a\x13\x99V[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\xCCW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x14\xF4W`\0\x80\xFD[a\x15\x03\x84\x825` \x84\x01a\x13\x99V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`\x9E\x19\x836\x03\x01\x81\x12a\x157W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[\x805a\x15L\x81a\x12\xD4V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x15cW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x06W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x15\x9DWa\x15\x9Da\x15sV[P`\x01\x01\x90V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10QW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a\x16\x1EW\x815a\x15\xE1\x81a\x12\xD4V[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x81\x83\x015a\x15\xF9\x81a\x15\xA4V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a\x15\xCEV[P\x94\x95\x94PPPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15LW`\0\x80\xFD[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01`\x05\x86\x90\x1B\x85\x01\x82\x01\x87\x85[\x88\x81\x10\x15a\x17=W\x87\x83\x03`?\x19\x01\x84R\x8156\x8B\x90\x03`\x9E\x19\x01\x81\x12a\x16\x82W`\0\x80\xFD[\x8A\x01`\xA0\x8156\x83\x90\x03`\x1E\x19\x01\x81\x12a\x16\x9BW`\0\x80\xFD[\x82\x01\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xB4W`\0\x80\xFD[\x80`\x06\x1B6\x03\x84\x13\x15a\x16\xC6W`\0\x80\xFD[\x82\x87Ra\x16\xD8\x83\x88\x01\x82\x8C\x85\x01a\x15\xBEV[\x92PPPa\x16\xE7\x88\x83\x01a\x15AV[`\x01`\x01`\xA0\x1B\x03\x16\x88\x86\x01R\x81\x87\x015\x87\x86\x01R``a\x17\t\x81\x84\x01a\x16)V[c\xFF\xFF\xFF\xFF\x16\x90\x86\x01R`\x80a\x17 \x83\x82\x01a\x16)V[c\xFF\xFF\xFF\xFF\x16\x95\x01\x94\x90\x94RP\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x16\\V[P\x90\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x17]W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x17vW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x13\x06W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17\x9FW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x13\x06W`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a\x17\xC3Wa\x17\xC3a\x15sV[P\x01\x90V[`\0`@\x82\x84\x03\x12\x15a\x17\xDAW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x17\xFDWa\x17\xFDa\x13ZV[`@R\x82Qa\x18\x0B\x81a\x12\xD4V[\x81R` \x83\x01Qa\x18\x1B\x81a\x15\xA4V[` \x82\x01R\x93\x92PPPV[` \x80\x82R`R\x90\x82\x01R\x7FServiceManagerBase.onlyRegistryC`@\x82\x01R\x7Foordinator: caller is not the re``\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x18\xC5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x18\xA9V[\x81\x81\x11\x15a\x18\xD7W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra\x19\x16`\xA0\x84\x01\x82a\x18\x9FV[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[` \x81R`\0a\x13\x06` \x83\x01\x84a\x18\x9FV[`\0\x82\x82\x10\x15a\x19\\Wa\x19\\a\x15sV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x19yWa\x19ya\x15sV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 IZZ\x87\xD5Z\xF5(\x9D\x8E\xF0b\xEE\x1F\x18\x11=\x9B\xC9\xCC$\x0F)\xF0\xC0\xBB(\xEF|\x0E*\xB6dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static AXONAVSSERVICEMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\xA3d\xF4\xDA\x11a\0qW\x80c\xA3d\xF4\xDA\x14a\x01]W\x80c\xA9\x8F\xB3U\x14a\x01pW\x80c\xBC\xFB\x0C\x88\x14a\x01\x83W\x80c\xC4\xD6m\xE8\x14a\x01\xAAW\x80c\xE4\x81\xAF\x9D\x14a\x01\xBDW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xC5W`\0\x80\xFD[\x80c\x1BDU\x16\x14a\0\xB9W\x80c3\xCF\xB7\xB7\x14a\0\xCEW\x80ck:\xA7.\x14a\0\xF7W\x80cqP\x18\xA6\x14a\x011W\x80c\x8D\xA5\xCB[\x14a\x019W\x80c\x99&\xEE}\x14a\x01JW[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\x12_V[a\x01\xD8V[\0[a\0\xE1a\0\xDC6`\x04a\x12\xE9V[a\x04`V[`@Qa\0\xEE\x91\x90a\x13\rV[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\0\xCCa\t0V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01\x19V[a\0\xCCa\x01X6`\x04a\x14\x0FV[a\tDV[a\0\xCCa\x01k6`\x04a\x12\xE9V[a\t\xE3V[a\0\xCCa\x01~6`\x04a\x14\xBAV[a\n\xAAV[a\x01\x19\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xCCa\x01\xB86`\x04a\x12\xE9V[a\n\xFEV[a\0\xE1a\x0C\x11V[a\0\xCCa\x01\xD36`\x04a\x12\xE9V[a\x0F\xDBV[a\x01\xE0a\x10TV[`\0[\x81\x81\x10\x15a\x03\xDBW\x82\x82\x82\x81\x81\x10a\x01\xFDWa\x01\xFDa\x15\x0BV[\x90P` \x02\x81\x01\x90a\x02\x0F\x91\x90a\x15!V[a\x02 \x90`@\x81\x01\x90` \x01a\x12\xE9V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86\x86\x86\x81\x81\x10a\x02BWa\x02Ba\x15\x0BV[\x90P` \x02\x81\x01\x90a\x02T\x91\x90a\x15!V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x93\x90\x92\x16`$\x84\x01R\x015`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCF\x91\x90a\x15QV[P\x82\x82\x82\x81\x81\x10a\x02\xE2Wa\x02\xE2a\x15\x0BV[\x90P` \x02\x81\x01\x90a\x02\xF4\x91\x90a\x15!V[a\x03\x05\x90`@\x81\x01\x90` \x01a\x12\xE9V[`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85\x85\x81\x81\x10a\x03FWa\x03Fa\x15\x0BV[\x90P` \x02\x81\x01\x90a\x03X\x91\x90a\x15!V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x04\x84\x01R\x015`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCA\x91\x90a\x15QV[Pa\x03\xD4\x81a\x15\x89V[\x90Pa\x01\xE3V[P`@Qc\r\xA2*\x8B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x1BDU\x16\x90a\x04*\x90\x85\x90\x85\x90`\x04\x01a\x16=V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04XW=`\0\x80>=`\0\xFD[PPPPPPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF0\x91\x90a\x17KV[`@Qc\x87\x1E\xF0I`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\x1E\xF0I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05[W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x7F\x91\x90a\x17dV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15\x80a\x06\x19WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x14\x91\x90a\x17\x8DV[`\xFF\x16\x15[\x15a\x065WPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x92\x91PPV[`\0a\x06I\x82`\x01`\x01`\xC0\x1B\x03\x16a\x10\xAEV[\x90P`\0\x80[\x82Q\x81\x10\x15a\x07\x1FW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c<\xA5\xA5\xF5\x84\x83\x81Q\x81\x10a\x06\x99Wa\x06\x99a\x15\x0BV[\x01` \x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x01\x91\x90a\x17KV[a\x07\x0B\x90\x83a\x17\xB0V[\x91P\x80a\x07\x17\x81a\x15\x89V[\x91PPa\x06OV[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07;Wa\x07;a\x13ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07dW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x84Q\x81\x10\x15a\t#W`\0\x85\x82\x81Q\x81\x10a\x07\x88Wa\x07\x88a\x15\x0BV[\x01` \x01Q`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08!\x91\x90a\x17KV[\x90P`\0[\x81\x81\x10\x15a\t\rW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBF\x91\x90a\x17\xC8V[`\0\x01Q\x86\x86\x81Q\x81\x10a\x08\xD5Wa\x08\xD5a\x15\x0BV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\x08\xF7\x81a\x15\x89V[\x95PP\x80\x80a\t\x05\x90a\x15\x89V[\x91PPa\x08&V[PPP\x80\x80a\t\x1B\x90a\x15\x89V[\x91PPa\x07kV[P\x90\x97\x96PPPPPPPV[a\t8a\x10TV[a\tB`\0a\x11qV[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a\x18'V[`@Q\x80\x91\x03\x90\xFD[`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\x04*\x90\x85\x90\x85\x90`\x04\x01a\x18\xECV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\x8C\x90a\x18'V[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x8FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xA3W=`\0\x80>=`\0\xFD[PPPPPV[a\n\xB2a\x10TV[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\nu\x90\x84\x90`\x04\x01a\x197V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0B\x1EWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0B8WP0;\x15\x80\x15a\x0B8WP`\0T`\xFF\x16`\x01\x14[a\x0B\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\t\x8CV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0B\xBEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0B\xC7\x82a\x11\xC3V[\x80\x15a\x0C\rW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x97\x91\x90a\x17\x8DV[`\xFF\x16\x90P\x80a\x0C\xB5WPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`\0\x80[\x82\x81\x10\x15a\rjW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rL\x91\x90a\x17KV[a\rV\x90\x83a\x17\xB0V[\x91P\x80a\rb\x81a\x15\x89V[\x91PPa\x0C\xB9V[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x86Wa\r\x86a\x13ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xAFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E8\x91\x90a\x17\x8DV[`\xFF\x16\x81\x10\x15a\x0F\xD1W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD0\x91\x90a\x17KV[\x90P`\0[\x81\x81\x10\x15a\x0F\xBCW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fn\x91\x90a\x17\xC8V[`\0\x01Q\x85\x85\x81Q\x81\x10a\x0F\x84Wa\x0F\x84a\x15\x0BV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\x0F\xA6\x81a\x15\x89V[\x94PP\x80\x80a\x0F\xB4\x90a\x15\x89V[\x91PPa\x0E\xD5V[PP\x80\x80a\x0F\xC9\x90a\x15\x89V[\x91PPa\r\xB6V[P\x90\x94\x93PPPPV[a\x0F\xE3a\x10TV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\t\x8CV[a\x10Q\x81a\x11qV[PV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\tBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\x8CV[```\0\x80a\x10\xBC\x84a\x12.V[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xD8Wa\x10\xD8a\x13ZV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x11\x02W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x11\x1AWPa\x01\0\x81\x10[\x15a\x0F\xD1W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x11aW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x11CWa\x11Ca\x15\x0BV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x11j\x81a\x15\x89V[\x90Pa\x11\tV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x10HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\t\x8CV[`\0\x80[\x82\x15a\x12YWa\x12C`\x01\x84a\x19JV[\x90\x92\x16\x91\x80a\x12Q\x81a\x19aV[\x91PPa\x122V[\x92\x91PPV[`\0\x80` \x83\x85\x03\x12\x15a\x12rW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\x8AW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x12\x9EW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x12\xADW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x12\xC2W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10QW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12\xFBW`\0\x80\xFD[\x815a\x13\x06\x81a\x12\xD4V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x13NW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x13)V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\x93Wa\x13\x93a\x13ZV[`@R\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x13\xB4Wa\x13\xB4a\x13ZV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x13\xDCWa\x13\xDCa\x13ZV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x13\xF5W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\"W`\0\x80\xFD[\x825a\x14-\x81a\x12\xD4V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14JW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x14^W`\0\x80\xFD[a\x14fa\x13pV[\x825\x82\x81\x11\x15a\x14uW`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x14\x88W`\0\x80\xFD[a\x14\x97\x87\x835` \x85\x01a\x13\x99V[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\xCCW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x14\xF4W`\0\x80\xFD[a\x15\x03\x84\x825` \x84\x01a\x13\x99V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`\x9E\x19\x836\x03\x01\x81\x12a\x157W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[\x805a\x15L\x81a\x12\xD4V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x15cW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x06W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x15\x9DWa\x15\x9Da\x15sV[P`\x01\x01\x90V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10QW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a\x16\x1EW\x815a\x15\xE1\x81a\x12\xD4V[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x81\x83\x015a\x15\xF9\x81a\x15\xA4V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a\x15\xCEV[P\x94\x95\x94PPPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15LW`\0\x80\xFD[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01`\x05\x86\x90\x1B\x85\x01\x82\x01\x87\x85[\x88\x81\x10\x15a\x17=W\x87\x83\x03`?\x19\x01\x84R\x8156\x8B\x90\x03`\x9E\x19\x01\x81\x12a\x16\x82W`\0\x80\xFD[\x8A\x01`\xA0\x8156\x83\x90\x03`\x1E\x19\x01\x81\x12a\x16\x9BW`\0\x80\xFD[\x82\x01\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xB4W`\0\x80\xFD[\x80`\x06\x1B6\x03\x84\x13\x15a\x16\xC6W`\0\x80\xFD[\x82\x87Ra\x16\xD8\x83\x88\x01\x82\x8C\x85\x01a\x15\xBEV[\x92PPPa\x16\xE7\x88\x83\x01a\x15AV[`\x01`\x01`\xA0\x1B\x03\x16\x88\x86\x01R\x81\x87\x015\x87\x86\x01R``a\x17\t\x81\x84\x01a\x16)V[c\xFF\xFF\xFF\xFF\x16\x90\x86\x01R`\x80a\x17 \x83\x82\x01a\x16)V[c\xFF\xFF\xFF\xFF\x16\x95\x01\x94\x90\x94RP\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x16\\V[P\x90\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x17]W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x17vW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x13\x06W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x17\x9FW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x13\x06W`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a\x17\xC3Wa\x17\xC3a\x15sV[P\x01\x90V[`\0`@\x82\x84\x03\x12\x15a\x17\xDAW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x17\xFDWa\x17\xFDa\x13ZV[`@R\x82Qa\x18\x0B\x81a\x12\xD4V[\x81R` \x83\x01Qa\x18\x1B\x81a\x15\xA4V[` \x82\x01R\x93\x92PPPV[` \x80\x82R`R\x90\x82\x01R\x7FServiceManagerBase.onlyRegistryC`@\x82\x01R\x7Foordinator: caller is not the re``\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x18\xC5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x18\xA9V[\x81\x81\x11\x15a\x18\xD7W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra\x19\x16`\xA0\x84\x01\x82a\x18\x9FV[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[` \x81R`\0a\x13\x06` \x83\x01\x84a\x18\x9FV[`\0\x82\x82\x10\x15a\x19\\Wa\x19\\a\x15sV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x19yWa\x19ya\x15sV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 IZZ\x87\xD5Z\xF5(\x9D\x8E\xF0b\xEE\x1F\x18\x11=\x9B\xC9\xCC$\x0F)\xF0\xC0\xBB(\xEF|\x0E*\xB6dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static AXONAVSSERVICEMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct AxonAVSServiceManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AxonAVSServiceManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AxonAVSServiceManager<M> {
        type Target = ::ethers::contract::Contract<M>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AxonAVSServiceManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AxonAVSServiceManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AxonAVSServiceManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AxonAVSServiceManager<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                AXONAVSSERVICEMANAGER_ABI.clone(),
                client,
            ))
        }

        /// Constructs the general purpose `Deployer` instance based on the
        /// provided constructor arguments and sends it. Returns a new
        /// instance of a deployer that returns an instance of this contract
        /// after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the
        ///   argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract
        /// instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the
        /// `greeter.json` artifact.
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
                AXONAVSSERVICEMANAGER_ABI.clone(),
                AXONAVSSERVICEMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }

        /// Calls the contract's `avsDirectory` (0x6b3aa72e) function
        pub fn avs_directory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([107, 58, 167, 46], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `axonAVSTaskManager` (0xbcfb0c88) function
        pub fn axon_avs_task_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([188, 251, 12, 136], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `deregisterOperatorFromAVS` (0xa364f4da)
        /// function
        pub fn deregister_operator_from_avs(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 100, 244, 218], operator)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `getOperatorRestakedStrategies` (0x33cfb7b7)
        /// function
        pub fn get_operator_restaked_strategies(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([51, 207, 183, 183], operator)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `getRestakeableStrategies` (0xe481af9d)
        /// function
        pub fn get_restakeable_strategies(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([228, 129, 175, 157], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], initial_owner)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `payForRange` (0x1b445516) function
        pub fn pay_for_range(
            &self,
            range_payments: ::std::vec::Vec<RangePayment>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 68, 85, 22], range_payments)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `registerOperatorToAVS` (0x9926ee7d) function
        pub fn register_operator_to_avs(
            &self,
            operator: ::ethers::core::types::Address,
            operator_signature: SignatureWithSaltAndExpiry,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 38, 238, 125], (operator, operator_signature))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `updateAVSMetadataURI` (0xa98fb355) function
        pub fn update_avs_metadata_uri(
            &self,
            metadata_uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 143, 179, 85], metadata_uri)
                .expect("method not found (this should never happen)")
        }

        /// Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }

        /// Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }

        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AxonAVSServiceManagerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for AxonAVSServiceManager<M>
    {
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
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner:      ::ethers::core::types::Address,
    }
    /// Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AxonAVSServiceManagerEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for AxonAVSServiceManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(AxonAVSServiceManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AxonAVSServiceManagerEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AxonAVSServiceManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for AxonAVSServiceManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for AxonAVSServiceManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    /// Container type for all input parameters for the `avsDirectory` function
    /// with signature `avsDirectory()` and selector `0x6b3aa72e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "avsDirectory", abi = "avsDirectory()")]
    pub struct AvsDirectoryCall;
    /// Container type for all input parameters for the `axonAVSTaskManager`
    /// function with signature `axonAVSTaskManager()` and selector `0xbcfb0c88`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "axonAVSTaskManager", abi = "axonAVSTaskManager()")]
    pub struct AxonAVSTaskManagerCall;
    /// Container type for all input parameters for the
    /// `deregisterOperatorFromAVS` function with signature
    /// `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "deregisterOperatorFromAVS",
        abi = "deregisterOperatorFromAVS(address)"
    )]
    pub struct DeregisterOperatorFromAVSCall {
        pub operator: ::ethers::core::types::Address,
    }
    /// Container type for all input parameters for the
    /// `getOperatorRestakedStrategies` function with signature
    /// `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getOperatorRestakedStrategies",
        abi = "getOperatorRestakedStrategies(address)"
    )]
    pub struct GetOperatorRestakedStrategiesCall {
        pub operator: ::ethers::core::types::Address,
    }
    /// Container type for all input parameters for the
    /// `getRestakeableStrategies` function with signature
    /// `getRestakeableStrategies()` and selector `0xe481af9d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRestakeableStrategies", abi = "getRestakeableStrategies()")]
    pub struct GetRestakeableStrategiesCall;
    /// Container type for all input parameters for the `initialize` function
    /// with signature `initialize(address)` and selector `0xc4d66de8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `owner` function with
    /// signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    /// Container type for all input parameters for the `payForRange` function
    /// with signature
    /// `payForRange(((address,uint96)[],address,uint256,uint32,uint32)[])` and
    /// selector `0x1b445516`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "payForRange",
        abi = "payForRange(((address,uint96)[],address,uint256,uint32,uint32)[])"
    )]
    pub struct PayForRangeCall {
        pub range_payments: ::std::vec::Vec<RangePayment>,
    }
    /// Container type for all input parameters for the `registerOperatorToAVS`
    /// function with signature
    /// `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector
    /// `0x9926ee7d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "registerOperatorToAVS",
        abi = "registerOperatorToAVS(address,(bytes,bytes32,uint256))"
    )]
    pub struct RegisterOperatorToAVSCall {
        pub operator:           ::ethers::core::types::Address,
        pub operator_signature: SignatureWithSaltAndExpiry,
    }
    /// Container type for all input parameters for the `renounceOwnership`
    /// function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    /// Container type for all input parameters for the `transferOwnership`
    /// function with signature `transferOwnership(address)` and selector
    /// `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `updateAVSMetadataURI`
    /// function with signature `updateAVSMetadataURI(string)` and selector
    /// `0xa98fb355`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateAVSMetadataURI", abi = "updateAVSMetadataURI(string)")]
    pub struct UpdateAVSMetadataURICall {
        pub metadata_uri: ::std::string::String,
    }
    /// Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AxonAVSServiceManagerCalls {
        AvsDirectory(AvsDirectoryCall),
        AxonAVSTaskManager(AxonAVSTaskManagerCall),
        DeregisterOperatorFromAVS(DeregisterOperatorFromAVSCall),
        GetOperatorRestakedStrategies(GetOperatorRestakedStrategiesCall),
        GetRestakeableStrategies(GetRestakeableStrategiesCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        PayForRange(PayForRangeCall),
        RegisterOperatorToAVS(RegisterOperatorToAVSCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateAVSMetadataURI(UpdateAVSMetadataURICall),
    }
    impl ::ethers::core::abi::AbiDecode for AxonAVSServiceManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AvsDirectoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AvsDirectory(decoded));
            }
            if let Ok(decoded) =
                <AxonAVSTaskManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AxonAVSTaskManager(decoded));
            }
            if let Ok(decoded) =
                <DeregisterOperatorFromAVSCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterOperatorFromAVS(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorRestakedStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorRestakedStrategies(decoded));
            }
            if let Ok(decoded) =
                <GetRestakeableStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRestakeableStrategies(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PayForRangeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PayForRange(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorToAVSCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperatorToAVS(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateAVSMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateAVSMetadataURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AxonAVSServiceManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AvsDirectory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AxonAVSTaskManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorFromAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorRestakedStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRestakeableStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayForRange(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterOperatorToAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateAVSMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AxonAVSServiceManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AvsDirectory(element) => ::core::fmt::Display::fmt(element, f),
                Self::AxonAVSTaskManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperatorFromAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorRestakedStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRestakeableStrategies(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayForRange(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorToAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAVSMetadataURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AvsDirectoryCall> for AxonAVSServiceManagerCalls {
        fn from(value: AvsDirectoryCall) -> Self {
            Self::AvsDirectory(value)
        }
    }
    impl ::core::convert::From<AxonAVSTaskManagerCall> for AxonAVSServiceManagerCalls {
        fn from(value: AxonAVSTaskManagerCall) -> Self {
            Self::AxonAVSTaskManager(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorFromAVSCall> for AxonAVSServiceManagerCalls {
        fn from(value: DeregisterOperatorFromAVSCall) -> Self {
            Self::DeregisterOperatorFromAVS(value)
        }
    }
    impl ::core::convert::From<GetOperatorRestakedStrategiesCall> for AxonAVSServiceManagerCalls {
        fn from(value: GetOperatorRestakedStrategiesCall) -> Self {
            Self::GetOperatorRestakedStrategies(value)
        }
    }
    impl ::core::convert::From<GetRestakeableStrategiesCall> for AxonAVSServiceManagerCalls {
        fn from(value: GetRestakeableStrategiesCall) -> Self {
            Self::GetRestakeableStrategies(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for AxonAVSServiceManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AxonAVSServiceManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PayForRangeCall> for AxonAVSServiceManagerCalls {
        fn from(value: PayForRangeCall) -> Self {
            Self::PayForRange(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorToAVSCall> for AxonAVSServiceManagerCalls {
        fn from(value: RegisterOperatorToAVSCall) -> Self {
            Self::RegisterOperatorToAVS(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AxonAVSServiceManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AxonAVSServiceManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateAVSMetadataURICall> for AxonAVSServiceManagerCalls {
        fn from(value: UpdateAVSMetadataURICall) -> Self {
            Self::UpdateAVSMetadataURI(value)
        }
    }
    /// Container type for all return fields from the `avsDirectory` function
    /// with signature `avsDirectory()` and selector `0x6b3aa72e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AvsDirectoryReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `axonAVSTaskManager`
    /// function with signature `axonAVSTaskManager()` and selector `0xbcfb0c88`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AxonAVSTaskManagerReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the
    /// `getOperatorRestakedStrategies` function with signature
    /// `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetOperatorRestakedStrategiesReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    /// Container type for all return fields from the `getRestakeableStrategies`
    /// function with signature `getRestakeableStrategies()` and selector
    /// `0xe481af9d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRestakeableStrategiesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    /// Container type for all return fields from the `owner` function with
    /// signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    /// `RangePayment((address,uint96)[],address,uint256,uint32,uint32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RangePayment {
        pub strategies_and_multipliers: ::std::vec::Vec<StrategyAndMultiplier>,
        pub token:                      ::ethers::core::types::Address,
        pub amount:                     ::ethers::core::types::U256,
        pub start_timestamp:            u32,
        pub duration:                   u32,
    }
    /// `StrategyAndMultiplier(address,uint96)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StrategyAndMultiplier {
        pub strategy:   ::ethers::core::types::Address,
        pub multiplier: u128,
    }
    /// `SignatureWithSaltAndExpiry(bytes,bytes32,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignatureWithSaltAndExpiry {
        pub signature: ::ethers::core::types::Bytes,
        pub salt:      [u8; 32],
        pub expiry:    ::ethers::core::types::U256,
    }
}
