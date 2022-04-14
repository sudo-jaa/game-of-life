use rand::Rng;
use std::time::{Duration, Instant};
use std::{collections::HashMap, fmt::Display};
use std::{thread, time};

fn main() {
    let mut world = World::new((60, 250));

    let millis = time::Duration::from_millis(30);

    println!("{}", world);
    loop {
        print!("{}[2J", 27 as char);
        world.tick().as_millis();
        println!("{}", world);
        thread::sleep(millis);
    }
}

/// Represents the world board, and contains the dimensions of the world,
/// A primary grid that contains the current state, a buffer grid that we write the next state to,
/// And a preinitialised empty grid that we can use to wipe a previous state to a clean state
struct World {
    dimensions: (usize, usize),
    grid: HashMap<Pos, bool>,
    buffer: HashMap<Pos, bool>,
    blank: HashMap<Pos, bool>,
}

impl World {
    /// Construct a new game board
    pub fn new(dimensions: (usize, usize)) -> World {
        let mut grid = HashMap::new();
        let mut buffer = HashMap::new();
        let mut blank = HashMap::new();

        let mut rng = rand::thread_rng();

        for x in 0..=dimensions.0 {
            for y in 0..=dimensions.1 {
                let alive = rng.gen_range(0..=5);
                grid.insert(Pos::new(x as isize, y as isize), alive == 5);
                buffer.insert(Pos::new(x as isize, y as isize), false);
                blank.insert(Pos::new(x as isize, y as isize), false);
            }
        }
        World {
            dimensions,
            grid,
            buffer,
            blank,
        }
    }

    /// Determine is the cell at a given position is alive
    fn is_alive(&self, pos: &Pos) -> bool {
        let item = self.grid.get(pos);
        match item {
            Some(cell) => *cell,
            None => false,
        }
    }

    /// Advance the board to the next state
    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        for x in 0..=self.dimensions.0 {
            for y in 0..=self.dimensions.1 {
                let next = self.next_cell_state(&Pos::new(x as isize, y as isize));
                self.buffer.insert(Pos::new(x as isize, y as isize), next);
            }
        }

        // TODO there is a better way of moving the buffer state to the real state, but I can't be bothered right now
        self.grid = self.buffer.clone();
        self.blank = self.blank.clone();
        now.elapsed()
    }

    /// Find the number of neighbors of this position that are alive
    fn num_living_neighbours(&self, pos: &Pos) -> usize {
        let neighbors = pos.neighbours();
        neighbors
            .iter()
            .filter_map(|n_pos| self.grid.get(n_pos))
            .filter(|x| **x)
            .count()
    }

    /// Find the next state of a given position on the board,
    /// depending on the game of life rules
    fn next_cell_state(&self, pos: &Pos) -> bool {
        let cell_state = self.is_alive(pos);
        let living_neighbors = self.num_living_neighbours(pos);
        match cell_state {
            true => {
                if living_neighbors < 2 {
                    false
                } else if living_neighbors == 2 || living_neighbors == 3 {
                    true
                } else {
                    false
                }
            }
            false => {
                if living_neighbors == 3 {
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = vec![];
        for x in 0..=self.dimensions.0 {
            let mut row = vec![];
            for y in 0..=self.dimensions.1 {
                let alive = self.is_alive(&Pos::new(x as isize, y as isize));
                row.push(if alive { '■' } else { ' ' });
                if y == self.dimensions.1 {
                    rows.push(row.clone())
                }
            }
        }
        rows.into_iter().for_each(|row| {
            write!(f, "\n");
            row.into_iter().for_each(|cell| {
                write!(f, "{}", cell);
            });
        });
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
/// Represents a position in the world grid
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    /// Construct a new carteesian position
    pub fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
    }

    /// Find the 8 neighbor positions of this position
    /// Note: this does not find the aliveness state of the neighbors,
    /// but rather just the actual location references, and is not associated with a particular game board.
    pub fn neighbours(&self) -> [Pos; 8] {
        [
            Pos::new(self.x + 1, self.y + 1),
            Pos::new(self.x + 1, self.y - 1),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x - 1, self.y - 1),
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x, self.y - 1),
        ]
    }
}
