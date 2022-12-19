pub use lp_pair::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod lp_pair {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "LpPair was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n    {\n        \"inputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Approval\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"sender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Burn\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"sender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Mint\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"sender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0In\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1In\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0Out\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1Out\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Swap\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint112\",\n                \"name\": \"reserve0\",\n                \"type\": \"uint112\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint112\",\n                \"name\": \"reserve1\",\n                \"type\": \"uint112\"\n            }\n        ],\n        \"name\": \"Sync\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Transfer\",\n        \"type\": \"event\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"DOMAIN_SEPARATOR\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MINIMUM_LIQUIDITY\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"PERMIT_TYPEHASH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"allowance\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"approve\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"burn\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"decimals\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"\",\n                \"type\": \"uint8\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"factory\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"getReserves\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint112\",\n                \"name\": \"_reserve0\",\n                \"type\": \"uint112\"\n            },\n            {\n                \"internalType\": \"uint112\",\n                \"name\": \"_reserve1\",\n                \"type\": \"uint112\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"_blockTimestampLast\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_token0\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_token1\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"initialize\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"kLast\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"mint\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"name\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"\",\n                \"type\": \"string\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"nonces\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"permit\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"price0CumulativeLast\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"price1CumulativeLast\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"skim\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0Out\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1Out\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"swap\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"symbol\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"\",\n                \"type\": \"string\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"sync\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"token0\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"token1\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"totalSupply\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transfer\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transferFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    }\n]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static LPPAIR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct LpPair<M>(ethers::contract::Contract<M>);
    impl<M> Clone for LpPair<M> {
        fn clone(&self) -> Self {
            LpPair(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for LpPair<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for LpPair<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LpPair))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> LpPair<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LPPAIR_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MINIMUM_LIQUIDITY` (0xba9a7a56) function"]
        pub fn minimum_liquidity(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([186, 154, 122, 86], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x89afcb44) function"]
        pub fn burn(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([137, 175, 203, 68], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserves` (0x0902f1ac) function"]
        pub fn get_reserves(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128, u32)> {
            self.0
                .method_hash([9, 2, 241, 172], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x485cc955) function"]
        pub fn initialize(
            &self,
            token_0: ethers::core::types::Address,
            token_1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (token_0, token_1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `kLast` (0x7464fc3d) function"]
        pub fn k_last(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([116, 100, 252, 61], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x6a627842) function"]
        pub fn mint(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([106, 98, 120, 66], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `price0CumulativeLast` (0x5909c0d5) function"]
        pub fn price_0_cumulative_last(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([89, 9, 192, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `price1CumulativeLast` (0x5a3d5493) function"]
        pub fn price_1_cumulative_last(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([90, 61, 84, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `skim` (0xbc25cf77) function"]
        pub fn skim(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 37, 207, 119], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x022c0d9f) function"]
        pub fn swap(
            &self,
            amount_0_out: ethers::core::types::U256,
            amount_1_out: ethers::core::types::U256,
            to: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 44, 13, 159], (amount_0_out, amount_1_out, to, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sync` (0xfff6cae9) function"]
        pub fn sync(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 246, 202, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token0` (0x0dfe1681) function"]
        pub fn token_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token1` (0xd21220a7) function"]
        pub fn token_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Sync` event"]
        pub fn sync_filter(&self) -> ethers::contract::builders::Event<M, SyncFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, LpPairEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LpPair<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Burn", abi = "Burn(address,uint256,uint256,address)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Mint", abi = "Mint(address,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,uint256,uint256,uint256,uint256,address)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub amount_0_in: ethers::core::types::U256,
        pub amount_1_in: ethers::core::types::U256,
        pub amount_0_out: ethers::core::types::U256,
        pub amount_1_out: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Sync", abi = "Sync(uint112,uint112)")]
    pub struct SyncFilter {
        pub reserve_0: u128,
        pub reserve_1: u128,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LpPairEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        MintFilter(MintFilter),
        SwapFilter(SwapFilter),
        SyncFilter(SyncFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for LpPairEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(LpPairEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(LpPairEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(LpPairEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(LpPairEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = SyncFilter::decode_log(log) {
                return Ok(LpPairEvents::SyncFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(LpPairEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for LpPairEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LpPairEvents::ApprovalFilter(element) => element.fmt(f),
                LpPairEvents::BurnFilter(element) => element.fmt(f),
                LpPairEvents::MintFilter(element) => element.fmt(f),
                LpPairEvents::SwapFilter(element) => element.fmt(f),
                LpPairEvents::SyncFilter(element) => element.fmt(f),
                LpPairEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `MINIMUM_LIQUIDITY` function with signature `MINIMUM_LIQUIDITY()` and selector `[186, 154, 122, 86]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MINIMUM_LIQUIDITY", abi = "MINIMUM_LIQUIDITY()")]
    pub struct MinimumLiquidityCall;
    #[doc = "Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address)` and selector `[137, 175, 203, 68]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(address)")]
    pub struct BurnCall {
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `factory` function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `getReserves` function with signature `getReserves()` and selector `[9, 2, 241, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getReserves", abi = "getReserves()")]
    pub struct GetReservesCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `[72, 92, 201, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub token_0: ethers::core::types::Address,
        pub token_1: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `kLast` function with signature `kLast()` and selector `[116, 100, 252, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "kLast", abi = "kLast()")]
    pub struct KlastCall;
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address)` and selector `[106, 98, 120, 66]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mint", abi = "mint(address)")]
    pub struct MintCall {
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `price0CumulativeLast` function with signature `price0CumulativeLast()` and selector `[89, 9, 192, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "price0CumulativeLast", abi = "price0CumulativeLast()")]
    pub struct Price0CumulativeLastCall;
    #[doc = "Container type for all input parameters for the `price1CumulativeLast` function with signature `price1CumulativeLast()` and selector `[90, 61, 84, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "price1CumulativeLast", abi = "price1CumulativeLast()")]
    pub struct Price1CumulativeLastCall;
    #[doc = "Container type for all input parameters for the `skim` function with signature `skim(address)` and selector `[188, 37, 207, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "skim", abi = "skim(address)")]
    pub struct SkimCall {
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `swap` function with signature `swap(uint256,uint256,address,bytes)` and selector `[2, 44, 13, 159]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swap", abi = "swap(uint256,uint256,address,bytes)")]
    pub struct SwapCall {
        pub amount_0_out: ethers::core::types::U256,
        pub amount_1_out: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `sync` function with signature `sync()` and selector `[255, 246, 202, 233]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sync", abi = "sync()")]
    pub struct SyncCall;
    #[doc = "Container type for all input parameters for the `token0` function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    #[doc = "Container type for all input parameters for the `token1` function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LpPairCalls {
        DomainSeparator(DomainSeparatorCall),
        MinimumLiquidity(MinimumLiquidityCall),
        PermitTypehash(PermitTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        Factory(FactoryCall),
        GetReserves(GetReservesCall),
        Initialize(InitializeCall),
        Klast(KlastCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Price0CumulativeLast(Price0CumulativeLastCall),
        Price1CumulativeLast(Price1CumulativeLastCall),
        Skim(SkimCall),
        Swap(SwapCall),
        Symbol(SymbolCall),
        Sync(SyncCall),
        Token0(Token0Call),
        Token1(Token1Call),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for LpPairCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <MinimumLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::MinimumLiquidity(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::PermitTypehash(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LpPairCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <GetReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::GetReserves(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <KlastCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Klast(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LpPairCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LpPairCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <Price0CumulativeLastCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Price0CumulativeLast(decoded));
            }
            if let Ok(decoded) =
                <Price1CumulativeLastCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Price1CumulativeLast(decoded));
            }
            if let Ok(decoded) = <SkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LpPairCalls::Skim(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LpPairCalls::Swap(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Symbol(decoded));
            }
            if let Ok(decoded) = <SyncCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LpPairCalls::Sync(decoded));
            }
            if let Ok(decoded) = <Token0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Token1(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LpPairCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LpPairCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LpPairCalls::DomainSeparator(element) => element.encode(),
                LpPairCalls::MinimumLiquidity(element) => element.encode(),
                LpPairCalls::PermitTypehash(element) => element.encode(),
                LpPairCalls::Allowance(element) => element.encode(),
                LpPairCalls::Approve(element) => element.encode(),
                LpPairCalls::BalanceOf(element) => element.encode(),
                LpPairCalls::Burn(element) => element.encode(),
                LpPairCalls::Decimals(element) => element.encode(),
                LpPairCalls::Factory(element) => element.encode(),
                LpPairCalls::GetReserves(element) => element.encode(),
                LpPairCalls::Initialize(element) => element.encode(),
                LpPairCalls::Klast(element) => element.encode(),
                LpPairCalls::Mint(element) => element.encode(),
                LpPairCalls::Name(element) => element.encode(),
                LpPairCalls::Nonces(element) => element.encode(),
                LpPairCalls::Permit(element) => element.encode(),
                LpPairCalls::Price0CumulativeLast(element) => element.encode(),
                LpPairCalls::Price1CumulativeLast(element) => element.encode(),
                LpPairCalls::Skim(element) => element.encode(),
                LpPairCalls::Swap(element) => element.encode(),
                LpPairCalls::Symbol(element) => element.encode(),
                LpPairCalls::Sync(element) => element.encode(),
                LpPairCalls::Token0(element) => element.encode(),
                LpPairCalls::Token1(element) => element.encode(),
                LpPairCalls::TotalSupply(element) => element.encode(),
                LpPairCalls::Transfer(element) => element.encode(),
                LpPairCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LpPairCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LpPairCalls::DomainSeparator(element) => element.fmt(f),
                LpPairCalls::MinimumLiquidity(element) => element.fmt(f),
                LpPairCalls::PermitTypehash(element) => element.fmt(f),
                LpPairCalls::Allowance(element) => element.fmt(f),
                LpPairCalls::Approve(element) => element.fmt(f),
                LpPairCalls::BalanceOf(element) => element.fmt(f),
                LpPairCalls::Burn(element) => element.fmt(f),
                LpPairCalls::Decimals(element) => element.fmt(f),
                LpPairCalls::Factory(element) => element.fmt(f),
                LpPairCalls::GetReserves(element) => element.fmt(f),
                LpPairCalls::Initialize(element) => element.fmt(f),
                LpPairCalls::Klast(element) => element.fmt(f),
                LpPairCalls::Mint(element) => element.fmt(f),
                LpPairCalls::Name(element) => element.fmt(f),
                LpPairCalls::Nonces(element) => element.fmt(f),
                LpPairCalls::Permit(element) => element.fmt(f),
                LpPairCalls::Price0CumulativeLast(element) => element.fmt(f),
                LpPairCalls::Price1CumulativeLast(element) => element.fmt(f),
                LpPairCalls::Skim(element) => element.fmt(f),
                LpPairCalls::Swap(element) => element.fmt(f),
                LpPairCalls::Symbol(element) => element.fmt(f),
                LpPairCalls::Sync(element) => element.fmt(f),
                LpPairCalls::Token0(element) => element.fmt(f),
                LpPairCalls::Token1(element) => element.fmt(f),
                LpPairCalls::TotalSupply(element) => element.fmt(f),
                LpPairCalls::Transfer(element) => element.fmt(f),
                LpPairCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for LpPairCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            LpPairCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<MinimumLiquidityCall> for LpPairCalls {
        fn from(var: MinimumLiquidityCall) -> Self {
            LpPairCalls::MinimumLiquidity(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for LpPairCalls {
        fn from(var: PermitTypehashCall) -> Self {
            LpPairCalls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for LpPairCalls {
        fn from(var: AllowanceCall) -> Self {
            LpPairCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for LpPairCalls {
        fn from(var: ApproveCall) -> Self {
            LpPairCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for LpPairCalls {
        fn from(var: BalanceOfCall) -> Self {
            LpPairCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for LpPairCalls {
        fn from(var: BurnCall) -> Self {
            LpPairCalls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for LpPairCalls {
        fn from(var: DecimalsCall) -> Self {
            LpPairCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for LpPairCalls {
        fn from(var: FactoryCall) -> Self {
            LpPairCalls::Factory(var)
        }
    }
    impl ::std::convert::From<GetReservesCall> for LpPairCalls {
        fn from(var: GetReservesCall) -> Self {
            LpPairCalls::GetReserves(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for LpPairCalls {
        fn from(var: InitializeCall) -> Self {
            LpPairCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<KlastCall> for LpPairCalls {
        fn from(var: KlastCall) -> Self {
            LpPairCalls::Klast(var)
        }
    }
    impl ::std::convert::From<MintCall> for LpPairCalls {
        fn from(var: MintCall) -> Self {
            LpPairCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for LpPairCalls {
        fn from(var: NameCall) -> Self {
            LpPairCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for LpPairCalls {
        fn from(var: NoncesCall) -> Self {
            LpPairCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<PermitCall> for LpPairCalls {
        fn from(var: PermitCall) -> Self {
            LpPairCalls::Permit(var)
        }
    }
    impl ::std::convert::From<Price0CumulativeLastCall> for LpPairCalls {
        fn from(var: Price0CumulativeLastCall) -> Self {
            LpPairCalls::Price0CumulativeLast(var)
        }
    }
    impl ::std::convert::From<Price1CumulativeLastCall> for LpPairCalls {
        fn from(var: Price1CumulativeLastCall) -> Self {
            LpPairCalls::Price1CumulativeLast(var)
        }
    }
    impl ::std::convert::From<SkimCall> for LpPairCalls {
        fn from(var: SkimCall) -> Self {
            LpPairCalls::Skim(var)
        }
    }
    impl ::std::convert::From<SwapCall> for LpPairCalls {
        fn from(var: SwapCall) -> Self {
            LpPairCalls::Swap(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for LpPairCalls {
        fn from(var: SymbolCall) -> Self {
            LpPairCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<SyncCall> for LpPairCalls {
        fn from(var: SyncCall) -> Self {
            LpPairCalls::Sync(var)
        }
    }
    impl ::std::convert::From<Token0Call> for LpPairCalls {
        fn from(var: Token0Call) -> Self {
            LpPairCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for LpPairCalls {
        fn from(var: Token1Call) -> Self {
            LpPairCalls::Token1(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for LpPairCalls {
        fn from(var: TotalSupplyCall) -> Self {
            LpPairCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for LpPairCalls {
        fn from(var: TransferCall) -> Self {
            LpPairCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for LpPairCalls {
        fn from(var: TransferFromCall) -> Self {
            LpPairCalls::TransferFrom(var)
        }
    }
    #[doc = "Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `MINIMUM_LIQUIDITY` function with signature `MINIMUM_LIQUIDITY()` and selector `[186, 154, 122, 86]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinimumLiquidityReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PermitTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `burn` function with signature `burn(address)` and selector `[137, 175, 203, 68]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BurnReturn {
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `factory` function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FactoryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getReserves` function with signature `getReserves()` and selector `[9, 2, 241, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReservesReturn {
        pub reserve_0: u128,
        pub reserve_1: u128,
        pub block_timestamp_last: u32,
    }
    #[doc = "Container type for all return fields from the `kLast` function with signature `kLast()` and selector `[116, 100, 252, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct KlastReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(address)` and selector `[106, 98, 120, 66]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintReturn {
        pub liquidity: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NoncesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `price0CumulativeLast` function with signature `price0CumulativeLast()` and selector `[89, 9, 192, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Price0CumulativeLastReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `price1CumulativeLast` function with signature `price1CumulativeLast()` and selector `[90, 61, 84, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Price1CumulativeLastReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `token0` function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Token0Return(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `token1` function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Token1Return(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferFromReturn(pub bool);
}
