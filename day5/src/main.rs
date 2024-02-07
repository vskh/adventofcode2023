mod almanac;
mod mapper;

use crate::almanac::Almanac;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let almanac_def = fs::read_to_string("input.txt")?;

    let mut almanac: Almanac = almanac_def.try_into()?;

    let mut closest_location = u64::MAX;
    let mut seed_idx = 0;
    while seed_idx < almanac.seeds.len() {
        let seed = almanac.seeds[seed_idx];

        if let Some(loc) = almanac.try_find_location_for_seed(seed) {
            if loc < closest_location {
                closest_location = loc;
            }
        }

        seed_idx += 1;
    }

    println!("Closest location for initial seeds is {}", closest_location);

    Ok(())
}
