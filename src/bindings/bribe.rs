pub use bribe::*;
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
pub mod bribe {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_voter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("_deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_deposit"),
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
                    ::std::borrow::ToOwned::to_owned("_withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_withdraw"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("checkpoints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkpoints"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("getEpochStart"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEpochStart"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("getRewardForOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRewardForOwner"),
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
                    ::std::borrow::ToOwned::to_owned("handleLeftOverRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "handleLeftOverRewards",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epochTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("tokenRewardsPerEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "tokenRewardsPerEpoch",
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
                    ::std::borrow::ToOwned::to_owned("HandleLeftOverRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HandleLeftOverRewards",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("originalEpoch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("updatedEpoch"),
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
                                    name: ::std::borrow::ToOwned::to_owned("epoch"),
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
    pub static BRIBE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Bribe<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Bribe<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Bribe<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Bribe<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Bribe<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Bribe)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Bribe<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BRIBE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `_deposit` (0xf3207723) function
        pub fn deposit(
            &self,
            amount: ::ethers::core::types::U256,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 32, 119, 35], (amount, token_id))
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
        ///Calls the contract's `_withdraw` (0x9e2bf22c) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 43, 242, 44], (amount, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x9cc7f708) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([156, 199, 247, 8], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkpoints` (0x49dcc204) function
        pub fn checkpoints(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([73, 220, 194, 4], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earned` (0x3e491d47) function
        pub fn earned(
            &self,
            token: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 73, 29, 71], (token, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEpochStart` (0x0175e23b) function
        pub fn get_epoch_start(
            &self,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 117, 226, 59], timestamp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriorBalanceIndex` (0xa28d4c9c) function
        pub fn get_prior_balance_index(
            &self,
            token_id: ::ethers::core::types::U256,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([162, 141, 76, 156], (token_id, timestamp))
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
        ///Calls the contract's `getReward` (0xf5f8d365) function
        pub fn get_reward(
            &self,
            token_id: ::ethers::core::types::U256,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 248, 211, 101], (token_id, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRewardForOwner` (0xa7852afa) function
        pub fn get_reward_for_owner(
            &self,
            token_id: ::ethers::core::types::U256,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 133, 42, 250], (token_id, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handleLeftOverRewards` (0x12a24cbb) function
        pub fn handle_left_over_rewards(
            &self,
            epoch_timestamp: ::ethers::core::types::U256,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 162, 76, 187], (epoch_timestamp, tokens))
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
        ///Calls the contract's `lastEarn` (0xf25e55a5) function
        pub fn last_earn(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([242, 94, 85, 165], (p0, p1))
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
        ///Calls the contract's `numCheckpoints` (0x50589793) function
        pub fn num_checkpoints(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 88, 151, 147], p0)
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
        ///Calls the contract's `tokenRewardsPerEpoch` (0x92777b29) function
        pub fn token_rewards_per_epoch(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([146, 119, 123, 41], (p0, p1))
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
        ///Gets the contract's `HandleLeftOverRewards` event
        pub fn handle_left_over_rewards_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HandleLeftOverRewardsFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BribeEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Bribe<M> {
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
    #[ethevent(
        name = "HandleLeftOverRewards",
        abi = "HandleLeftOverRewards(address,uint256,uint256,uint256)"
    )]
    pub struct HandleLeftOverRewardsFilter {
        #[ethevent(indexed)]
        pub reward: ::ethers::core::types::Address,
        pub original_epoch: ::ethers::core::types::U256,
        pub updated_epoch: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "NotifyReward",
        abi = "NotifyReward(address,address,uint256,uint256)"
    )]
    pub struct NotifyRewardFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reward: ::ethers::core::types::Address,
        pub epoch: ::ethers::core::types::U256,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,uint256,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BribeEvents {
        ClaimRewardsFilter(ClaimRewardsFilter),
        DepositFilter(DepositFilter),
        HandleLeftOverRewardsFilter(HandleLeftOverRewardsFilter),
        NotifyRewardFilter(NotifyRewardFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for BribeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ClaimRewardsFilter::decode_log(log) {
                return Ok(BribeEvents::ClaimRewardsFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(BribeEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = HandleLeftOverRewardsFilter::decode_log(log) {
                return Ok(BribeEvents::HandleLeftOverRewardsFilter(decoded));
            }
            if let Ok(decoded) = NotifyRewardFilter::decode_log(log) {
                return Ok(BribeEvents::NotifyRewardFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(BribeEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BribeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimRewardsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::HandleLeftOverRewardsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotifyRewardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimRewardsFilter> for BribeEvents {
        fn from(value: ClaimRewardsFilter) -> Self {
            Self::ClaimRewardsFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for BribeEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<HandleLeftOverRewardsFilter> for BribeEvents {
        fn from(value: HandleLeftOverRewardsFilter) -> Self {
            Self::HandleLeftOverRewardsFilter(value)
        }
    }
    impl ::core::convert::From<NotifyRewardFilter> for BribeEvents {
        fn from(value: NotifyRewardFilter) -> Self {
            Self::NotifyRewardFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for BribeEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `_deposit` function with signature `_deposit(uint256,uint256)` and selector `0xf3207723`
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
    #[ethcall(name = "_deposit", abi = "_deposit(uint256,uint256)")]
    pub struct DepositCall {
        pub amount: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `_withdraw` function with signature `_withdraw(uint256,uint256)` and selector `0x9e2bf22c`
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
    #[ethcall(name = "_withdraw", abi = "_withdraw(uint256,uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(uint256)` and selector `0x9cc7f708`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(uint256)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `checkpoints` function with signature `checkpoints(uint256,uint256)` and selector `0x49dcc204`
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
    #[ethcall(name = "checkpoints", abi = "checkpoints(uint256,uint256)")]
    pub struct CheckpointsCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `earned` function with signature `earned(address,uint256)` and selector `0x3e491d47`
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
    #[ethcall(name = "earned", abi = "earned(address,uint256)")]
    pub struct EarnedCall {
        pub token: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getEpochStart` function with signature `getEpochStart(uint256)` and selector `0x0175e23b`
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
    #[ethcall(name = "getEpochStart", abi = "getEpochStart(uint256)")]
    pub struct GetEpochStartCall {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPriorBalanceIndex` function with signature `getPriorBalanceIndex(uint256,uint256)` and selector `0xa28d4c9c`
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
        abi = "getPriorBalanceIndex(uint256,uint256)"
    )]
    pub struct GetPriorBalanceIndexCall {
        pub token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `getReward` function with signature `getReward(uint256,address[])` and selector `0xf5f8d365`
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
    #[ethcall(name = "getReward", abi = "getReward(uint256,address[])")]
    pub struct GetRewardCall {
        pub token_id: ::ethers::core::types::U256,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getRewardForOwner` function with signature `getRewardForOwner(uint256,address[])` and selector `0xa7852afa`
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
    #[ethcall(name = "getRewardForOwner", abi = "getRewardForOwner(uint256,address[])")]
    pub struct GetRewardForOwnerCall {
        pub token_id: ::ethers::core::types::U256,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `handleLeftOverRewards` function with signature `handleLeftOverRewards(uint256,address[])` and selector `0x12a24cbb`
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
        name = "handleLeftOverRewards",
        abi = "handleLeftOverRewards(uint256,address[])"
    )]
    pub struct HandleLeftOverRewardsCall {
        pub epoch_timestamp: ::ethers::core::types::U256,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
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
    ///Container type for all input parameters for the `lastEarn` function with signature `lastEarn(address,uint256)` and selector `0xf25e55a5`
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
    #[ethcall(name = "lastEarn", abi = "lastEarn(address,uint256)")]
    pub struct LastEarnCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `numCheckpoints` function with signature `numCheckpoints(uint256)` and selector `0x50589793`
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
    #[ethcall(name = "numCheckpoints", abi = "numCheckpoints(uint256)")]
    pub struct NumCheckpointsCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `tokenRewardsPerEpoch` function with signature `tokenRewardsPerEpoch(address,uint256)` and selector `0x92777b29`
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
        name = "tokenRewardsPerEpoch",
        abi = "tokenRewardsPerEpoch(address,uint256)"
    )]
    pub struct TokenRewardsPerEpochCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BribeCalls {
        Deposit(DepositCall),
        Ve(VeCall),
        Withdraw(WithdrawCall),
        BalanceOf(BalanceOfCall),
        Checkpoints(CheckpointsCall),
        Earned(EarnedCall),
        GetEpochStart(GetEpochStartCall),
        GetPriorBalanceIndex(GetPriorBalanceIndexCall),
        GetPriorSupplyIndex(GetPriorSupplyIndexCall),
        GetReward(GetRewardCall),
        GetRewardForOwner(GetRewardForOwnerCall),
        HandleLeftOverRewards(HandleLeftOverRewardsCall),
        IsReward(IsRewardCall),
        LastEarn(LastEarnCall),
        LastTimeRewardApplicable(LastTimeRewardApplicableCall),
        Left(LeftCall),
        NotifyRewardAmount(NotifyRewardAmountCall),
        NumCheckpoints(NumCheckpointsCall),
        PeriodFinish(PeriodFinishCall),
        Rewards(RewardsCall),
        RewardsListLength(RewardsListLengthCall),
        SupplyCheckpoints(SupplyCheckpointsCall),
        SupplyNumCheckpoints(SupplyNumCheckpointsCall),
        SwapOutRewardToken(SwapOutRewardTokenCall),
        TokenRewardsPerEpoch(TokenRewardsPerEpochCall),
        TotalSupply(TotalSupplyCall),
        Voter(VoterCall),
    }
    impl ::ethers::core::abi::AbiDecode for BribeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <VeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ve(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <CheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Checkpoints(decoded));
            }
            if let Ok(decoded)
                = <EarnedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Earned(decoded));
            }
            if let Ok(decoded)
                = <GetEpochStartCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetEpochStart(decoded));
            }
            if let Ok(decoded)
                = <GetPriorBalanceIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPriorBalanceIndex(decoded));
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
                = <GetRewardForOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRewardForOwner(decoded));
            }
            if let Ok(decoded)
                = <HandleLeftOverRewardsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HandleLeftOverRewards(decoded));
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
                = <PeriodFinishCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PeriodFinish(decoded));
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
                = <TokenRewardsPerEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TokenRewardsPerEpoch(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <VoterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BribeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Checkpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Earned(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEpochStart(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriorBalanceIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriorSupplyIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRewardForOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HandleLeftOverRewards(element) => {
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
                Self::Left(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotifyRewardAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeriodFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardsListLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyNumCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapOutRewardToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenRewardsPerEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BribeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Checkpoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::Earned(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEpochStart(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriorBalanceIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPriorSupplyIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRewardForOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::HandleLeftOverRewards(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastEarn(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastTimeRewardApplicable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Left(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotifyRewardAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NumCheckpoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeriodFinish(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardsListLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyCheckpoints(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyNumCheckpoints(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapOutRewardToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenRewardsPerEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Voter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for BribeCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<VeCall> for BribeCalls {
        fn from(value: VeCall) -> Self {
            Self::Ve(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for BribeCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for BribeCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<CheckpointsCall> for BribeCalls {
        fn from(value: CheckpointsCall) -> Self {
            Self::Checkpoints(value)
        }
    }
    impl ::core::convert::From<EarnedCall> for BribeCalls {
        fn from(value: EarnedCall) -> Self {
            Self::Earned(value)
        }
    }
    impl ::core::convert::From<GetEpochStartCall> for BribeCalls {
        fn from(value: GetEpochStartCall) -> Self {
            Self::GetEpochStart(value)
        }
    }
    impl ::core::convert::From<GetPriorBalanceIndexCall> for BribeCalls {
        fn from(value: GetPriorBalanceIndexCall) -> Self {
            Self::GetPriorBalanceIndex(value)
        }
    }
    impl ::core::convert::From<GetPriorSupplyIndexCall> for BribeCalls {
        fn from(value: GetPriorSupplyIndexCall) -> Self {
            Self::GetPriorSupplyIndex(value)
        }
    }
    impl ::core::convert::From<GetRewardCall> for BribeCalls {
        fn from(value: GetRewardCall) -> Self {
            Self::GetReward(value)
        }
    }
    impl ::core::convert::From<GetRewardForOwnerCall> for BribeCalls {
        fn from(value: GetRewardForOwnerCall) -> Self {
            Self::GetRewardForOwner(value)
        }
    }
    impl ::core::convert::From<HandleLeftOverRewardsCall> for BribeCalls {
        fn from(value: HandleLeftOverRewardsCall) -> Self {
            Self::HandleLeftOverRewards(value)
        }
    }
    impl ::core::convert::From<IsRewardCall> for BribeCalls {
        fn from(value: IsRewardCall) -> Self {
            Self::IsReward(value)
        }
    }
    impl ::core::convert::From<LastEarnCall> for BribeCalls {
        fn from(value: LastEarnCall) -> Self {
            Self::LastEarn(value)
        }
    }
    impl ::core::convert::From<LastTimeRewardApplicableCall> for BribeCalls {
        fn from(value: LastTimeRewardApplicableCall) -> Self {
            Self::LastTimeRewardApplicable(value)
        }
    }
    impl ::core::convert::From<LeftCall> for BribeCalls {
        fn from(value: LeftCall) -> Self {
            Self::Left(value)
        }
    }
    impl ::core::convert::From<NotifyRewardAmountCall> for BribeCalls {
        fn from(value: NotifyRewardAmountCall) -> Self {
            Self::NotifyRewardAmount(value)
        }
    }
    impl ::core::convert::From<NumCheckpointsCall> for BribeCalls {
        fn from(value: NumCheckpointsCall) -> Self {
            Self::NumCheckpoints(value)
        }
    }
    impl ::core::convert::From<PeriodFinishCall> for BribeCalls {
        fn from(value: PeriodFinishCall) -> Self {
            Self::PeriodFinish(value)
        }
    }
    impl ::core::convert::From<RewardsCall> for BribeCalls {
        fn from(value: RewardsCall) -> Self {
            Self::Rewards(value)
        }
    }
    impl ::core::convert::From<RewardsListLengthCall> for BribeCalls {
        fn from(value: RewardsListLengthCall) -> Self {
            Self::RewardsListLength(value)
        }
    }
    impl ::core::convert::From<SupplyCheckpointsCall> for BribeCalls {
        fn from(value: SupplyCheckpointsCall) -> Self {
            Self::SupplyCheckpoints(value)
        }
    }
    impl ::core::convert::From<SupplyNumCheckpointsCall> for BribeCalls {
        fn from(value: SupplyNumCheckpointsCall) -> Self {
            Self::SupplyNumCheckpoints(value)
        }
    }
    impl ::core::convert::From<SwapOutRewardTokenCall> for BribeCalls {
        fn from(value: SwapOutRewardTokenCall) -> Self {
            Self::SwapOutRewardToken(value)
        }
    }
    impl ::core::convert::From<TokenRewardsPerEpochCall> for BribeCalls {
        fn from(value: TokenRewardsPerEpochCall) -> Self {
            Self::TokenRewardsPerEpoch(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for BribeCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<VoterCall> for BribeCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
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
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(uint256)` and selector `0x9cc7f708`
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
    ///Container type for all return fields from the `checkpoints` function with signature `checkpoints(uint256,uint256)` and selector `0x49dcc204`
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
    ///Container type for all return fields from the `earned` function with signature `earned(address,uint256)` and selector `0x3e491d47`
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
    ///Container type for all return fields from the `getEpochStart` function with signature `getEpochStart(uint256)` and selector `0x0175e23b`
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
    pub struct GetEpochStartReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPriorBalanceIndex` function with signature `getPriorBalanceIndex(uint256,uint256)` and selector `0xa28d4c9c`
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
    ///Container type for all return fields from the `lastEarn` function with signature `lastEarn(address,uint256)` and selector `0xf25e55a5`
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
    ///Container type for all return fields from the `numCheckpoints` function with signature `numCheckpoints(uint256)` and selector `0x50589793`
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
    ///Container type for all return fields from the `tokenRewardsPerEpoch` function with signature `tokenRewardsPerEpoch(address,uint256)` and selector `0x92777b29`
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
    pub struct TokenRewardsPerEpochReturn(pub ::ethers::core::types::U256);
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
