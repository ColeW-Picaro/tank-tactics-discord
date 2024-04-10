mod server;

use server::{Position, Tank, Team};

fn main() {
    let tank: Tank = Tank::new(
        3,
        1,
        Team::Blue,
        2,
        Position { x: 0, y: 0 },
        String::from("Test"),
        1,
    );
    println!("{:?}", tank);
}
