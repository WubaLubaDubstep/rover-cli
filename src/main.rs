use clap::Parser;

#[derive(Parser)]
struct Cli{
    command: String,
    args: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
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
struct Position{
    x: i32,
    y: i32,
    direction: Direction
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

struct Rover{
    position: Position
}

impl Rover{
    pub fn new() -> Rover{
        return Rover{position: Position{x: 0, y: 0, direction: Direction::N}}
    }

    pub fn get_position(&self) -> Position{
        return self.position.clone();
    }

    pub fn set_position(&mut self, position: Position){
        self.position = position;
    }

    fn move_rover(&mut self, command_string: &String){

        for c in command_string.chars(){
            match c{
                'M' => {
                    match self.position.direction{
                        Direction::N => {self.position.y += 1},
                        Direction::E => {self.position.x += 1},
                        Direction::S => {self.position.y -= 1},
                        Direction::W => {self.position.x -= 1}
                    }
                },
                'L' => {self.position.direction = Direction::rotate_left(&self.position.direction)},
                'R' => {self.position.direction = Direction::rotate_right(&self.position.direction)},
                _ => {}
            }
        }
    }

    pub fn to_string(self) -> String{
        return format!("{} {} {}", self.position.x, self.position.y, self.position.direction.to_string());
    }
}

fn main() {
    let args = Cli::parse();
    let mut rover = Rover::new();

    if args.command == "set"{
        rover.set_position(Position::new_from_args(args.args.clone()));
    }
    
    if args.command == "move"{
        rover.move_rover(&args.args[0]);
    }
    println!("{:?}", rover.to_string());
}