mod almanac;
mod mapper;

use crate::almanac::Almanac;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let almanac_def = fs::read_to_string("input.txt")?;

    let mut almanac: Almanac = almanac_def.try_into()?;

    let mut closest_location = u64::MAX;
    let mut seed_range_idx = 0;
    while seed_range_idx < almanac.seeds.len() {
        let seed_range = &almanac.seeds[seed_range_idx];
        println!("Processing seed range #{}: {:?}", seed_range_idx, seed_range);

        for seed in seed_range.clone() {
            if let Some(loc) = almanac.try_find_location_for_seed(seed) {
                if loc < closest_location {
                    closest_location = loc;

                    println!("Found closest location {} for seed {}.", loc, seed);
                }
            }
        }

        seed_range_idx += 1;
    }

    println!("Closest location for initial seeds is {}", closest_location);

    Ok(())
}
