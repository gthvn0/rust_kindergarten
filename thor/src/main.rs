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
struct Point {
    x: i8,
    y: i8,
}

const N: Point  = Point {x: 0, y:-1};
const NE: Point = Point {x: 1, y: -1};
const E: Point  = Point {x: 1, y: 0};
const SE: Point = Point {x: 1, y: 1};
const S: Point  = Point {x: 0, y: 1};
const SW: Point = Point {x: -1, y: 1};
const W: Point  = Point {x: -1, y: 0};
const NW: Point = Point {x: -1, y: -1};

impl Point {
    fn add(&self, pt: &Point) -> Point {
        Point {x: self.x + pt.x, y: self.y + pt.y}
    }

    fn move_to(&self, dest: &str) -> Point {
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

    fn path_to(&self, dest: &Point) -> String {
        let pt = Point {x:compare(self.x, dest.x), y: compare(self.y, dest.y)};
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

fn main() {
    let thor  = Point {x:5, y: 4};
    let light = Point {x:31, y: 4};

    println!("Start at {:?}", thor);
    println!("Goto {:?}", light);

    let direction = thor.path_to(&light);
    println!("Go to {}", direction);
    println!("Thor is now at {:?}", thor.move_to(&direction));
}


#[test]
fn test_all_destinations() {
    let start = Point {x:5, y:5};

    assert_eq!(start.path_to(&Point{x:5,y:5}), "X");
    assert_eq!(start.path_to(&Point{x:5,y:2}), "N");
    assert_eq!(start.path_to(&Point{x:7,y:2}), "NE");
    assert_eq!(start.path_to(&Point{x:6,y:2}), "NE");
    assert_eq!(start.path_to(&Point{x:8,y:5}), "E");
    assert_eq!(start.path_to(&Point{x:7,y:9}), "SE");
    assert_eq!(start.path_to(&Point{x:5,y:9}), "S");
    assert_eq!(start.path_to(&Point{x:3,y:9}), "SW");
    assert_eq!(start.path_to(&Point{x:3,y:5}), "W");
    assert_eq!(start.path_to(&Point{x:3,y:2}), "NW");
}
