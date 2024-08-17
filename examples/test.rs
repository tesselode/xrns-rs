use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let song = xrns_rs::load("test.xrns")?;
    dbg!(song);
    Ok(())
}
