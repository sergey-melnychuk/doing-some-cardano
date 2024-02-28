use blockfrost::{BlockFrostSettings, BlockfrostAPI, BlockfrostResult, Pagination};

fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    // let api = BlockfrostAPI::new("mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be", settings);
    let api = BlockfrostAPI::new("preprododflHjPhpRp4NzRFL1m9zzd6ZJb1RjYi", settings);
    Ok(api)
}

// Scan block of transactions for Tx outputs to a given address.
#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    // https://github.com/blockfrost/blockfrost-rust/blob/v1.0.1/examples/fetch_all.rs
    let api = build_api()?;

    let address = "addr_test1vqx8mqt8wmyfccltrv5sqc7dx0fh30p8892yjz63af6zcvgszmek6";

    let utxos = api.addresses_utxos(address, Pagination::all()).await?;
    println!("{utxos:#?}");

    let block = api.blocks_latest().await?;
    println!("block: {} (latest)", block.height.unwrap_or_default());
    let block = api
        .blocks_by_id("8c20a129c5e033d2f2f44199928c84c716317fb63e13cc2ad2e8db410265530c")
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
