https://github.com/uniVocity/cardano-tutorials/blob/master/cardano-addresses.md

```
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
```

---

```
cat key_for_account_0_address_0.txt | ./cardano-address address payment --network-tag 0 > test.addr

cat test.addr
addr_test1vqx8mqt8wmyfccltrv5sqc7dx0fh30p8892yjz63af6zcvgszmek6

cat test.addr | ./cardano-address address inspect
{
    "address_style": "Shelley",
    "address_type": 6,
    "network_tag": 0,
    "spending_key_hash": "0c7d816776c89c63eb1b290063cd33d378bc273954490b51ea742c31",
    "spending_key_hash_bech32": "addr_vkh1p37czemkezwx86cm9yqx8nfn6dutcfee23ysk502wskrz2wfa4t",
    "stake_reference": "none"
}

PreProd faucet Tx hash: 846bbc78e51c785f2e8c2a4c141068d7bf84556d350456af159f4ccf5668b3e5
```

---

Exaple from `./cardano-address key`:

```
<snip>
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

---

More: 

https://developers.cardano.org/docs/get-started/create-simple-transaction
