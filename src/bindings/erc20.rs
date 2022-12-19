pub use erc20::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod erc20 {
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
    #[doc = "ERC20 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"_name\",\n                \"type\": \"string\"\n            },\n            {\n                \"internalType\": \"string\",\n                \"name\": \"_symbol\",\n                \"type\": \"string\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"_decimals\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Approval\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"oldOwner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"newOwner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"uint256\",\n                \"name\": \"effectiveTime\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"LogChangeDCRMOwner\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"bytes32\",\n                \"name\": \"txhash\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"LogSwapin\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"bindaddr\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"LogSwapout\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Transfer\",\n        \"type\": \"event\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"DOMAIN_SEPARATOR\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"PERMIT_TYPEHASH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"txhash\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Swapin\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"bindaddr\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Swapout\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"TRANSFER_TYPEHASH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"allowance\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"approve\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"approveAndCall\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"newOwner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"changeDCRMOwner\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"decimals\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"\",\n                \"type\": \"uint8\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"name\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"\",\n                \"type\": \"string\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"nonces\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"owner\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"target\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"permit\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"symbol\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"\",\n                \"type\": \"string\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"totalSupply\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transfer\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"transferAndCall\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transferFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"target\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"transferWithPermit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    }\n]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ERC20_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ERC20<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC20<M> {
        fn clone(&self) -> Self {
            ERC20(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC20<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ERC20<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC20<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC20_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Swapin` (0xec126c77) function"]
        pub fn swapin(
            &self,
            txhash: [u8; 32],
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([236, 18, 108, 119], (txhash, account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Swapout` (0x628d6cba) function"]
        pub fn swapout(
            &self,
            amount: ethers::core::types::U256,
            bindaddr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([98, 141, 108, 186], (amount, bindaddr))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TRANSFER_TYPEHASH` (0x00bf26f4) function"]
        pub fn transfer_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([0, 191, 38, 244], ())
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
        #[doc = "Calls the contract's `approveAndCall` (0xcae9ca51) function"]
        pub fn approve_and_call(
            &self,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([202, 233, 202, 81], (spender, value, data))
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
        #[doc = "Calls the contract's `changeDCRMOwner` (0xb524f3a5) function"]
        pub fn change_dcrm_owner(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([181, 36, 243, 165], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
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
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            target: ethers::core::types::Address,
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
                    (target, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
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
        #[doc = "Calls the contract's `transferAndCall` (0x4000aea0) function"]
        pub fn transfer_and_call(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([64, 0, 174, 160], (to, value, data))
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
        #[doc = "Calls the contract's `transferWithPermit` (0x605629d6) function"]
        pub fn transfer_with_permit(
            &self,
            target: ethers::core::types::Address,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 86, 41, 214], (target, to, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogChangeDCRMOwner` event"]
        pub fn log_change_dcrm_owner_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogChangeDCRMOwnerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogSwapin` event"]
        pub fn log_swapin_filter(&self) -> ethers::contract::builders::Event<M, LogSwapinFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogSwapout` event"]
        pub fn log_swapout_filter(&self) -> ethers::contract::builders::Event<M, LogSwapoutFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ERC20Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ERC20<M> {
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
    #[ethevent(
        name = "LogChangeDCRMOwner",
        abi = "LogChangeDCRMOwner(address,address,uint256)"
    )]
    pub struct LogChangeDCRMOwnerFilter {
        #[ethevent(indexed)]
        pub old_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub effective_time: ethers::core::types::U256,
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
    #[ethevent(name = "LogSwapin", abi = "LogSwapin(bytes32,address,uint256)")]
    pub struct LogSwapinFilter {
        #[ethevent(indexed)]
        pub txhash: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "LogSwapout", abi = "LogSwapout(address,address,uint256)")]
    pub struct LogSwapoutFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub bindaddr: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    pub enum ERC20Events {
        ApprovalFilter(ApprovalFilter),
        LogChangeDCRMOwnerFilter(LogChangeDCRMOwnerFilter),
        LogSwapinFilter(LogSwapinFilter),
        LogSwapoutFilter(LogSwapoutFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ERC20Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC20Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = LogChangeDCRMOwnerFilter::decode_log(log) {
                return Ok(ERC20Events::LogChangeDCRMOwnerFilter(decoded));
            }
            if let Ok(decoded) = LogSwapinFilter::decode_log(log) {
                return Ok(ERC20Events::LogSwapinFilter(decoded));
            }
            if let Ok(decoded) = LogSwapoutFilter::decode_log(log) {
                return Ok(ERC20Events::LogSwapoutFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20Events::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ERC20Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC20Events::ApprovalFilter(element) => element.fmt(f),
                ERC20Events::LogChangeDCRMOwnerFilter(element) => element.fmt(f),
                ERC20Events::LogSwapinFilter(element) => element.fmt(f),
                ERC20Events::LogSwapoutFilter(element) => element.fmt(f),
                ERC20Events::TransferFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `Swapin` function with signature `Swapin(bytes32,address,uint256)` and selector `[236, 18, 108, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Swapin", abi = "Swapin(bytes32,address,uint256)")]
    pub struct SwapinCall {
        pub txhash: [u8; 32],
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `Swapout` function with signature `Swapout(uint256,address)` and selector `[98, 141, 108, 186]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Swapout", abi = "Swapout(uint256,address)")]
    pub struct SwapoutCall {
        pub amount: ethers::core::types::U256,
        pub bindaddr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `TRANSFER_TYPEHASH` function with signature `TRANSFER_TYPEHASH()` and selector `[0, 191, 38, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "TRANSFER_TYPEHASH", abi = "TRANSFER_TYPEHASH()")]
    pub struct TransferTypehashCall;
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
    #[doc = "Container type for all input parameters for the `approveAndCall` function with signature `approveAndCall(address,uint256,bytes)` and selector `[202, 233, 202, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approveAndCall", abi = "approveAndCall(address,uint256,bytes)")]
    pub struct ApproveAndCallCall {
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `changeDCRMOwner` function with signature `changeDCRMOwner(address)` and selector `[181, 36, 243, 165]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "changeDCRMOwner", abi = "changeDCRMOwner(address)")]
    pub struct ChangeDCRMOwnerCall {
        pub new_owner: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
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
        pub target: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
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
    #[doc = "Container type for all input parameters for the `transferAndCall` function with signature `transferAndCall(address,uint256,bytes)` and selector `[64, 0, 174, 160]`"]
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
        name = "transferAndCall",
        abi = "transferAndCall(address,uint256,bytes)"
    )]
    pub struct TransferAndCallCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `transferWithPermit` function with signature `transferWithPermit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[96, 86, 41, 214]`"]
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
        name = "transferWithPermit",
        abi = "transferWithPermit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct TransferWithPermitCall {
        pub target: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC20Calls {
        DomainSeparator(DomainSeparatorCall),
        PermitTypehash(PermitTypehashCall),
        Swapin(SwapinCall),
        Swapout(SwapoutCall),
        TransferTypehash(TransferTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        ApproveAndCall(ApproveAndCallCall),
        BalanceOf(BalanceOfCall),
        ChangeDCRMOwner(ChangeDCRMOwnerCall),
        Decimals(DecimalsCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferAndCall(TransferAndCallCall),
        TransferFrom(TransferFromCall),
        TransferWithPermit(TransferWithPermitCall),
    }
    impl ethers::core::abi::AbiDecode for ERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::PermitTypehash(decoded));
            }
            if let Ok(decoded) = <SwapinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Swapin(decoded));
            }
            if let Ok(decoded) =
                <SwapoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Swapout(decoded));
            }
            if let Ok(decoded) =
                <TransferTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::TransferTypehash(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Approve(decoded));
            }
            if let Ok(decoded) =
                <ApproveAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::ApproveAndCall(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ChangeDCRMOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::ChangeDCRMOwner(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Decimals(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ERC20Calls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Permit(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::TransferAndCall(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferWithPermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20Calls::TransferWithPermit(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC20Calls::DomainSeparator(element) => element.encode(),
                ERC20Calls::PermitTypehash(element) => element.encode(),
                ERC20Calls::Swapin(element) => element.encode(),
                ERC20Calls::Swapout(element) => element.encode(),
                ERC20Calls::TransferTypehash(element) => element.encode(),
                ERC20Calls::Allowance(element) => element.encode(),
                ERC20Calls::Approve(element) => element.encode(),
                ERC20Calls::ApproveAndCall(element) => element.encode(),
                ERC20Calls::BalanceOf(element) => element.encode(),
                ERC20Calls::ChangeDCRMOwner(element) => element.encode(),
                ERC20Calls::Decimals(element) => element.encode(),
                ERC20Calls::Name(element) => element.encode(),
                ERC20Calls::Nonces(element) => element.encode(),
                ERC20Calls::Owner(element) => element.encode(),
                ERC20Calls::Permit(element) => element.encode(),
                ERC20Calls::Symbol(element) => element.encode(),
                ERC20Calls::TotalSupply(element) => element.encode(),
                ERC20Calls::Transfer(element) => element.encode(),
                ERC20Calls::TransferAndCall(element) => element.encode(),
                ERC20Calls::TransferFrom(element) => element.encode(),
                ERC20Calls::TransferWithPermit(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC20Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC20Calls::DomainSeparator(element) => element.fmt(f),
                ERC20Calls::PermitTypehash(element) => element.fmt(f),
                ERC20Calls::Swapin(element) => element.fmt(f),
                ERC20Calls::Swapout(element) => element.fmt(f),
                ERC20Calls::TransferTypehash(element) => element.fmt(f),
                ERC20Calls::Allowance(element) => element.fmt(f),
                ERC20Calls::Approve(element) => element.fmt(f),
                ERC20Calls::ApproveAndCall(element) => element.fmt(f),
                ERC20Calls::BalanceOf(element) => element.fmt(f),
                ERC20Calls::ChangeDCRMOwner(element) => element.fmt(f),
                ERC20Calls::Decimals(element) => element.fmt(f),
                ERC20Calls::Name(element) => element.fmt(f),
                ERC20Calls::Nonces(element) => element.fmt(f),
                ERC20Calls::Owner(element) => element.fmt(f),
                ERC20Calls::Permit(element) => element.fmt(f),
                ERC20Calls::Symbol(element) => element.fmt(f),
                ERC20Calls::TotalSupply(element) => element.fmt(f),
                ERC20Calls::Transfer(element) => element.fmt(f),
                ERC20Calls::TransferAndCall(element) => element.fmt(f),
                ERC20Calls::TransferFrom(element) => element.fmt(f),
                ERC20Calls::TransferWithPermit(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for ERC20Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            ERC20Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for ERC20Calls {
        fn from(var: PermitTypehashCall) -> Self {
            ERC20Calls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<SwapinCall> for ERC20Calls {
        fn from(var: SwapinCall) -> Self {
            ERC20Calls::Swapin(var)
        }
    }
    impl ::std::convert::From<SwapoutCall> for ERC20Calls {
        fn from(var: SwapoutCall) -> Self {
            ERC20Calls::Swapout(var)
        }
    }
    impl ::std::convert::From<TransferTypehashCall> for ERC20Calls {
        fn from(var: TransferTypehashCall) -> Self {
            ERC20Calls::TransferTypehash(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for ERC20Calls {
        fn from(var: AllowanceCall) -> Self {
            ERC20Calls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for ERC20Calls {
        fn from(var: ApproveCall) -> Self {
            ERC20Calls::Approve(var)
        }
    }
    impl ::std::convert::From<ApproveAndCallCall> for ERC20Calls {
        fn from(var: ApproveAndCallCall) -> Self {
            ERC20Calls::ApproveAndCall(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ERC20Calls {
        fn from(var: BalanceOfCall) -> Self {
            ERC20Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<ChangeDCRMOwnerCall> for ERC20Calls {
        fn from(var: ChangeDCRMOwnerCall) -> Self {
            ERC20Calls::ChangeDCRMOwner(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for ERC20Calls {
        fn from(var: DecimalsCall) -> Self {
            ERC20Calls::Decimals(var)
        }
    }
    impl ::std::convert::From<NameCall> for ERC20Calls {
        fn from(var: NameCall) -> Self {
            ERC20Calls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for ERC20Calls {
        fn from(var: NoncesCall) -> Self {
            ERC20Calls::Nonces(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ERC20Calls {
        fn from(var: OwnerCall) -> Self {
            ERC20Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PermitCall> for ERC20Calls {
        fn from(var: PermitCall) -> Self {
            ERC20Calls::Permit(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ERC20Calls {
        fn from(var: SymbolCall) -> Self {
            ERC20Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ERC20Calls {
        fn from(var: TotalSupplyCall) -> Self {
            ERC20Calls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for ERC20Calls {
        fn from(var: TransferCall) -> Self {
            ERC20Calls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferAndCallCall> for ERC20Calls {
        fn from(var: TransferAndCallCall) -> Self {
            ERC20Calls::TransferAndCall(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ERC20Calls {
        fn from(var: TransferFromCall) -> Self {
            ERC20Calls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferWithPermitCall> for ERC20Calls {
        fn from(var: TransferWithPermitCall) -> Self {
            ERC20Calls::TransferWithPermit(var)
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
    #[doc = "Container type for all return fields from the `Swapin` function with signature `Swapin(bytes32,address,uint256)` and selector `[236, 18, 108, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapinReturn(pub bool);
    #[doc = "Container type for all return fields from the `Swapout` function with signature `Swapout(uint256,address)` and selector `[98, 141, 108, 186]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapoutReturn(pub bool);
    #[doc = "Container type for all return fields from the `TRANSFER_TYPEHASH` function with signature `TRANSFER_TYPEHASH()` and selector `[0, 191, 38, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferTypehashReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `approveAndCall` function with signature `approveAndCall(address,uint256,bytes)` and selector `[202, 233, 202, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ApproveAndCallReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `changeDCRMOwner` function with signature `changeDCRMOwner(address)` and selector `[181, 36, 243, 165]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ChangeDCRMOwnerReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `transferAndCall` function with signature `transferAndCall(address,uint256,bytes)` and selector `[64, 0, 174, 160]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferAndCallReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `transferWithPermit` function with signature `transferWithPermit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[96, 86, 41, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferWithPermitReturn(pub bool);
}
