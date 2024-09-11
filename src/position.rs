#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn from_index(i: usize) -> Direction {
        match i {
            0 => Direction::N,
            1 => Direction::E,
            2 => Direction::S,
            3 => Direction::W,
            _ => panic!("Invalid index"),
        }
    }

    pub fn rotate_left(&self) -> Direction {
        let i = self.index();
        let new_index = (i + 3) % 4; // Rotate left is like subtracting 1, but we add 3 and mod
        Direction::from_index(new_index)
    }

    pub fn rotate_right(&self) -> Direction {
        let i = self.index();
        let new_index = (i + 1) % 4; // Rotate right is like adding 1 and mod
        Direction::from_index(new_index)
    }

    pub fn to_string(&self) -> String {
        match self {
            Direction::N => "N".to_string(),
            Direction::E => "E".to_string(),
            Direction::S => "S".to_string(),
            Direction::W => "W".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position{
    pub x: i32,
    pub y: i32,
    pub direction: Direction
}

impl Position{
    pub fn new_from_args(args : Vec<String>) -> Position{
        if args.len() < 3 {
            eprintln!("Error: Not enough arguments provided for 'set' command.");
            std::process::exit(1);
        }

        let x = args[0].parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Error: Invalid x coordinate.");
            std::process::exit(1);
        });
        let y = args[1].parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Error: Invalid y coordinate.");
            std::process::exit(1);
        });
        let direction = match args[2].as_str() {
            "N" => Direction::N,
            "E" => Direction::E,
            "S" => Direction::S,
            "W" => Direction::W,
            _ => {
                eprintln!("Error: Invalid direction.");
                std::process::exit(1);
            }
        };
        return Position{x: x, y: y, direction: direction}
    }
}