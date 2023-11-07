pub use rewards_distributer::*;
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
pub mod rewards_distributer {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_voting_escrow"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkpoint_token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkpoint_token"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkpoint_total_supply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkpoint_total_supply",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claim"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claim_many"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claim_many"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenIds"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimable"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositor"),
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
                    ::std::borrow::ToOwned::to_owned("last_token_time"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("last_token_time"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDepositor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDepositor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_depositor"),
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
                    ::std::borrow::ToOwned::to_owned("start_time"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("start_time"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("time_cursor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("time_cursor"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("time_cursor_of"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("time_cursor_of"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("timestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("timestamp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("token_last_balance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token_last_balance"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokens_per_week"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokens_per_week"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("user_epoch_of"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("user_epoch_of"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ve_for_at"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ve_for_at"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timestamp"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ve_supply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ve_supply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("voting_escrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voting_escrow"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CheckpointToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CheckpointToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("time"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Claimed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Claimed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("claim_epoch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("max_epoch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
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
    pub static REWARDSDISTRIBUTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct RewardsDistributer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RewardsDistributer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RewardsDistributer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RewardsDistributer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RewardsDistributer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RewardsDistributer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RewardsDistributer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REWARDSDISTRIBUTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `checkpoint_token` (0x811a40fe) function
        pub fn checkpoint_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 26, 64, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpoint_total_supply` (0xb21ed502) function
        pub fn checkpoint_total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 30, 213, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claim` (0x379607f5) function
        pub fn claim(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 150, 7, 245], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claim_many` (0x1f1db043) function
        pub fn claim_many(
            &self,
            token_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 29, 176, 67], token_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimable` (0xd1d58b25) function
        pub fn claimable(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([209, 213, 139, 37], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositor` (0xc7c4ff46) function
        pub fn depositor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([199, 196, 255, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `last_token_time` (0x7f58e8f8) function
        pub fn last_token_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([127, 88, 232, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDepositor` (0xf2c098b7) function
        pub fn set_depositor(
            &self,
            depositor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 192, 152, 183], depositor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `start_time` (0x834ee417) function
        pub fn start_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 78, 228, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `time_cursor` (0x127dcbd3) function
        pub fn time_cursor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 125, 203, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `time_cursor_of` (0x486d25fe) function
        pub fn time_cursor_of(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([72, 109, 37, 254], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timestamp` (0xb80777ea) function
        pub fn timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([184, 7, 119, 234], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token_last_balance` (0x22b04bfc) function
        pub fn token_last_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([34, 176, 75, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokens_per_week` (0xedf59997) function
        pub fn tokens_per_week(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([237, 245, 153, 151], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `user_epoch_of` (0x16aea5c0) function
        pub fn user_epoch_of(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([22, 174, 165, 192], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ve_for_at` (0x68809889) function
        pub fn ve_for_at(
            &self,
            token_id: ::ethers::core::types::U256,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([104, 128, 152, 137], (token_id, timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ve_supply` (0xd4dafba8) function
        pub fn ve_supply(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([212, 218, 251, 168], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voting_escrow` (0xdfe05031) function
        pub fn voting_escrow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 224, 80, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CheckpointToken` event
        pub fn checkpoint_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CheckpointTokenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Claimed` event
        pub fn claimed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ClaimedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardsDistributerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RewardsDistributer<M> {
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
    #[ethevent(name = "CheckpointToken", abi = "CheckpointToken(uint256,uint256)")]
    pub struct CheckpointTokenFilter {
        pub time: ::ethers::core::types::U256,
        pub tokens: ::ethers::core::types::U256,
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
    #[ethevent(name = "Claimed", abi = "Claimed(uint256,uint256,uint256,uint256)")]
    pub struct ClaimedFilter {
        pub token_id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub claim_epoch: ::ethers::core::types::U256,
        pub max_epoch: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RewardsDistributerEvents {
        CheckpointTokenFilter(CheckpointTokenFilter),
        ClaimedFilter(ClaimedFilter),
    }
    impl ::ethers::contract::EthLogDecode for RewardsDistributerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CheckpointTokenFilter::decode_log(log) {
                return Ok(RewardsDistributerEvents::CheckpointTokenFilter(decoded));
            }
            if let Ok(decoded) = ClaimedFilter::decode_log(log) {
                return Ok(RewardsDistributerEvents::ClaimedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RewardsDistributerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckpointTokenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckpointTokenFilter> for RewardsDistributerEvents {
        fn from(value: CheckpointTokenFilter) -> Self {
            Self::CheckpointTokenFilter(value)
        }
    }
    impl ::core::convert::From<ClaimedFilter> for RewardsDistributerEvents {
        fn from(value: ClaimedFilter) -> Self {
            Self::ClaimedFilter(value)
        }
    }
    ///Container type for all input parameters for the `checkpoint_token` function with signature `checkpoint_token()` and selector `0x811a40fe`
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
    #[ethcall(name = "checkpoint_token", abi = "checkpoint_token()")]
    pub struct CheckpointTokenCall;
    ///Container type for all input parameters for the `checkpoint_total_supply` function with signature `checkpoint_total_supply()` and selector `0xb21ed502`
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
    #[ethcall(name = "checkpoint_total_supply", abi = "checkpoint_total_supply()")]
    pub struct CheckpointTotalSupplyCall;
    ///Container type for all input parameters for the `claim` function with signature `claim(uint256)` and selector `0x379607f5`
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
    #[ethcall(name = "claim", abi = "claim(uint256)")]
    pub struct ClaimCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `claim_many` function with signature `claim_many(uint256[])` and selector `0x1f1db043`
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
    #[ethcall(name = "claim_many", abi = "claim_many(uint256[])")]
    pub struct ClaimManyCall {
        pub token_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `claimable` function with signature `claimable(uint256)` and selector `0xd1d58b25`
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
    #[ethcall(name = "claimable", abi = "claimable(uint256)")]
    pub struct ClaimableCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `depositor` function with signature `depositor()` and selector `0xc7c4ff46`
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
    #[ethcall(name = "depositor", abi = "depositor()")]
    pub struct DepositorCall;
    ///Container type for all input parameters for the `last_token_time` function with signature `last_token_time()` and selector `0x7f58e8f8`
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
    #[ethcall(name = "last_token_time", abi = "last_token_time()")]
    pub struct LastTokenTimeCall;
    ///Container type for all input parameters for the `setDepositor` function with signature `setDepositor(address)` and selector `0xf2c098b7`
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
    #[ethcall(name = "setDepositor", abi = "setDepositor(address)")]
    pub struct SetDepositorCall {
        pub depositor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `start_time` function with signature `start_time()` and selector `0x834ee417`
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
    #[ethcall(name = "start_time", abi = "start_time()")]
    pub struct StartTimeCall;
    ///Container type for all input parameters for the `time_cursor` function with signature `time_cursor()` and selector `0x127dcbd3`
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
    #[ethcall(name = "time_cursor", abi = "time_cursor()")]
    pub struct TimeCursorCall;
    ///Container type for all input parameters for the `time_cursor_of` function with signature `time_cursor_of(uint256)` and selector `0x486d25fe`
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
    #[ethcall(name = "time_cursor_of", abi = "time_cursor_of(uint256)")]
    pub struct TimeCursorOfCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `timestamp` function with signature `timestamp()` and selector `0xb80777ea`
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
    #[ethcall(name = "timestamp", abi = "timestamp()")]
    pub struct TimestampCall;
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `token_last_balance` function with signature `token_last_balance()` and selector `0x22b04bfc`
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
    #[ethcall(name = "token_last_balance", abi = "token_last_balance()")]
    pub struct TokenLastBalanceCall;
    ///Container type for all input parameters for the `tokens_per_week` function with signature `tokens_per_week(uint256)` and selector `0xedf59997`
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
    #[ethcall(name = "tokens_per_week", abi = "tokens_per_week(uint256)")]
    pub struct TokensPerWeekCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `user_epoch_of` function with signature `user_epoch_of(uint256)` and selector `0x16aea5c0`
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
    #[ethcall(name = "user_epoch_of", abi = "user_epoch_of(uint256)")]
    pub struct UserEpochOfCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `ve_for_at` function with signature `ve_for_at(uint256,uint256)` and selector `0x68809889`
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
    #[ethcall(name = "ve_for_at", abi = "ve_for_at(uint256,uint256)")]
    pub struct VeForAtCall {
        pub token_id: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `ve_supply` function with signature `ve_supply(uint256)` and selector `0xd4dafba8`
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
    #[ethcall(name = "ve_supply", abi = "ve_supply(uint256)")]
    pub struct VeSupplyCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `voting_escrow` function with signature `voting_escrow()` and selector `0xdfe05031`
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
    #[ethcall(name = "voting_escrow", abi = "voting_escrow()")]
    pub struct VotingEscrowCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RewardsDistributerCalls {
        CheckpointToken(CheckpointTokenCall),
        CheckpointTotalSupply(CheckpointTotalSupplyCall),
        Claim(ClaimCall),
        ClaimMany(ClaimManyCall),
        Claimable(ClaimableCall),
        Depositor(DepositorCall),
        LastTokenTime(LastTokenTimeCall),
        SetDepositor(SetDepositorCall),
        StartTime(StartTimeCall),
        TimeCursor(TimeCursorCall),
        TimeCursorOf(TimeCursorOfCall),
        Timestamp(TimestampCall),
        Token(TokenCall),
        TokenLastBalance(TokenLastBalanceCall),
        TokensPerWeek(TokensPerWeekCall),
        UserEpochOf(UserEpochOfCall),
        VeForAt(VeForAtCall),
        VeSupply(VeSupplyCall),
        VotingEscrow(VotingEscrowCall),
    }
    impl ::ethers::core::abi::AbiDecode for RewardsDistributerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CheckpointTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckpointToken(decoded));
            }
            if let Ok(decoded)
                = <CheckpointTotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CheckpointTotalSupply(decoded));
            }
            if let Ok(decoded)
                = <ClaimCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Claim(decoded));
            }
            if let Ok(decoded)
                = <ClaimManyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimMany(decoded));
            }
            if let Ok(decoded)
                = <ClaimableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Claimable(decoded));
            }
            if let Ok(decoded)
                = <DepositorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Depositor(decoded));
            }
            if let Ok(decoded)
                = <LastTokenTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastTokenTime(decoded));
            }
            if let Ok(decoded)
                = <SetDepositorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDepositor(decoded));
            }
            if let Ok(decoded)
                = <StartTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StartTime(decoded));
            }
            if let Ok(decoded)
                = <TimeCursorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TimeCursor(decoded));
            }
            if let Ok(decoded)
                = <TimeCursorOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TimeCursorOf(decoded));
            }
            if let Ok(decoded)
                = <TimestampCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Timestamp(decoded));
            }
            if let Ok(decoded)
                = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded)
                = <TokenLastBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TokenLastBalance(decoded));
            }
            if let Ok(decoded)
                = <TokensPerWeekCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokensPerWeek(decoded));
            }
            if let Ok(decoded)
                = <UserEpochOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UserEpochOf(decoded));
            }
            if let Ok(decoded)
                = <VeForAtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VeForAt(decoded));
            }
            if let Ok(decoded)
                = <VeSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VeSupply(decoded));
            }
            if let Ok(decoded)
                = <VotingEscrowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VotingEscrow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RewardsDistributerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckpointToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckpointTotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Claim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimMany(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Claimable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Depositor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastTokenTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDepositor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimeCursor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimeCursorOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Timestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenLastBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokensPerWeek(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserEpochOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VeForAt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VeSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VotingEscrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RewardsDistributerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckpointToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckpointTotalSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Claim(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimMany(element) => ::core::fmt::Display::fmt(element, f),
                Self::Claimable(element) => ::core::fmt::Display::fmt(element, f),
                Self::Depositor(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastTokenTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositor(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimeCursor(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimeCursorOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Timestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenLastBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokensPerWeek(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserEpochOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::VeForAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::VeSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingEscrow(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckpointTokenCall> for RewardsDistributerCalls {
        fn from(value: CheckpointTokenCall) -> Self {
            Self::CheckpointToken(value)
        }
    }
    impl ::core::convert::From<CheckpointTotalSupplyCall> for RewardsDistributerCalls {
        fn from(value: CheckpointTotalSupplyCall) -> Self {
            Self::CheckpointTotalSupply(value)
        }
    }
    impl ::core::convert::From<ClaimCall> for RewardsDistributerCalls {
        fn from(value: ClaimCall) -> Self {
            Self::Claim(value)
        }
    }
    impl ::core::convert::From<ClaimManyCall> for RewardsDistributerCalls {
        fn from(value: ClaimManyCall) -> Self {
            Self::ClaimMany(value)
        }
    }
    impl ::core::convert::From<ClaimableCall> for RewardsDistributerCalls {
        fn from(value: ClaimableCall) -> Self {
            Self::Claimable(value)
        }
    }
    impl ::core::convert::From<DepositorCall> for RewardsDistributerCalls {
        fn from(value: DepositorCall) -> Self {
            Self::Depositor(value)
        }
    }
    impl ::core::convert::From<LastTokenTimeCall> for RewardsDistributerCalls {
        fn from(value: LastTokenTimeCall) -> Self {
            Self::LastTokenTime(value)
        }
    }
    impl ::core::convert::From<SetDepositorCall> for RewardsDistributerCalls {
        fn from(value: SetDepositorCall) -> Self {
            Self::SetDepositor(value)
        }
    }
    impl ::core::convert::From<StartTimeCall> for RewardsDistributerCalls {
        fn from(value: StartTimeCall) -> Self {
            Self::StartTime(value)
        }
    }
    impl ::core::convert::From<TimeCursorCall> for RewardsDistributerCalls {
        fn from(value: TimeCursorCall) -> Self {
            Self::TimeCursor(value)
        }
    }
    impl ::core::convert::From<TimeCursorOfCall> for RewardsDistributerCalls {
        fn from(value: TimeCursorOfCall) -> Self {
            Self::TimeCursorOf(value)
        }
    }
    impl ::core::convert::From<TimestampCall> for RewardsDistributerCalls {
        fn from(value: TimestampCall) -> Self {
            Self::Timestamp(value)
        }
    }
    impl ::core::convert::From<TokenCall> for RewardsDistributerCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<TokenLastBalanceCall> for RewardsDistributerCalls {
        fn from(value: TokenLastBalanceCall) -> Self {
            Self::TokenLastBalance(value)
        }
    }
    impl ::core::convert::From<TokensPerWeekCall> for RewardsDistributerCalls {
        fn from(value: TokensPerWeekCall) -> Self {
            Self::TokensPerWeek(value)
        }
    }
    impl ::core::convert::From<UserEpochOfCall> for RewardsDistributerCalls {
        fn from(value: UserEpochOfCall) -> Self {
            Self::UserEpochOf(value)
        }
    }
    impl ::core::convert::From<VeForAtCall> for RewardsDistributerCalls {
        fn from(value: VeForAtCall) -> Self {
            Self::VeForAt(value)
        }
    }
    impl ::core::convert::From<VeSupplyCall> for RewardsDistributerCalls {
        fn from(value: VeSupplyCall) -> Self {
            Self::VeSupply(value)
        }
    }
    impl ::core::convert::From<VotingEscrowCall> for RewardsDistributerCalls {
        fn from(value: VotingEscrowCall) -> Self {
            Self::VotingEscrow(value)
        }
    }
    ///Container type for all return fields from the `claim` function with signature `claim(uint256)` and selector `0x379607f5`
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
    pub struct ClaimReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `claim_many` function with signature `claim_many(uint256[])` and selector `0x1f1db043`
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
    pub struct ClaimManyReturn(pub bool);
    ///Container type for all return fields from the `claimable` function with signature `claimable(uint256)` and selector `0xd1d58b25`
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
    pub struct ClaimableReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `depositor` function with signature `depositor()` and selector `0xc7c4ff46`
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
    pub struct DepositorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `last_token_time` function with signature `last_token_time()` and selector `0x7f58e8f8`
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
    pub struct LastTokenTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `start_time` function with signature `start_time()` and selector `0x834ee417`
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
    pub struct StartTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `time_cursor` function with signature `time_cursor()` and selector `0x127dcbd3`
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
    pub struct TimeCursorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `time_cursor_of` function with signature `time_cursor_of(uint256)` and selector `0x486d25fe`
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
    pub struct TimeCursorOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `timestamp` function with signature `timestamp()` and selector `0xb80777ea`
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
    pub struct TimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token_last_balance` function with signature `token_last_balance()` and selector `0x22b04bfc`
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
    pub struct TokenLastBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokens_per_week` function with signature `tokens_per_week(uint256)` and selector `0xedf59997`
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
    pub struct TokensPerWeekReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `user_epoch_of` function with signature `user_epoch_of(uint256)` and selector `0x16aea5c0`
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
    pub struct UserEpochOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ve_for_at` function with signature `ve_for_at(uint256,uint256)` and selector `0x68809889`
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
    pub struct VeForAtReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ve_supply` function with signature `ve_supply(uint256)` and selector `0xd4dafba8`
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
    pub struct VeSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `voting_escrow` function with signature `voting_escrow()` and selector `0xdfe05031`
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
    pub struct VotingEscrowReturn(pub ::ethers::core::types::Address);
}
