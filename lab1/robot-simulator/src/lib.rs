// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    lat: i32,
    long: i32,
    direzione: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self{
            lat: x,
            long: y,
            direzione : d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let returnrobot = match self.direzione{
            Direction::North =>  Robot::new(self.lat,self.long,Direction::East),//self.direzione = Direction::East,
            Direction::East =>  Robot::new(self.lat,self.long,Direction::South),//self.direzione = Direction::South,
            Direction::South => Robot::new(self.lat,self.long,Direction::West),//self.direzione = Direction::West,
            Direction::West => Robot::new(self.lat,self.long,Direction::North),//self.direzione = Direction::North,
        };
        return returnrobot;
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let returnrobot = match self.direzione{
            Direction::North =>  Robot::new(self.lat,self.long,Direction::West),//self.direzione = Direction::East,
            Direction::East =>  Robot::new(self.lat,self.long,Direction::North),//self.direzione = Direction::South,
            Direction::South => Robot::new(self.lat,self.long,Direction::East),//self.direzione = Direction::West,
            Direction::West => Robot::new(self.lat,self.long,Direction::South),//self.direzione = Direction::North,
        };
        return returnrobot;
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let returnrobot = match self.direzione{
            Direction::North => Robot::new(self.lat,self.long +1,self.direzione),//self.long += 1,
            Direction::East => Robot::new(self.lat+1,self.long,self.direzione),//self.lat += 1,
            Direction::South => Robot::new(self.lat,self.long -1,self.direzione),//self.long += -1,
            Direction::West => Robot::new(self.lat-1,self.long ,self.direzione),//self.lat += -1,
        };
        return returnrobot;
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = Robot::new(self.lat,self.long,self.direzione);
        for c in instructions.chars(){
            match c{
                'R' => robot = robot.turn_right(),
                'L' => robot = robot.turn_left(),
                'A' => robot = robot.advance(),
                _ => robot= robot,
            };
        }
        return robot;
    }

    pub fn position(&self) -> (i32, i32) {
        return (self.lat,self.long);
    }

    pub fn direction(&self) -> &Direction {
        return &self.direzione;
    }
}
