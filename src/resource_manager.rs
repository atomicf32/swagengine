use std::{fs::{self, File}, io::{BufWriter, BufReader, Write, Read}, path::Path};
use anyhow::Result;
use tar::{Builder, Archive};
use zstd::stream::{Encoder, Decoder};
use crate::world::Region;

pub fn save_region (region: Region) -> Result<()> {
    fs::create_dir_all("regions")?;
    let file_path = format!("regions/{},{}.bin", &region.get_position().0, &region.get_position().1);
    let file = File::create(Path::new(&file_path))?;
    let mut writer = BufWriter::new(file);
    bincode::encode_into_std_write(&region, &mut writer, bincode::config::standard())?;
    writer.flush()?;
    println!("Region {},{} saved", region.get_position().0, region.get_position().1);
    Ok(())
}

pub fn load_region (position: (i32, i32)) -> Result<Region> {
    let mut file = File::open(format!("regions/{},{}.bin", position.0, position.1))?;
    let mut reader = BufReader::new(file);
    let region = bincode::decode_from_std_read(&mut reader, bincode::config::standard())?;
    Ok(region) 
}

pub fn load_regions (position: (i32, i32)) -> Vec<Region> {
    let (x, y) = position;
    let mut regions = Vec::with_capacity(9);

    for dy in -1..=1 {
        for dx in -1..=1 {
            let pos = (x + dx, y + dy);
            regions.push(load_region(pos).unwrap());
        }
    }

    regions
}

pub fn compress_all_regions() -> Result<()> {
    let encoder = Encoder::new(BufWriter::new(File::create("regions.tar.zst")?), 1);
    let mut tar = Builder::new(encoder.unwrap().auto_finish());
    tar.append_dir_all(".", "regions")?;
    tar.finish()?;
    Ok(())
}

pub fn decompress_all_regions() -> Result<()> {
    let decoder = Decoder::new(BufReader::new(File::open("regions.tar.zst")?));
    let mut archive = Archive::new(decoder.unwrap());
    archive.unpack("regions")?;
    Ok(())
}