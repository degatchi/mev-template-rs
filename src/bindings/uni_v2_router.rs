pub use uni_v2_router::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod uni_v2_router {
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
    #[doc = "UniV2Router was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_factory\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_WETH\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"WETH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"tokenA\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"tokenB\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountADesired\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountBDesired\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountAMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountBMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"addLiquidity\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountA\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountB\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountTokenDesired\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountTokenMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETHMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"addLiquidityETH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToken\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETH\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"factory\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"reserveIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"reserveOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"getAmountIn\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"reserveIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"reserveOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"getAmountOut\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"name\": \"getAmountsIn\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"name\": \"getAmountsOut\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountA\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"reserveA\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"reserveB\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"quote\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountB\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"tokenA\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"tokenB\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountAMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountBMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"removeLiquidity\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountA\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountB\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountTokenMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETHMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"removeLiquidityETH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToken\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETH\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountTokenMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETHMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"removeLiquidityETHSupportingFeeOnTransferTokens\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETH\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountTokenMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETHMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"approveMax\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"removeLiquidityETHWithPermit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToken\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETH\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountTokenMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETHMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"approveMax\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"removeLiquidityETHWithPermitSupportingFeeOnTransferTokens\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountETH\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"tokenA\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"tokenB\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountAMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountBMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"approveMax\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"removeLiquidityWithPermit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountA\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountB\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapETHForExactTokens\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOutMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapExactETHForTokens\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOutMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapExactETHForTokensSupportingFeeOnTransferTokens\",\n        \"outputs\": [],\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOutMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapExactTokensForETH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOutMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapExactTokensForETHSupportingFeeOnTransferTokens\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOutMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapExactTokensForTokens\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOutMin\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapExactTokensForTokensSupportingFeeOnTransferTokens\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountInMax\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapTokensForExactETH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountInMax\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"swapTokensForExactTokens\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"stateMutability\": \"payable\",\n        \"type\": \"receive\"\n    }\n]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static UNIV2ROUTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct UniV2Router<M>(ethers::contract::Contract<M>);
    impl<M> Clone for UniV2Router<M> {
        fn clone(&self) -> Self {
            UniV2Router(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for UniV2Router<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for UniV2Router<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniV2Router))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> UniV2Router<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), UNIV2ROUTER_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidity` (0xe8e33700) function"]
        pub fn add_liquidity(
            &self,
            token_a: ethers::core::types::Address,
            token_b: ethers::core::types::Address,
            amount_a_desired: ethers::core::types::U256,
            amount_b_desired: ethers::core::types::U256,
            amount_a_min: ethers::core::types::U256,
            amount_b_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [232, 227, 55, 0],
                    (
                        token_a,
                        token_b,
                        amount_a_desired,
                        amount_b_desired,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityETH` (0xf305d719) function"]
        pub fn add_liquidity_eth(
            &self,
            token: ethers::core::types::Address,
            amount_token_desired: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [243, 5, 215, 25],
                    (
                        token,
                        amount_token_desired,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
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
        #[doc = "Calls the contract's `getAmountIn` (0x85f8c259) function"]
        pub fn get_amount_in(
            &self,
            amount_out: ethers::core::types::U256,
            reserve_in: ethers::core::types::U256,
            reserve_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([133, 248, 194, 89], (amount_out, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountOut` (0x054d50d4) function"]
        pub fn get_amount_out(
            &self,
            amount_in: ethers::core::types::U256,
            reserve_in: ethers::core::types::U256,
            reserve_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([5, 77, 80, 212], (amount_in, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountsIn` (0x1f00ca74) function"]
        pub fn get_amounts_in(
            &self,
            amount_out: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([31, 0, 202, 116], (amount_out, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountsOut` (0xd06ca61f) function"]
        pub fn get_amounts_out(
            &self,
            amount_in: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([208, 108, 166, 31], (amount_in, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quote` (0xad615dec) function"]
        pub fn quote(
            &self,
            amount_a: ethers::core::types::U256,
            reserve_a: ethers::core::types::U256,
            reserve_b: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([173, 97, 93, 236], (amount_a, reserve_a, reserve_b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidity` (0xbaa2abde) function"]
        pub fn remove_liquidity(
            &self,
            token_a: ethers::core::types::Address,
            token_b: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_a_min: ethers::core::types::U256,
            amount_b_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [186, 162, 171, 222],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETH` (0x02751cec) function"]
        pub fn remove_liquidity_eth(
            &self,
            token: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [2, 117, 28, 236],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHSupportingFeeOnTransferTokens` (0xaf2979eb) function"]
        pub fn remove_liquidity_eth_supporting_fee_on_transfer_tokens(
            &self,
            token: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [175, 41, 121, 235],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHWithPermit` (0xded9382a) function"]
        pub fn remove_liquidity_eth_with_permit(
            &self,
            token: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [222, 217, 56, 42],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens` (0x5b0d5984) function"]
        pub fn remove_liquidity_eth_with_permit_supporting_fee_on_transfer_tokens(
            &self,
            token: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [91, 13, 89, 132],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityWithPermit` (0x2195995c) function"]
        pub fn remove_liquidity_with_permit(
            &self,
            token_a: ethers::core::types::Address,
            token_b: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_a_min: ethers::core::types::U256,
            amount_b_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [33, 149, 153, 92],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapETHForExactTokens` (0xfb3bdb41) function"]
        pub fn swap_eth_for_exact_tokens(
            &self,
            amount_out: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([251, 59, 219, 65], (amount_out, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactETHForTokens` (0x7ff36ab5) function"]
        pub fn swap_exact_eth_for_tokens(
            &self,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([127, 243, 106, 181], (amount_out_min, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactETHForTokensSupportingFeeOnTransferTokens` (0xb6f9de95) function"]
        pub fn swap_exact_eth_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 249, 222, 149], (amount_out_min, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForETH` (0x18cbafe5) function"]
        pub fn swap_exact_tokens_for_eth(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [24, 203, 175, 229],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForETHSupportingFeeOnTransferTokens` (0x791ac947) function"]
        pub fn swap_exact_tokens_for_eth_supporting_fee_on_transfer_tokens(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [121, 26, 201, 71],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForTokens` (0x38ed1739) function"]
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [56, 237, 23, 57],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForTokensSupportingFeeOnTransferTokens` (0x5c11d795) function"]
        pub fn swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [92, 17, 215, 149],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensForExactETH` (0x4a25d94a) function"]
        pub fn swap_tokens_for_exact_eth(
            &self,
            amount_out: ethers::core::types::U256,
            amount_in_max: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [74, 37, 217, 74],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensForExactTokens` (0x8803dbee) function"]
        pub fn swap_tokens_for_exact_tokens(
            &self,
            amount_out: ethers::core::types::U256,
            amount_in_max: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [136, 3, 219, 238],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for UniV2Router<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `[173, 92, 70, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    #[doc = "Container type for all input parameters for the `addLiquidity` function with signature `addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `[232, 227, 55, 0]`"]
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
        name = "addLiquidity",
        abi = "addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityCall {
        pub token_a: ethers::core::types::Address,
        pub token_b: ethers::core::types::Address,
        pub amount_a_desired: ethers::core::types::U256,
        pub amount_b_desired: ethers::core::types::U256,
        pub amount_a_min: ethers::core::types::U256,
        pub amount_b_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `addLiquidityETH` function with signature `addLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `[243, 5, 215, 25]`"]
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
        name = "addLiquidityETH",
        abi = "addLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityETHCall {
        pub token: ethers::core::types::Address,
        pub amount_token_desired: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `getAmountIn` function with signature `getAmountIn(uint256,uint256,uint256)` and selector `[133, 248, 194, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAmountIn", abi = "getAmountIn(uint256,uint256,uint256)")]
    pub struct GetAmountInCall {
        pub amount_out: ethers::core::types::U256,
        pub reserve_in: ethers::core::types::U256,
        pub reserve_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint256,uint256,uint256)` and selector `[5, 77, 80, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint256,uint256,uint256)")]
    pub struct GetAmountOutCall {
        pub amount_in: ethers::core::types::U256,
        pub reserve_in: ethers::core::types::U256,
        pub reserve_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAmountsIn` function with signature `getAmountsIn(uint256,address[])` and selector `[31, 0, 202, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAmountsIn", abi = "getAmountsIn(uint256,address[])")]
    pub struct GetAmountsInCall {
        pub amount_out: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `getAmountsOut` function with signature `getAmountsOut(uint256,address[])` and selector `[208, 108, 166, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAmountsOut", abi = "getAmountsOut(uint256,address[])")]
    pub struct GetAmountsOutCall {
        pub amount_in: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `quote` function with signature `quote(uint256,uint256,uint256)` and selector `[173, 97, 93, 236]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "quote", abi = "quote(uint256,uint256,uint256)")]
    pub struct QuoteCall {
        pub amount_a: ethers::core::types::U256,
        pub reserve_a: ethers::core::types::U256,
        pub reserve_b: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidity` function with signature `removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)` and selector `[186, 162, 171, 222]`"]
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
        name = "removeLiquidity",
        abi = "removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityCall {
        pub token_a: ethers::core::types::Address,
        pub token_b: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_a_min: ethers::core::types::U256,
        pub amount_b_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETH` function with signature `removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `[2, 117, 28, 236]`"]
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
        name = "removeLiquidityETH",
        abi = "removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHCall {
        pub token: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETHSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)` and selector `[175, 41, 121, 235]`"]
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
        name = "removeLiquidityETHSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHSupportingFeeOnTransferTokensCall {
        pub token: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETHWithPermit` function with signature `removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[222, 217, 56, 42]`"]
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
        name = "removeLiquidityETHWithPermit",
        abi = "removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitCall {
        pub token: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[91, 13, 89, 132]`"]
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
        name = "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall {
        pub token: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityWithPermit` function with signature `removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[33, 149, 153, 92]`"]
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
        name = "removeLiquidityWithPermit",
        abi = "removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityWithPermitCall {
        pub token_a: ethers::core::types::Address,
        pub token_b: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_a_min: ethers::core::types::U256,
        pub amount_b_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `swapETHForExactTokens` function with signature `swapETHForExactTokens(uint256,address[],address,uint256)` and selector `[251, 59, 219, 65]`"]
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
        name = "swapETHForExactTokens",
        abi = "swapETHForExactTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapETHForExactTokensCall {
        pub amount_out: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactETHForTokens` function with signature `swapExactETHForTokens(uint256,address[],address,uint256)` and selector `[127, 243, 106, 181]`"]
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
        name = "swapExactETHForTokens",
        abi = "swapExactETHForTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensCall {
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactETHForTokensSupportingFeeOnTransferTokens` function with signature `swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)` and selector `[182, 249, 222, 149]`"]
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
        name = "swapExactETHForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensSupportingFeeOnTransferTokensCall {
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForETH` function with signature `swapExactTokensForETH(uint256,uint256,address[],address,uint256)` and selector `[24, 203, 175, 229]`"]
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
        name = "swapExactTokensForETH",
        abi = "swapExactTokensForETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForETHSupportingFeeOnTransferTokens` function with signature `swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)` and selector `[121, 26, 201, 71]`"]
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
        name = "swapExactTokensForETHSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHSupportingFeeOnTransferTokensCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address,uint256)` and selector `[56, 237, 23, 57]`"]
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
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForTokensSupportingFeeOnTransferTokens` function with signature `swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)` and selector `[92, 17, 215, 149]`"]
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
        name = "swapExactTokensForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensSupportingFeeOnTransferTokensCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapTokensForExactETH` function with signature `swapTokensForExactETH(uint256,uint256,address[],address,uint256)` and selector `[74, 37, 217, 74]`"]
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
        name = "swapTokensForExactETH",
        abi = "swapTokensForExactETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactETHCall {
        pub amount_out: ethers::core::types::U256,
        pub amount_in_max: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(uint256,uint256,address[],address,uint256)` and selector `[136, 3, 219, 238]`"]
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
        name = "swapTokensForExactTokens",
        abi = "swapTokensForExactTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactTokensCall {
        pub amount_out: ethers::core::types::U256,
        pub amount_in_max: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniV2RouterCalls {
        Weth(WethCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityETH(AddLiquidityETHCall),
        Factory(FactoryCall),
        GetAmountIn(GetAmountInCall),
        GetAmountOut(GetAmountOutCall),
        GetAmountsIn(GetAmountsInCall),
        GetAmountsOut(GetAmountsOutCall),
        Quote(QuoteCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        RemoveLiquidityETHSupportingFeeOnTransferTokens(
            RemoveLiquidityETHSupportingFeeOnTransferTokensCall,
        ),
        RemoveLiquidityETHWithPermit(RemoveLiquidityETHWithPermitCall),
        RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
            RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall,
        ),
        RemoveLiquidityWithPermit(RemoveLiquidityWithPermitCall),
        SwapETHForExactTokens(SwapETHForExactTokensCall),
        SwapExactETHForTokens(SwapExactETHForTokensCall),
        SwapExactETHForTokensSupportingFeeOnTransferTokens(
            SwapExactETHForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForETH(SwapExactTokensForETHCall),
        SwapExactTokensForETHSupportingFeeOnTransferTokens(
            SwapExactTokensForETHSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        SwapExactTokensForTokensSupportingFeeOnTransferTokens(
            SwapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapTokensForExactETH(SwapTokensForExactETHCall),
        SwapTokensForExactTokens(SwapTokensForExactTokensCall),
    }
    impl ethers::core::abi::AbiDecode for UniV2RouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniV2RouterCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::AddLiquidity(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <GetAmountInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::GetAmountIn(decoded));
            }
            if let Ok(decoded) =
                <GetAmountOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::GetAmountOut(decoded));
            }
            if let Ok(decoded) =
                <GetAmountsInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::GetAmountsIn(decoded));
            }
            if let Ok(decoded) =
                <GetAmountsOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::GetAmountsOut(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::Quote(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::RemoveLiquidityETH(decoded));
            }
            if let Ok (decoded) = < RemoveLiquidityETHSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniV2RouterCalls :: RemoveLiquidityETHSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <RemoveLiquidityETHWithPermitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniV2RouterCalls::RemoveLiquidityETHWithPermit(decoded));
            }
            if let Ok (decoded) = < RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniV2RouterCalls :: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <RemoveLiquidityWithPermitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniV2RouterCalls::RemoveLiquidityWithPermit(decoded));
            }
            if let Ok(decoded) =
                <SwapETHForExactTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::SwapETHForExactTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapExactETHForTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::SwapExactETHForTokens(decoded));
            }
            if let Ok (decoded) = < SwapExactETHForTokensSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniV2RouterCalls :: SwapExactETHForTokensSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapExactTokensForETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::SwapExactTokensForETH(decoded));
            }
            if let Ok (decoded) = < SwapExactTokensForETHSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniV2RouterCalls :: SwapExactTokensForETHSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapExactTokensForTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniV2RouterCalls::SwapExactTokensForTokens(decoded));
            }
            if let Ok (decoded) = < SwapExactTokensForTokensSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniV2RouterCalls :: SwapExactTokensForTokensSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapTokensForExactETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniV2RouterCalls::SwapTokensForExactETH(decoded));
            }
            if let Ok(decoded) =
                <SwapTokensForExactTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniV2RouterCalls::SwapTokensForExactTokens(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniV2RouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniV2RouterCalls::Weth(element) => element.encode(),
                UniV2RouterCalls::AddLiquidity(element) => element.encode(),
                UniV2RouterCalls::AddLiquidityETH(element) => element.encode(),
                UniV2RouterCalls::Factory(element) => element.encode(),
                UniV2RouterCalls::GetAmountIn(element) => element.encode(),
                UniV2RouterCalls::GetAmountOut(element) => element.encode(),
                UniV2RouterCalls::GetAmountsIn(element) => element.encode(),
                UniV2RouterCalls::GetAmountsOut(element) => element.encode(),
                UniV2RouterCalls::Quote(element) => element.encode(),
                UniV2RouterCalls::RemoveLiquidity(element) => element.encode(),
                UniV2RouterCalls::RemoveLiquidityETH(element) => element.encode(),
                UniV2RouterCalls::RemoveLiquidityETHSupportingFeeOnTransferTokens(element) => {
                    element.encode()
                }
                UniV2RouterCalls::RemoveLiquidityETHWithPermit(element) => element.encode(),
                UniV2RouterCalls::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
                    element,
                ) => element.encode(),
                UniV2RouterCalls::RemoveLiquidityWithPermit(element) => element.encode(),
                UniV2RouterCalls::SwapETHForExactTokens(element) => element.encode(),
                UniV2RouterCalls::SwapExactETHForTokens(element) => element.encode(),
                UniV2RouterCalls::SwapExactETHForTokensSupportingFeeOnTransferTokens(element) => {
                    element.encode()
                }
                UniV2RouterCalls::SwapExactTokensForETH(element) => element.encode(),
                UniV2RouterCalls::SwapExactTokensForETHSupportingFeeOnTransferTokens(element) => {
                    element.encode()
                }
                UniV2RouterCalls::SwapExactTokensForTokens(element) => element.encode(),
                UniV2RouterCalls::SwapExactTokensForTokensSupportingFeeOnTransferTokens(
                    element,
                ) => element.encode(),
                UniV2RouterCalls::SwapTokensForExactETH(element) => element.encode(),
                UniV2RouterCalls::SwapTokensForExactTokens(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniV2RouterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniV2RouterCalls::Weth(element) => element.fmt(f),
                UniV2RouterCalls::AddLiquidity(element) => element.fmt(f),
                UniV2RouterCalls::AddLiquidityETH(element) => element.fmt(f),
                UniV2RouterCalls::Factory(element) => element.fmt(f),
                UniV2RouterCalls::GetAmountIn(element) => element.fmt(f),
                UniV2RouterCalls::GetAmountOut(element) => element.fmt(f),
                UniV2RouterCalls::GetAmountsIn(element) => element.fmt(f),
                UniV2RouterCalls::GetAmountsOut(element) => element.fmt(f),
                UniV2RouterCalls::Quote(element) => element.fmt(f),
                UniV2RouterCalls::RemoveLiquidity(element) => element.fmt(f),
                UniV2RouterCalls::RemoveLiquidityETH(element) => element.fmt(f),
                UniV2RouterCalls::RemoveLiquidityETHSupportingFeeOnTransferTokens(element) => {
                    element.fmt(f)
                }
                UniV2RouterCalls::RemoveLiquidityETHWithPermit(element) => element.fmt(f),
                UniV2RouterCalls::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
                    element,
                ) => element.fmt(f),
                UniV2RouterCalls::RemoveLiquidityWithPermit(element) => element.fmt(f),
                UniV2RouterCalls::SwapETHForExactTokens(element) => element.fmt(f),
                UniV2RouterCalls::SwapExactETHForTokens(element) => element.fmt(f),
                UniV2RouterCalls::SwapExactETHForTokensSupportingFeeOnTransferTokens(element) => {
                    element.fmt(f)
                }
                UniV2RouterCalls::SwapExactTokensForETH(element) => element.fmt(f),
                UniV2RouterCalls::SwapExactTokensForETHSupportingFeeOnTransferTokens(element) => {
                    element.fmt(f)
                }
                UniV2RouterCalls::SwapExactTokensForTokens(element) => element.fmt(f),
                UniV2RouterCalls::SwapExactTokensForTokensSupportingFeeOnTransferTokens(
                    element,
                ) => element.fmt(f),
                UniV2RouterCalls::SwapTokensForExactETH(element) => element.fmt(f),
                UniV2RouterCalls::SwapTokensForExactTokens(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<WethCall> for UniV2RouterCalls {
        fn from(var: WethCall) -> Self {
            UniV2RouterCalls::Weth(var)
        }
    }
    impl ::std::convert::From<AddLiquidityCall> for UniV2RouterCalls {
        fn from(var: AddLiquidityCall) -> Self {
            UniV2RouterCalls::AddLiquidity(var)
        }
    }
    impl ::std::convert::From<AddLiquidityETHCall> for UniV2RouterCalls {
        fn from(var: AddLiquidityETHCall) -> Self {
            UniV2RouterCalls::AddLiquidityETH(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for UniV2RouterCalls {
        fn from(var: FactoryCall) -> Self {
            UniV2RouterCalls::Factory(var)
        }
    }
    impl ::std::convert::From<GetAmountInCall> for UniV2RouterCalls {
        fn from(var: GetAmountInCall) -> Self {
            UniV2RouterCalls::GetAmountIn(var)
        }
    }
    impl ::std::convert::From<GetAmountOutCall> for UniV2RouterCalls {
        fn from(var: GetAmountOutCall) -> Self {
            UniV2RouterCalls::GetAmountOut(var)
        }
    }
    impl ::std::convert::From<GetAmountsInCall> for UniV2RouterCalls {
        fn from(var: GetAmountsInCall) -> Self {
            UniV2RouterCalls::GetAmountsIn(var)
        }
    }
    impl ::std::convert::From<GetAmountsOutCall> for UniV2RouterCalls {
        fn from(var: GetAmountsOutCall) -> Self {
            UniV2RouterCalls::GetAmountsOut(var)
        }
    }
    impl ::std::convert::From<QuoteCall> for UniV2RouterCalls {
        fn from(var: QuoteCall) -> Self {
            UniV2RouterCalls::Quote(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for UniV2RouterCalls {
        fn from(var: RemoveLiquidityCall) -> Self {
            UniV2RouterCalls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHCall> for UniV2RouterCalls {
        fn from(var: RemoveLiquidityETHCall) -> Self {
            UniV2RouterCalls::RemoveLiquidityETH(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHSupportingFeeOnTransferTokensCall>
        for UniV2RouterCalls
    {
        fn from(var: RemoveLiquidityETHSupportingFeeOnTransferTokensCall) -> Self {
            UniV2RouterCalls::RemoveLiquidityETHSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHWithPermitCall> for UniV2RouterCalls {
        fn from(var: RemoveLiquidityETHWithPermitCall) -> Self {
            UniV2RouterCalls::RemoveLiquidityETHWithPermit(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall>
        for UniV2RouterCalls
    {
        fn from(var: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall) -> Self {
            UniV2RouterCalls::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityWithPermitCall> for UniV2RouterCalls {
        fn from(var: RemoveLiquidityWithPermitCall) -> Self {
            UniV2RouterCalls::RemoveLiquidityWithPermit(var)
        }
    }
    impl ::std::convert::From<SwapETHForExactTokensCall> for UniV2RouterCalls {
        fn from(var: SwapETHForExactTokensCall) -> Self {
            UniV2RouterCalls::SwapETHForExactTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactETHForTokensCall> for UniV2RouterCalls {
        fn from(var: SwapExactETHForTokensCall) -> Self {
            UniV2RouterCalls::SwapExactETHForTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactETHForTokensSupportingFeeOnTransferTokensCall>
        for UniV2RouterCalls
    {
        fn from(var: SwapExactETHForTokensSupportingFeeOnTransferTokensCall) -> Self {
            UniV2RouterCalls::SwapExactETHForTokensSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForETHCall> for UniV2RouterCalls {
        fn from(var: SwapExactTokensForETHCall) -> Self {
            UniV2RouterCalls::SwapExactTokensForETH(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForETHSupportingFeeOnTransferTokensCall>
        for UniV2RouterCalls
    {
        fn from(var: SwapExactTokensForETHSupportingFeeOnTransferTokensCall) -> Self {
            UniV2RouterCalls::SwapExactTokensForETHSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForTokensCall> for UniV2RouterCalls {
        fn from(var: SwapExactTokensForTokensCall) -> Self {
            UniV2RouterCalls::SwapExactTokensForTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForTokensSupportingFeeOnTransferTokensCall>
        for UniV2RouterCalls
    {
        fn from(var: SwapExactTokensForTokensSupportingFeeOnTransferTokensCall) -> Self {
            UniV2RouterCalls::SwapExactTokensForTokensSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapTokensForExactETHCall> for UniV2RouterCalls {
        fn from(var: SwapTokensForExactETHCall) -> Self {
            UniV2RouterCalls::SwapTokensForExactETH(var)
        }
    }
    impl ::std::convert::From<SwapTokensForExactTokensCall> for UniV2RouterCalls {
        fn from(var: SwapTokensForExactTokensCall) -> Self {
            UniV2RouterCalls::SwapTokensForExactTokens(var)
        }
    }
    #[doc = "Container type for all return fields from the `WETH` function with signature `WETH()` and selector `[173, 92, 70, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WethReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `addLiquidity` function with signature `addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `[232, 227, 55, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AddLiquidityReturn {
        pub amount_a: ethers::core::types::U256,
        pub amount_b: ethers::core::types::U256,
        pub liquidity: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `addLiquidityETH` function with signature `addLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `[243, 5, 215, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AddLiquidityETHReturn {
        pub amount_token: ethers::core::types::U256,
        pub amount_eth: ethers::core::types::U256,
        pub liquidity: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all return fields from the `getAmountIn` function with signature `getAmountIn(uint256,uint256,uint256)` and selector `[133, 248, 194, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAmountInReturn {
        pub amount_in: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint256,uint256,uint256)` and selector `[5, 77, 80, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAmountOutReturn {
        pub amount_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getAmountsIn` function with signature `getAmountsIn(uint256,address[])` and selector `[31, 0, 202, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAmountsInReturn {
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `getAmountsOut` function with signature `getAmountsOut(uint256,address[])` and selector `[208, 108, 166, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAmountsOutReturn {
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `quote` function with signature `quote(uint256,uint256,uint256)` and selector `[173, 97, 93, 236]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct QuoteReturn {
        pub amount_b: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `removeLiquidity` function with signature `removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)` and selector `[186, 162, 171, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RemoveLiquidityReturn {
        pub amount_a: ethers::core::types::U256,
        pub amount_b: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `removeLiquidityETH` function with signature `removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `[2, 117, 28, 236]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RemoveLiquidityETHReturn {
        pub amount_token: ethers::core::types::U256,
        pub amount_eth: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `removeLiquidityETHSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)` and selector `[175, 41, 121, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RemoveLiquidityETHSupportingFeeOnTransferTokensReturn {
        pub amount_eth: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `removeLiquidityETHWithPermit` function with signature `removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[222, 217, 56, 42]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RemoveLiquidityETHWithPermitReturn {
        pub amount_token: ethers::core::types::U256,
        pub amount_eth: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[91, 13, 89, 132]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensReturn {
        pub amount_eth: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `removeLiquidityWithPermit` function with signature `removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[33, 149, 153, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RemoveLiquidityWithPermitReturn {
        pub amount_a: ethers::core::types::U256,
        pub amount_b: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `swapETHForExactTokens` function with signature `swapETHForExactTokens(uint256,address[],address,uint256)` and selector `[251, 59, 219, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapETHForExactTokensReturn {
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `swapExactETHForTokens` function with signature `swapExactETHForTokens(uint256,address[],address,uint256)` and selector `[127, 243, 106, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapExactETHForTokensReturn {
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `swapExactTokensForETH` function with signature `swapExactTokensForETH(uint256,uint256,address[],address,uint256)` and selector `[24, 203, 175, 229]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapExactTokensForETHReturn {
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address,uint256)` and selector `[56, 237, 23, 57]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapExactTokensForTokensReturn {
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `swapTokensForExactETH` function with signature `swapTokensForExactETH(uint256,uint256,address[],address,uint256)` and selector `[74, 37, 217, 74]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapTokensForExactETHReturn {
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(uint256,uint256,address[],address,uint256)` and selector `[136, 3, 219, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapTokensForExactTokensReturn {
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
}
