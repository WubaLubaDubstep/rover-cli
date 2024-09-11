use crate::position::Position;
use crate::position::Direction;

pub struct Rover{
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

    pub fn move_rover(&mut self, command_string: &String){

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