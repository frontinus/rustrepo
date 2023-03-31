use clap::Parser;
// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(Parser,Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   x: i32,

   #[arg(short, long)]
    y: i32,

    #[arg(short, long)]
    dir: char,

    #[arg(short, long)]
    instruct: String,
}

#[derive(PartialEq, Eq, Debug,Clone)]
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

const DIREZIONI: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

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
        //let returnrobot =
        let new_direction = (self.direzione as usize + 1) % DIREZIONI.len();
        Robot::new(self.lat, self.long, DIREZIONI[new_direction].clone())
        /*match self.direzione{
            Direction::North =>  Robot::new(self.lat,self.long,Direction::East),//self.direzione = Direction::East,
            Direction::East =>  Robot::new(self.lat,self.long,Direction::South),//self.direzione = Direction::South,
            Direction::South => Robot::new(self.lat,self.long,Direction::West),//self.direzione = Direction::West,
            Direction::West => Robot::new(self.lat,self.long,Direction::North),//self.direzione = Direction::North,
        }*/
        //return returnrobot;
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        //let returnrobot = 
        let new_direction = (self.direzione as usize + DIREZIONI.len() - 1) % DIREZIONI.len();
        Robot::new(self.lat, self.long, DIREZIONI[new_direction].clone())
        /*match self.direzione{
        

            Direction::North =>  Robot::new(self.lat,self.long,Direction::West),//self.direzione = Direction::East,
            Direction::East =>  Robot::new(self.lat,self.long,Direction::North),//self.direzione = Direction::South,
            Direction::South => Robot::new(self.lat,self.long,Direction::East),//self.direzione = Direction::West,
            Direction::West => Robot::new(self.lat,self.long,Direction::South),//self.direzione = Direction::North,
        }*/
        //return returnrobot;
    }

    #[must_use]
    pub fn advance(self) -> Self {
        //let returnrobot = 
        let (lat, long) = match self.direzione {
            Direction::North => (self.lat, self.long + 1),
            Direction::East => (self.lat + 1, self.long),
            Direction::South => (self.lat, self.long - 1),
            Direction::West => (self.lat - 1, self.long),
        };
        Robot::new(lat, long, self.direzione)
        //return returnrobot;
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = Robot::new(self.lat,self.long,self.direzione);
        for c in instructions.chars(){
            match c{
                'R' => robot = robot.turn_right(),
                'L' => robot = robot.turn_left(),
                'A' => robot = robot.advance(),
                _ => println!("erroreerrore"),
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.lat,self.long)
    }

    pub fn direction(&self) -> &Direction {
        &self.direzione
    }
}




fn main(){
    let argomenti: Args = Args::parse();
    let direzione:Direction = match argomenti.dir{
        'N' => Direction::North,
        'S' => Direction::South,
        'E' => Direction::East,
        'W' => Direction::West,
        _ => Direction::North,
    };
    let mut robo1:Robot = Robot::new(argomenti.x,argomenti.y,direzione);
    let istru: String = argomenti.instruct;
    robo1 = robo1.instructions(&istru);
    match robo1.direction(){
        Direction::East => println!("{} {} E",robo1.position().0,robo1.position().1),//println!(" East"),
        Direction::North => println!("{} {} N",robo1.position().0,robo1.position().1),//println!( " North"),
        Direction::West => println!("{} {} W",robo1.position().0,robo1.position().1),//println!(" West"),
        Direction::South => println!("{} {} S",robo1.position().0,robo1.position().1),//println!(" South"),
    }

}

 