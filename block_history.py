import json
from web3 import HTTPProvider, Web3
from web3.middleware import geth_poa_middleware

# - Installing modules: `pip install web3`
# - Running script `python3 block_history.py`
network = "https://api.avax.network/ext/bc/C/rpc"
my_web3 = Web3(HTTPProvider(network))
my_web3.middleware_onion.inject(geth_poa_middleware, layer=0)

with open("src/abi/LpPair.json") as f:
    pair_abi = json.load(f)

block = 16265055

pair_addresses = [
    "0x781655d802670bbA3c89aeBaaEa59D3182fD755D",
]

print()
print("Block: %s" % (block))

pair_contract = my_web3.eth.contract(address=pair_addresses[0], abi=pair_abi)
reserves = pair_contract.functions.getReserves().call({}, block)[:2]
print("Reserves_0: %s %s" % (pair_addresses[0], reserves))

print()
