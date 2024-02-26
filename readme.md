https://github.com/uniVocity/cardano-tutorials/blob/master/cardano-addresses.md

wget https://github.com/cardano-foundation/cardano-wallet/releases/download/v2023-12-18/cardano-wallet-v2023-12-18-linux64.tar.gz

tar -xvf cardano-wallet-v2023-12-18-linux64.tar.gz

./cardano-address recovery-phrase generate --size 24 > seed.txt

cat seed.txt | ./cardano-address key from-recovery-phrase Shelley > private_key.txt

cat private_key.txt | ./cardano-address key public --without-chain-code > public_key.txt

cat private_key.txt | ./cardano-address key child 1852H/1815H/0H > account_0_key_path.txt

cat account_0_key_path.txt | ./cardano-address key public --with-chain-code > account_0_public_root_key.txt

cat account_0_public_root_key.txt | ./cardano-address key child 0/0 > key_for_account_0_address_0.txt

## --network-tag: 1 = mainnet, 0 = testnet
cat key_for_account_0_address_0.txt | ./cardano-address address payment --network-tag 1 > pay_to_account_0_address_0.txt

addr1v8fet8gavr6elqt6q50skkjf025zthqu6vr56l5k39sp9aqlvz2g4

---

TODO: https://developers.cardano.org/docs/get-started/create-simple-transaction
