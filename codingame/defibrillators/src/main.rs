use std::fmt;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug)]
struct Position {
    lon: f32, // longitude in Radian
    lat: f32, // latitude in Radian
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "longitude: {}, latitude: {}", self.lon, self.lat)
    }
}

impl Position {
    fn distance(&self, pos: &Position) -> f32 {

        let x: f32 = (pos.lon - self.lon) * ((pos.lat + self.lat)/2.0).cos();
        let y: f32 = pos.lat - self.lat;
        return (x*x + y*y).sqrt() * 6371.0;
    }
}

fn str_to_f32(s: String) -> f32{
    s.trim()
     .replace(',', ".")
     .parse()
     .expect("float as a string is excepected")
}

fn main() {

    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("failed to read input");
    let lon: f32 = str_to_f32(input_line);

    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("failed to read input");
    let lat: f32 = str_to_f32(input_line);

    let user_pos = Position {
        lon: lon.to_radians(),
        lat: lat.to_radians(),
    };

    let mut input_line: String = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut closest_dist: f32 = 40000.0; // let's initialze with the earth circumference
    let mut closest_name: String = String::from("Around the world");

    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        // defib = "ID; Name; address; Phone; Long; Lat"
        let defib = input_line.trim_matches('\n').to_string();
        let defib_vec: Vec<&str> = defib.split(';').collect();
        let defib_lon: f32 = str_to_f32(defib_vec[4].to_string());
        let defib_lat: f32 = str_to_f32(defib_vec[5].to_string());

        let defib_pos = Position {
            lon: defib_lon.to_radians(),
            lat: defib_lat.to_radians(),
        };

        if user_pos.distance(&defib_pos) < closest_dist {
            closest_name = defib_vec[1].to_string();
            closest_dist = user_pos.distance(&defib_pos);
        }
    }

    println!("{}", closest_name);
}
