#[allow(unused_imports)]
use std::{fs::File, io::{BufWriter, Write}};
use ddcore_rs::models::spawnset::{Spawnset, V3Enemies};

fn main() -> anyhow::Result<()> {
    let spawnset_file = std::env::args().nth(1).unwrap();
    let mut spawnset_file = File::open(spawnset_file)?;
    let spawnset = Spawnset::<V3Enemies>::deserialize(&mut spawnset_file)?;
    println!("{:#?}", spawnset);
    // use this commented line to have an actual file writer and comment the md5 comp
    // let mut file_writer = BufWriter::new(File::create("spawnset_output")?);
    let mut file_writer = Vec::new();
    spawnset.serialize(&mut file_writer)?;
    file_writer.flush()?;
    println!("{:x}", md5::compute(&file_writer));
    Ok(())
}
