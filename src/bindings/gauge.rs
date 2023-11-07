pub use gauge::*;
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
pub mod gauge {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stake"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_external_bribe"),
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
                        name: ::std::borrow::ToOwned::to_owned("_voter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_oFlow"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gaugeFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_forPair"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_allowedRewardTokens"),
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
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("batchRewardPerToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "batchRewardPerToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxRuns"),
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
                    ::std::borrow::ToOwned::to_owned("batchUpdateRewardPerToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "batchUpdateRewardPerToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxRuns"),
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
                    ::std::borrow::ToOwned::to_owned("checkpoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkpoints"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("depositAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("derivedBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("derivedBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("derivedBalances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("derivedBalances"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("derivedSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("derivedSupply"),
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
                    ::std::borrow::ToOwned::to_owned("earned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("earned"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("fees0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fees0"),
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
                    ::std::borrow::ToOwned::to_owned("fees1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fees1"),
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
                    ::std::borrow::ToOwned::to_owned("flow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flow"),
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
                    ::std::borrow::ToOwned::to_owned("gaugeFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("getPriorBalanceIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPriorBalanceIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("getPriorRewardPerToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPriorRewardPerToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("getPriorSupplyIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPriorSupplyIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("getReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isForPair"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isForPair"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("lastEarn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastEarn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("lastTimeRewardApplicable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastTimeRewardApplicable",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("lastUpdateTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastUpdateTime"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("left"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("left"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("numCheckpoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numCheckpoints"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("oFlow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oFlow"),
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
                    ::std::borrow::ToOwned::to_owned("periodFinish"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("periodFinish"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("rewardPerToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardPerToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("rewardPerTokenCheckpoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rewardPerTokenCheckpoints",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewardPerToken"),
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
                    ::std::borrow::ToOwned::to_owned("rewardPerTokenNumCheckpoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rewardPerTokenNumCheckpoints",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("rewardPerTokenStored"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rewardPerTokenStored",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("rewardRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardRate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("rewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewards"),
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
                    ::std::borrow::ToOwned::to_owned("rewardsListLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardsListLength"),
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
                    ::std::borrow::ToOwned::to_owned("setOFlow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOFlow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_oFlow"),
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
                    ::std::borrow::ToOwned::to_owned("stake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stake"),
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
                    ::std::borrow::ToOwned::to_owned("supplyCheckpoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplyCheckpoints"),
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
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("supply"),
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
                    ::std::borrow::ToOwned::to_owned("supplyNumCheckpoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "supplyNumCheckpoints",
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
                    ::std::borrow::ToOwned::to_owned("swapOutRewardToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapOutRewardToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("i"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oldToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newToken"),
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
                    ::std::borrow::ToOwned::to_owned("tokenIds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenIds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("userRewardPerTokenStored"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "userRewardPerTokenStored",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("voter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voter"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ClaimRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ClaimRewards"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotifyReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NotifyReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("OFlowSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OFlowSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_oFlow"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
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
    pub static GAUGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Gauge<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Gauge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Gauge<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Gauge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Gauge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Gauge)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Gauge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GAUGE_ABI.clone(),
                    client,
                ),
            )
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
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchRewardPerToken` (0x5a45d052) function
        pub fn batch_reward_per_token(
            &self,
            token: ::ethers::core::types::Address,
            max_runs: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 69, 208, 82], (token, max_runs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchUpdateRewardPerToken` (0x68fcee1a) function
        pub fn batch_update_reward_per_token(
            &self,
            token: ::ethers::core::types::Address,
            max_runs: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 252, 238, 26], (token, max_runs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpoints` (0x0cdfebfa) function
        pub fn checkpoints(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([12, 223, 235, 250], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xe2bbb158) function
        pub fn deposit(
            &self,
            amount: ::ethers::core::types::U256,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 187, 177, 88], (amount, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositAll` (0xc6f678bd) function
        pub fn deposit_all(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 246, 120, 189], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `derivedBalance` (0xd35e2544) function
        pub fn derived_balance(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([211, 94, 37, 68], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `derivedBalances` (0x63fb415b) function
        pub fn derived_balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([99, 251, 65, 91], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `derivedSupply` (0xd7da4bb0) function
        pub fn derived_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 218, 75, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earned` (0x211dc32d) function
        pub fn earned(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([33, 29, 195, 45], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fees0` (0x93f1c442) function
        pub fn fees_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 241, 196, 66], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fees1` (0x4c02a21c) function
        pub fn fees_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([76, 2, 162, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flow` (0x343aad82) function
        pub fn flow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([52, 58, 173, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gaugeFactory` (0x0d52333c) function
        pub fn gauge_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 82, 51, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriorBalanceIndex` (0x115c6f39) function
        pub fn get_prior_balance_index(
            &self,
            account: ::ethers::core::types::Address,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([17, 92, 111, 57], (account, timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriorRewardPerToken` (0xfd314098) function
        pub fn get_prior_reward_per_token(
            &self,
            token: ::ethers::core::types::Address,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([253, 49, 64, 152], (token, timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriorSupplyIndex` (0x76f4be36) function
        pub fn get_prior_supply_index(
            &self,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([118, 244, 190, 54], timestamp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReward` (0x31279d3d) function
        pub fn get_reward(
            &self,
            account: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 39, 157, 61], (account, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isForPair` (0xe5748213) function
        pub fn is_for_pair(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 116, 130, 19], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isReward` (0x4d5ce038) function
        pub fn is_reward(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([77, 92, 224, 56], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastEarn` (0xa495e5b5) function
        pub fn last_earn(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([164, 149, 229, 181], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastTimeRewardApplicable` (0x638634ee) function
        pub fn last_time_reward_applicable(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([99, 134, 52, 238], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastUpdateTime` (0x2ce9aead) function
        pub fn last_update_time(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([44, 233, 174, 173], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `left` (0x99bcc052) function
        pub fn left(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([153, 188, 192, 82], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `notifyRewardAmount` (0xb66503cf) function
        pub fn notify_reward_amount(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 101, 3, 207], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numCheckpoints` (0x6fcfff45) function
        pub fn num_checkpoints(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([111, 207, 255, 69], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oFlow` (0x4665b5c9) function
        pub fn o_flow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 101, 181, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `periodFinish` (0xda09d19d) function
        pub fn period_finish(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([218, 9, 209, 157], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerToken` (0xf1229777) function
        pub fn reward_per_token(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 34, 151, 119], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerTokenCheckpoints` (0x01316ddf) function
        pub fn reward_per_token_checkpoints(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([1, 49, 109, 223], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerTokenNumCheckpoints` (0xaa479652) function
        pub fn reward_per_token_num_checkpoints(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([170, 71, 150, 82], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerTokenStored` (0x9ce43f90) function
        pub fn reward_per_token_stored(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([156, 228, 63, 144], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardRate` (0x221ca18c) function
        pub fn reward_rate(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([34, 28, 161, 140], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewards` (0xf301af42) function
        pub fn rewards(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([243, 1, 175, 66], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardsListLength` (0xe6886396) function
        pub fn rewards_list_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([230, 136, 99, 150], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOFlow` (0xf00dc8ef) function
        pub fn set_o_flow(
            &self,
            o_flow: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 13, 200, 239], o_flow)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0x3a4b66f1) function
        pub fn stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([58, 75, 102, 241], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyCheckpoints` (0xf7412baf) function
        pub fn supply_checkpoints(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([247, 65, 43, 175], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyNumCheckpoints` (0xe8111a12) function
        pub fn supply_num_checkpoints(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 17, 26, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapOutRewardToken` (0x9418f939) function
        pub fn swap_out_reward_token(
            &self,
            i: ::ethers::core::types::U256,
            old_token: ::ethers::core::types::Address,
            new_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 24, 249, 57], (i, old_token, new_token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenIds` (0xfc97a303) function
        pub fn token_ids(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([252, 151, 163, 3], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userRewardPerTokenStored` (0x3ca068b6) function
        pub fn user_reward_per_token_stored(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 160, 104, 182], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voter` (0x46c96aac) function
        pub fn voter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 201, 106, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawAll` (0x853828b6) function
        pub fn withdraw_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 56, 40, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawToken` (0xfdb483c7) function
        pub fn withdraw_token(
            &self,
            amount: ::ethers::core::types::U256,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 180, 131, 199], (amount, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ClaimRewards` event
        pub fn claim_rewards_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClaimRewardsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `NotifyReward` event
        pub fn notify_reward_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NotifyRewardFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OFlowSet` event
        pub fn o_flow_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OflowSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GaugeEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Gauge<M> {
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
    #[ethevent(name = "ClaimRewards", abi = "ClaimRewards(address,address,uint256)")]
    pub struct ClaimRewardsFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reward: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "NotifyReward", abi = "NotifyReward(address,address,uint256)")]
    pub struct NotifyRewardFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reward: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "OFlowSet", abi = "OFlowSet(address)")]
    pub struct OflowSetFilter {
        #[ethevent(indexed)]
        pub o_flow: ::ethers::core::types::Address,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,uint256,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GaugeEvents {
        ClaimRewardsFilter(ClaimRewardsFilter),
        DepositFilter(DepositFilter),
        NotifyRewardFilter(NotifyRewardFilter),
        OflowSetFilter(OflowSetFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for GaugeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ClaimRewardsFilter::decode_log(log) {
                return Ok(GaugeEvents::ClaimRewardsFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(GaugeEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = NotifyRewardFilter::decode_log(log) {
                return Ok(GaugeEvents::NotifyRewardFilter(decoded));
            }
            if let Ok(decoded) = OflowSetFilter::decode_log(log) {
                return Ok(GaugeEvents::OflowSetFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(GaugeEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GaugeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimRewardsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotifyRewardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OflowSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimRewardsFilter> for GaugeEvents {
        fn from(value: ClaimRewardsFilter) -> Self {
            Self::ClaimRewardsFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for GaugeEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<NotifyRewardFilter> for GaugeEvents {
        fn from(value: NotifyRewardFilter) -> Self {
            Self::NotifyRewardFilter(value)
        }
    }
    impl ::core::convert::From<OflowSetFilter> for GaugeEvents {
        fn from(value: OflowSetFilter) -> Self {
            Self::OflowSetFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for GaugeEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `batchRewardPerToken` function with signature `batchRewardPerToken(address,uint256)` and selector `0x5a45d052`
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
        name = "batchRewardPerToken",
        abi = "batchRewardPerToken(address,uint256)"
    )]
    pub struct BatchRewardPerTokenCall {
        pub token: ::ethers::core::types::Address,
        pub max_runs: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `batchUpdateRewardPerToken` function with signature `batchUpdateRewardPerToken(address,uint256)` and selector `0x68fcee1a`
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
        name = "batchUpdateRewardPerToken",
        abi = "batchUpdateRewardPerToken(address,uint256)"
    )]
    pub struct BatchUpdateRewardPerTokenCall {
        pub token: ::ethers::core::types::Address,
        pub max_runs: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkpoints` function with signature `checkpoints(address,uint256)` and selector `0x0cdfebfa`
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
    #[ethcall(name = "checkpoints", abi = "checkpoints(address,uint256)")]
    pub struct CheckpointsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,uint256)` and selector `0xe2bbb158`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,uint256)")]
    pub struct DepositCall {
        pub amount: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `depositAll` function with signature `depositAll(uint256)` and selector `0xc6f678bd`
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
    #[ethcall(name = "depositAll", abi = "depositAll(uint256)")]
    pub struct DepositAllCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `derivedBalance` function with signature `derivedBalance(address)` and selector `0xd35e2544`
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
    #[ethcall(name = "derivedBalance", abi = "derivedBalance(address)")]
    pub struct DerivedBalanceCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `derivedBalances` function with signature `derivedBalances(address)` and selector `0x63fb415b`
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
    #[ethcall(name = "derivedBalances", abi = "derivedBalances(address)")]
    pub struct DerivedBalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `derivedSupply` function with signature `derivedSupply()` and selector `0xd7da4bb0`
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
    #[ethcall(name = "derivedSupply", abi = "derivedSupply()")]
    pub struct DerivedSupplyCall;
    ///Container type for all input parameters for the `earned` function with signature `earned(address,address)` and selector `0x211dc32d`
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
    #[ethcall(name = "earned", abi = "earned(address,address)")]
    pub struct EarnedCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fees0` function with signature `fees0()` and selector `0x93f1c442`
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
    #[ethcall(name = "fees0", abi = "fees0()")]
    pub struct Fees0Call;
    ///Container type for all input parameters for the `fees1` function with signature `fees1()` and selector `0x4c02a21c`
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
    #[ethcall(name = "fees1", abi = "fees1()")]
    pub struct Fees1Call;
    ///Container type for all input parameters for the `flow` function with signature `flow()` and selector `0x343aad82`
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
    #[ethcall(name = "flow", abi = "flow()")]
    pub struct FlowCall;
    ///Container type for all input parameters for the `gaugeFactory` function with signature `gaugeFactory()` and selector `0x0d52333c`
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
    #[ethcall(name = "gaugeFactory", abi = "gaugeFactory()")]
    pub struct GaugeFactoryCall;
    ///Container type for all input parameters for the `getPriorBalanceIndex` function with signature `getPriorBalanceIndex(address,uint256)` and selector `0x115c6f39`
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
        name = "getPriorBalanceIndex",
        abi = "getPriorBalanceIndex(address,uint256)"
    )]
    pub struct GetPriorBalanceIndexCall {
        pub account: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPriorRewardPerToken` function with signature `getPriorRewardPerToken(address,uint256)` and selector `0xfd314098`
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
        name = "getPriorRewardPerToken",
        abi = "getPriorRewardPerToken(address,uint256)"
    )]
    pub struct GetPriorRewardPerTokenCall {
        pub token: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPriorSupplyIndex` function with signature `getPriorSupplyIndex(uint256)` and selector `0x76f4be36`
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
    #[ethcall(name = "getPriorSupplyIndex", abi = "getPriorSupplyIndex(uint256)")]
    pub struct GetPriorSupplyIndexCall {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getReward` function with signature `getReward(address,address[])` and selector `0x31279d3d`
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
    #[ethcall(name = "getReward", abi = "getReward(address,address[])")]
    pub struct GetRewardCall {
        pub account: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `isForPair` function with signature `isForPair()` and selector `0xe5748213`
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
    #[ethcall(name = "isForPair", abi = "isForPair()")]
    pub struct IsForPairCall;
    ///Container type for all input parameters for the `isReward` function with signature `isReward(address)` and selector `0x4d5ce038`
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
    #[ethcall(name = "isReward", abi = "isReward(address)")]
    pub struct IsRewardCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `lastEarn` function with signature `lastEarn(address,address)` and selector `0xa495e5b5`
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
    #[ethcall(name = "lastEarn", abi = "lastEarn(address,address)")]
    pub struct LastEarnCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `lastTimeRewardApplicable` function with signature `lastTimeRewardApplicable(address)` and selector `0x638634ee`
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
        name = "lastTimeRewardApplicable",
        abi = "lastTimeRewardApplicable(address)"
    )]
    pub struct LastTimeRewardApplicableCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lastUpdateTime` function with signature `lastUpdateTime(address)` and selector `0x2ce9aead`
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
    #[ethcall(name = "lastUpdateTime", abi = "lastUpdateTime(address)")]
    pub struct LastUpdateTimeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `left` function with signature `left(address)` and selector `0x99bcc052`
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
    #[ethcall(name = "left", abi = "left(address)")]
    pub struct LeftCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `notifyRewardAmount` function with signature `notifyRewardAmount(address,uint256)` and selector `0xb66503cf`
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
    #[ethcall(name = "notifyRewardAmount", abi = "notifyRewardAmount(address,uint256)")]
    pub struct NotifyRewardAmountCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `numCheckpoints` function with signature `numCheckpoints(address)` and selector `0x6fcfff45`
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
    #[ethcall(name = "numCheckpoints", abi = "numCheckpoints(address)")]
    pub struct NumCheckpointsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `oFlow` function with signature `oFlow()` and selector `0x4665b5c9`
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
    #[ethcall(name = "oFlow", abi = "oFlow()")]
    pub struct OflowCall;
    ///Container type for all input parameters for the `periodFinish` function with signature `periodFinish(address)` and selector `0xda09d19d`
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
    #[ethcall(name = "periodFinish", abi = "periodFinish(address)")]
    pub struct PeriodFinishCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `rewardPerToken` function with signature `rewardPerToken(address)` and selector `0xf1229777`
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
    #[ethcall(name = "rewardPerToken", abi = "rewardPerToken(address)")]
    pub struct RewardPerTokenCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rewardPerTokenCheckpoints` function with signature `rewardPerTokenCheckpoints(address,uint256)` and selector `0x01316ddf`
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
        name = "rewardPerTokenCheckpoints",
        abi = "rewardPerTokenCheckpoints(address,uint256)"
    )]
    pub struct RewardPerTokenCheckpointsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `rewardPerTokenNumCheckpoints` function with signature `rewardPerTokenNumCheckpoints(address)` and selector `0xaa479652`
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
        name = "rewardPerTokenNumCheckpoints",
        abi = "rewardPerTokenNumCheckpoints(address)"
    )]
    pub struct RewardPerTokenNumCheckpointsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `rewardPerTokenStored` function with signature `rewardPerTokenStored(address)` and selector `0x9ce43f90`
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
    #[ethcall(name = "rewardPerTokenStored", abi = "rewardPerTokenStored(address)")]
    pub struct RewardPerTokenStoredCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `rewardRate` function with signature `rewardRate(address)` and selector `0x221ca18c`
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
    #[ethcall(name = "rewardRate", abi = "rewardRate(address)")]
    pub struct RewardRateCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `rewards` function with signature `rewards(uint256)` and selector `0xf301af42`
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
    #[ethcall(name = "rewards", abi = "rewards(uint256)")]
    pub struct RewardsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `rewardsListLength` function with signature `rewardsListLength()` and selector `0xe6886396`
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
    #[ethcall(name = "rewardsListLength", abi = "rewardsListLength()")]
    pub struct RewardsListLengthCall;
    ///Container type for all input parameters for the `setOFlow` function with signature `setOFlow(address)` and selector `0xf00dc8ef`
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
    #[ethcall(name = "setOFlow", abi = "setOFlow(address)")]
    pub struct SetOFlowCall {
        pub o_flow: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stake` function with signature `stake()` and selector `0x3a4b66f1`
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
    #[ethcall(name = "stake", abi = "stake()")]
    pub struct StakeCall;
    ///Container type for all input parameters for the `supplyCheckpoints` function with signature `supplyCheckpoints(uint256)` and selector `0xf7412baf`
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
    #[ethcall(name = "supplyCheckpoints", abi = "supplyCheckpoints(uint256)")]
    pub struct SupplyCheckpointsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `supplyNumCheckpoints` function with signature `supplyNumCheckpoints()` and selector `0xe8111a12`
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
    #[ethcall(name = "supplyNumCheckpoints", abi = "supplyNumCheckpoints()")]
    pub struct SupplyNumCheckpointsCall;
    ///Container type for all input parameters for the `swapOutRewardToken` function with signature `swapOutRewardToken(uint256,address,address)` and selector `0x9418f939`
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
        name = "swapOutRewardToken",
        abi = "swapOutRewardToken(uint256,address,address)"
    )]
    pub struct SwapOutRewardTokenCall {
        pub i: ::ethers::core::types::U256,
        pub old_token: ::ethers::core::types::Address,
        pub new_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `tokenIds` function with signature `tokenIds(address)` and selector `0xfc97a303`
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
    #[ethcall(name = "tokenIds", abi = "tokenIds(address)")]
    pub struct TokenIdsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `userRewardPerTokenStored` function with signature `userRewardPerTokenStored(address,address)` and selector `0x3ca068b6`
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
        name = "userRewardPerTokenStored",
        abi = "userRewardPerTokenStored(address,address)"
    )]
    pub struct UserRewardPerTokenStoredCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `voter` function with signature `voter()` and selector `0x46c96aac`
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
    #[ethcall(name = "voter", abi = "voter()")]
    pub struct VoterCall;
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawAll` function with signature `withdrawAll()` and selector `0x853828b6`
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
    #[ethcall(name = "withdrawAll", abi = "withdrawAll()")]
    pub struct WithdrawAllCall;
    ///Container type for all input parameters for the `withdrawToken` function with signature `withdrawToken(uint256,uint256)` and selector `0xfdb483c7`
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
    #[ethcall(name = "withdrawToken", abi = "withdrawToken(uint256,uint256)")]
    pub struct WithdrawTokenCall {
        pub amount: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GaugeCalls {
        Ve(VeCall),
        BalanceOf(BalanceOfCall),
        BatchRewardPerToken(BatchRewardPerTokenCall),
        BatchUpdateRewardPerToken(BatchUpdateRewardPerTokenCall),
        Checkpoints(CheckpointsCall),
        Deposit(DepositCall),
        DepositAll(DepositAllCall),
        DerivedBalance(DerivedBalanceCall),
        DerivedBalances(DerivedBalancesCall),
        DerivedSupply(DerivedSupplyCall),
        Earned(EarnedCall),
        Fees0(Fees0Call),
        Fees1(Fees1Call),
        Flow(FlowCall),
        GaugeFactory(GaugeFactoryCall),
        GetPriorBalanceIndex(GetPriorBalanceIndexCall),
        GetPriorRewardPerToken(GetPriorRewardPerTokenCall),
        GetPriorSupplyIndex(GetPriorSupplyIndexCall),
        GetReward(GetRewardCall),
        IsForPair(IsForPairCall),
        IsReward(IsRewardCall),
        LastEarn(LastEarnCall),
        LastTimeRewardApplicable(LastTimeRewardApplicableCall),
        LastUpdateTime(LastUpdateTimeCall),
        Left(LeftCall),
        NotifyRewardAmount(NotifyRewardAmountCall),
        NumCheckpoints(NumCheckpointsCall),
        Oflow(OflowCall),
        PeriodFinish(PeriodFinishCall),
        RewardPerToken(RewardPerTokenCall),
        RewardPerTokenCheckpoints(RewardPerTokenCheckpointsCall),
        RewardPerTokenNumCheckpoints(RewardPerTokenNumCheckpointsCall),
        RewardPerTokenStored(RewardPerTokenStoredCall),
        RewardRate(RewardRateCall),
        Rewards(RewardsCall),
        RewardsListLength(RewardsListLengthCall),
        SetOFlow(SetOFlowCall),
        Stake(StakeCall),
        SupplyCheckpoints(SupplyCheckpointsCall),
        SupplyNumCheckpoints(SupplyNumCheckpointsCall),
        SwapOutRewardToken(SwapOutRewardTokenCall),
        TokenIds(TokenIdsCall),
        TotalSupply(TotalSupplyCall),
        UserRewardPerTokenStored(UserRewardPerTokenStoredCall),
        Voter(VoterCall),
        Withdraw(WithdrawCall),
        WithdrawAll(WithdrawAllCall),
        WithdrawToken(WithdrawTokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for GaugeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <VeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BatchRewardPerTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BatchRewardPerToken(decoded));
            }
            if let Ok(decoded)
                = <BatchUpdateRewardPerTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BatchUpdateRewardPerToken(decoded));
            }
            if let Ok(decoded)
                = <CheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Checkpoints(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <DepositAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositAll(decoded));
            }
            if let Ok(decoded)
                = <DerivedBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DerivedBalance(decoded));
            }
            if let Ok(decoded)
                = <DerivedBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DerivedBalances(decoded));
            }
            if let Ok(decoded)
                = <DerivedSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DerivedSupply(decoded));
            }
            if let Ok(decoded)
                = <EarnedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Earned(decoded));
            }
            if let Ok(decoded)
                = <Fees0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fees0(decoded));
            }
            if let Ok(decoded)
                = <Fees1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fees1(decoded));
            }
            if let Ok(decoded)
                = <FlowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Flow(decoded));
            }
            if let Ok(decoded)
                = <GaugeFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GaugeFactory(decoded));
            }
            if let Ok(decoded)
                = <GetPriorBalanceIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPriorBalanceIndex(decoded));
            }
            if let Ok(decoded)
                = <GetPriorRewardPerTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPriorRewardPerToken(decoded));
            }
            if let Ok(decoded)
                = <GetPriorSupplyIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPriorSupplyIndex(decoded));
            }
            if let Ok(decoded)
                = <GetRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReward(decoded));
            }
            if let Ok(decoded)
                = <IsForPairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsForPair(decoded));
            }
            if let Ok(decoded)
                = <IsRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsReward(decoded));
            }
            if let Ok(decoded)
                = <LastEarnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastEarn(decoded));
            }
            if let Ok(decoded)
                = <LastTimeRewardApplicableCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LastTimeRewardApplicable(decoded));
            }
            if let Ok(decoded)
                = <LastUpdateTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastUpdateTime(decoded));
            }
            if let Ok(decoded)
                = <LeftCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Left(decoded));
            }
            if let Ok(decoded)
                = <NotifyRewardAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotifyRewardAmount(decoded));
            }
            if let Ok(decoded)
                = <NumCheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NumCheckpoints(decoded));
            }
            if let Ok(decoded)
                = <OflowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Oflow(decoded));
            }
            if let Ok(decoded)
                = <PeriodFinishCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeriodFinish(decoded));
            }
            if let Ok(decoded)
                = <RewardPerTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardPerToken(decoded));
            }
            if let Ok(decoded)
                = <RewardPerTokenCheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RewardPerTokenCheckpoints(decoded));
            }
            if let Ok(decoded)
                = <RewardPerTokenNumCheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RewardPerTokenNumCheckpoints(decoded));
            }
            if let Ok(decoded)
                = <RewardPerTokenStoredCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RewardPerTokenStored(decoded));
            }
            if let Ok(decoded)
                = <RewardRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardRate(decoded));
            }
            if let Ok(decoded)
                = <RewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rewards(decoded));
            }
            if let Ok(decoded)
                = <RewardsListLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RewardsListLength(decoded));
            }
            if let Ok(decoded)
                = <SetOFlowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOFlow(decoded));
            }
            if let Ok(decoded)
                = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded)
                = <SupplyCheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupplyCheckpoints(decoded));
            }
            if let Ok(decoded)
                = <SupplyNumCheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupplyNumCheckpoints(decoded));
            }
            if let Ok(decoded)
                = <SwapOutRewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapOutRewardToken(decoded));
            }
            if let Ok(decoded)
                = <TokenIdsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenIds(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <UserRewardPerTokenStoredCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UserRewardPerTokenStored(decoded));
            }
            if let Ok(decoded)
                = <VoterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded)
                = <WithdrawAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawAll(decoded));
            }
            if let Ok(decoded)
                = <WithdrawTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawToken(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GaugeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Ve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchRewardPerToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchUpdateRewardPerToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Checkpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DerivedBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DerivedBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DerivedSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Earned(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fees0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fees1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Flow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GaugeFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriorBalanceIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriorRewardPerToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriorSupplyIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsForPair(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastEarn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastTimeRewardApplicable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastUpdateTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Left(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotifyRewardAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Oflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeriodFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPerToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPerTokenCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPerTokenNumCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPerTokenStored(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardsListLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOFlow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupplyCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyNumCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapOutRewardToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserRewardPerTokenStored(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GaugeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Ve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchRewardPerToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchUpdateRewardPerToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Checkpoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::DerivedBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::DerivedBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::DerivedSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Earned(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Flow(element) => ::core::fmt::Display::fmt(element, f),
                Self::GaugeFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriorBalanceIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPriorRewardPerToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPriorSupplyIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsForPair(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastEarn(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastTimeRewardApplicable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastUpdateTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::Left(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotifyRewardAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NumCheckpoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::Oflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeriodFinish(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPerToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPerTokenCheckpoints(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardPerTokenNumCheckpoints(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardPerTokenStored(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardsListLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOFlow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyCheckpoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyNumCheckpoints(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapOutRewardToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserRewardPerTokenStored(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawToken(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<VeCall> for GaugeCalls {
        fn from(value: VeCall) -> Self {
            Self::Ve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for GaugeCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BatchRewardPerTokenCall> for GaugeCalls {
        fn from(value: BatchRewardPerTokenCall) -> Self {
            Self::BatchRewardPerToken(value)
        }
    }
    impl ::core::convert::From<BatchUpdateRewardPerTokenCall> for GaugeCalls {
        fn from(value: BatchUpdateRewardPerTokenCall) -> Self {
            Self::BatchUpdateRewardPerToken(value)
        }
    }
    impl ::core::convert::From<CheckpointsCall> for GaugeCalls {
        fn from(value: CheckpointsCall) -> Self {
            Self::Checkpoints(value)
        }
    }
    impl ::core::convert::From<DepositCall> for GaugeCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositAllCall> for GaugeCalls {
        fn from(value: DepositAllCall) -> Self {
            Self::DepositAll(value)
        }
    }
    impl ::core::convert::From<DerivedBalanceCall> for GaugeCalls {
        fn from(value: DerivedBalanceCall) -> Self {
            Self::DerivedBalance(value)
        }
    }
    impl ::core::convert::From<DerivedBalancesCall> for GaugeCalls {
        fn from(value: DerivedBalancesCall) -> Self {
            Self::DerivedBalances(value)
        }
    }
    impl ::core::convert::From<DerivedSupplyCall> for GaugeCalls {
        fn from(value: DerivedSupplyCall) -> Self {
            Self::DerivedSupply(value)
        }
    }
    impl ::core::convert::From<EarnedCall> for GaugeCalls {
        fn from(value: EarnedCall) -> Self {
            Self::Earned(value)
        }
    }
    impl ::core::convert::From<Fees0Call> for GaugeCalls {
        fn from(value: Fees0Call) -> Self {
            Self::Fees0(value)
        }
    }
    impl ::core::convert::From<Fees1Call> for GaugeCalls {
        fn from(value: Fees1Call) -> Self {
            Self::Fees1(value)
        }
    }
    impl ::core::convert::From<FlowCall> for GaugeCalls {
        fn from(value: FlowCall) -> Self {
            Self::Flow(value)
        }
    }
    impl ::core::convert::From<GaugeFactoryCall> for GaugeCalls {
        fn from(value: GaugeFactoryCall) -> Self {
            Self::GaugeFactory(value)
        }
    }
    impl ::core::convert::From<GetPriorBalanceIndexCall> for GaugeCalls {
        fn from(value: GetPriorBalanceIndexCall) -> Self {
            Self::GetPriorBalanceIndex(value)
        }
    }
    impl ::core::convert::From<GetPriorRewardPerTokenCall> for GaugeCalls {
        fn from(value: GetPriorRewardPerTokenCall) -> Self {
            Self::GetPriorRewardPerToken(value)
        }
    }
    impl ::core::convert::From<GetPriorSupplyIndexCall> for GaugeCalls {
        fn from(value: GetPriorSupplyIndexCall) -> Self {
            Self::GetPriorSupplyIndex(value)
        }
    }
    impl ::core::convert::From<GetRewardCall> for GaugeCalls {
        fn from(value: GetRewardCall) -> Self {
            Self::GetReward(value)
        }
    }
    impl ::core::convert::From<IsForPairCall> for GaugeCalls {
        fn from(value: IsForPairCall) -> Self {
            Self::IsForPair(value)
        }
    }
    impl ::core::convert::From<IsRewardCall> for GaugeCalls {
        fn from(value: IsRewardCall) -> Self {
            Self::IsReward(value)
        }
    }
    impl ::core::convert::From<LastEarnCall> for GaugeCalls {
        fn from(value: LastEarnCall) -> Self {
            Self::LastEarn(value)
        }
    }
    impl ::core::convert::From<LastTimeRewardApplicableCall> for GaugeCalls {
        fn from(value: LastTimeRewardApplicableCall) -> Self {
            Self::LastTimeRewardApplicable(value)
        }
    }
    impl ::core::convert::From<LastUpdateTimeCall> for GaugeCalls {
        fn from(value: LastUpdateTimeCall) -> Self {
            Self::LastUpdateTime(value)
        }
    }
    impl ::core::convert::From<LeftCall> for GaugeCalls {
        fn from(value: LeftCall) -> Self {
            Self::Left(value)
        }
    }
    impl ::core::convert::From<NotifyRewardAmountCall> for GaugeCalls {
        fn from(value: NotifyRewardAmountCall) -> Self {
            Self::NotifyRewardAmount(value)
        }
    }
    impl ::core::convert::From<NumCheckpointsCall> for GaugeCalls {
        fn from(value: NumCheckpointsCall) -> Self {
            Self::NumCheckpoints(value)
        }
    }
    impl ::core::convert::From<OflowCall> for GaugeCalls {
        fn from(value: OflowCall) -> Self {
            Self::Oflow(value)
        }
    }
    impl ::core::convert::From<PeriodFinishCall> for GaugeCalls {
        fn from(value: PeriodFinishCall) -> Self {
            Self::PeriodFinish(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenCall> for GaugeCalls {
        fn from(value: RewardPerTokenCall) -> Self {
            Self::RewardPerToken(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenCheckpointsCall> for GaugeCalls {
        fn from(value: RewardPerTokenCheckpointsCall) -> Self {
            Self::RewardPerTokenCheckpoints(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenNumCheckpointsCall> for GaugeCalls {
        fn from(value: RewardPerTokenNumCheckpointsCall) -> Self {
            Self::RewardPerTokenNumCheckpoints(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenStoredCall> for GaugeCalls {
        fn from(value: RewardPerTokenStoredCall) -> Self {
            Self::RewardPerTokenStored(value)
        }
    }
    impl ::core::convert::From<RewardRateCall> for GaugeCalls {
        fn from(value: RewardRateCall) -> Self {
            Self::RewardRate(value)
        }
    }
    impl ::core::convert::From<RewardsCall> for GaugeCalls {
        fn from(value: RewardsCall) -> Self {
            Self::Rewards(value)
        }
    }
    impl ::core::convert::From<RewardsListLengthCall> for GaugeCalls {
        fn from(value: RewardsListLengthCall) -> Self {
            Self::RewardsListLength(value)
        }
    }
    impl ::core::convert::From<SetOFlowCall> for GaugeCalls {
        fn from(value: SetOFlowCall) -> Self {
            Self::SetOFlow(value)
        }
    }
    impl ::core::convert::From<StakeCall> for GaugeCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<SupplyCheckpointsCall> for GaugeCalls {
        fn from(value: SupplyCheckpointsCall) -> Self {
            Self::SupplyCheckpoints(value)
        }
    }
    impl ::core::convert::From<SupplyNumCheckpointsCall> for GaugeCalls {
        fn from(value: SupplyNumCheckpointsCall) -> Self {
            Self::SupplyNumCheckpoints(value)
        }
    }
    impl ::core::convert::From<SwapOutRewardTokenCall> for GaugeCalls {
        fn from(value: SwapOutRewardTokenCall) -> Self {
            Self::SwapOutRewardToken(value)
        }
    }
    impl ::core::convert::From<TokenIdsCall> for GaugeCalls {
        fn from(value: TokenIdsCall) -> Self {
            Self::TokenIds(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for GaugeCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<UserRewardPerTokenStoredCall> for GaugeCalls {
        fn from(value: UserRewardPerTokenStoredCall) -> Self {
            Self::UserRewardPerTokenStored(value)
        }
    }
    impl ::core::convert::From<VoterCall> for GaugeCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for GaugeCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawAllCall> for GaugeCalls {
        fn from(value: WithdrawAllCall) -> Self {
            Self::WithdrawAll(value)
        }
    }
    impl ::core::convert::From<WithdrawTokenCall> for GaugeCalls {
        fn from(value: WithdrawTokenCall) -> Self {
            Self::WithdrawToken(value)
        }
    }
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
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `checkpoints` function with signature `checkpoints(address,uint256)` and selector `0x0cdfebfa`
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
    pub struct CheckpointsReturn {
        pub timestamp: ::ethers::core::types::U256,
        pub balance_of: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `derivedBalance` function with signature `derivedBalance(address)` and selector `0xd35e2544`
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
    pub struct DerivedBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `derivedBalances` function with signature `derivedBalances(address)` and selector `0x63fb415b`
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
    pub struct DerivedBalancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `derivedSupply` function with signature `derivedSupply()` and selector `0xd7da4bb0`
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
    pub struct DerivedSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `earned` function with signature `earned(address,address)` and selector `0x211dc32d`
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
    pub struct EarnedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fees0` function with signature `fees0()` and selector `0x93f1c442`
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
    pub struct Fees0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fees1` function with signature `fees1()` and selector `0x4c02a21c`
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
    pub struct Fees1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `flow` function with signature `flow()` and selector `0x343aad82`
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
    ///Container type for all return fields from the `gaugeFactory` function with signature `gaugeFactory()` and selector `0x0d52333c`
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
    pub struct GaugeFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPriorBalanceIndex` function with signature `getPriorBalanceIndex(address,uint256)` and selector `0x115c6f39`
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
    pub struct GetPriorBalanceIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPriorRewardPerToken` function with signature `getPriorRewardPerToken(address,uint256)` and selector `0xfd314098`
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
    pub struct GetPriorRewardPerTokenReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getPriorSupplyIndex` function with signature `getPriorSupplyIndex(uint256)` and selector `0x76f4be36`
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
    pub struct GetPriorSupplyIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isForPair` function with signature `isForPair()` and selector `0xe5748213`
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
    pub struct IsForPairReturn(pub bool);
    ///Container type for all return fields from the `isReward` function with signature `isReward(address)` and selector `0x4d5ce038`
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
    pub struct IsRewardReturn(pub bool);
    ///Container type for all return fields from the `lastEarn` function with signature `lastEarn(address,address)` and selector `0xa495e5b5`
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
    pub struct LastEarnReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastTimeRewardApplicable` function with signature `lastTimeRewardApplicable(address)` and selector `0x638634ee`
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
    pub struct LastTimeRewardApplicableReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastUpdateTime` function with signature `lastUpdateTime(address)` and selector `0x2ce9aead`
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
    pub struct LastUpdateTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `left` function with signature `left(address)` and selector `0x99bcc052`
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
    pub struct LeftReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `numCheckpoints` function with signature `numCheckpoints(address)` and selector `0x6fcfff45`
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
    pub struct NumCheckpointsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `oFlow` function with signature `oFlow()` and selector `0x4665b5c9`
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
    pub struct OflowReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `periodFinish` function with signature `periodFinish(address)` and selector `0xda09d19d`
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
    pub struct PeriodFinishReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPerToken` function with signature `rewardPerToken(address)` and selector `0xf1229777`
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
    pub struct RewardPerTokenReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPerTokenCheckpoints` function with signature `rewardPerTokenCheckpoints(address,uint256)` and selector `0x01316ddf`
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
    pub struct RewardPerTokenCheckpointsReturn {
        pub timestamp: ::ethers::core::types::U256,
        pub reward_per_token: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `rewardPerTokenNumCheckpoints` function with signature `rewardPerTokenNumCheckpoints(address)` and selector `0xaa479652`
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
    pub struct RewardPerTokenNumCheckpointsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPerTokenStored` function with signature `rewardPerTokenStored(address)` and selector `0x9ce43f90`
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
    pub struct RewardPerTokenStoredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardRate` function with signature `rewardRate(address)` and selector `0x221ca18c`
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
    pub struct RewardRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewards` function with signature `rewards(uint256)` and selector `0xf301af42`
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
    pub struct RewardsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewardsListLength` function with signature `rewardsListLength()` and selector `0xe6886396`
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
    pub struct RewardsListLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stake` function with signature `stake()` and selector `0x3a4b66f1`
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
    pub struct StakeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `supplyCheckpoints` function with signature `supplyCheckpoints(uint256)` and selector `0xf7412baf`
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
    pub struct SupplyCheckpointsReturn {
        pub timestamp: ::ethers::core::types::U256,
        pub supply: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `supplyNumCheckpoints` function with signature `supplyNumCheckpoints()` and selector `0xe8111a12`
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
    pub struct SupplyNumCheckpointsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenIds` function with signature `tokenIds(address)` and selector `0xfc97a303`
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
    pub struct TokenIdsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userRewardPerTokenStored` function with signature `userRewardPerTokenStored(address,address)` and selector `0x3ca068b6`
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
    pub struct UserRewardPerTokenStoredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `voter` function with signature `voter()` and selector `0x46c96aac`
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
}
