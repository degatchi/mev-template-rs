# MEV Template

A template to build a MEV bot from.

## Quick Start
- Get a node from [QuickNode](https://www.quicknode.com) as they have a free websocket option or you can run your own node.
- Import your EOA priv key for the bot to execute transactions.
- Add your discord websocket to send alerts to.
- For testing, run `cargo run`
- For production, run `cargo run --release`

## Features

- [x] Simple discord message system.
- [x] Historic block function caller.
- [x] Uniswap `amountIn` + `amountOut` functions
- [x] Contract ABI binding.
- [x] Template contract
  - [x] Uniswap getters.
  - [x] Withdraw ERC20 + ETH functions.

## Testing Onchain Data

Setting up environment:

- Install python3: `pip install`.
- Install modules: `pip install web3`.
- Run: `python3 block_history.py`.

5 main components to consider:

- `provider`: Which chain is being utilised.
- `abi`: The contract interface to interact with.
- `block`: Which mined block you're calling from (can be past block).
- `contract address`: Contract address to call.
- `function call`: Function to call from the `contract address`.
