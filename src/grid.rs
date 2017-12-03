use std::ops::{Index, IndexMut};
use std::iter;

pub struct Grid {
    rows: Vec<Vec<Option<u32>>>,
    mid_index: (usize, usize),
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let mut rows = Vec::<Vec<Option<u32>>>::with_capacity(size);

        for _ in 0..size {
            let mut row = Vec::<Option<u32>>::new();
            row.extend(iter::repeat(None).take(size));
            rows.push(row);
        }
        let mid = size / 2;

        Grid {
            rows: rows,
            mid_index: (mid, mid),
        }
    }

    fn index_origin(&self, index: (i32, i32)) -> (usize, usize) {
        let corrected_x = (self.mid_index.0 as i32 + index.0) as usize;
        let corrected_y = (self.mid_index.1 as i32 + index.1) as usize;
        (corrected_x, corrected_y)
    }

    pub fn sum_of_neighbours(&self, index: (i32, i32)) -> u32 {
        let actual_index = self.index_origin(index);
        let neighbours: [(i32, i32); 8] = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1),
                                           (0, -1), (1, -1)];


        println!("Neighbour sum for: {:?}", index);
        neighbours.iter().fold(0, |acc, &(x, y)| {
            let neighbour_index = ((actual_index.0 as i32) + x, (actual_index.1 as i32) + y);
            if neighbour_index.0 >= self.rows.len() as i32 || neighbour_index.0 < 0 {
                return acc;
            }
            let row = &self.rows[neighbour_index.0 as usize];

            if neighbour_index.1 >= row.len() as i32 || neighbour_index.1 < 0 {
                return acc;
            }

            println!("{} from {:?}",
                     row[neighbour_index.1 as usize].unwrap_or(0),
                     neighbour_index);
            acc + row[neighbour_index.1 as usize].unwrap_or(0)
        })
    }
}

impl Index<(i32, i32)> for Grid {
    type Output = Option<u32>;

    fn index(&self, index: (i32, i32)) -> &Option<u32> {
        let actual_index = self.index_origin(index);
        &self.rows[actual_index.0][actual_index.1]
    }
}

impl IndexMut<(i32, i32)> for Grid {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Option<u32> {
        let actual_index = self.index_origin(index);
        assert!(actual_index.0 < self.rows.len() && actual_index.1 < self.rows[0].len(),
                "Location: {:?} is out of bounds",
                actual_index);
        &mut self.rows[actual_index.0][actual_index.1]
    }
}
