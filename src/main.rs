// For a given position found the correct path to the given destination
//
// map is from upper left (0, 0) -> bottom right (40, 18)
// Direction are give by N NE E SE S SW W or NW
type Point = (i8, i8);

const N: Point = (0, 1);
const NE: Point = (1, 1);
const E: Point = (1, 0);
const SE: Point = (1, -1);
const S: Point = (0, -1);
const SW: Point = (-1, -1);
const W: Point = (-1, 0);
const NW: Point = (-1, 1);

fn get_direction(orig: Point, dest: Point) -> (Point, String) {
    let pt: Point = (compare(orig.0, dest.0), compare(orig.1, dest.1));
    let dir = match pt {
        N => String::from("N"),
        NE => String::from("NE"),
        E => String::from("E"),
        SE => String::from("SE"),
        S => String::from("S"),
        SW => String::from("SW"),
        W => String::from("W"),
        NW => String::from("NW"),
        _ => String::from("X"), // Don't move :-)
    };

    (pt, dir)
}

fn compare(x: i8, y: i8) -> i8 {
    if x < y {
        return 1;
    }

    if x > y {
        return -1;
    }

    0
}

fn main() {
    let thor: Point = (5, 4);
    let light: Point = (31, 4);

    println!("Start at {:?}", thor);
    println!("Goto {:?}", light);

    let (next_point, direction) = get_direction(thor, light);
    println!("Next point direction {:?}", next_point);
    println!("Go to {}", direction);
}
