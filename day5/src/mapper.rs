use std::collections::HashSet;
use std::ops::Add;
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

    pub fn src_breakpoints(&self) -> HashSet<u64> {
        let mut result = HashSet::new();

        for (src, _dst) in self.mappings.iter() {
            result.insert(src.start);
            result.insert(src.end);
        }

        result
    }

    pub fn dest_breakpoints(&self) -> HashSet<u64> {
        let mut result = HashSet::new();

        for (_src, dst) in self.mappings.iter() {
            result.insert(dst.start);
            result.insert(dst.end);
        }

        result
    }
}

impl Display for Mapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-to-{} map", self.source, self.destination)
    }
}

impl Add for &mut Mapper {
    type Output = Option<Mapper>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.destination != rhs.source {
            None
        } else {
            let mut breakpoints: Vec<u64> = (&(&self.src_breakpoints() | &self.dest_breakpoints())
                | &rhs.src_breakpoints())
                .into_iter()
                .collect();
            breakpoints.sort();

            let mut result_maps: Vec<(u64, u64, u64)> = vec![];
            for w in breakpoints.windows(2) {
                if let &[src_start, src_end] = w {
                    let dst_start = rhs.map(self.map(src_start));
                    let len = src_end - src_start;

                    result_maps.push((dst_start, src_start, len));
                } else {
                    return None;
                }
            }

            Some(Mapper::new(&self.source, &rhs.destination, result_maps))
        }
    }
}

#[cfg(test)]
mod test {
    use super::Mapper;
    use std::collections::HashSet;

    #[test]
    fn mapper_map_test() {
        let mut mapper = Mapper::new("seed", "soil", vec![(50, 98, 2), (52, 50, 48)]);

        assert_eq!(mapper.map(98), 50);
        assert_eq!(mapper.map(99), 51);
        assert_eq!(mapper.map(53), 55);
        assert_eq!(mapper.map(10), 10);
    }

    #[test]
    fn mapper_breakpoints_test() {
        let mapper = Mapper::new("seed", "soil", vec![(50, 98, 2), (52, 50, 48)]);

        assert_eq!(mapper.src_breakpoints(), HashSet::from([50, 98, 100]));
    }

    #[test]
    fn mapper_add_mapper_src_dest_success_test() {
        let mut m1 = Mapper::new("s", "d", vec![]);
        let mut m2 = Mapper::new("d", "d2", vec![]);

        let res = &mut m1 + &mut m2;

        assert!(
            matches!(res, Some(Mapper { source, destination, .. }) if source == m1.source && destination == m2.destination)
        );
    }

    #[test]
    fn mapper_add_mapper_src_dest_fail_test() {
        let mut m1 = Mapper::new("s", "d", vec![]);
        let mut m2 = Mapper::new("s2", "d2", vec![]);

        let res = &mut m1 + &mut m2;

        assert!(matches!(res, None));
    }

    #[test]
    fn mapper_add_remap_test() {
        let mut m1 = Mapper::new("s", "d", vec![(5, 3, 2), (3, 5, 2)]);
        println!("m1 breakpoints = {:?}", m1.src_breakpoints());
        let mut m2 = Mapper::new("d", "d2", vec![(3, 2, 1), (2, 3, 1)]);
        println!("m2 breakpoints = {:?}", m2.src_breakpoints());

        let res = &mut m1 + &mut m2;

        println!("res maps: {:?}", res);

        assert!(
            matches!(res, Some(Mapper { mappings, .. }) if mappings == vec![(2..3, 3..4), (3..5, 5..7), (5..6, 2..3), (6..7, 4..5)])
        );
    }
}
