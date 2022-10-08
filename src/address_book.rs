use ethers::prelude::*;

pub(crate) const SPOOKY_SWAP_ROUTER: &str = "0xF491e7B69E4244ad4002BC14e878a34207E38c29";
pub(crate) const SPOOKY_SWAP_FACTORY: &str = "0x152eE697f2E276fA89E96742e9bB9aB1F2E61bE3";

abigen!(UniV2Router, "src/abi/UniV2Router.json");
abigen!(UniV2Factory, "src/abi/UniV2Factory.json");
