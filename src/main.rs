use clap::Parser;

#[derive(Parser)]
struct Cli{
    command: String,
    args: Vec<String>,
}

enum Direction {
    N,
    E,
    S,
    W
}

impl Direction {
    pub fn index(&self) -> usize {
        return *self as usize
    }

    pub fn rotate_left(&self) -> Option<Direction>{
        let i = *self-1 as usize;
        if(i < 0){
            return *self+4 as Option<Direction>
        }
        return i as Option<Direction>;
    }

    pub fn rotate_right(&self) -> Option<Direction>{
        let i = *self+1 as usize;
        if(i > 4){
            return *self-4 as Option<Direction>
        }
        return i as Option<Direction>;
    }
}

struct Position{
    x: i32,
    y: i32,
    direction: Direction
}

struct Rover{
    position: Position
}

impl Rover{
    pub fn new() -> Rover{
        return Rover{position: Position{x: 0, y: 0, direction: Direction::N}}
    }

    pub fn get_position(&self) -> Position{
        return self.position;
    }

    pub fn set_position(&mut self, position: Position){
        self.position = position;
    }

    fn move_rover(&mut self, command_string: &String){

        for c in command_string.chars(){
            match c{
                "M" => {
                    match rover.position.direction{
                        Direction::N => {self.position.y += 1},
                        Direction::E => {self.position.x += 1},
                        Direction::S => {self.position.y -= 1},
                        Direction::W => {self.position.x -= 1}
                    }
                },
                "L" => {self.position = Direction::rotate_left(&self.position.direction)},
                "R" => {self.position = Direction::rotate_right(&self.position.direction)},
                _ => {}
            }
        }
        
    }
}

fn main() {
    let args = Cli::parse();
    
    println!("{:?}", args.command);
}
