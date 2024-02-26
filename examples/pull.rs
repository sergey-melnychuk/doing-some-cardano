use blockfrost::{BlockFrostSettings, BlockfrostAPI, BlockfrostResult, Pagination};

fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    let api = BlockfrostAPI::new("mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be", settings);
    Ok(api)
}

// https://github.com/blockfrost/blockfrost-rust/blob/v1.0.1/examples/fetch_all.rs

#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    let api = build_api()?;

    let address = "addr1q94ar6cqj4dh7qsapjqwh52sd0x035n37z7ds9gntsc94rl7x4sv002yhxau33ujw232gzlwre6jac2ny9ywj93942eqx2sjuc";

    // let block = api.blocks_latest().await?;
    // println!("block: {}", block.height.unwrap_or_default());
    let block = api.blocks_by_id("9970549").await?;
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
