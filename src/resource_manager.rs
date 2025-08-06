use std::{fs::{self, File}, io::{BufWriter, Write}, path::Path};
use anyhow::Result;

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
    let region = bincode::decode_from_std_read(&mut file, bincode::config::standard())?;
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