use blockfrost::{BlockFrostSettings, BlockfrostAPI, BlockfrostResult};
use pallas::ledger::addresses::{
    Network, ShelleyAddress, ShelleyDelegationPart, ShelleyPaymentPart,
};
use pallas::ledger::primitives::Fragment;
use pallas::txbuilder::BuildBabbage;
use pallas::{
    crypto::{
        hash::{Hash, Hasher},
        key::ed25519::SecretKeyExtended,
    },
    ledger::{
        addresses::Address,
        primitives::{
            babbage::AuxiliaryData,
            babbage::{PseudoTx, TransactionBody, WitnessSet},
        },
    },
    txbuilder::{Input, Output, StagingTransaction},
    wallet::PrivateKey,
};

pub type Tx = PseudoTx<TransactionBody, WitnessSet, AuxiliaryData>;

#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    let key: [u8; 64] =
        hex::decode("88d92bbcc81bfe08a9ef02d9a23dca9a59873bc636ec78a70c95d9bf7ca7de5e9df4e7d8bbac69ff0816a385b289493b5bc3e0f9d046fa91c3288f16f4d8bcf0")
            .unwrap()
            .try_into()
            .unwrap();
    let sk: SecretKeyExtended = key.into();
    let pk = sk.public_key();
    println!("SK: {}", hex::encode(key));
    let hash = Hasher::<224>::hash(&pk.as_ref());
    println!("PK: {pk} | {hash}");

    let addr = Address::Shelley(ShelleyAddress::new(
        Network::Testnet,
        ShelleyPaymentPart::key_hash(hash),
        ShelleyDelegationPart::Null,
    ));

    // Preprod Faucet TX
    let tx_hash = "41ad304e0826489399c220046239b8e19971f956ec35d1020b78a46067bc61b3";
    let tx_hash = Hash::new(hex::decode(tx_hash).unwrap().try_into().unwrap());

    let tx = StagingTransaction::new()
        .input(Input::new(tx_hash, 0))
        .output(Output::new(addr, 9_999_000_000))
        .fee(1_000_000)
        .build_babbage_raw()
        .unwrap()
        .sign(PrivateKey::Extended(sk))
        .unwrap();
    println!("\n{}", serde_json::to_string_pretty(&tx).unwrap());

    let parsed = Tx::decode_fragment(tx.tx_bytes.as_ref()).unwrap();
    println!("\n{parsed:#?}");

    let api = build_api()?;
    match api.transactions_submit(tx.tx_bytes.as_ref().to_vec()).await {
        Ok(ret) => println!("\n{ret}"),
        Err(e) => println!("\n{e:?}"),
    }

    // TX: f4e883de0a4061db698adb1fd8f7de43bd704aa01b9f23204f993d4db1ec81d8
    Ok(())
}

#[allow(unused)]
fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    // let api = BlockfrostAPI::new("mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be", settings);
    let api = BlockfrostAPI::new("preprododflHjPhpRp4NzRFL1m9zzd6ZJb1RjYi", settings);
    Ok(api)
}
