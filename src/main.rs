use clap::Parser;
use app::rover;
use app::position;

#[derive(Parser)]
struct Cli{
    command: String,
    args: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    let mut rover = rover::Rover::new();

    if args.command == "set"{
        rover.set_position(position::Position::new_from_args(args.args.clone()));
    }
    
    if args.command == "move"{
        rover.move_rover(&args.args[0]);
    }
    println!("{:?}", rover.to_string());
}