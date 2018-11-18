#[derive(Debug)]
struct List {
    storage: Vec<u8>,
    size: usize,
    current: usize,
    skip_offset: usize,
}

impl List {
    fn new(size: usize) -> Self {
        assert!(size >= 2, "Size must be greater than 2");
        List {
            storage: (0..(size as u16)).map(|v| v as u8).collect(),
            size,
            current: 0,
            skip_offset: 0,
        }
    }

    fn apply_length(self, length: usize) -> List {
        assert!(
            length <= self.size,
            "Length must not be larger than the list size"
        );

        let mut new_storage = self.storage.clone();
        let reverse_range = (self.current..self.current + length)
            .map(|index| index % self.size)
            .collect::<Vec<usize>>();
        let reverse_range_reversed = reverse_range.clone().into_iter().rev();
        let range = reverse_range.into_iter().zip(reverse_range_reversed);

        for (i1, i2) in range {
            new_storage[i1] = self.storage[i2];
        }

        List {
            storage: new_storage,
            current: (self.current + length + self.skip_offset) % self.size,
            size: self.size,
            skip_offset: self.skip_offset + 1,
        }
    }

    fn head_product(&self) -> u16 {
        (self.storage[0] as u16) * (self.storage[1] as u16)
    }

    fn dense_hash(&self) -> String {
        assert!(
            self.size % 16 == 0,
            "Cannot create the dense hash of a list that is not a multiple of 16 in length"
        );
        let num_blocks = self.size / 16;

        (0..num_blocks)
            .map(|i| {
                (0..16)
                    .map(|ii| i * 16 + ii)
                    .fold(0 as u8, |acc, index| acc ^ self.storage[index])
            }).map(|c| format!("{:01$x}", c, 2))
            .fold(String::new(), |acc, character| acc + &character)
    }
}

pub fn knot_hash(input: Vec<usize>, size: usize, rounds: usize) -> String {
    let suffix = [17, 31, 73, 47, 23];
    let mut lengths = input.clone();
    lengths.extend(&suffix);
    let mut list = List::new(size);

    for _ in (0..rounds) {
        for length in lengths.iter() {
            list = list.apply_length(*length);
        }
    }

    list.dense_hash()
}

pub fn solve(input: &str, size: usize) -> u16 {
    let lengths = input.trim().split(',').map(|v| {
        v.trim()
            .parse::<usize>()
            .expect(&format!("Expected only valid numbers found {}", v))
    });
    let mut list = List::new(size);

    for length in lengths {
        list = list.apply_length(length);
    }

    list.head_product()
}

pub fn solve2(input: &str, size: usize, rounds: usize) -> String {
    let mut lengths = input
        .trim()
        .chars()
        .map(|c| c as u8)
        .map(|c| c as usize)
        .collect::<Vec<usize>>();

    knot_hash(lengths, size, rounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_star_one() {
        assert_eq!(solve("3, 4, 1, 5", 5), 12);
    }

    #[test]
    fn test_cases_star_two() {
        assert_eq!(solve2("1,2,3", 256, 64), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(solve2("", 256, 64), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(
            solve2("AoC 2017", 256, 64),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(solve2("1,2,4", 256, 64), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
