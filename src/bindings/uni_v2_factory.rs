pub use uni_v2_factory::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod uni_v2_factory {
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
    #[doc = "UniV2Factory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_feeToSetter\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"token0\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"token1\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"pair\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"PairCreated\",\n        \"type\": \"event\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"INIT_CODE_PAIR_HASH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"allPairs\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"allPairsLength\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"tokenA\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"tokenB\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"createPair\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pair\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"feeTo\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"feeToSetter\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"getPair\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"pairCodeHash\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_feeTo\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setFeeTo\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_feeToSetter\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setFeeToSetter\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    }\n]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static UNIV2FACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct UniV2Factory<M>(ethers::contract::Contract<M>);
    impl<M> Clone for UniV2Factory<M> {
        fn clone(&self) -> Self {
            UniV2Factory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for UniV2Factory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for UniV2Factory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniV2Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> UniV2Factory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), UNIV2FACTORY_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `INIT_CODE_PAIR_HASH` (0x5855a25a) function"]
        pub fn init_code_pair_hash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([88, 85, 162, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allPairs` (0x1e3dd18b) function"]
        pub fn all_pairs(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([30, 61, 209, 139], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allPairsLength` (0x574f2ba3) function"]
        pub fn all_pairs_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([87, 79, 43, 163], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createPair` (0xc9c65396) function"]
        pub fn create_pair(
            &self,
            token_a: ethers::core::types::Address,
            token_b: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([201, 198, 83, 150], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeTo` (0x017e7e58) function"]
        pub fn fee_to(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([1, 126, 126, 88], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeToSetter` (0x094b7415) function"]
        pub fn fee_to_setter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([9, 75, 116, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPair` (0xe6a43905) function"]
        pub fn get_pair(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([230, 164, 57, 5], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pairCodeHash` (0x9aab9248) function"]
        pub fn pair_code_hash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([154, 171, 146, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeTo` (0xf46901ed) function"]
        pub fn set_fee_to(
            &self,
            fee_to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 105, 1, 237], fee_to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeToSetter` (0xa2e74af6) function"]
        pub fn set_fee_to_setter(
            &self,
            fee_to_setter: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 231, 74, 246], fee_to_setter)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PairCreated` event"]
        pub fn pair_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PairCreatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PairCreatedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for UniV2Factory<M> {
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
    #[ethevent(
        name = "PairCreated",
        abi = "PairCreated(address,address,address,uint256)"
    )]
    pub struct PairCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ethers::core::types::Address,
        pub pair: ethers::core::types::Address,
        pub p3: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `INIT_CODE_PAIR_HASH` function with signature `INIT_CODE_PAIR_HASH()` and selector `[88, 85, 162, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "INIT_CODE_PAIR_HASH", abi = "INIT_CODE_PAIR_HASH()")]
    pub struct InitCodePairHashCall;
    #[doc = "Container type for all input parameters for the `allPairs` function with signature `allPairs(uint256)` and selector `[30, 61, 209, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "allPairs", abi = "allPairs(uint256)")]
    pub struct AllPairsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `allPairsLength` function with signature `allPairsLength()` and selector `[87, 79, 43, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "allPairsLength", abi = "allPairsLength()")]
    pub struct AllPairsLengthCall;
    #[doc = "Container type for all input parameters for the `createPair` function with signature `createPair(address,address)` and selector `[201, 198, 83, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub token_a: ethers::core::types::Address,
        pub token_b: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `feeTo` function with signature `feeTo()` and selector `[1, 126, 126, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeTo", abi = "feeTo()")]
    pub struct FeeToCall;
    #[doc = "Container type for all input parameters for the `feeToSetter` function with signature `feeToSetter()` and selector `[9, 75, 116, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeToSetter", abi = "feeToSetter()")]
    pub struct FeeToSetterCall;
    #[doc = "Container type for all input parameters for the `getPair` function with signature `getPair(address,address)` and selector `[230, 164, 57, 5]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPair", abi = "getPair(address,address)")]
    pub struct GetPairCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `pairCodeHash` function with signature `pairCodeHash()` and selector `[154, 171, 146, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pairCodeHash", abi = "pairCodeHash()")]
    pub struct PairCodeHashCall;
    #[doc = "Container type for all input parameters for the `setFeeTo` function with signature `setFeeTo(address)` and selector `[244, 105, 1, 237]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeTo", abi = "setFeeTo(address)")]
    pub struct SetFeeToCall {
        pub fee_to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setFeeToSetter` function with signature `setFeeToSetter(address)` and selector `[162, 231, 74, 246]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeToSetter", abi = "setFeeToSetter(address)")]
    pub struct SetFeeToSetterCall {
        pub fee_to_setter: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniV2FactoryCalls {
        InitCodePairHash(InitCodePairHashCall),
        AllPairs(AllPairsCall),
        AllPairsLength(AllPairsLengthCall),
        CreatePair(CreatePairCall),
        FeeTo(FeeToCall),
        FeeToSetter(FeeToSetterCall),
        GetPair(GetPairCall),
        PairCodeHash(PairCodeHashCall),
        SetFeeTo(SetFeeToCall),
        SetFeeToSetter(SetFeeToSetterCall),
    }
    impl ethers::core::abi::AbiDecode for UniV2FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <InitCodePairHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::InitCodePairHash(decoded));
            }
            if let Ok(decoded) =
                <AllPairsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::AllPairs(decoded));
            }
            if let Ok(decoded) =
                <AllPairsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::AllPairsLength(decoded));
            }
            if let Ok(decoded) =
                <CreatePairCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::CreatePair(decoded));
            }
            if let Ok(decoded) = <FeeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::FeeTo(decoded));
            }
            if let Ok(decoded) =
                <FeeToSetterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::FeeToSetter(decoded));
            }
            if let Ok(decoded) =
                <GetPairCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::GetPair(decoded));
            }
            if let Ok(decoded) =
                <PairCodeHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::PairCodeHash(decoded));
            }
            if let Ok(decoded) =
                <SetFeeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::SetFeeTo(decoded));
            }
            if let Ok(decoded) =
                <SetFeeToSetterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2FactoryCalls::SetFeeToSetter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniV2FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniV2FactoryCalls::InitCodePairHash(element) => element.encode(),
                UniV2FactoryCalls::AllPairs(element) => element.encode(),
                UniV2FactoryCalls::AllPairsLength(element) => element.encode(),
                UniV2FactoryCalls::CreatePair(element) => element.encode(),
                UniV2FactoryCalls::FeeTo(element) => element.encode(),
                UniV2FactoryCalls::FeeToSetter(element) => element.encode(),
                UniV2FactoryCalls::GetPair(element) => element.encode(),
                UniV2FactoryCalls::PairCodeHash(element) => element.encode(),
                UniV2FactoryCalls::SetFeeTo(element) => element.encode(),
                UniV2FactoryCalls::SetFeeToSetter(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniV2FactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniV2FactoryCalls::InitCodePairHash(element) => element.fmt(f),
                UniV2FactoryCalls::AllPairs(element) => element.fmt(f),
                UniV2FactoryCalls::AllPairsLength(element) => element.fmt(f),
                UniV2FactoryCalls::CreatePair(element) => element.fmt(f),
                UniV2FactoryCalls::FeeTo(element) => element.fmt(f),
                UniV2FactoryCalls::FeeToSetter(element) => element.fmt(f),
                UniV2FactoryCalls::GetPair(element) => element.fmt(f),
                UniV2FactoryCalls::PairCodeHash(element) => element.fmt(f),
                UniV2FactoryCalls::SetFeeTo(element) => element.fmt(f),
                UniV2FactoryCalls::SetFeeToSetter(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<InitCodePairHashCall> for UniV2FactoryCalls {
        fn from(var: InitCodePairHashCall) -> Self {
            UniV2FactoryCalls::InitCodePairHash(var)
        }
    }
    impl ::std::convert::From<AllPairsCall> for UniV2FactoryCalls {
        fn from(var: AllPairsCall) -> Self {
            UniV2FactoryCalls::AllPairs(var)
        }
    }
    impl ::std::convert::From<AllPairsLengthCall> for UniV2FactoryCalls {
        fn from(var: AllPairsLengthCall) -> Self {
            UniV2FactoryCalls::AllPairsLength(var)
        }
    }
    impl ::std::convert::From<CreatePairCall> for UniV2FactoryCalls {
        fn from(var: CreatePairCall) -> Self {
            UniV2FactoryCalls::CreatePair(var)
        }
    }
    impl ::std::convert::From<FeeToCall> for UniV2FactoryCalls {
        fn from(var: FeeToCall) -> Self {
            UniV2FactoryCalls::FeeTo(var)
        }
    }
    impl ::std::convert::From<FeeToSetterCall> for UniV2FactoryCalls {
        fn from(var: FeeToSetterCall) -> Self {
            UniV2FactoryCalls::FeeToSetter(var)
        }
    }
    impl ::std::convert::From<GetPairCall> for UniV2FactoryCalls {
        fn from(var: GetPairCall) -> Self {
            UniV2FactoryCalls::GetPair(var)
        }
    }
    impl ::std::convert::From<PairCodeHashCall> for UniV2FactoryCalls {
        fn from(var: PairCodeHashCall) -> Self {
            UniV2FactoryCalls::PairCodeHash(var)
        }
    }
    impl ::std::convert::From<SetFeeToCall> for UniV2FactoryCalls {
        fn from(var: SetFeeToCall) -> Self {
            UniV2FactoryCalls::SetFeeTo(var)
        }
    }
    impl ::std::convert::From<SetFeeToSetterCall> for UniV2FactoryCalls {
        fn from(var: SetFeeToSetterCall) -> Self {
            UniV2FactoryCalls::SetFeeToSetter(var)
        }
    }
    #[doc = "Container type for all return fields from the `INIT_CODE_PAIR_HASH` function with signature `INIT_CODE_PAIR_HASH()` and selector `[88, 85, 162, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct InitCodePairHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `allPairs` function with signature `allPairs(uint256)` and selector `[30, 61, 209, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AllPairsReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `allPairsLength` function with signature `allPairsLength()` and selector `[87, 79, 43, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AllPairsLengthReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `createPair` function with signature `createPair(address,address)` and selector `[201, 198, 83, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreatePairReturn {
        pub pair: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `feeTo` function with signature `feeTo()` and selector `[1, 126, 126, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeToReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `feeToSetter` function with signature `feeToSetter()` and selector `[9, 75, 116, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeToSetterReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPair` function with signature `getPair(address,address)` and selector `[230, 164, 57, 5]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPairReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `pairCodeHash` function with signature `pairCodeHash()` and selector `[154, 171, 146, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PairCodeHashReturn(pub [u8; 32]);
}
