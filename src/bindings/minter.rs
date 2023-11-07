pub use minter::*;
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
pub mod minter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("__voter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("__ve"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("__rewards_distributor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EMISSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EMISSION"),
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
                    ::std::borrow::ToOwned::to_owned("MAX_TEAM_RATE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_TEAM_RATE"),
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
                    ::std::borrow::ToOwned::to_owned("_flow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_flow"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IFlow"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_rewards_distributor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_rewards_distributor",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRewardsDistributor",
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
                    ::std::borrow::ToOwned::to_owned("_ve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_ve"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVotingEscrow"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_voter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_voter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVoter"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("acceptTeam"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acceptTeam"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("active_period"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("active_period"),
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
                    ::std::borrow::ToOwned::to_owned("calculate_emission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculate_emission"),
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
                    ::std::borrow::ToOwned::to_owned("calculate_growth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculate_growth"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minted"),
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
                    ::std::borrow::ToOwned::to_owned("circulating_emission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "circulating_emission",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("circulating_supply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("circulating_supply"),
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
                    ::std::borrow::ToOwned::to_owned("initialMintAndLock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialMintAndLock"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("claims"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Minter.Claim[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("pendingTeam"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pendingTeam"),
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
                    ::std::borrow::ToOwned::to_owned("setEmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEmission"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_emission"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("setTeam"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTeam"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_team"),
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
                    ::std::borrow::ToOwned::to_owned("setTeamRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTeamRate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_teamRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("startActivePeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startActivePeriod"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("team"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("team"),
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
                    ::std::borrow::ToOwned::to_owned("teamRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("teamRate"),
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
                    ::std::borrow::ToOwned::to_owned("update_period"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("update_period"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weekly"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weekly"),
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
                    ::std::borrow::ToOwned::to_owned("weekly_emission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weekly_emission"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EmissionSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EmissionSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("setter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("emission"),
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
                    ::std::borrow::ToOwned::to_owned("Mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weekly"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "circulating_supply",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "circulating_emission",
                                    ),
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
    pub static MINTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Minter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Minter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Minter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Minter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Minter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Minter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Minter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MINTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `EMISSION` (0xa18cb956) function
        pub fn emission(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 140, 185, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_TEAM_RATE` (0x01c8e6fd) function
        pub fn max_team_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 200, 230, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_flow` (0xfe75cac5) function
        pub fn flow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([254, 117, 202, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_rewards_distributor` (0x4b1cd5da) function
        pub fn rewards_distributor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([75, 28, 213, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_ve` (0x8dd598fb) function
        pub fn ve(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 213, 152, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_voter` (0x3db9b42a) function
        pub fn voter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([61, 185, 180, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptTeam` (0xb5cc143a) function
        pub fn accept_team(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 204, 20, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `active_period` (0xd1399608) function
        pub fn active_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([209, 57, 150, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculate_emission` (0x36d96faf) function
        pub fn calculate_emission(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([54, 217, 111, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculate_growth` (0x8e01fbfa) function
        pub fn calculate_growth(
            &self,
            minted: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 1, 251, 250], minted)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `circulating_emission` (0x1eebae80) function
        pub fn circulating_emission(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([30, 235, 174, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `circulating_supply` (0xe038c75a) function
        pub fn circulating_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([224, 56, 199, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialMintAndLock` (0x4642feb3) function
        pub fn initial_mint_and_lock(
            &self,
            claims: ::std::vec::Vec<Claim>,
            max: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 66, 254, 179], (claims, max))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingTeam` (0x59d46ffc) function
        pub fn pending_team(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([89, 212, 111, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEmission` (0xddce102f) function
        pub fn set_emission(
            &self,
            emission: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 206, 16, 47], emission)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTeam` (0x095cf5c6) function
        pub fn set_team(
            &self,
            team: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 92, 245, 198], team)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTeamRate` (0x2e8f7b1f) function
        pub fn set_team_rate(
            &self,
            team_rate: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 143, 123, 31], team_rate)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startActivePeriod` (0x8489973a) function
        pub fn start_active_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 137, 151, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `team` (0x85f2aef2) function
        pub fn team(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([133, 242, 174, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `teamRate` (0x78ef7f02) function
        pub fn team_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 239, 127, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update_period` (0xed29fc11) function
        pub fn update_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([237, 41, 252, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weekly` (0x26cfc17b) function
        pub fn weekly(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 207, 193, 123], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weekly_emission` (0xcfc6c8ff) function
        pub fn weekly_emission(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 198, 200, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EmissionSet` event
        pub fn emission_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EmissionSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MinterEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Minter<M> {
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
    #[ethevent(name = "EmissionSet", abi = "EmissionSet(address,uint256)")]
    pub struct EmissionSetFilter {
        #[ethevent(indexed)]
        pub setter: ::ethers::core::types::Address,
        pub emission: ::ethers::core::types::U256,
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
    #[ethevent(name = "Mint", abi = "Mint(address,uint256,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub weekly: ::ethers::core::types::U256,
        pub circulating_supply: ::ethers::core::types::U256,
        pub circulating_emission: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MinterEvents {
        EmissionSetFilter(EmissionSetFilter),
        MintFilter(MintFilter),
    }
    impl ::ethers::contract::EthLogDecode for MinterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EmissionSetFilter::decode_log(log) {
                return Ok(MinterEvents::EmissionSetFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(MinterEvents::MintFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MinterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EmissionSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EmissionSetFilter> for MinterEvents {
        fn from(value: EmissionSetFilter) -> Self {
            Self::EmissionSetFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for MinterEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    ///Container type for all input parameters for the `EMISSION` function with signature `EMISSION()` and selector `0xa18cb956`
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
    #[ethcall(name = "EMISSION", abi = "EMISSION()")]
    pub struct EmissionCall;
    ///Container type for all input parameters for the `MAX_TEAM_RATE` function with signature `MAX_TEAM_RATE()` and selector `0x01c8e6fd`
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
    #[ethcall(name = "MAX_TEAM_RATE", abi = "MAX_TEAM_RATE()")]
    pub struct MaxTeamRateCall;
    ///Container type for all input parameters for the `_flow` function with signature `_flow()` and selector `0xfe75cac5`
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
    #[ethcall(name = "_flow", abi = "_flow()")]
    pub struct FlowCall;
    ///Container type for all input parameters for the `_rewards_distributor` function with signature `_rewards_distributor()` and selector `0x4b1cd5da`
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
    #[ethcall(name = "_rewards_distributor", abi = "_rewards_distributor()")]
    pub struct RewardsDistributorCall;
    ///Container type for all input parameters for the `_ve` function with signature `_ve()` and selector `0x8dd598fb`
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
    #[ethcall(name = "_ve", abi = "_ve()")]
    pub struct VeCall;
    ///Container type for all input parameters for the `_voter` function with signature `_voter()` and selector `0x3db9b42a`
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
    #[ethcall(name = "_voter", abi = "_voter()")]
    pub struct VoterCall;
    ///Container type for all input parameters for the `acceptTeam` function with signature `acceptTeam()` and selector `0xb5cc143a`
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
    #[ethcall(name = "acceptTeam", abi = "acceptTeam()")]
    pub struct AcceptTeamCall;
    ///Container type for all input parameters for the `active_period` function with signature `active_period()` and selector `0xd1399608`
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
    #[ethcall(name = "active_period", abi = "active_period()")]
    pub struct ActivePeriodCall;
    ///Container type for all input parameters for the `calculate_emission` function with signature `calculate_emission()` and selector `0x36d96faf`
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
    #[ethcall(name = "calculate_emission", abi = "calculate_emission()")]
    pub struct CalculateEmissionCall;
    ///Container type for all input parameters for the `calculate_growth` function with signature `calculate_growth(uint256)` and selector `0x8e01fbfa`
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
    #[ethcall(name = "calculate_growth", abi = "calculate_growth(uint256)")]
    pub struct CalculateGrowthCall {
        pub minted: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `circulating_emission` function with signature `circulating_emission()` and selector `0x1eebae80`
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
    #[ethcall(name = "circulating_emission", abi = "circulating_emission()")]
    pub struct CirculatingEmissionCall;
    ///Container type for all input parameters for the `circulating_supply` function with signature `circulating_supply()` and selector `0xe038c75a`
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
    #[ethcall(name = "circulating_supply", abi = "circulating_supply()")]
    pub struct CirculatingSupplyCall;
    ///Container type for all input parameters for the `initialMintAndLock` function with signature `initialMintAndLock((address,uint256,uint256)[],uint256)` and selector `0x4642feb3`
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
        name = "initialMintAndLock",
        abi = "initialMintAndLock((address,uint256,uint256)[],uint256)"
    )]
    pub struct InitialMintAndLockCall {
        pub claims: ::std::vec::Vec<Claim>,
        pub max: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pendingTeam` function with signature `pendingTeam()` and selector `0x59d46ffc`
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
    #[ethcall(name = "pendingTeam", abi = "pendingTeam()")]
    pub struct PendingTeamCall;
    ///Container type for all input parameters for the `setEmission` function with signature `setEmission(uint256)` and selector `0xddce102f`
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
    #[ethcall(name = "setEmission", abi = "setEmission(uint256)")]
    pub struct SetEmissionCall {
        pub emission: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setTeam` function with signature `setTeam(address)` and selector `0x095cf5c6`
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
    #[ethcall(name = "setTeam", abi = "setTeam(address)")]
    pub struct SetTeamCall {
        pub team: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTeamRate` function with signature `setTeamRate(uint256)` and selector `0x2e8f7b1f`
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
    #[ethcall(name = "setTeamRate", abi = "setTeamRate(uint256)")]
    pub struct SetTeamRateCall {
        pub team_rate: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `startActivePeriod` function with signature `startActivePeriod()` and selector `0x8489973a`
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
    #[ethcall(name = "startActivePeriod", abi = "startActivePeriod()")]
    pub struct StartActivePeriodCall;
    ///Container type for all input parameters for the `team` function with signature `team()` and selector `0x85f2aef2`
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
    #[ethcall(name = "team", abi = "team()")]
    pub struct TeamCall;
    ///Container type for all input parameters for the `teamRate` function with signature `teamRate()` and selector `0x78ef7f02`
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
    #[ethcall(name = "teamRate", abi = "teamRate()")]
    pub struct TeamRateCall;
    ///Container type for all input parameters for the `update_period` function with signature `update_period()` and selector `0xed29fc11`
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
    #[ethcall(name = "update_period", abi = "update_period()")]
    pub struct UpdatePeriodCall;
    ///Container type for all input parameters for the `weekly` function with signature `weekly()` and selector `0x26cfc17b`
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
    #[ethcall(name = "weekly", abi = "weekly()")]
    pub struct WeeklyCall;
    ///Container type for all input parameters for the `weekly_emission` function with signature `weekly_emission()` and selector `0xcfc6c8ff`
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
    #[ethcall(name = "weekly_emission", abi = "weekly_emission()")]
    pub struct WeeklyEmissionCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MinterCalls {
        Emission(EmissionCall),
        MaxTeamRate(MaxTeamRateCall),
        Flow(FlowCall),
        RewardsDistributor(RewardsDistributorCall),
        Ve(VeCall),
        Voter(VoterCall),
        AcceptTeam(AcceptTeamCall),
        ActivePeriod(ActivePeriodCall),
        CalculateEmission(CalculateEmissionCall),
        CalculateGrowth(CalculateGrowthCall),
        CirculatingEmission(CirculatingEmissionCall),
        CirculatingSupply(CirculatingSupplyCall),
        InitialMintAndLock(InitialMintAndLockCall),
        PendingTeam(PendingTeamCall),
        SetEmission(SetEmissionCall),
        SetTeam(SetTeamCall),
        SetTeamRate(SetTeamRateCall),
        StartActivePeriod(StartActivePeriodCall),
        Team(TeamCall),
        TeamRate(TeamRateCall),
        UpdatePeriod(UpdatePeriodCall),
        Weekly(WeeklyCall),
        WeeklyEmission(WeeklyEmissionCall),
    }
    impl ::ethers::core::abi::AbiDecode for MinterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <EmissionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Emission(decoded));
            }
            if let Ok(decoded)
                = <MaxTeamRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxTeamRate(decoded));
            }
            if let Ok(decoded)
                = <FlowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Flow(decoded));
            }
            if let Ok(decoded)
                = <RewardsDistributorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RewardsDistributor(decoded));
            }
            if let Ok(decoded)
                = <VeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ve(decoded));
            }
            if let Ok(decoded)
                = <VoterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter(decoded));
            }
            if let Ok(decoded)
                = <AcceptTeamCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AcceptTeam(decoded));
            }
            if let Ok(decoded)
                = <ActivePeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ActivePeriod(decoded));
            }
            if let Ok(decoded)
                = <CalculateEmissionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateEmission(decoded));
            }
            if let Ok(decoded)
                = <CalculateGrowthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CalculateGrowth(decoded));
            }
            if let Ok(decoded)
                = <CirculatingEmissionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CirculatingEmission(decoded));
            }
            if let Ok(decoded)
                = <CirculatingSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CirculatingSupply(decoded));
            }
            if let Ok(decoded)
                = <InitialMintAndLockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InitialMintAndLock(decoded));
            }
            if let Ok(decoded)
                = <PendingTeamCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PendingTeam(decoded));
            }
            if let Ok(decoded)
                = <SetEmissionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEmission(decoded));
            }
            if let Ok(decoded)
                = <SetTeamCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTeam(decoded));
            }
            if let Ok(decoded)
                = <SetTeamRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTeamRate(decoded));
            }
            if let Ok(decoded)
                = <StartActivePeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::StartActivePeriod(decoded));
            }
            if let Ok(decoded)
                = <TeamCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Team(decoded));
            }
            if let Ok(decoded)
                = <TeamRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TeamRate(decoded));
            }
            if let Ok(decoded)
                = <UpdatePeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdatePeriod(decoded));
            }
            if let Ok(decoded)
                = <WeeklyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weekly(decoded));
            }
            if let Ok(decoded)
                = <WeeklyEmissionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WeeklyEmission(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MinterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Emission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxTeamRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Flow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardsDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Voter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AcceptTeam(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ActivePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateEmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateGrowth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CirculatingEmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CirculatingSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitialMintAndLock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingTeam(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTeam(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetTeamRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartActivePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Team(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TeamRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Weekly(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WeeklyEmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MinterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Emission(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxTeamRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Flow(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardsDistributor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Ve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Voter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptTeam(element) => ::core::fmt::Display::fmt(element, f),
                Self::ActivePeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateEmission(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateGrowth(element) => ::core::fmt::Display::fmt(element, f),
                Self::CirculatingEmission(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CirculatingSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitialMintAndLock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingTeam(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEmission(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTeam(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTeamRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartActivePeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::Team(element) => ::core::fmt::Display::fmt(element, f),
                Self::TeamRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weekly(element) => ::core::fmt::Display::fmt(element, f),
                Self::WeeklyEmission(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EmissionCall> for MinterCalls {
        fn from(value: EmissionCall) -> Self {
            Self::Emission(value)
        }
    }
    impl ::core::convert::From<MaxTeamRateCall> for MinterCalls {
        fn from(value: MaxTeamRateCall) -> Self {
            Self::MaxTeamRate(value)
        }
    }
    impl ::core::convert::From<FlowCall> for MinterCalls {
        fn from(value: FlowCall) -> Self {
            Self::Flow(value)
        }
    }
    impl ::core::convert::From<RewardsDistributorCall> for MinterCalls {
        fn from(value: RewardsDistributorCall) -> Self {
            Self::RewardsDistributor(value)
        }
    }
    impl ::core::convert::From<VeCall> for MinterCalls {
        fn from(value: VeCall) -> Self {
            Self::Ve(value)
        }
    }
    impl ::core::convert::From<VoterCall> for MinterCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
        }
    }
    impl ::core::convert::From<AcceptTeamCall> for MinterCalls {
        fn from(value: AcceptTeamCall) -> Self {
            Self::AcceptTeam(value)
        }
    }
    impl ::core::convert::From<ActivePeriodCall> for MinterCalls {
        fn from(value: ActivePeriodCall) -> Self {
            Self::ActivePeriod(value)
        }
    }
    impl ::core::convert::From<CalculateEmissionCall> for MinterCalls {
        fn from(value: CalculateEmissionCall) -> Self {
            Self::CalculateEmission(value)
        }
    }
    impl ::core::convert::From<CalculateGrowthCall> for MinterCalls {
        fn from(value: CalculateGrowthCall) -> Self {
            Self::CalculateGrowth(value)
        }
    }
    impl ::core::convert::From<CirculatingEmissionCall> for MinterCalls {
        fn from(value: CirculatingEmissionCall) -> Self {
            Self::CirculatingEmission(value)
        }
    }
    impl ::core::convert::From<CirculatingSupplyCall> for MinterCalls {
        fn from(value: CirculatingSupplyCall) -> Self {
            Self::CirculatingSupply(value)
        }
    }
    impl ::core::convert::From<InitialMintAndLockCall> for MinterCalls {
        fn from(value: InitialMintAndLockCall) -> Self {
            Self::InitialMintAndLock(value)
        }
    }
    impl ::core::convert::From<PendingTeamCall> for MinterCalls {
        fn from(value: PendingTeamCall) -> Self {
            Self::PendingTeam(value)
        }
    }
    impl ::core::convert::From<SetEmissionCall> for MinterCalls {
        fn from(value: SetEmissionCall) -> Self {
            Self::SetEmission(value)
        }
    }
    impl ::core::convert::From<SetTeamCall> for MinterCalls {
        fn from(value: SetTeamCall) -> Self {
            Self::SetTeam(value)
        }
    }
    impl ::core::convert::From<SetTeamRateCall> for MinterCalls {
        fn from(value: SetTeamRateCall) -> Self {
            Self::SetTeamRate(value)
        }
    }
    impl ::core::convert::From<StartActivePeriodCall> for MinterCalls {
        fn from(value: StartActivePeriodCall) -> Self {
            Self::StartActivePeriod(value)
        }
    }
    impl ::core::convert::From<TeamCall> for MinterCalls {
        fn from(value: TeamCall) -> Self {
            Self::Team(value)
        }
    }
    impl ::core::convert::From<TeamRateCall> for MinterCalls {
        fn from(value: TeamRateCall) -> Self {
            Self::TeamRate(value)
        }
    }
    impl ::core::convert::From<UpdatePeriodCall> for MinterCalls {
        fn from(value: UpdatePeriodCall) -> Self {
            Self::UpdatePeriod(value)
        }
    }
    impl ::core::convert::From<WeeklyCall> for MinterCalls {
        fn from(value: WeeklyCall) -> Self {
            Self::Weekly(value)
        }
    }
    impl ::core::convert::From<WeeklyEmissionCall> for MinterCalls {
        fn from(value: WeeklyEmissionCall) -> Self {
            Self::WeeklyEmission(value)
        }
    }
    ///Container type for all return fields from the `EMISSION` function with signature `EMISSION()` and selector `0xa18cb956`
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
    pub struct EmissionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_TEAM_RATE` function with signature `MAX_TEAM_RATE()` and selector `0x01c8e6fd`
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
    pub struct MaxTeamRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_flow` function with signature `_flow()` and selector `0xfe75cac5`
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
    pub struct FlowReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `_rewards_distributor` function with signature `_rewards_distributor()` and selector `0x4b1cd5da`
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
    pub struct RewardsDistributorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `_ve` function with signature `_ve()` and selector `0x8dd598fb`
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
    pub struct VeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `_voter` function with signature `_voter()` and selector `0x3db9b42a`
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
    pub struct VoterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `active_period` function with signature `active_period()` and selector `0xd1399608`
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
    pub struct ActivePeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculate_emission` function with signature `calculate_emission()` and selector `0x36d96faf`
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
    pub struct CalculateEmissionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculate_growth` function with signature `calculate_growth(uint256)` and selector `0x8e01fbfa`
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
    pub struct CalculateGrowthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `circulating_emission` function with signature `circulating_emission()` and selector `0x1eebae80`
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
    pub struct CirculatingEmissionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `circulating_supply` function with signature `circulating_supply()` and selector `0xe038c75a`
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
    pub struct CirculatingSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pendingTeam` function with signature `pendingTeam()` and selector `0x59d46ffc`
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
    pub struct PendingTeamReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `team` function with signature `team()` and selector `0x85f2aef2`
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
    pub struct TeamReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `teamRate` function with signature `teamRate()` and selector `0x78ef7f02`
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
    pub struct TeamRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `update_period` function with signature `update_period()` and selector `0xed29fc11`
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
    pub struct UpdatePeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `weekly` function with signature `weekly()` and selector `0x26cfc17b`
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
    pub struct WeeklyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `weekly_emission` function with signature `weekly_emission()` and selector `0xcfc6c8ff`
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
    pub struct WeeklyEmissionReturn(pub ::ethers::core::types::U256);
    ///`Claim(address,uint256,uint256)`
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
    pub struct Claim {
        pub claimant: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub lock_time: ::ethers::core::types::U256,
    }
}
