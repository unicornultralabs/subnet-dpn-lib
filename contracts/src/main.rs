use ethers::prelude::Abigen;
use eyre::Result;

fn main() -> Result<()> {
    let dpn_abi = include_str!("DPN.json");

    Abigen::new("DPN", dpn_abi)?
        .generate()?
        .write_to_file("src/dpn.rs")?;

    Ok(())
}
