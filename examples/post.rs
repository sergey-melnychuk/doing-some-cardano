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

// Build and sent a Tx to Cardano mainnet.
#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    /*{
        // https://github.com/txpipe/pallas/blob/v0.23.0/test_data/babbage2.tx
        let bytes = hex::decode("84a5008282582096ad42555f60d076d1d86637c248432e31818e288120909548696887715b9b37018258202b4d18700aec5488599d4b864259ac9a7b09ea04cec6042964dd76f9a5832a05030184a300583931ec3422599965ef95b3574c102313371078ec63851fdd7b266cd1bcc05ea481523030b23a495286ca1a18bd141a493e9b5a19d889953f6cdb01821a00286144a1581c0df03e726bb329f8ba9ce709a03b2c033ef5687a337c2ba17d229e9aa150000643b0537472616e6765456767323201028201d818590148d87982a7446e616d654e537472616e67654567672023323245436f6c6f7245426c61636b4566696c657381a3437372635835697066733a2f2f516d623347744a7a365539614a7855776a796d6b4e716932503166316f4251766f783870765a725077694850316f446e616d654e537472616e676545676720233232496d656469615479706549696d6167652f706e6745696d6167655835697066733a2f2f516d623347744a7a365539614a7855776a796d6b4e716932503166316f4251766f783870765a725077694850316f496d656469615479706549696d6167652f706e674a4261636b67726f756e6444426c75654c636f6e747261637444617461d879860181581c3f1d8ef8617f27510a4c213029feced2afaf0b06b25f72785adad603d87980581c0df03e726bb329f8ba9ce709a03b2c033ef5687a337c2ba17d229e9ad87a80d87a800182583901dfcb5f6c42f7529f31be82e0e68a75c2db94e425409770341011ef805ea481523030b23a495286ca1a18bd141a493e9b5a19d889953f6cdb1a000fa3e88258390192e8a9e8538aa4a72fa6d236ab50d4432872ccf4d6fbed11203060a6967aecccd47781661edb1e7c856c7c2e0781b51de468a796809f22b1821a001226b8a1581c0df03e726bb329f8ba9ce709a03b2c033ef5687a337c2ba17d229e9aa150000de140537472616e67654567673232018258390192e8a9e8538aa4a72fa6d236ab50d4432872ccf4d6fbed11203060a6967aecccd47781661edb1e7c856c7c2e0781b51de468a796809f22b1821a008e08beab581c1564a0fe01cdc09a25b5884b1d0bd23a915a2871001523fb80983043a145746573743301581c2d4e5ff9c9217aa6ea71dc7fc34e33eb93882365e7c785c8abd7a46ca2581c5468655661756c744b65657065722d436f6d6963732d3030312d333601581c5468655661756c744b65657065722d436f6d6963732d3030322d373701581c3270d31cb0803a014924e186d1f0b672ccff0ecb9f6d4b4e2709b301a3484669676874353036014846696768743534300148466967687435383701581c3f75c1332868430e28e156f6309836de8df19b620ab604c166741836a4505468654d616e6472696c6c7a3133383101505468654d616e6472696c6c7a3532363201505468654d616e6472696c6c7a3734343201505468654d616e6472696c6c7a3737333101581c4e7d0d6db3ab5a7ffc58b42b2c7c9d21940cd8a8c2650865a51267e2a155466f7274476f7474656e457030314b69643036373301581c544571c086d0e5c5022aca9717dd0f438e21190abb48f37b3ae129f0a14447524f5701581c5ad8deb64bfec21ad2d96e1270b5873d0c4d0f231b928b4c39eb2435a14661646f7369611a047868c0581c604972deb3891e0dc8a6133163053262d89c3f9ba3ed0c34ba5a51e8a44b4f75746c61772023373236014b4f75746c61772023373535014b4f75746c61772023393231014b4f75746c6177202339373701581c641f0571d02b45b868ac1c479fc8118c5be6744ec3d2c5e13bd888b6a1465a4f4d424945191388581c6954264b15bc92d6d592febeac84f14645e1ed46ca5ebb9acdb5c15fa14553545249501988b8581c8654e8b350e298c80d2451beb5ed80fc9eee9f38ce6b039fb8706bc3a1474c4f42535445521a000f4240021a0003955d031a04fdbaa609a1581c0df03e726bb329f8ba9ce709a03b2c033ef5687a337c2ba17d229e9aa250000643b0537472616e676545676732320150000de140537472616e6765456767323201a20082825820e24fabaa6269567e4a129d1b8e84c653ace22addecfef2f0e5e253bab19de8cd584062ee64a666b1d5027df48daf9a34bf15878e4cd4dfbde2fa5ff176e3d642e5dc20d282404fd28a70b3997c6ab6d9520b5116cb5ee1a2b21275a698e82cca2400825820b0342238145d9d7541c720bc49f5dcecd6bd93ec393ca5a68210b0b55a216ec05840b6ee3663bab18f755bac47aface2cdf5690bfced8e176fc547bec5856e670a3a67e9cec2d25c386ed826eaf9d0eb2e2ffda27f369f7609599e45800059f15f0b01818201828200581c3f1d8ef8617f27510a4c213029feced2afaf0b06b25f72785adad60382051a06b4b5c7f5f6").unwrap();
        let tx = Tx::decode_fragment(&bytes).unwrap();
        println!("{tx:#?}");
    }*/

    // TODO: make sure the tx is signed with the key that matches the key-hash from the error message
    // TODO: (the key hash must match "0c7d816776c89c63eb1b290063cd33d378bc273954490b51ea742c31")
    let key: [u8; 64] =
        hex::decode("801c54f28d09538f0c00b678dd52147793ff8bc18e705c8d4c467555cf123e5d57deca236482df886ce86413bad744b015aa1cb94e4a9bd21ef927ddbe0d8768")
            .unwrap()
            .try_into()
            .unwrap();
    let sk: SecretKeyExtended = key.into();
    let pk = sk.public_key();
    println!("SK: {}", hex::encode(key));
    println!("PK: {pk}");

    let tx_hash = "846bbc78e51c785f2e8c2a4c141068d7bf84556d350456af159f4ccf5668b3e5";
    let tx_hash = Hash::new(hex::decode(tx_hash).unwrap().try_into().unwrap());

    let addr = "12a605ef43e7a5abd2644be6b126f237c5a8b5736dabd66ced624b7f823175e6";
    let addr = hex::decode(addr).unwrap();
    let hash = Hasher::<224>::hash(&addr);
    let addr = Address::Shelley(ShelleyAddress::new(
        Network::Testnet,
        ShelleyPaymentPart::key_hash(hash),
        ShelleyDelegationPart::Null,
    ));

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

    /*
    ERROR:

    transaction submit error ShelleyTxValidationError
    ShelleyBasedEraBabbage (
        ApplyTxError [
            UtxowFailure (
                FromAlonzoUtxowFail (
                    WrappedShelleyEraFailure (
                        MissingVKeyWitnessesUTXOW (
                            WitHashes (
                                fromList [
                                    KeyHash "0c7d816776c89c63eb1b290063cd33d378bc273954490b51ea742c31"
                                ]
                            )
                        )
                    )
                )
            )
        ]
    )

    */

    Ok(())
}

#[allow(unused)]
fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    // let api = BlockfrostAPI::new("mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be", settings);
    let api = BlockfrostAPI::new("preprododflHjPhpRp4NzRFL1m9zzd6ZJb1RjYi", settings);
    Ok(api)
}
