https://github.com/uniVocity/cardano-tutorials/blob/master/cardano-addresses.md

https://developers.cardano.org/docs/get-started/create-simple-transaction

```
wget https://github.com/cardano-foundation/cardano-wallet/releases/download/v2023-12-18/cardano-wallet-v2023-12-18-linux64.tar.gz
tar -xvf cardano-wallet-v2023-12-18-linux64.tar.gz
cd cardano-wallet-v2023-12-18-linux64/
mkdir ../bin && mv cardano-* bech32 ../bin/
cd .. && rm -rf cardano-wallet-v2023-12-18-linux64*
export PATH="$PWD/bin:$PATH"

cardano-address recovery-phrase generate --size 24 > seed.txt

cat seed.txt | cardano-address key from-recovery-phrase Shelley > private_key.txt

cat private_key.txt | cardano-address key public --without-chain-code > public_key.txt

cat private_key.txt | cardano-address key child 1852H/1815H/0H > account_0_key_path.txt

cat private_key.txt | cardano-address key child 1852H/1815H/0H/0/0 | cardano-address key inspect
{
    "chain_code": "26ad780cbcd54482ebf406d82d9ba0c8398577df88903e8c98e73dcba5ad4d44",
    "extended_key": "88d92bbcc81bfe08a9ef02d9a23dca9a59873bc636ec78a70c95d9bf7ca7de5e9df4e7d8bbac69ff0816a385b289493b5bc3e0f9d046fa91c3288f16f4d8bcf0",
    "key_type": "private"
}

cat private_key.txt | cardano-address key child 1852H/1815H/0H/0/0 | cardano-address key public --without-chain-code
addr_vk1495qcu6px5rcy7k0k00uw58lva8hhdndc7k3zsd62zt229d6vs3synqule

cat account_0_key_path.txt | cardano-address key inspect
{
    "chain_code": "d0f60c70e2ff3b78005ee82b80578bf8de2fc83c54fafce726861baa94356a79",
    "extended_key": "50399ea7c8d4bfc170300db0640adc6b7c5dee6fc58394042da3685270a7de5edd14fc28a2da7af7da849fcfa0e2e8b1eb0518038e78ec8533cba735ec866da7",
    "key_type": "private"
}

cat account_0_key_path.txt | cardano-address key public --with-chain-code > account_0_public_root_key.txt

cat account_0_public_root_key.txt | cardano-address key child 0/0 > key_for_account_0_address_0.txt

cat key_for_account_0_address_0.txt | cardano-address address payment --network-tag testnet > addr0.txt

cat addr0.txt
addr_test1vp4ar6cqj4dh7qsapjqwh52sd0x035n37z7ds9gntsc94rcdr8ark

cat key_for_account_0_address_1.txt | cardano-address address payment --network-tag testnet > addr1.txt

cat addr1.txt
addr_test1vzlj767yytyslms8p2dxug04tq5n48usac38z4xkdt5nzagxzueyq

cardano-cli address key-hash --payment-verification-key-file key_for_account_0_address_0.txt 
6bd1eb00955b7f021d0c80ebd1506bccf8d271f0bcd815135c305a8f

cardano-cli address key-hash --payment-verification-key-file key_for_account_0_address_1.txt 
bf2f6bc422c90fee070a9a6e21f558293a9f90ee227154d66ae93175
```

---

```
cat key_for_account_0_address_0.txt | cardano-address address payment --network-tag testnet
addr_test1vp4ar6cqj4dh7qsapjqwh52sd0x035n37z7ds9gntsc94rcdr8ark

echo addr_test1vp4ar6cqj4dh7qsapjqwh52sd0x035n37z7ds9gntsc94rcdr8ark | cardano-address address inspect
{
    "address_style": "Shelley",
    "address_type": 6,
    "network_tag": 0,
    "spending_key_hash": "6bd1eb00955b7f021d0c80ebd1506bccf8d271f0bcd815135c305a8f",
    "spending_key_hash_bech32": "addr_vkh1d0g7kqy4tdlsy8gvsr4az5rtenudyu0shnvp2y6uxpdg7z7sjz4",
    "stake_reference": "none"
}

---

cat key_for_account_0_address_1.txt | cardano-address address payment --network-tag testnet
addr_test1vzlj767yytyslms8p2dxug04tq5n48usac38z4xkdt5nzagxzueyq

echo addr_test1vzlj767yytyslms8p2dxug04tq5n48usac38z4xkdt5nzagxzueyq | cardano-address address inspect
{
    "address_style": "Shelley",
    "address_type": 6,
    "network_tag": 0,
    "spending_key_hash": "bf2f6bc422c90fee070a9a6e21f558293a9f90ee227154d66ae93175",
    "spending_key_hash_bech32": "addr_vkh1huhkh3pzey87upc2nfhzra2c9yafly8wyfc4f4n2aych2fxgknj",
    "stake_reference": "none"
}
```

[PreProd Faucet](https://docs.cardano.org/cardano-testnets/tools/faucet): [846bbc78e51c785f2e8c2a4c141068d7bf84556d350456af159f4ccf5668b3e5](https://preprod.cardanoscan.io/transaction/846bbc78e51c785f2e8c2a4c141068d7bf84556d350456af159f4ccf5668b3e5
)

---

```
# ./cardano-address key
...
$ cardano-address recovery-phrase generate --size 15 \
  | cardano-address key from-recovery-phrase Shelley > root.prv

$ cat root.prv \
  | cardano-address key child 1852H/1815H/0H \
  | tee acct.prv \
  | cardano-address key public --with-chain-code > acct.pub

$ cardano-address key inspect <<< $(cat acct.prv)
{
    "key_type": "private",
    "chain_code": "67bef6f80df02c7452e20e76ffb4bb57cae8aac2adf042b21a6b19e4f7b1f511",
    "extended_key": "90ead3efad7aacac242705ede323665387f49ed847bed025eb333708ccf6aa54403482a867daeb18f38c57d6cddd7e6fd6aed4a3209f7425a3d1c5d9987a9c5f"
}

$ cardano-address key inspect <<< $(cat acct.pub)
{
    "key_type": "public",
    "chain_code": "67bef6f80df02c7452e20e76ffb4bb57cae8aac2adf042b21a6b19e4f7b1f511",
    "extended_key": "d306350ee88f51fb710252e27f0c40006c58e994761b383e02d400e2be59b3cc"
}
```
