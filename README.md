# MEV Template

A template to build a MEV bot from.

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
