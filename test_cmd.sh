near deploy --wasmFile res/lottery_contract.wasm --accountId neariot.testnet

near call ciuz.testnet new "{}" --accountId ciuz.testnet

near call neariot.testnet new_cluster '{"name":"aloha","description":"Hello World"}' --accountId neariot.testnet

near call neariot.testnet get_clusters --accountId neariot.testnet

near call neariot.testnet get_cluster '{"id":"Y2l1ei50ZXN0bmV0XzE2NTM1ODAyMjgzMTE5OTkxMTE="}' --accountId neariot.testnet

near call neariot.testnet get_cluster_data '{"id":"Y2l1ei50ZXN0bmV0XzE2NTM1ODAyMjgzMTE5OTkxMTE="}' --accountId neariot.testnet

