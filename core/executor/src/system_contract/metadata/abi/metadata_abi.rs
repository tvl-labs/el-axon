pub use metadata_contract::*;
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
pub mod metadata_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions:   ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("appendMetadata"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name:             ::std::borrow::ToOwned::to_owned("appendMetadata"),
                        inputs:           ::std::vec![::ethers::core::abi::ethabi::Param {
                            name:          ::std::borrow::ToOwned::to_owned("metadata"),
                            kind:          ::ethers::core::abi::ethabi::ParamType::Tuple(
                                ::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                                        32usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                                        32usize
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                                        64usize
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                ],
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct MetadataType.Metadata",),
                            ),
                        },],
                        outputs:          ::std::vec![],
                        constant:         ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCkbRelatedInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name:             ::std::borrow::ToOwned::to_owned("setCkbRelatedInfo"),
                        inputs:           ::std::vec![::ethers::core::abi::ethabi::Param {
                            name:          ::std::borrow::ToOwned::to_owned("info"),
                            kind:          ::ethers::core::abi::ethabi::ParamType::Tuple(
                                ::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ],
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct MetadataType.CkbRelatedInfo",
                                ),
                            ),
                        },],
                        outputs:          ::std::vec![],
                        constant:         ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateConsensusConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name:             ::std::borrow::ToOwned::to_owned("updateConsensusConfig",),
                        inputs:           ::std::vec![::ethers::core::abi::ethabi::Param {
                            name:          ::std::borrow::ToOwned::to_owned("config"),
                            kind:          ::ethers::core::abi::ethabi::ParamType::Tuple(
                                ::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct MetadataType.ConsensusConfig",
                                ),
                            ),
                        },],
                        outputs:          ::std::vec![],
                        constant:         ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateValidatorList"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name:             ::std::borrow::ToOwned::to_owned("updateValidatorList",),
                        inputs:           ::std::vec![::ethers::core::abi::ethabi::Param {
                            name:          ::std::borrow::ToOwned::to_owned("validators"),
                            kind:          ::ethers::core::abi::ethabi::ParamType::Tuple(
                                ::std::vec![::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],),
                                    ),
                                ),],
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct MetadataType.ValidatorList",
                                ),
                            ),
                        },],
                        outputs:          ::std::vec![],
                        constant:         ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events:      ::std::collections::BTreeMap::new(),
            errors:      ::std::collections::BTreeMap::new(),
            receive:     false,
            fallback:    false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static METADATACONTRACT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct MetadataContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MetadataContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MetadataContract<M> {
        type Target = ::ethers::contract::Contract<M>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MetadataContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MetadataContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MetadataContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MetadataContract<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                METADATACONTRACT_ABI.clone(),
                client,
            ))
        }

        /// Calls the contract's `appendMetadata` (0x53ec79e6) function
        pub fn append_metadata(
            &self,
            metadata: Metadata,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 236, 121, 230], (metadata,))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `setCkbRelatedInfo` (0x804afc59) function
        pub fn set_ckb_related_info(
            &self,
            info: CkbRelatedInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 74, 252, 89], (info,))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `updateConsensusConfig` (0xb76fac01) function
        pub fn update_consensus_config(
            &self,
            config: ConsensusConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 111, 172, 1], (config,))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `updateValidatorList` (0xa9f73eaf) function
        pub fn update_validator_list(
            &self,
            validators: ValidatorList,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 247, 62, 175], (validators,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MetadataContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Container type for all input parameters for the `appendMetadata`
    /// function with signature
    /// `appendMetadata(((uint64,uint64),uint64,(bytes,bytes,address,uint32,
    /// uint32)[],(address,uint64)[],(uint64,uint64,uint64,uint64,uint64,uint64,
    /// uint64,uint64,uint64)))` and selector `0x53ec79e6`
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
        name = "appendMetadata",
        abi = "appendMetadata(((uint64,uint64),uint64,(bytes,bytes,address,uint32,uint32)[],(address,uint64)[],(uint64,uint64,uint64,uint64,uint64,uint64,uint64,uint64,uint64)))"
    )]
    pub struct AppendMetadataCall {
        pub metadata: Metadata,
    }
    /// Container type for all input parameters for the `setCkbRelatedInfo`
    /// function with signature
    /// `setCkbRelatedInfo((bytes32,bytes32,bytes32,bytes32,bytes32,bytes32))`
    /// and selector `0x804afc59`
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
        name = "setCkbRelatedInfo",
        abi = "setCkbRelatedInfo((bytes32,bytes32,bytes32,bytes32,bytes32,bytes32))"
    )]
    pub struct SetCkbRelatedInfoCall {
        pub info: CkbRelatedInfo,
    }
    /// Container type for all input parameters for the `updateConsensusConfig`
    /// function with signature
    /// `updateConsensusConfig((uint64,uint64,uint64,uint64,uint64,uint64,
    /// uint64,uint64,uint64))` and selector `0xb76fac01`
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
        name = "updateConsensusConfig",
        abi = "updateConsensusConfig((uint64,uint64,uint64,uint64,uint64,uint64,uint64,uint64,uint64))"
    )]
    pub struct UpdateConsensusConfigCall {
        pub config: ConsensusConfig,
    }
    /// Container type for all input parameters for the `updateValidatorList`
    /// function with signature
    /// `updateValidatorList(((bytes,bytes,address,uint32,uint32)[]))` and
    /// selector `0xa9f73eaf`
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
        name = "updateValidatorList",
        abi = "updateValidatorList(((bytes,bytes,address,uint32,uint32)[]))"
    )]
    pub struct UpdateValidatorListCall {
        pub validators: ValidatorList,
    }
    /// Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MetadataContractCalls {
        AppendMetadata(AppendMetadataCall),
        SetCkbRelatedInfo(SetCkbRelatedInfoCall),
        UpdateConsensusConfig(UpdateConsensusConfigCall),
        UpdateValidatorList(UpdateValidatorListCall),
    }
    impl ::ethers::core::abi::AbiDecode for MetadataContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AppendMetadataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AppendMetadata(decoded));
            }
            if let Ok(decoded) =
                <SetCkbRelatedInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetCkbRelatedInfo(decoded));
            }
            if let Ok(decoded) =
                <UpdateConsensusConfigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateConsensusConfig(decoded));
            }
            if let Ok(decoded) =
                <UpdateValidatorListCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateValidatorList(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MetadataContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AppendMetadata(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCkbRelatedInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateConsensusConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateValidatorList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MetadataContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AppendMetadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCkbRelatedInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateConsensusConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateValidatorList(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AppendMetadataCall> for MetadataContractCalls {
        fn from(value: AppendMetadataCall) -> Self {
            Self::AppendMetadata(value)
        }
    }
    impl ::core::convert::From<SetCkbRelatedInfoCall> for MetadataContractCalls {
        fn from(value: SetCkbRelatedInfoCall) -> Self {
            Self::SetCkbRelatedInfo(value)
        }
    }
    impl ::core::convert::From<UpdateConsensusConfigCall> for MetadataContractCalls {
        fn from(value: UpdateConsensusConfigCall) -> Self {
            Self::UpdateConsensusConfig(value)
        }
    }
    impl ::core::convert::From<UpdateValidatorListCall> for MetadataContractCalls {
        fn from(value: UpdateValidatorListCall) -> Self {
            Self::UpdateValidatorList(value)
        }
    }
    /// `CkbRelatedInfo(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)`
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
    pub struct CkbRelatedInfo {
        pub metadata_type_id:     [u8; 32],
        pub checkpoint_type_id:   [u8; 32],
        pub xudt_args:            [u8; 32],
        pub stake_smt_type_id:    [u8; 32],
        pub delegate_smt_type_id: [u8; 32],
        pub reward_smt_type_id:   [u8; 32],
    }
    /// `ConsensusConfig(uint64,uint64,uint64,uint64,uint64,uint64,uint64,
    /// uint64,uint64)`
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
    pub struct ConsensusConfig {
        pub propose_ratio:      u64,
        pub prevote_ratio:      u64,
        pub precommit_ratio:    u64,
        pub brake_ratio:        u64,
        pub tx_num_limit:       u64,
        pub max_tx_size:        u64,
        pub gas_limit:          u64,
        pub interval:           u64,
        pub max_contract_limit: u64,
    }
    /// `Metadata((uint64,uint64),uint64,(bytes,bytes,address,uint32,uint32)[],
    /// (address,uint64)[],(uint64,uint64,uint64,uint64,uint64,uint64,uint64,
    /// uint64,uint64))`
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
    pub struct Metadata {
        pub version:          MetadataVersion,
        pub epoch:            u64,
        pub verifier_list:    ::std::vec::Vec<ValidatorExtend>,
        pub propose_counter:  ::std::vec::Vec<ProposeCount>,
        pub consensus_config: ConsensusConfig,
    }
    /// `MetadataVersion(uint64,uint64)`
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
    pub struct MetadataVersion {
        pub start: u64,
        pub end:   u64,
    }
    /// `ProposeCount(address,uint64)`
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
    pub struct ProposeCount {
        pub address: ::ethers::core::types::Address,
        pub count:   u64,
    }
    /// `ValidatorExtend(bytes,bytes,address,uint32,uint32)`
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
    pub struct ValidatorExtend {
        pub bls_pub_key:    ::ethers::core::types::Bytes,
        pub pub_key:        ::ethers::core::types::Bytes,
        pub address:        ::ethers::core::types::Address,
        pub propose_weight: u32,
        pub vote_weight:    u32,
    }
    /// `ValidatorList((bytes,bytes,address,uint32,uint32)[])`
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
    pub struct ValidatorList {
        pub verifier_list: ::std::vec::Vec<ValidatorExtend>,
    }
}
