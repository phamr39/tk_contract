yarn build && near deploy --wasmFile res/lottery_contract.wasm --accountId lottery_game.testnet

near call lottery_game.testnet new '{}' --accountId lottery_game.testnet

near call lottery_game.testnet new_game '{}' --accountId lottery_game.testnet

near call lottery_game.testnet get_game '{"id":"bG90dGVyeV9nYW1lLnRlc3RuZXRfMTY1Mzk4MDE3MzQwNzkwMTczMg=="}' --accountId lottery_game.testnet

near call lottery_game.testnet get_current_game '{}' --accountId lottery_game.testnet

near call lottery_game.testnet get_previous_game '{}' --accountId lottery_game.testnet

near call lottery_game.testnet buy_ticket '{"num":16}' --accountId hieutest1.testnet --deposit 1

near call lottery_game.testnet end_game '{}' --accountId lottery_game.testnet
