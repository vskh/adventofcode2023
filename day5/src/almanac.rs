use std::ops::Range;
use crate::mapper::Mapper;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<Range<u64>>,
    maps: Vec<Mapper>,
}

impl Almanac {
    pub fn new(seeds: Vec<Range<u64>>, maps: Vec<Mapper>) -> Self {
        // pre-cache maps for seed to anything

        Almanac { seeds, maps }
    }

    pub fn try_find_location_for_seed(&mut self, seed: u64) -> Option<u64> {
        let map_target = "location";
        let mut curr_mapped_item = "seed";
        let mut curr_mapped_value = seed;

        while curr_mapped_item != map_target {
            if let Some(idx) = self.maps.iter().position(|m| m.source == curr_mapped_item) {
                curr_mapped_value = self.maps[idx].map(curr_mapped_value);
                curr_mapped_item = &self.maps[idx].destination;
            } else {
                return None;
            }
        }

        Some(curr_mapped_value)
    }
}

impl TryFrom<String> for Almanac {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        let [seeds_def, map_defs @ ..] = &value.split("\n\n").collect::<Vec<&str>>()[..] else {
            Err(anyhow!("Invalid structure of almanac definition: at least seeds definition is required: {}", value))?
        };

        let seeds = (*seeds_def)
            .trim()
            .strip_prefix("seeds: ")
            .ok_or(anyhow!("Invalid seeds definition: {}", *seeds_def))
            .and_then(|seed_nums_def| {
                seed_nums_def
                    .split_whitespace()
                    .map(|s| {
                        s.parse::<u64>()
                            .map_err(|e| anyhow!("Failed to parse seed: {}", e))
                    })
                    .collect::<Result<Vec<u64>>>()
            })?
            .windows(2)
            .map(|w| {
                if let &[start, length] = w {
                    Ok(start..(start + length))
                } else {
                    Err(anyhow!("Invalid seed ranges definition."))
                }
            })
            .collect::<Result<Vec<Range<u64>>>>()?;

        let mut maps: Vec<Mapper> = vec![];
        for map_def in map_defs {
            let [name_def, range_defs @ ..] = &(map_def.lines().collect::<Vec<&str>>())[..] else {
                Err(anyhow!("Invalid structure of the map definition: name and at least one range map is required: {}", map_def))?
            };

            let (src, dest) = name_def
                .trim()
                .strip_suffix(" map:")
                .ok_or(anyhow!("Invalid map definition: {}", name_def))
                .and_then(|name| {
                    let name_parts: Vec<&str> = name.split("-").collect();
                    match &(name_parts)[..] {
                        [s, _, d] => Ok((*s, *d)),
                        _ => Err(anyhow!("Invalid map name: {}", name)),
                    }
                })?;

            let mut range_maps: Vec<(u64, u64, u64)> = vec![];
            for range_def in range_defs {
                let [dst_start, src_start, length] = range_def
                    .split_whitespace()
                    .map(|el| {
                        el.parse::<u64>()
                            .map_err(|e| anyhow!("Failed to parse range definition element: {}", e))
                    })
                    .collect::<Result<Vec<u64>>>()?[..]
                else {
                    Err(anyhow!("Failed to parse range definition: {}", range_def))?
                };

                range_maps.push((dst_start, src_start, length));
            }

            maps.push(Mapper::new(src, dest, range_maps));
        }

        Ok(Almanac { seeds, maps })
    }
}

#[cfg(test)]
mod test {
    use crate::almanac::Almanac;

    #[test]
    fn almanac_tryfrom_happy_test() {
        let almanac_def = String::from(
            "seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48",
        );
        let almanac = Almanac::try_from(almanac_def);
        println!("Almanac: {:?}", almanac);
        assert!(matches!(almanac, Ok(_)));
    }
}
