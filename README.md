# mev-template

A boilerplate foundation to build on top of instead of building a bot from scratch.

## Testing Onchain Data

### Using Python3

- Install python3 `pip install`.
- Install modules: `pip install web3`.
- Running script `python3 block_history.py`.

5 main components to consider:

- `provider`: Which chain is being utilised.
- `abi`: The contract interface to interact with.
- `block`: Which mined block you're calling from (can be past block).
- `contract address`: Contract address to call.
- `function call`: Function to call from the `contract address`.
