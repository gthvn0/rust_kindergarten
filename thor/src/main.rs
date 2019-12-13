// For a given position found the correct path to the given destination
//
// map is from upper left (0, 0) -> bottom right (40, 18)
// Direction are give by N NE E SE S SW W or NW
//           x
// (0,0)   +--->-----------------+ (40, 0)
//       y |                     |
//         v                     |
//         |                     |
// (0,18)  +---------------------+ (40, 18)
//

#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: i8,
    y: i8,
}

const N: Coord  = Coord {x: 0, y:-1};
const NE: Coord = Coord {x: 1, y: -1};
const E: Coord  = Coord {x: 1, y: 0};
const SE: Coord = Coord {x: 1, y: 1};
const S: Coord  = Coord {x: 0, y: 1};
const SW: Coord = Coord {x: -1, y: 1};
const W: Coord  = Coord {x: -1, y: 0};
const NW: Coord = Coord {x: -1, y: -1};

impl Coord {
    fn add(&self, pt: &Coord) -> Coord {
        Coord {x: self.x + pt.x, y: self.y + pt.y}
    }

    fn move_to(&self, dest: &str) -> Coord {
        match dest {
            "N" =>  self.add(&N),
            "NE" => self.add(&NE),
            "E" =>  self.add(&E),
            "SE" => self.add(&SE),
            "S" =>  self.add(&S),
            "SW" => self.add(&SW),
            "W" =>  self.add(&W),
            "NW" => self.add(&NW),
            _ => panic!("PANIC: you are heading to a black hole")
        }
    }

    fn direction_to(&self, dest: &Coord) -> String {
        let pt = Coord {x:compare(self.x, dest.x), y: compare(self.y, dest.y)};
        match pt {
            N =>  String::from("N"),
            NE => String::from("NE"),
            E =>  String::from("E"),
            SE => String::from("SE"),
            S =>  String::from("S"),
            SW => String::from("SW"),
            W =>  String::from("W"),
            NW => String::from("NW"),
            _ =>  String::from("X"), // Don't move, you reached the destination
        }
    }

}

fn compare(x: i8, y: i8) -> i8 {
    if x < y { return 1; }
    if x > y { return -1; }
    0
}

fn compute_journey(start: &Coord, dest: &Coord) -> Vec<String> {
    let mut thor = Coord {x:start.x, y: start.y};
    let mut journey = Vec::new();

        loop {
            let direction = thor.direction_to(&dest);
            if direction == "X" {
                break;
            }
            thor = thor.move_to(&direction);
            journey.push(direction); // journey own direction now
        }

        journey
}

fn main() {
    let thor  = Coord {x:5, y: 4};
    let light = Coord {x:31, y: 12};

    println!("Start at {:?}", thor);
    println!("Goto {:?}", light);

    let journey = compute_journey(&thor, &light);
    println!("It took {} steps to reach the light", journey.len());
    println!("journey: {:?}", journey);
    // To replay the journey just reverse the vec and pop until empty
}


#[test]
fn test_all_destinations() {
    let start = Coord {x:5, y:5};

    assert_eq!(start.direction_to(&Coord{x:5,y:5}), "X");
    assert_eq!(start.direction_to(&Coord{x:5,y:2}), "N");
    assert_eq!(start.direction_to(&Coord{x:7,y:2}), "NE");
    assert_eq!(start.direction_to(&Coord{x:6,y:2}), "NE");
    assert_eq!(start.direction_to(&Coord{x:8,y:5}), "E");
    assert_eq!(start.direction_to(&Coord{x:7,y:9}), "SE");
    assert_eq!(start.direction_to(&Coord{x:5,y:9}), "S");
    assert_eq!(start.direction_to(&Coord{x:3,y:9}), "SW");
    assert_eq!(start.direction_to(&Coord{x:3,y:5}), "W");
    assert_eq!(start.direction_to(&Coord{x:3,y:2}), "NW");
}
