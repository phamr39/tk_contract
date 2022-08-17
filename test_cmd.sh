yarn build && near deploy --wasmFile res/lottery_contract.wasm --accountId lottery_game.testnet

near call lottery_game.testnet new '{}' --accountId lottery_game.testnet

near call lottery_game.testnet new_game '{}' --accountId lottery_game.testnet

near call lottery_game.testnet get_game '{"id":"bG90dGVyeV9nYW1lLnRlc3RuZXRfMTY2MDc1MTUxMjU3NjU3NzIzMA=="}' --accountId neutrino.testnet

near call lottery_game.testnet get_current_game '{}' --accountId neutrino.testnet

near call lottery_game.testnet get_previous_game '{}' --accountId neutrino.testnet

near call lottery_game.testnet buy_ticket '{"num":16}' --accountId hieutest1.testnet --deposit 1

near call lottery_game.testnet end_game '{}' --accountId lottery_game.testnet

near call lottery_game.testnet get_user_ticket '{"id":"bG90dGVyeV9nYW1lLnRlc3RuZXRfMTY1NDAwODg5MjE4MjA3NDA3Ng==", "user_id":"hieutest1.testnet"}' --accountId lottery_game.testnet

near call lottery_game.testnet new_lottery_game '{}' --accountId neutrino.testnet

near call lottery_game.testnet new_lottery_game '{}' --accountId lottery_game.testnet

