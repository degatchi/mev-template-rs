# MEV Template

> Designed by [DeGatchi](https://twitter.com/DeGatchi).

Bootstrap your MEV bot strategies with a simple boilerplate to build on top of.

## How To Use This Template

I wrote an article [How To Build A MEV Bot](https://www.degatchi.com/articles/how-to-build-a-mev-bot) that explains the overall architecture you need to think of to build a bot - from getting a strategy and fetching data to general tips on design structure (that you most likely wont find anywhere else).

I've built out a fair few foundational features that you can customsie to your liking. Whether your strategy utilises a mempool scanner, next block strategy, event streaming, this repo lays the basics out so you don't need to copy and paste your bots for new strategies.
s
Majority of these features can be expanded upon to become much more comprehensive, however it's not wise to open source it since your competition will either use it/know how advance your system is ;)

Having said that, the aim of this was to create something to help people that are struggling to get into MEV. Despite it being a highly competitive field where this repo would enhance/onboard new competitors, I wanted to give people that are in the same position as I once was a chance to break into the field. It's a cold world out there in MEV-land but hopefully this will give you that inspiration you need to keep pushing forward <3

## Quick Start

- Get a node from [QuickNode](https://www.quicknode.com) as they have a free websocket option or you can run your own node.
- Import your EOA priv key for the bot to execute transactions.
- Add your discord websocket to send alerts to.
- For testing, run `cargo run`
- For production, run `cargo run --release`

## Features

This repo comes with the following features implemented.

- [x] Simple discord message system.
- [x] Historic block function caller.
- [x] Uniswap `amountIn` + `amountOut` functions.
- [x] Contract ABI binding.
- [x] Template contract
  - [x] Uniswap getters.
  - [x] Withdraw ERC20 + ETH functions.
- [x] Mempool Monitoring.
  - [x] Simple tx decoding.

### Mempool Monitoring Example

```
---------- MONITORING MEMPOOL ----------
Transaction: Transaction {
    hash: 0xcb3647deb3b7ada364a6643752bf9243b27e84cea78cc0010d26fa3ae52b5e13,
    nonce: 22387,
    block_hash: None,
    block_number: None,
    transaction_index: None,
    from: 0xe88102f2900483c63d0adcdaf4839c2759949de6,
    to: Some(
        0x16327e3fbdaca3bcf7e38f5af2599d2ddc33ae52,
    ),
    value: 9024569904524523678,
    gas_price: Some(
        120000000000,
    ),
    gas: 1000000,
    input: Bytes(
        b"\x7f\xf3j\xb5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\xbbdEK\xa0[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\0\0\0\0\0\0\0\0\0\0\0\0\xe8\x81\x02\xf2\x90\x04\x83\xc6=\n\xdc\xda\xf4\x83\x9c'Y\x94\x9d\xe6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\xf0\\\x10\xa0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0!\xbe7\rS\x12\xf4L\xb4,\xe3w\xbc\x9b\x8a\x0c\xef\x1aL\x83\0\0\0\0\0\0\0\0\0\0\0\0\xbeAw%\x87\x87*\x92\x18Hs\xd5[\t\xc6\xbboY\xf8\x95",
    ),
    v: 535,
    r: 44692797049587778392645963656820336298473713424700451186489839760496971858835,
    s: 36641529939556041694165250732768051817286656083457538183752182129357071704354,
    transaction_type: Some(
        0,
    ),
    access_list: None,
    max_priority_fee_per_gas: None,
    max_fee_per_gas: None,
    chain_id: None,
    other: OtherFields {
        inner: {},
    },
}
Router Call: SwapExactETHForTokens(
    SwapExactETHForTokensCall {
        amount_out_min: 4709638961078363,
        path: [
            0x21be370d5312f44cb42ce377bc9b8a0cef1a4c83,
            0xbe41772587872a92184873d55b09c6bb6f59f895,
        ],
        to: 0xe88102f2900483c63d0adcdaf4839c2759949de6,
        deadline: 12622500000,
    },
)
```

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
