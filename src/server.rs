use std::fmt::Display;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Team {
    Red,
    Blue,
    Green,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Tank {
    pub life: u8,
    pub action_points: u8,
    pub team: Team,
    pub range: u8,
    pub position: Position,
    pub name: String,
    pub id: u32,
}

pub struct Board {
    pub length: u32,
    pub height: u32,
}

pub struct Game {
    board: Board,
    tanks: Vec<Tank>,
}

impl Tank {
    pub fn new(
        life: u8,
        action_points: u8,
        team: Team,
        range: u8,
        position: Position,
        name: String,
        id: u32,
    ) -> Self {
        Self {
            life,
            action_points,
            team,
            range,
            position,
            name,
            id,
        }
    }
}

impl Game {
    pub fn make_move(&mut self, tank_id: u32, direction: Direction) -> Result<Position, String> {
        let tanks = self.tanks.clone();
        let tank_option = self.tanks.iter_mut().find(|x| x.id == tank_id);
        if let Some(tank) = tank_option {
            let err = Err(format!(
                "Illegal to move {} from {}",
                direction, tank.position
            ));
            let dest_position = match direction {
                Direction::Right => match tank.position.x + 1 < self.board.length {
                    true => Position {
                        x: tank.position.x + 1,
                        y: tank.position.y,
                    },
                    false => return err,
                },
                Direction::Up => match tank.position.y.checked_sub(1) {
                    Some(y) => Position {
                        x: tank.position.x,
                        y,
                    },
                    None => return err,
                },
                Direction::Down => match tank.position.y + 1 < self.board.height {
                    true => Position {
                        x: tank.position.x,
                        y: tank.position.y + 1,
                    },
                    false => return err,
                },
                Direction::Left => match tank.position.x.checked_sub(1) {
                    Some(x) => Position {
                        x,
                        y: tank.position.y,
                    },
                    None => return err,
                },
            };
            if let Some(_) = tanks.iter().find(|x| x.position == dest_position) {
                return err;
            } else {
                tank.position = dest_position;
            }
            return Ok(tank.position);
        } else {
            return Err(String::from("Tank id not found"));
        }
    }

    pub fn get_tank(&self, tank_id: u32) -> Option<&Tank> {
        self.tanks.iter().find(|x| x.id == tank_id)
    }

    pub fn get_tank_mut(&mut self, tank_id: u32) -> Option<&mut Tank> {
        self.tanks.iter_mut().find(|x| x.id == tank_id)
    }

    pub fn set_tank_position(&mut self, tank_id: u32, position: Position) {
        let l = self.board.length;
        let h = self.board.height;
        match self.get_tank_mut(tank_id) {
            Some(tank) => {
                if position.x < l && position.y < h {
                    tank.position = position;
                }
            }
            None => todo!(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn tank_new() {
        let tank: Tank = Tank::new(
            3,
            1,
            Team::Blue,
            2,
            Position { x: 0, y: 0 },
            String::from("Test"),
            1,
        );

        assert_eq!(tank.life, 3);
        assert_eq!(tank.action_points, 1);
        assert_eq!(tank.team, Team::Blue);
        assert_eq!(tank.range, 2);
        assert_eq!(tank.position, Position { x: 0, y: 0 });
        assert_eq!(tank.name, "Test");
    }

    #[test]
    fn tank_make_move_legal_sets_position() {
        let board = Board {
            length: 8,
            height: 8,
        };
        let tank = Tank::new(
            3,
            1,
            Team::Blue,
            2,
            Position { x: 0, y: 0 },
            String::from("Test"),
            1,
        );

        let mut game = Game {
            board,
            tanks: vec![tank],
        };

        let position = game.make_move(1, Direction::Right).unwrap();
        assert_eq!(game.get_tank(1).unwrap().position, Position { x: 1, y: 0 });
        assert_eq!(position, Position { x: 1, y: 0 });

        let position = game.make_move(1, Direction::Down).unwrap();
        assert_eq!(game.get_tank(1).unwrap().position, Position { x: 1, y: 1 });
        assert_eq!(position, Position { x: 1, y: 1 });

        let position = game.make_move(1, Direction::Left).unwrap();
        assert_eq!(game.get_tank(1).unwrap().position, Position { x: 0, y: 1 });
        assert_eq!(position, Position { x: 0, y: 1 });

        let position = game.make_move(1, Direction::Up).unwrap();
        assert_eq!(game.get_tank(1).unwrap().position, Position { x: 0, y: 0 });
        assert_eq!(position, Position { x: 0, y: 0 });
    }

    #[test]
    fn tank_make_move_illegal_board_edge_expects_error() {
        let board = Board {
            length: 8,
            height: 8,
        };
        let mut tank = Tank::new(
            3,
            1,
            Team::Blue,
            2,
            Position { x: 0, y: 0 },
            String::from("Test"),
            1,
        );

        let mut game = Game {
            board,
            tanks: vec![tank],
        };

        game.make_move(1, Direction::Left)
            .expect_err("Illegal to move Left from 0,0");
        assert_eq!(game.get_tank(1).unwrap().position, Position { x: 0, y: 0 });

        game.set_tank_position(1, Position { x: 7, y: 0 });
        game.make_move(1, Direction::Right)
            .expect_err("Illegal to move Right from 7,0");
        assert_eq!(game.get_tank(1).unwrap().position, Position { x: 7, y: 0 });

        game.set_tank_position(1, Position { x: 0, y: 7 });
        game.make_move(1, Direction::Down)
            .expect_err("Illegal to move Down from 0,7");
        assert_eq!(game.get_tank(1).unwrap().position, Position { x: 0, y: 7 });

        game.set_tank_position(1, Position { x: 0, y: 0 });
        game.make_move(1, Direction::Up)
            .expect_err("Illegal to move Up from 0,0");
        assert_eq!(game.get_tank(1).unwrap().position, Position { x: 0, y: 0 });
    }

    #[test]
    fn tank_make_move_illegal_tank_collision_expects_error() {
        let board = Board {
            length: 8,
            height: 8,
        };

        let tank1 = Tank::new(
            3,
            1,
            Team::Blue,
            2,
            Position { x: 0, y: 0 },
            String::from("Test"),
            1,
        );

        let tank2 = Tank::new(
            3,
            1,
            Team::Blue,
            2,
            Position { x: 0, y: 1 },
            String::from("Test"),
            2,
        );

        let mut game = Game {
            board,
            tanks: vec![tank1, tank2],
        };

        game.make_move(1, Direction::Down).expect_err("");
    }
}
