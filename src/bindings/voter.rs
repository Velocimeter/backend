pub use voter::*;
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
pub mod voter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("__ve"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gauges"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bribes"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_factories"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_factories"),
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
                    ::std::borrow::ToOwned::to_owned("_gaugeFactories"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_gaugeFactories"),
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
                    ::std::borrow::ToOwned::to_owned("_killedGauges"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_killedGauges"),
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
                    ::std::borrow::ToOwned::to_owned("addFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addFactory"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pairFactory"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("attachTokenToGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("attachTokenToGauge"),
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
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("blacklist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blacklist"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token"),
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
                    ::std::borrow::ToOwned::to_owned("bribefactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bribefactory"),
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
                    ::std::borrow::ToOwned::to_owned("claimBribes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimBribes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bribes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[][]"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimRewards"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauges"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[][]"),
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
                    ::std::borrow::ToOwned::to_owned("claimable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimable"),
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
                    ::std::borrow::ToOwned::to_owned("createGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gaugeType"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("detachTokenFromGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "detachTokenFromGauge",
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
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("distribute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distribute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauges"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distribute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distribute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("finish"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distribute"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("distro"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distro"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emergencyCouncil"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emergencyCouncil"),
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
                    ::std::borrow::ToOwned::to_owned("emitDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitDeposit"),
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
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("emitWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitWithdraw"),
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
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("external_bribes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("external_bribes"),
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
                    ::std::borrow::ToOwned::to_owned("factories"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factories"),
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
                    ::std::borrow::ToOwned::to_owned("factoryLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factoryLength"),
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
                    ::std::borrow::ToOwned::to_owned("gaugeFactories"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gaugeFactories"),
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
                    ::std::borrow::ToOwned::to_owned("gaugeFactoriesLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "gaugeFactoriesLength",
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
                    ::std::borrow::ToOwned::to_owned("gauges"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gauges"),
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
                    ::std::borrow::ToOwned::to_owned("governor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("governor"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minter"),
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
                    ::std::borrow::ToOwned::to_owned("isAlive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isAlive"),
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
                    ::std::borrow::ToOwned::to_owned("isFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isFactory"),
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
                    ::std::borrow::ToOwned::to_owned("isGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isGauge"),
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
                    ::std::borrow::ToOwned::to_owned("isGaugeFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isGaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("isWhitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isWhitelisted"),
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
                    ::std::borrow::ToOwned::to_owned("killGaugeTotally"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("killGaugeTotally"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                    ::std::borrow::ToOwned::to_owned("killedGauges"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("killedGauges"),
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
                    ::std::borrow::ToOwned::to_owned("killedGaugesLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("killedGaugesLength"),
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
                    ::std::borrow::ToOwned::to_owned("lastVoted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastVoted"),
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
                    ::std::borrow::ToOwned::to_owned("length"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("length"),
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
                    ::std::borrow::ToOwned::to_owned("minter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minter"),
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
                    ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
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
                    ::std::borrow::ToOwned::to_owned("pauseGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauseGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                    ::std::borrow::ToOwned::to_owned("poke"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poke"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("poolForGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poolForGauge"),
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
                    ::std::borrow::ToOwned::to_owned("poolVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poolVote"),
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
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pools"),
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
                    ::std::borrow::ToOwned::to_owned("removeFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeFactory"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pos"),
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
                    ::std::borrow::ToOwned::to_owned("replaceFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("replaceFactory"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pairFactory"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_pos"),
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
                    ::std::borrow::ToOwned::to_owned("reset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reset"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("restartGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("restartGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                    ::std::borrow::ToOwned::to_owned("setBribeFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBribeFactory"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bribeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("setEmergencyCouncil"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setEmergencyCouncil",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_council"),
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
                    ::std::borrow::ToOwned::to_owned("setExternalBribeFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setExternalBribeFor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_external"),
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
                    ::std::borrow::ToOwned::to_owned("setGovernor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGovernor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_governor"),
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
                    ::std::borrow::ToOwned::to_owned("totalWeight"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalWeight"),
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
                    ::std::borrow::ToOwned::to_owned("updateAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateFor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauges"),
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
                    ::std::borrow::ToOwned::to_owned("updateForRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateForRange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end"),
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
                    ::std::borrow::ToOwned::to_owned("updateGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                    ::std::borrow::ToOwned::to_owned("usedWeights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("usedWeights"),
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
                    ::std::borrow::ToOwned::to_owned("vote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vote"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_poolVote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_weights"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("votes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("votes"),
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
                    ::std::borrow::ToOwned::to_owned("weights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weights"),
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
                    ::std::borrow::ToOwned::to_owned("whitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("whitelist"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Abstained"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Abstained"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
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
                    ::std::borrow::ToOwned::to_owned("Attach"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Attach"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Blacklisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Blacklisted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blacklister"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BribeFactorySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BribeFactorySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("setter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newBribeFatory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                                    name: ::std::borrow::ToOwned::to_owned("lp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
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
                    ::std::borrow::ToOwned::to_owned("Detach"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Detach"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DistributeReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DistributeReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
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
                    ::std::borrow::ToOwned::to_owned("ExternalBribeSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExternalBribeSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("setter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("externalBribe"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FactoryAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FactoryAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("setter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pairFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FactoryRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FactoryRemoved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("setter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pos"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FactoryReplaced"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FactoryReplaced"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("setter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pairFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pos"),
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
                    ::std::borrow::ToOwned::to_owned("GaugeCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GaugeCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("creator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("external_bribe"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GaugeKilledTotally"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GaugeKilledTotally"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GaugePaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GaugePaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GaugeRestarted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GaugeRestarted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
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
                    ::std::borrow::ToOwned::to_owned("Voted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Voted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voter"),
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
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
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
                    ::std::borrow::ToOwned::to_owned("Whitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Whitelisted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("whitelister"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                                    name: ::std::borrow::ToOwned::to_owned("lp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
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
    pub static VOTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Voter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Voter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Voter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Voter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Voter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Voter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Voter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VOTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `_factories` (0xe9f6adfa) function
        pub fn _factories(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([233, 246, 173, 250], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_gaugeFactories` (0x9fb5dc05) function
        pub fn _gauge_factories(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([159, 181, 220, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_killedGauges` (0xcbadada4) function
        pub fn _killed_gauges(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([203, 173, 173, 164], ())
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
        ///Calls the contract's `addFactory` (0x6566afad) function
        pub fn add_factory(
            &self,
            pair_factory: ::ethers::core::types::Address,
            gauge_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 102, 175, 173], (pair_factory, gauge_factory))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `attachTokenToGauge` (0x698473e3) function
        pub fn attach_token_to_gauge(
            &self,
            token_id: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 132, 115, 227], (token_id, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blacklist` (0xf9f92be4) function
        pub fn blacklist(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 249, 43, 228], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bribefactory` (0x38752a9d) function
        pub fn bribefactory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 117, 42, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimBribes` (0x7715ee75) function
        pub fn claim_bribes(
            &self,
            bribes: ::std::vec::Vec<::ethers::core::types::Address>,
            tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 21, 238, 117], (bribes, tokens, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimRewards` (0x20b1cb6f) function
        pub fn claim_rewards(
            &self,
            gauges: ::std::vec::Vec<::ethers::core::types::Address>,
            tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 177, 203, 111], (gauges, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimable` (0x402914f5) function
        pub fn claimable(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 41, 20, 245], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createGauge` (0xdcd9e47a) function
        pub fn create_gauge(
            &self,
            pool: ::ethers::core::types::Address,
            gauge_type: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([220, 217, 228, 122], (pool, gauge_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `detachTokenFromGauge` (0x411b1f77) function
        pub fn detach_token_from_gauge(
            &self,
            token_id: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 27, 31, 119], (token_id, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distribute` (0x6138889b) function
        pub fn distribute_1(
            &self,
            gauges: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 56, 136, 155], gauges)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distribute` (0x63453ae1) function
        pub fn distribute_2(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 69, 58, 225], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distribute` (0x7625391a) function
        pub fn distribute_3(
            &self,
            start: ::ethers::core::types::U256,
            finish: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 37, 57, 26], (start, finish))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distribute` (0xe4fc6b6d) function
        pub fn distribute_0(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 252, 107, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distro` (0x47b3c6ba) function
        pub fn distro(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 179, 198, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emergencyCouncil` (0x7778960e) function
        pub fn emergency_council(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([119, 120, 150, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitDeposit` (0xa61c713a) function
        pub fn emit_deposit(
            &self,
            token_id: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 28, 113, 58], (token_id, account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitWithdraw` (0xea94ee44) function
        pub fn emit_withdraw(
            &self,
            token_id: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 148, 238, 68], (token_id, account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `external_bribes` (0xae21c4cb) function
        pub fn external_bribes(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([174, 33, 196, 203], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factories` (0x672383c4) function
        pub fn factories(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([103, 35, 131, 196], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factoryLength` (0x470f4985) function
        pub fn factory_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 15, 73, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gaugeFactories` (0x23e1af42) function
        pub fn gauge_factories(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([35, 225, 175, 66], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gaugeFactoriesLength` (0xb52a3151) function
        pub fn gauge_factories_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([181, 42, 49, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauges` (0xb9a09fd5) function
        pub fn gauges(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([185, 160, 159, 213], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `governor` (0x0c340a24) function
        pub fn governor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([12, 52, 10, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x462d0b2e) function
        pub fn initialize(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            minter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 45, 11, 46], (tokens, minter))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAlive` (0x1703e5f9) function
        pub fn is_alive(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 3, 229, 249], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isFactory` (0x0f04ba67) function
        pub fn is_factory(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([15, 4, 186, 103], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isGauge` (0xaa79979b) function
        pub fn is_gauge(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([170, 121, 151, 155], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isGaugeFactory` (0x657021fb) function
        pub fn is_gauge_factory(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([101, 112, 33, 251], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isWhitelisted` (0x3af32abf) function
        pub fn is_whitelisted(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([58, 243, 42, 191], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `killGaugeTotally` (0x9edfd460) function
        pub fn kill_gauge_totally(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 223, 212, 96], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `killedGauges` (0x173de600) function
        pub fn killed_gauges(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([23, 61, 230, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `killedGaugesLength` (0xc448c78d) function
        pub fn killed_gauges_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 72, 199, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastVoted` (0xf3594be0) function
        pub fn last_voted(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 89, 75, 224], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `length` (0x1f7b6d32) function
        pub fn length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([31, 123, 109, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minter` (0x07546172) function
        pub fn minter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([7, 84, 97, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `notifyRewardAmount` (0x3c6b16ab) function
        pub fn notify_reward_amount(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 107, 22, 171], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseGauge` (0x03c39b00) function
        pub fn pause_gauge(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 195, 155, 0], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poke` (0x32145f90) function
        pub fn poke(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 20, 95, 144], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolForGauge` (0x06d6a1b2) function
        pub fn pool_for_gauge(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([6, 214, 161, 178], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolVote` (0xa86a366d) function
        pub fn pool_vote(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 106, 54, 109], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0xac4afa38) function
        pub fn pools(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([172, 74, 250, 56], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeFactory` (0x577387b5) function
        pub fn remove_factory(
            &self,
            pos: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 115, 135, 181], pos)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `replaceFactory` (0x27e5c823) function
        pub fn replace_factory(
            &self,
            pair_factory: ::ethers::core::types::Address,
            gauge_factory: ::ethers::core::types::Address,
            pos: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 229, 200, 35], (pair_factory, gauge_factory, pos))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reset` (0x310bd74b) function
        pub fn reset(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 11, 215, 75], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `restartGauge` (0xa82029f9) function
        pub fn restart_gauge(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([168, 32, 41, 249], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBribeFactory` (0xa9b5aa7e) function
        pub fn set_bribe_factory(
            &self,
            bribe_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 181, 170, 126], bribe_factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEmergencyCouncil` (0xe586875f) function
        pub fn set_emergency_council(
            &self,
            council: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 134, 135, 95], council)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setExternalBribeFor` (0xdaa168bd) function
        pub fn set_external_bribe_for(
            &self,
            gauge: ::ethers::core::types::Address,
            external: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 161, 104, 189], (gauge, external))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGovernor` (0xc42cf535) function
        pub fn set_governor(
            &self,
            governor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 44, 245, 53], governor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalWeight` (0x96c82e57) function
        pub fn total_weight(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([150, 200, 46, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateAll` (0x53d78693) function
        pub fn update_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 215, 134, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFor` (0xd560b0d7) function
        pub fn update_for(
            &self,
            gauges: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 96, 176, 215], gauges)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateForRange` (0x9b6a9d72) function
        pub fn update_for_range(
            &self,
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 106, 157, 114], (start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateGauge` (0x6ecbe38a) function
        pub fn update_gauge(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 203, 227, 138], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usedWeights` (0x79e93824) function
        pub fn used_weights(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([121, 233, 56, 36], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vote` (0x7ac09bf7) function
        pub fn vote(
            &self,
            token_id: ::ethers::core::types::U256,
            pool_vote: ::std::vec::Vec<::ethers::core::types::Address>,
            weights: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 192, 155, 247], (token_id, pool_vote, weights))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votes` (0xd23254b4) function
        pub fn votes(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([210, 50, 84, 180], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weights` (0xa7cac846) function
        pub fn weights(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 202, 200, 70], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelist` (0x9b19251a) function
        pub fn whitelist(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 25, 37, 26], token)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Abstained` event
        pub fn abstained_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AbstainedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Attach` event
        pub fn attach_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AttachFilter> {
            self.0.event()
        }
        ///Gets the contract's `Blacklisted` event
        pub fn blacklisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlacklistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BribeFactorySet` event
        pub fn bribe_factory_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BribeFactorySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `Detach` event
        pub fn detach_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DetachFilter> {
            self.0.event()
        }
        ///Gets the contract's `DistributeReward` event
        pub fn distribute_reward_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DistributeRewardFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExternalBribeSet` event
        pub fn external_bribe_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExternalBribeSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FactoryAdded` event
        pub fn factory_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FactoryAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FactoryRemoved` event
        pub fn factory_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FactoryRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FactoryReplaced` event
        pub fn factory_replaced_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FactoryReplacedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GaugeCreated` event
        pub fn gauge_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GaugeCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GaugeKilledTotally` event
        pub fn gauge_killed_totally_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GaugeKilledTotallyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GaugePaused` event
        pub fn gauge_paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GaugePausedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GaugeRestarted` event
        pub fn gauge_restarted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GaugeRestartedFilter,
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
        ///Gets the contract's `Voted` event
        pub fn voted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, VotedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Whitelisted` event
        pub fn whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WhitelistedFilter,
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, VoterEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Voter<M> {
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
    #[ethevent(name = "Abstained", abi = "Abstained(uint256,uint256)")]
    pub struct AbstainedFilter {
        pub token_id: ::ethers::core::types::U256,
        pub weight: ::ethers::core::types::U256,
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
    #[ethevent(name = "Attach", abi = "Attach(address,address,uint256)")]
    pub struct AttachFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Blacklisted", abi = "Blacklisted(address,address)")]
    pub struct BlacklistedFilter {
        #[ethevent(indexed)]
        pub blacklister: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
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
    #[ethevent(name = "BribeFactorySet", abi = "BribeFactorySet(address,address)")]
    pub struct BribeFactorySetFilter {
        #[ethevent(indexed)]
        pub setter: ::ethers::core::types::Address,
        pub new_bribe_fatory: ::ethers::core::types::Address,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub lp: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
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
    #[ethevent(name = "Detach", abi = "Detach(address,address,uint256)")]
    pub struct DetachFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
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
        name = "DistributeReward",
        abi = "DistributeReward(address,address,uint256)"
    )]
    pub struct DistributeRewardFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
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
        name = "ExternalBribeSet",
        abi = "ExternalBribeSet(address,address,address)"
    )]
    pub struct ExternalBribeSetFilter {
        #[ethevent(indexed)]
        pub setter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
        pub external_bribe: ::ethers::core::types::Address,
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
    #[ethevent(name = "FactoryAdded", abi = "FactoryAdded(address,address,address)")]
    pub struct FactoryAddedFilter {
        #[ethevent(indexed)]
        pub setter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pair_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge_factory: ::ethers::core::types::Address,
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
    #[ethevent(name = "FactoryRemoved", abi = "FactoryRemoved(address,uint256)")]
    pub struct FactoryRemovedFilter {
        #[ethevent(indexed)]
        pub setter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pos: ::ethers::core::types::U256,
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
        name = "FactoryReplaced",
        abi = "FactoryReplaced(address,address,address,uint256)"
    )]
    pub struct FactoryReplacedFilter {
        #[ethevent(indexed)]
        pub setter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pair_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge_factory: ::ethers::core::types::Address,
        pub pos: ::ethers::core::types::U256,
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
        name = "GaugeCreated",
        abi = "GaugeCreated(address,address,address,address)"
    )]
    pub struct GaugeCreatedFilter {
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
        pub creator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub external_bribe: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
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
    #[ethevent(name = "GaugeKilledTotally", abi = "GaugeKilledTotally(address)")]
    pub struct GaugeKilledTotallyFilter {
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
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
    #[ethevent(name = "GaugePaused", abi = "GaugePaused(address)")]
    pub struct GaugePausedFilter {
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
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
    #[ethevent(name = "GaugeRestarted", abi = "GaugeRestarted(address)")]
    pub struct GaugeRestartedFilter {
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
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
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "Voted", abi = "Voted(address,uint256,uint256)")]
    pub struct VotedFilter {
        #[ethevent(indexed)]
        pub voter: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub weight: ::ethers::core::types::U256,
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
    #[ethevent(name = "Whitelisted", abi = "Whitelisted(address,address)")]
    pub struct WhitelistedFilter {
        #[ethevent(indexed)]
        pub whitelister: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,address,uint256,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub lp: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VoterEvents {
        AbstainedFilter(AbstainedFilter),
        AttachFilter(AttachFilter),
        BlacklistedFilter(BlacklistedFilter),
        BribeFactorySetFilter(BribeFactorySetFilter),
        DepositFilter(DepositFilter),
        DetachFilter(DetachFilter),
        DistributeRewardFilter(DistributeRewardFilter),
        ExternalBribeSetFilter(ExternalBribeSetFilter),
        FactoryAddedFilter(FactoryAddedFilter),
        FactoryRemovedFilter(FactoryRemovedFilter),
        FactoryReplacedFilter(FactoryReplacedFilter),
        GaugeCreatedFilter(GaugeCreatedFilter),
        GaugeKilledTotallyFilter(GaugeKilledTotallyFilter),
        GaugePausedFilter(GaugePausedFilter),
        GaugeRestartedFilter(GaugeRestartedFilter),
        NotifyRewardFilter(NotifyRewardFilter),
        VotedFilter(VotedFilter),
        WhitelistedFilter(WhitelistedFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for VoterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AbstainedFilter::decode_log(log) {
                return Ok(VoterEvents::AbstainedFilter(decoded));
            }
            if let Ok(decoded) = AttachFilter::decode_log(log) {
                return Ok(VoterEvents::AttachFilter(decoded));
            }
            if let Ok(decoded) = BlacklistedFilter::decode_log(log) {
                return Ok(VoterEvents::BlacklistedFilter(decoded));
            }
            if let Ok(decoded) = BribeFactorySetFilter::decode_log(log) {
                return Ok(VoterEvents::BribeFactorySetFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(VoterEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = DetachFilter::decode_log(log) {
                return Ok(VoterEvents::DetachFilter(decoded));
            }
            if let Ok(decoded) = DistributeRewardFilter::decode_log(log) {
                return Ok(VoterEvents::DistributeRewardFilter(decoded));
            }
            if let Ok(decoded) = ExternalBribeSetFilter::decode_log(log) {
                return Ok(VoterEvents::ExternalBribeSetFilter(decoded));
            }
            if let Ok(decoded) = FactoryAddedFilter::decode_log(log) {
                return Ok(VoterEvents::FactoryAddedFilter(decoded));
            }
            if let Ok(decoded) = FactoryRemovedFilter::decode_log(log) {
                return Ok(VoterEvents::FactoryRemovedFilter(decoded));
            }
            if let Ok(decoded) = FactoryReplacedFilter::decode_log(log) {
                return Ok(VoterEvents::FactoryReplacedFilter(decoded));
            }
            if let Ok(decoded) = GaugeCreatedFilter::decode_log(log) {
                return Ok(VoterEvents::GaugeCreatedFilter(decoded));
            }
            if let Ok(decoded) = GaugeKilledTotallyFilter::decode_log(log) {
                return Ok(VoterEvents::GaugeKilledTotallyFilter(decoded));
            }
            if let Ok(decoded) = GaugePausedFilter::decode_log(log) {
                return Ok(VoterEvents::GaugePausedFilter(decoded));
            }
            if let Ok(decoded) = GaugeRestartedFilter::decode_log(log) {
                return Ok(VoterEvents::GaugeRestartedFilter(decoded));
            }
            if let Ok(decoded) = NotifyRewardFilter::decode_log(log) {
                return Ok(VoterEvents::NotifyRewardFilter(decoded));
            }
            if let Ok(decoded) = VotedFilter::decode_log(log) {
                return Ok(VoterEvents::VotedFilter(decoded));
            }
            if let Ok(decoded) = WhitelistedFilter::decode_log(log) {
                return Ok(VoterEvents::WhitelistedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(VoterEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VoterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AbstainedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttachFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlacklistedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BribeFactorySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DetachFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DistributeRewardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExternalBribeSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FactoryAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FactoryRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FactoryReplacedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GaugeCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GaugeKilledTotallyFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GaugePausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GaugeRestartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotifyRewardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VotedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AbstainedFilter> for VoterEvents {
        fn from(value: AbstainedFilter) -> Self {
            Self::AbstainedFilter(value)
        }
    }
    impl ::core::convert::From<AttachFilter> for VoterEvents {
        fn from(value: AttachFilter) -> Self {
            Self::AttachFilter(value)
        }
    }
    impl ::core::convert::From<BlacklistedFilter> for VoterEvents {
        fn from(value: BlacklistedFilter) -> Self {
            Self::BlacklistedFilter(value)
        }
    }
    impl ::core::convert::From<BribeFactorySetFilter> for VoterEvents {
        fn from(value: BribeFactorySetFilter) -> Self {
            Self::BribeFactorySetFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for VoterEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<DetachFilter> for VoterEvents {
        fn from(value: DetachFilter) -> Self {
            Self::DetachFilter(value)
        }
    }
    impl ::core::convert::From<DistributeRewardFilter> for VoterEvents {
        fn from(value: DistributeRewardFilter) -> Self {
            Self::DistributeRewardFilter(value)
        }
    }
    impl ::core::convert::From<ExternalBribeSetFilter> for VoterEvents {
        fn from(value: ExternalBribeSetFilter) -> Self {
            Self::ExternalBribeSetFilter(value)
        }
    }
    impl ::core::convert::From<FactoryAddedFilter> for VoterEvents {
        fn from(value: FactoryAddedFilter) -> Self {
            Self::FactoryAddedFilter(value)
        }
    }
    impl ::core::convert::From<FactoryRemovedFilter> for VoterEvents {
        fn from(value: FactoryRemovedFilter) -> Self {
            Self::FactoryRemovedFilter(value)
        }
    }
    impl ::core::convert::From<FactoryReplacedFilter> for VoterEvents {
        fn from(value: FactoryReplacedFilter) -> Self {
            Self::FactoryReplacedFilter(value)
        }
    }
    impl ::core::convert::From<GaugeCreatedFilter> for VoterEvents {
        fn from(value: GaugeCreatedFilter) -> Self {
            Self::GaugeCreatedFilter(value)
        }
    }
    impl ::core::convert::From<GaugeKilledTotallyFilter> for VoterEvents {
        fn from(value: GaugeKilledTotallyFilter) -> Self {
            Self::GaugeKilledTotallyFilter(value)
        }
    }
    impl ::core::convert::From<GaugePausedFilter> for VoterEvents {
        fn from(value: GaugePausedFilter) -> Self {
            Self::GaugePausedFilter(value)
        }
    }
    impl ::core::convert::From<GaugeRestartedFilter> for VoterEvents {
        fn from(value: GaugeRestartedFilter) -> Self {
            Self::GaugeRestartedFilter(value)
        }
    }
    impl ::core::convert::From<NotifyRewardFilter> for VoterEvents {
        fn from(value: NotifyRewardFilter) -> Self {
            Self::NotifyRewardFilter(value)
        }
    }
    impl ::core::convert::From<VotedFilter> for VoterEvents {
        fn from(value: VotedFilter) -> Self {
            Self::VotedFilter(value)
        }
    }
    impl ::core::convert::From<WhitelistedFilter> for VoterEvents {
        fn from(value: WhitelistedFilter) -> Self {
            Self::WhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for VoterEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `_factories` function with signature `_factories()` and selector `0xe9f6adfa`
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
    #[ethcall(name = "_factories", abi = "_factories()")]
    pub struct _FactoriesCall;
    ///Container type for all input parameters for the `_gaugeFactories` function with signature `_gaugeFactories()` and selector `0x9fb5dc05`
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
    #[ethcall(name = "_gaugeFactories", abi = "_gaugeFactories()")]
    pub struct _GaugeFactoriesCall;
    ///Container type for all input parameters for the `_killedGauges` function with signature `_killedGauges()` and selector `0xcbadada4`
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
    #[ethcall(name = "_killedGauges", abi = "_killedGauges()")]
    pub struct _KilledGaugesCall;
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
    ///Container type for all input parameters for the `addFactory` function with signature `addFactory(address,address)` and selector `0x6566afad`
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
    #[ethcall(name = "addFactory", abi = "addFactory(address,address)")]
    pub struct AddFactoryCall {
        pub pair_factory: ::ethers::core::types::Address,
        pub gauge_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `attachTokenToGauge` function with signature `attachTokenToGauge(uint256,address)` and selector `0x698473e3`
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
    #[ethcall(name = "attachTokenToGauge", abi = "attachTokenToGauge(uint256,address)")]
    pub struct AttachTokenToGaugeCall {
        pub token_id: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `blacklist` function with signature `blacklist(address)` and selector `0xf9f92be4`
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
    #[ethcall(name = "blacklist", abi = "blacklist(address)")]
    pub struct BlacklistCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `bribefactory` function with signature `bribefactory()` and selector `0x38752a9d`
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
    #[ethcall(name = "bribefactory", abi = "bribefactory()")]
    pub struct BribefactoryCall;
    ///Container type for all input parameters for the `claimBribes` function with signature `claimBribes(address[],address[][],uint256)` and selector `0x7715ee75`
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
    #[ethcall(name = "claimBribes", abi = "claimBribes(address[],address[][],uint256)")]
    pub struct ClaimBribesCall {
        pub bribes: ::std::vec::Vec<::ethers::core::types::Address>,
        pub tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `claimRewards` function with signature `claimRewards(address[],address[][])` and selector `0x20b1cb6f`
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
    #[ethcall(name = "claimRewards", abi = "claimRewards(address[],address[][])")]
    pub struct ClaimRewardsCall {
        pub gauges: ::std::vec::Vec<::ethers::core::types::Address>,
        pub tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
    }
    ///Container type for all input parameters for the `claimable` function with signature `claimable(address)` and selector `0x402914f5`
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
    #[ethcall(name = "claimable", abi = "claimable(address)")]
    pub struct ClaimableCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `createGauge` function with signature `createGauge(address,uint256)` and selector `0xdcd9e47a`
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
    #[ethcall(name = "createGauge", abi = "createGauge(address,uint256)")]
    pub struct CreateGaugeCall {
        pub pool: ::ethers::core::types::Address,
        pub gauge_type: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `detachTokenFromGauge` function with signature `detachTokenFromGauge(uint256,address)` and selector `0x411b1f77`
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
        name = "detachTokenFromGauge",
        abi = "detachTokenFromGauge(uint256,address)"
    )]
    pub struct DetachTokenFromGaugeCall {
        pub token_id: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `distribute` function with signature `distribute(address[])` and selector `0x6138889b`
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
    #[ethcall(name = "distribute", abi = "distribute(address[])")]
    pub struct Distribute1Call {
        pub gauges: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `distribute` function with signature `distribute(address)` and selector `0x63453ae1`
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
    #[ethcall(name = "distribute", abi = "distribute(address)")]
    pub struct Distribute2Call {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `distribute` function with signature `distribute(uint256,uint256)` and selector `0x7625391a`
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
    #[ethcall(name = "distribute", abi = "distribute(uint256,uint256)")]
    pub struct Distribute3Call {
        pub start: ::ethers::core::types::U256,
        pub finish: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `distribute` function with signature `distribute()` and selector `0xe4fc6b6d`
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
    #[ethcall(name = "distribute", abi = "distribute()")]
    pub struct Distribute0Call;
    ///Container type for all input parameters for the `distro` function with signature `distro()` and selector `0x47b3c6ba`
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
    #[ethcall(name = "distro", abi = "distro()")]
    pub struct DistroCall;
    ///Container type for all input parameters for the `emergencyCouncil` function with signature `emergencyCouncil()` and selector `0x7778960e`
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
    #[ethcall(name = "emergencyCouncil", abi = "emergencyCouncil()")]
    pub struct EmergencyCouncilCall;
    ///Container type for all input parameters for the `emitDeposit` function with signature `emitDeposit(uint256,address,uint256)` and selector `0xa61c713a`
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
    #[ethcall(name = "emitDeposit", abi = "emitDeposit(uint256,address,uint256)")]
    pub struct EmitDepositCall {
        pub token_id: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitWithdraw` function with signature `emitWithdraw(uint256,address,uint256)` and selector `0xea94ee44`
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
    #[ethcall(name = "emitWithdraw", abi = "emitWithdraw(uint256,address,uint256)")]
    pub struct EmitWithdrawCall {
        pub token_id: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `external_bribes` function with signature `external_bribes(address)` and selector `0xae21c4cb`
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
    #[ethcall(name = "external_bribes", abi = "external_bribes(address)")]
    pub struct ExternalBribesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `factories` function with signature `factories(uint256)` and selector `0x672383c4`
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
    #[ethcall(name = "factories", abi = "factories(uint256)")]
    pub struct FactoriesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `factoryLength` function with signature `factoryLength()` and selector `0x470f4985`
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
    #[ethcall(name = "factoryLength", abi = "factoryLength()")]
    pub struct FactoryLengthCall;
    ///Container type for all input parameters for the `gaugeFactories` function with signature `gaugeFactories(uint256)` and selector `0x23e1af42`
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
    #[ethcall(name = "gaugeFactories", abi = "gaugeFactories(uint256)")]
    pub struct GaugeFactoriesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `gaugeFactoriesLength` function with signature `gaugeFactoriesLength()` and selector `0xb52a3151`
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
    #[ethcall(name = "gaugeFactoriesLength", abi = "gaugeFactoriesLength()")]
    pub struct GaugeFactoriesLengthCall;
    ///Container type for all input parameters for the `gauges` function with signature `gauges(address)` and selector `0xb9a09fd5`
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
    #[ethcall(name = "gauges", abi = "gauges(address)")]
    pub struct GaugesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `governor` function with signature `governor()` and selector `0x0c340a24`
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
    #[ethcall(name = "governor", abi = "governor()")]
    pub struct GovernorCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address[],address)` and selector `0x462d0b2e`
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
    #[ethcall(name = "initialize", abi = "initialize(address[],address)")]
    pub struct InitializeCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub minter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isAlive` function with signature `isAlive(address)` and selector `0x1703e5f9`
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
    #[ethcall(name = "isAlive", abi = "isAlive(address)")]
    pub struct IsAliveCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isFactory` function with signature `isFactory(address)` and selector `0x0f04ba67`
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
    #[ethcall(name = "isFactory", abi = "isFactory(address)")]
    pub struct IsFactoryCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isGauge` function with signature `isGauge(address)` and selector `0xaa79979b`
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
    #[ethcall(name = "isGauge", abi = "isGauge(address)")]
    pub struct IsGaugeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isGaugeFactory` function with signature `isGaugeFactory(address)` and selector `0x657021fb`
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
    #[ethcall(name = "isGaugeFactory", abi = "isGaugeFactory(address)")]
    pub struct IsGaugeFactoryCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isWhitelisted` function with signature `isWhitelisted(address)` and selector `0x3af32abf`
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
    #[ethcall(name = "isWhitelisted", abi = "isWhitelisted(address)")]
    pub struct IsWhitelistedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `killGaugeTotally` function with signature `killGaugeTotally(address)` and selector `0x9edfd460`
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
    #[ethcall(name = "killGaugeTotally", abi = "killGaugeTotally(address)")]
    pub struct KillGaugeTotallyCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `killedGauges` function with signature `killedGauges(uint256)` and selector `0x173de600`
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
    #[ethcall(name = "killedGauges", abi = "killedGauges(uint256)")]
    pub struct KilledGaugesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `killedGaugesLength` function with signature `killedGaugesLength()` and selector `0xc448c78d`
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
    #[ethcall(name = "killedGaugesLength", abi = "killedGaugesLength()")]
    pub struct KilledGaugesLengthCall;
    ///Container type for all input parameters for the `lastVoted` function with signature `lastVoted(uint256)` and selector `0xf3594be0`
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
    #[ethcall(name = "lastVoted", abi = "lastVoted(uint256)")]
    pub struct LastVotedCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `length` function with signature `length()` and selector `0x1f7b6d32`
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
    #[ethcall(name = "length", abi = "length()")]
    pub struct LengthCall;
    ///Container type for all input parameters for the `minter` function with signature `minter()` and selector `0x07546172`
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
    #[ethcall(name = "minter", abi = "minter()")]
    pub struct MinterCall;
    ///Container type for all input parameters for the `notifyRewardAmount` function with signature `notifyRewardAmount(uint256)` and selector `0x3c6b16ab`
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
    #[ethcall(name = "notifyRewardAmount", abi = "notifyRewardAmount(uint256)")]
    pub struct NotifyRewardAmountCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pauseGauge` function with signature `pauseGauge(address)` and selector `0x03c39b00`
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
    #[ethcall(name = "pauseGauge", abi = "pauseGauge(address)")]
    pub struct PauseGaugeCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `poke` function with signature `poke(uint256)` and selector `0x32145f90`
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
    #[ethcall(name = "poke", abi = "poke(uint256)")]
    pub struct PokeCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `poolForGauge` function with signature `poolForGauge(address)` and selector `0x06d6a1b2`
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
    #[ethcall(name = "poolForGauge", abi = "poolForGauge(address)")]
    pub struct PoolForGaugeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `poolVote` function with signature `poolVote(uint256,uint256)` and selector `0xa86a366d`
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
    #[ethcall(name = "poolVote", abi = "poolVote(uint256,uint256)")]
    pub struct PoolVoteCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
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
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `removeFactory` function with signature `removeFactory(uint256)` and selector `0x577387b5`
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
    #[ethcall(name = "removeFactory", abi = "removeFactory(uint256)")]
    pub struct RemoveFactoryCall {
        pub pos: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `replaceFactory` function with signature `replaceFactory(address,address,uint256)` and selector `0x27e5c823`
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
    #[ethcall(name = "replaceFactory", abi = "replaceFactory(address,address,uint256)")]
    pub struct ReplaceFactoryCall {
        pub pair_factory: ::ethers::core::types::Address,
        pub gauge_factory: ::ethers::core::types::Address,
        pub pos: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `reset` function with signature `reset(uint256)` and selector `0x310bd74b`
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
    #[ethcall(name = "reset", abi = "reset(uint256)")]
    pub struct ResetCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `restartGauge` function with signature `restartGauge(address)` and selector `0xa82029f9`
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
    #[ethcall(name = "restartGauge", abi = "restartGauge(address)")]
    pub struct RestartGaugeCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setBribeFactory` function with signature `setBribeFactory(address)` and selector `0xa9b5aa7e`
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
    #[ethcall(name = "setBribeFactory", abi = "setBribeFactory(address)")]
    pub struct SetBribeFactoryCall {
        pub bribe_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEmergencyCouncil` function with signature `setEmergencyCouncil(address)` and selector `0xe586875f`
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
    #[ethcall(name = "setEmergencyCouncil", abi = "setEmergencyCouncil(address)")]
    pub struct SetEmergencyCouncilCall {
        pub council: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setExternalBribeFor` function with signature `setExternalBribeFor(address,address)` and selector `0xdaa168bd`
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
        name = "setExternalBribeFor",
        abi = "setExternalBribeFor(address,address)"
    )]
    pub struct SetExternalBribeForCall {
        pub gauge: ::ethers::core::types::Address,
        pub external: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGovernor` function with signature `setGovernor(address)` and selector `0xc42cf535`
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
    #[ethcall(name = "setGovernor", abi = "setGovernor(address)")]
    pub struct SetGovernorCall {
        pub governor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `totalWeight` function with signature `totalWeight()` and selector `0x96c82e57`
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
    #[ethcall(name = "totalWeight", abi = "totalWeight()")]
    pub struct TotalWeightCall;
    ///Container type for all input parameters for the `updateAll` function with signature `updateAll()` and selector `0x53d78693`
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
    #[ethcall(name = "updateAll", abi = "updateAll()")]
    pub struct UpdateAllCall;
    ///Container type for all input parameters for the `updateFor` function with signature `updateFor(address[])` and selector `0xd560b0d7`
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
    #[ethcall(name = "updateFor", abi = "updateFor(address[])")]
    pub struct UpdateForCall {
        pub gauges: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `updateForRange` function with signature `updateForRange(uint256,uint256)` and selector `0x9b6a9d72`
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
    #[ethcall(name = "updateForRange", abi = "updateForRange(uint256,uint256)")]
    pub struct UpdateForRangeCall {
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateGauge` function with signature `updateGauge(address)` and selector `0x6ecbe38a`
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
    #[ethcall(name = "updateGauge", abi = "updateGauge(address)")]
    pub struct UpdateGaugeCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `usedWeights` function with signature `usedWeights(uint256)` and selector `0x79e93824`
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
    #[ethcall(name = "usedWeights", abi = "usedWeights(uint256)")]
    pub struct UsedWeightsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `vote` function with signature `vote(uint256,address[],uint256[])` and selector `0x7ac09bf7`
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
    #[ethcall(name = "vote", abi = "vote(uint256,address[],uint256[])")]
    pub struct VoteCall {
        pub token_id: ::ethers::core::types::U256,
        pub pool_vote: ::std::vec::Vec<::ethers::core::types::Address>,
        pub weights: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `votes` function with signature `votes(uint256,address)` and selector `0xd23254b4`
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
    #[ethcall(name = "votes", abi = "votes(uint256,address)")]
    pub struct VotesCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `weights` function with signature `weights(address)` and selector `0xa7cac846`
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
    #[ethcall(name = "weights", abi = "weights(address)")]
    pub struct WeightsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `whitelist` function with signature `whitelist(address)` and selector `0x9b19251a`
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
    #[ethcall(name = "whitelist", abi = "whitelist(address)")]
    pub struct WhitelistCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VoterCalls {
        _Factories(_FactoriesCall),
        _GaugeFactories(_GaugeFactoriesCall),
        _KilledGauges(_KilledGaugesCall),
        Ve(VeCall),
        AddFactory(AddFactoryCall),
        AttachTokenToGauge(AttachTokenToGaugeCall),
        Blacklist(BlacklistCall),
        Bribefactory(BribefactoryCall),
        ClaimBribes(ClaimBribesCall),
        ClaimRewards(ClaimRewardsCall),
        Claimable(ClaimableCall),
        CreateGauge(CreateGaugeCall),
        DetachTokenFromGauge(DetachTokenFromGaugeCall),
        Distribute1(Distribute1Call),
        Distribute2(Distribute2Call),
        Distribute3(Distribute3Call),
        Distribute0(Distribute0Call),
        Distro(DistroCall),
        EmergencyCouncil(EmergencyCouncilCall),
        EmitDeposit(EmitDepositCall),
        EmitWithdraw(EmitWithdrawCall),
        ExternalBribes(ExternalBribesCall),
        Factories(FactoriesCall),
        FactoryLength(FactoryLengthCall),
        GaugeFactories(GaugeFactoriesCall),
        GaugeFactoriesLength(GaugeFactoriesLengthCall),
        Gauges(GaugesCall),
        Governor(GovernorCall),
        Initialize(InitializeCall),
        IsAlive(IsAliveCall),
        IsFactory(IsFactoryCall),
        IsGauge(IsGaugeCall),
        IsGaugeFactory(IsGaugeFactoryCall),
        IsWhitelisted(IsWhitelistedCall),
        KillGaugeTotally(KillGaugeTotallyCall),
        KilledGauges(KilledGaugesCall),
        KilledGaugesLength(KilledGaugesLengthCall),
        LastVoted(LastVotedCall),
        Length(LengthCall),
        Minter(MinterCall),
        NotifyRewardAmount(NotifyRewardAmountCall),
        PauseGauge(PauseGaugeCall),
        Poke(PokeCall),
        PoolForGauge(PoolForGaugeCall),
        PoolVote(PoolVoteCall),
        Pools(PoolsCall),
        RemoveFactory(RemoveFactoryCall),
        ReplaceFactory(ReplaceFactoryCall),
        Reset(ResetCall),
        RestartGauge(RestartGaugeCall),
        SetBribeFactory(SetBribeFactoryCall),
        SetEmergencyCouncil(SetEmergencyCouncilCall),
        SetExternalBribeFor(SetExternalBribeForCall),
        SetGovernor(SetGovernorCall),
        TotalWeight(TotalWeightCall),
        UpdateAll(UpdateAllCall),
        UpdateFor(UpdateForCall),
        UpdateForRange(UpdateForRangeCall),
        UpdateGauge(UpdateGaugeCall),
        UsedWeights(UsedWeightsCall),
        Vote(VoteCall),
        Votes(VotesCall),
        Weights(WeightsCall),
        Whitelist(WhitelistCall),
    }
    impl ::ethers::core::abi::AbiDecode for VoterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <_FactoriesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::_Factories(decoded));
            }
            if let Ok(decoded)
                = <_GaugeFactoriesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::_GaugeFactories(decoded));
            }
            if let Ok(decoded)
                = <_KilledGaugesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::_KilledGauges(decoded));
            }
            if let Ok(decoded)
                = <VeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ve(decoded));
            }
            if let Ok(decoded)
                = <AddFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddFactory(decoded));
            }
            if let Ok(decoded)
                = <AttachTokenToGaugeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AttachTokenToGauge(decoded));
            }
            if let Ok(decoded)
                = <BlacklistCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Blacklist(decoded));
            }
            if let Ok(decoded)
                = <BribefactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Bribefactory(decoded));
            }
            if let Ok(decoded)
                = <ClaimBribesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimBribes(decoded));
            }
            if let Ok(decoded)
                = <ClaimRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimRewards(decoded));
            }
            if let Ok(decoded)
                = <ClaimableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Claimable(decoded));
            }
            if let Ok(decoded)
                = <CreateGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateGauge(decoded));
            }
            if let Ok(decoded)
                = <DetachTokenFromGaugeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DetachTokenFromGauge(decoded));
            }
            if let Ok(decoded)
                = <Distribute1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distribute1(decoded));
            }
            if let Ok(decoded)
                = <Distribute2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distribute2(decoded));
            }
            if let Ok(decoded)
                = <Distribute3Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distribute3(decoded));
            }
            if let Ok(decoded)
                = <Distribute0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distribute0(decoded));
            }
            if let Ok(decoded)
                = <DistroCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distro(decoded));
            }
            if let Ok(decoded)
                = <EmergencyCouncilCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EmergencyCouncil(decoded));
            }
            if let Ok(decoded)
                = <EmitDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EmitDeposit(decoded));
            }
            if let Ok(decoded)
                = <EmitWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EmitWithdraw(decoded));
            }
            if let Ok(decoded)
                = <ExternalBribesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExternalBribes(decoded));
            }
            if let Ok(decoded)
                = <FactoriesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factories(decoded));
            }
            if let Ok(decoded)
                = <FactoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FactoryLength(decoded));
            }
            if let Ok(decoded)
                = <GaugeFactoriesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GaugeFactories(decoded));
            }
            if let Ok(decoded)
                = <GaugeFactoriesLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GaugeFactoriesLength(decoded));
            }
            if let Ok(decoded)
                = <GaugesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Gauges(decoded));
            }
            if let Ok(decoded)
                = <GovernorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Governor(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <IsAliveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsAlive(decoded));
            }
            if let Ok(decoded)
                = <IsFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsFactory(decoded));
            }
            if let Ok(decoded)
                = <IsGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsGauge(decoded));
            }
            if let Ok(decoded)
                = <IsGaugeFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsGaugeFactory(decoded));
            }
            if let Ok(decoded)
                = <IsWhitelistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsWhitelisted(decoded));
            }
            if let Ok(decoded)
                = <KillGaugeTotallyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::KillGaugeTotally(decoded));
            }
            if let Ok(decoded)
                = <KilledGaugesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::KilledGauges(decoded));
            }
            if let Ok(decoded)
                = <KilledGaugesLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::KilledGaugesLength(decoded));
            }
            if let Ok(decoded)
                = <LastVotedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastVoted(decoded));
            }
            if let Ok(decoded)
                = <LengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Length(decoded));
            }
            if let Ok(decoded)
                = <MinterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Minter(decoded));
            }
            if let Ok(decoded)
                = <NotifyRewardAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotifyRewardAmount(decoded));
            }
            if let Ok(decoded)
                = <PauseGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PauseGauge(decoded));
            }
            if let Ok(decoded)
                = <PokeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Poke(decoded));
            }
            if let Ok(decoded)
                = <PoolForGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolForGauge(decoded));
            }
            if let Ok(decoded)
                = <PoolVoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolVote(decoded));
            }
            if let Ok(decoded)
                = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded)
                = <RemoveFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveFactory(decoded));
            }
            if let Ok(decoded)
                = <ReplaceFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReplaceFactory(decoded));
            }
            if let Ok(decoded)
                = <ResetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Reset(decoded));
            }
            if let Ok(decoded)
                = <RestartGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RestartGauge(decoded));
            }
            if let Ok(decoded)
                = <SetBribeFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBribeFactory(decoded));
            }
            if let Ok(decoded)
                = <SetEmergencyCouncilCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetEmergencyCouncil(decoded));
            }
            if let Ok(decoded)
                = <SetExternalBribeForCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetExternalBribeFor(decoded));
            }
            if let Ok(decoded)
                = <SetGovernorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetGovernor(decoded));
            }
            if let Ok(decoded)
                = <TotalWeightCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalWeight(decoded));
            }
            if let Ok(decoded)
                = <UpdateAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateAll(decoded));
            }
            if let Ok(decoded)
                = <UpdateForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateFor(decoded));
            }
            if let Ok(decoded)
                = <UpdateForRangeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateForRange(decoded));
            }
            if let Ok(decoded)
                = <UpdateGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateGauge(decoded));
            }
            if let Ok(decoded)
                = <UsedWeightsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UsedWeights(decoded));
            }
            if let Ok(decoded)
                = <VoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vote(decoded));
            }
            if let Ok(decoded)
                = <VotesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Votes(decoded));
            }
            if let Ok(decoded)
                = <WeightsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weights(decoded));
            }
            if let Ok(decoded)
                = <WhitelistCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Whitelist(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VoterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::_Factories(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::_GaugeFactories(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::_KilledGauges(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttachTokenToGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Blacklist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bribefactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimBribes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Claimable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DetachTokenFromGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Distribute1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Distribute2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Distribute3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Distribute0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Distro(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmergencyCouncil(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExternalBribes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factories(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FactoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GaugeFactories(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GaugeFactoriesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gauges(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Governor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAlive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsGauge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsGaugeFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsWhitelisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KillGaugeTotally(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KilledGauges(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KilledGaugesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastVoted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Length(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Minter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotifyRewardAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PauseGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Poke(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PoolForGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolVote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReplaceFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RestartGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBribeFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEmergencyCouncil(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetExternalBribeFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGovernor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateForRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsedWeights(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Votes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weights(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Whitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VoterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::_Factories(element) => ::core::fmt::Display::fmt(element, f),
                Self::_GaugeFactories(element) => ::core::fmt::Display::fmt(element, f),
                Self::_KilledGauges(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ve(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttachTokenToGauge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Blacklist(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bribefactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimBribes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::Claimable(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::DetachTokenFromGauge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Distribute1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distribute2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distribute3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distribute0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distro(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmergencyCouncil(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExternalBribes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factories(element) => ::core::fmt::Display::fmt(element, f),
                Self::FactoryLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GaugeFactories(element) => ::core::fmt::Display::fmt(element, f),
                Self::GaugeFactoriesLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Gauges(element) => ::core::fmt::Display::fmt(element, f),
                Self::Governor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAlive(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsGaugeFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsWhitelisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::KillGaugeTotally(element) => ::core::fmt::Display::fmt(element, f),
                Self::KilledGauges(element) => ::core::fmt::Display::fmt(element, f),
                Self::KilledGaugesLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastVoted(element) => ::core::fmt::Display::fmt(element, f),
                Self::Length(element) => ::core::fmt::Display::fmt(element, f),
                Self::Minter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotifyRewardAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PauseGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::Poke(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolForGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolVote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReplaceFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reset(element) => ::core::fmt::Display::fmt(element, f),
                Self::RestartGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBribeFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEmergencyCouncil(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetExternalBribeFor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetGovernor(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalWeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateForRange(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedWeights(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Votes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weights(element) => ::core::fmt::Display::fmt(element, f),
                Self::Whitelist(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<_FactoriesCall> for VoterCalls {
        fn from(value: _FactoriesCall) -> Self {
            Self::_Factories(value)
        }
    }
    impl ::core::convert::From<_GaugeFactoriesCall> for VoterCalls {
        fn from(value: _GaugeFactoriesCall) -> Self {
            Self::_GaugeFactories(value)
        }
    }
    impl ::core::convert::From<_KilledGaugesCall> for VoterCalls {
        fn from(value: _KilledGaugesCall) -> Self {
            Self::_KilledGauges(value)
        }
    }
    impl ::core::convert::From<VeCall> for VoterCalls {
        fn from(value: VeCall) -> Self {
            Self::Ve(value)
        }
    }
    impl ::core::convert::From<AddFactoryCall> for VoterCalls {
        fn from(value: AddFactoryCall) -> Self {
            Self::AddFactory(value)
        }
    }
    impl ::core::convert::From<AttachTokenToGaugeCall> for VoterCalls {
        fn from(value: AttachTokenToGaugeCall) -> Self {
            Self::AttachTokenToGauge(value)
        }
    }
    impl ::core::convert::From<BlacklistCall> for VoterCalls {
        fn from(value: BlacklistCall) -> Self {
            Self::Blacklist(value)
        }
    }
    impl ::core::convert::From<BribefactoryCall> for VoterCalls {
        fn from(value: BribefactoryCall) -> Self {
            Self::Bribefactory(value)
        }
    }
    impl ::core::convert::From<ClaimBribesCall> for VoterCalls {
        fn from(value: ClaimBribesCall) -> Self {
            Self::ClaimBribes(value)
        }
    }
    impl ::core::convert::From<ClaimRewardsCall> for VoterCalls {
        fn from(value: ClaimRewardsCall) -> Self {
            Self::ClaimRewards(value)
        }
    }
    impl ::core::convert::From<ClaimableCall> for VoterCalls {
        fn from(value: ClaimableCall) -> Self {
            Self::Claimable(value)
        }
    }
    impl ::core::convert::From<CreateGaugeCall> for VoterCalls {
        fn from(value: CreateGaugeCall) -> Self {
            Self::CreateGauge(value)
        }
    }
    impl ::core::convert::From<DetachTokenFromGaugeCall> for VoterCalls {
        fn from(value: DetachTokenFromGaugeCall) -> Self {
            Self::DetachTokenFromGauge(value)
        }
    }
    impl ::core::convert::From<Distribute1Call> for VoterCalls {
        fn from(value: Distribute1Call) -> Self {
            Self::Distribute1(value)
        }
    }
    impl ::core::convert::From<Distribute2Call> for VoterCalls {
        fn from(value: Distribute2Call) -> Self {
            Self::Distribute2(value)
        }
    }
    impl ::core::convert::From<Distribute3Call> for VoterCalls {
        fn from(value: Distribute3Call) -> Self {
            Self::Distribute3(value)
        }
    }
    impl ::core::convert::From<Distribute0Call> for VoterCalls {
        fn from(value: Distribute0Call) -> Self {
            Self::Distribute0(value)
        }
    }
    impl ::core::convert::From<DistroCall> for VoterCalls {
        fn from(value: DistroCall) -> Self {
            Self::Distro(value)
        }
    }
    impl ::core::convert::From<EmergencyCouncilCall> for VoterCalls {
        fn from(value: EmergencyCouncilCall) -> Self {
            Self::EmergencyCouncil(value)
        }
    }
    impl ::core::convert::From<EmitDepositCall> for VoterCalls {
        fn from(value: EmitDepositCall) -> Self {
            Self::EmitDeposit(value)
        }
    }
    impl ::core::convert::From<EmitWithdrawCall> for VoterCalls {
        fn from(value: EmitWithdrawCall) -> Self {
            Self::EmitWithdraw(value)
        }
    }
    impl ::core::convert::From<ExternalBribesCall> for VoterCalls {
        fn from(value: ExternalBribesCall) -> Self {
            Self::ExternalBribes(value)
        }
    }
    impl ::core::convert::From<FactoriesCall> for VoterCalls {
        fn from(value: FactoriesCall) -> Self {
            Self::Factories(value)
        }
    }
    impl ::core::convert::From<FactoryLengthCall> for VoterCalls {
        fn from(value: FactoryLengthCall) -> Self {
            Self::FactoryLength(value)
        }
    }
    impl ::core::convert::From<GaugeFactoriesCall> for VoterCalls {
        fn from(value: GaugeFactoriesCall) -> Self {
            Self::GaugeFactories(value)
        }
    }
    impl ::core::convert::From<GaugeFactoriesLengthCall> for VoterCalls {
        fn from(value: GaugeFactoriesLengthCall) -> Self {
            Self::GaugeFactoriesLength(value)
        }
    }
    impl ::core::convert::From<GaugesCall> for VoterCalls {
        fn from(value: GaugesCall) -> Self {
            Self::Gauges(value)
        }
    }
    impl ::core::convert::From<GovernorCall> for VoterCalls {
        fn from(value: GovernorCall) -> Self {
            Self::Governor(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for VoterCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsAliveCall> for VoterCalls {
        fn from(value: IsAliveCall) -> Self {
            Self::IsAlive(value)
        }
    }
    impl ::core::convert::From<IsFactoryCall> for VoterCalls {
        fn from(value: IsFactoryCall) -> Self {
            Self::IsFactory(value)
        }
    }
    impl ::core::convert::From<IsGaugeCall> for VoterCalls {
        fn from(value: IsGaugeCall) -> Self {
            Self::IsGauge(value)
        }
    }
    impl ::core::convert::From<IsGaugeFactoryCall> for VoterCalls {
        fn from(value: IsGaugeFactoryCall) -> Self {
            Self::IsGaugeFactory(value)
        }
    }
    impl ::core::convert::From<IsWhitelistedCall> for VoterCalls {
        fn from(value: IsWhitelistedCall) -> Self {
            Self::IsWhitelisted(value)
        }
    }
    impl ::core::convert::From<KillGaugeTotallyCall> for VoterCalls {
        fn from(value: KillGaugeTotallyCall) -> Self {
            Self::KillGaugeTotally(value)
        }
    }
    impl ::core::convert::From<KilledGaugesCall> for VoterCalls {
        fn from(value: KilledGaugesCall) -> Self {
            Self::KilledGauges(value)
        }
    }
    impl ::core::convert::From<KilledGaugesLengthCall> for VoterCalls {
        fn from(value: KilledGaugesLengthCall) -> Self {
            Self::KilledGaugesLength(value)
        }
    }
    impl ::core::convert::From<LastVotedCall> for VoterCalls {
        fn from(value: LastVotedCall) -> Self {
            Self::LastVoted(value)
        }
    }
    impl ::core::convert::From<LengthCall> for VoterCalls {
        fn from(value: LengthCall) -> Self {
            Self::Length(value)
        }
    }
    impl ::core::convert::From<MinterCall> for VoterCalls {
        fn from(value: MinterCall) -> Self {
            Self::Minter(value)
        }
    }
    impl ::core::convert::From<NotifyRewardAmountCall> for VoterCalls {
        fn from(value: NotifyRewardAmountCall) -> Self {
            Self::NotifyRewardAmount(value)
        }
    }
    impl ::core::convert::From<PauseGaugeCall> for VoterCalls {
        fn from(value: PauseGaugeCall) -> Self {
            Self::PauseGauge(value)
        }
    }
    impl ::core::convert::From<PokeCall> for VoterCalls {
        fn from(value: PokeCall) -> Self {
            Self::Poke(value)
        }
    }
    impl ::core::convert::From<PoolForGaugeCall> for VoterCalls {
        fn from(value: PoolForGaugeCall) -> Self {
            Self::PoolForGauge(value)
        }
    }
    impl ::core::convert::From<PoolVoteCall> for VoterCalls {
        fn from(value: PoolVoteCall) -> Self {
            Self::PoolVote(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for VoterCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<RemoveFactoryCall> for VoterCalls {
        fn from(value: RemoveFactoryCall) -> Self {
            Self::RemoveFactory(value)
        }
    }
    impl ::core::convert::From<ReplaceFactoryCall> for VoterCalls {
        fn from(value: ReplaceFactoryCall) -> Self {
            Self::ReplaceFactory(value)
        }
    }
    impl ::core::convert::From<ResetCall> for VoterCalls {
        fn from(value: ResetCall) -> Self {
            Self::Reset(value)
        }
    }
    impl ::core::convert::From<RestartGaugeCall> for VoterCalls {
        fn from(value: RestartGaugeCall) -> Self {
            Self::RestartGauge(value)
        }
    }
    impl ::core::convert::From<SetBribeFactoryCall> for VoterCalls {
        fn from(value: SetBribeFactoryCall) -> Self {
            Self::SetBribeFactory(value)
        }
    }
    impl ::core::convert::From<SetEmergencyCouncilCall> for VoterCalls {
        fn from(value: SetEmergencyCouncilCall) -> Self {
            Self::SetEmergencyCouncil(value)
        }
    }
    impl ::core::convert::From<SetExternalBribeForCall> for VoterCalls {
        fn from(value: SetExternalBribeForCall) -> Self {
            Self::SetExternalBribeFor(value)
        }
    }
    impl ::core::convert::From<SetGovernorCall> for VoterCalls {
        fn from(value: SetGovernorCall) -> Self {
            Self::SetGovernor(value)
        }
    }
    impl ::core::convert::From<TotalWeightCall> for VoterCalls {
        fn from(value: TotalWeightCall) -> Self {
            Self::TotalWeight(value)
        }
    }
    impl ::core::convert::From<UpdateAllCall> for VoterCalls {
        fn from(value: UpdateAllCall) -> Self {
            Self::UpdateAll(value)
        }
    }
    impl ::core::convert::From<UpdateForCall> for VoterCalls {
        fn from(value: UpdateForCall) -> Self {
            Self::UpdateFor(value)
        }
    }
    impl ::core::convert::From<UpdateForRangeCall> for VoterCalls {
        fn from(value: UpdateForRangeCall) -> Self {
            Self::UpdateForRange(value)
        }
    }
    impl ::core::convert::From<UpdateGaugeCall> for VoterCalls {
        fn from(value: UpdateGaugeCall) -> Self {
            Self::UpdateGauge(value)
        }
    }
    impl ::core::convert::From<UsedWeightsCall> for VoterCalls {
        fn from(value: UsedWeightsCall) -> Self {
            Self::UsedWeights(value)
        }
    }
    impl ::core::convert::From<VoteCall> for VoterCalls {
        fn from(value: VoteCall) -> Self {
            Self::Vote(value)
        }
    }
    impl ::core::convert::From<VotesCall> for VoterCalls {
        fn from(value: VotesCall) -> Self {
            Self::Votes(value)
        }
    }
    impl ::core::convert::From<WeightsCall> for VoterCalls {
        fn from(value: WeightsCall) -> Self {
            Self::Weights(value)
        }
    }
    impl ::core::convert::From<WhitelistCall> for VoterCalls {
        fn from(value: WhitelistCall) -> Self {
            Self::Whitelist(value)
        }
    }
    ///Container type for all return fields from the `_factories` function with signature `_factories()` and selector `0xe9f6adfa`
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
    pub struct _FactoriesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `_gaugeFactories` function with signature `_gaugeFactories()` and selector `0x9fb5dc05`
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
    pub struct _GaugeFactoriesReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `_killedGauges` function with signature `_killedGauges()` and selector `0xcbadada4`
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
    pub struct _KilledGaugesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
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
    ///Container type for all return fields from the `bribefactory` function with signature `bribefactory()` and selector `0x38752a9d`
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
    pub struct BribefactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `claimable` function with signature `claimable(address)` and selector `0x402914f5`
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
    ///Container type for all return fields from the `createGauge` function with signature `createGauge(address,uint256)` and selector `0xdcd9e47a`
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
    pub struct CreateGaugeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `emergencyCouncil` function with signature `emergencyCouncil()` and selector `0x7778960e`
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
    pub struct EmergencyCouncilReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `external_bribes` function with signature `external_bribes(address)` and selector `0xae21c4cb`
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
    pub struct ExternalBribesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `factories` function with signature `factories(uint256)` and selector `0x672383c4`
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
    pub struct FactoriesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `factoryLength` function with signature `factoryLength()` and selector `0x470f4985`
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
    pub struct FactoryLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gaugeFactories` function with signature `gaugeFactories(uint256)` and selector `0x23e1af42`
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
    pub struct GaugeFactoriesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `gaugeFactoriesLength` function with signature `gaugeFactoriesLength()` and selector `0xb52a3151`
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
    pub struct GaugeFactoriesLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gauges` function with signature `gauges(address)` and selector `0xb9a09fd5`
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
    pub struct GaugesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `governor` function with signature `governor()` and selector `0x0c340a24`
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
    pub struct GovernorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isAlive` function with signature `isAlive(address)` and selector `0x1703e5f9`
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
    pub struct IsAliveReturn(pub bool);
    ///Container type for all return fields from the `isFactory` function with signature `isFactory(address)` and selector `0x0f04ba67`
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
    pub struct IsFactoryReturn(pub bool);
    ///Container type for all return fields from the `isGauge` function with signature `isGauge(address)` and selector `0xaa79979b`
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
    pub struct IsGaugeReturn(pub bool);
    ///Container type for all return fields from the `isGaugeFactory` function with signature `isGaugeFactory(address)` and selector `0x657021fb`
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
    pub struct IsGaugeFactoryReturn(pub bool);
    ///Container type for all return fields from the `isWhitelisted` function with signature `isWhitelisted(address)` and selector `0x3af32abf`
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
    pub struct IsWhitelistedReturn(pub bool);
    ///Container type for all return fields from the `killedGauges` function with signature `killedGauges(uint256)` and selector `0x173de600`
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
    pub struct KilledGaugesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `killedGaugesLength` function with signature `killedGaugesLength()` and selector `0xc448c78d`
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
    pub struct KilledGaugesLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastVoted` function with signature `lastVoted(uint256)` and selector `0xf3594be0`
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
    pub struct LastVotedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `length` function with signature `length()` and selector `0x1f7b6d32`
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
    pub struct LengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minter` function with signature `minter()` and selector `0x07546172`
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
    pub struct MinterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `poolForGauge` function with signature `poolForGauge(address)` and selector `0x06d6a1b2`
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
    pub struct PoolForGaugeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `poolVote` function with signature `poolVote(uint256,uint256)` and selector `0xa86a366d`
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
    pub struct PoolVoteReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
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
    pub struct PoolsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalWeight` function with signature `totalWeight()` and selector `0x96c82e57`
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
    pub struct TotalWeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `usedWeights` function with signature `usedWeights(uint256)` and selector `0x79e93824`
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
    pub struct UsedWeightsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `votes` function with signature `votes(uint256,address)` and selector `0xd23254b4`
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
    pub struct VotesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `weights` function with signature `weights(address)` and selector `0xa7cac846`
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
    pub struct WeightsReturn(pub ::ethers::core::types::U256);
}
