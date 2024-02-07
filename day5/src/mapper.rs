use std::{fmt::Display, ops::Range};

#[derive(Debug)]
pub struct Mapper {
    pub source: String,
    pub destination: String,
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl Mapper {
    pub fn new(source: &str, destination: &str, maps: Vec<(u64, u64, u64)>) -> Self {
        Mapper {
            source: source.to_string(),
            destination: destination.to_string(),
            mappings: maps
                .iter()
                .map(|&(dst_start, src_start, len)| {
                    (src_start..(src_start + len), dst_start..(dst_start + len))
                })
                .collect(),
        }
    }

    pub fn map(&mut self, src_id: u64) -> u64 {
        for mapping in self.mappings.iter_mut() {
            if mapping.0.contains(&src_id) {
                return mapping.1.start + (src_id - mapping.0.start);
            }
        }

        src_id
    }
}

impl Display for Mapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-to-{} map", self.source, self.destination)
    }
}

#[cfg(test)]
mod test {
    use super::Mapper;

    #[test]
    fn mapper_test() {
        let mut mapper = Mapper::new("seed", "soil", vec![(50, 98, 2), (52, 50, 48)]);

        assert_eq!(mapper.map(98), 50);
        assert_eq!(mapper.map(99), 51);
        assert_eq!(mapper.map(53), 55);
        assert_eq!(mapper.map(10), 10);
    }
}
