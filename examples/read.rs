use blockfrost::{BlockFrostSettings, BlockfrostAPI, BlockfrostResult, Pagination};

#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    let api = build_api()?;

    // https://preprod.cardanoscan.io/address/addr_test1vp4ar6cqj4dh7qsapjqwh52sd0x035n37z7ds9gntsc94rcdr8ark
    let address = "addr_test1vp4ar6cqj4dh7qsapjqwh52sd0x035n37z7ds9gntsc94rcdr8ark";
    let utxos = api.addresses_utxos(address, Pagination::all()).await?;
    println!("{utxos:#?}");

    let block = api.blocks_latest().await?;
    println!("block: {} (latest)", block.height.unwrap_or_default());
    let block = api
        .blocks_by_id("8df5934b603291ad30f906eec3bcbc8d88855c2fe4736805f2619fbfed67a4e4")
        .await?;
    println!("block: {}", block.height.unwrap_or_default());
    let txs = api.blocks_txs(&block.hash, Pagination::all()).await?;
    for hash in txs {
        let tx = api.transaction_by_hash(&hash).await?;
        let utxos = api
            .transactions_utxos(&tx.hash)
            .await?
            .outputs
            .into_iter()
            .filter(|utxo| utxo.address == address)
            .collect::<Vec<_>>();
        if !utxos.is_empty() {
            println!("TX: {hash}");
            println!("{utxos:#?}");
        }
    }
    
    Ok(())
}

fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    // let api = BlockfrostAPI::new("mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be", settings);
    let api = BlockfrostAPI::new("preprododflHjPhpRp4NzRFL1m9zzd6ZJb1RjYi", settings);
    Ok(api)
}
