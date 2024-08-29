pub struct OddNumbers {
    current: usize,
}

impl OddNumbers {
    pub fn new() -> Self {
        Self { current: 1 }
    }
}

impl Default for OddNumbers {
    fn default() -> Self {
        OddNumbers::new()
    }
}

impl Iterator for OddNumbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let odd: usize = self.current;
        self.current += 2;
        Some(odd)
    }
}
fn main() {
    let mut v = OddNumbers::default();

    for _ in 0..5 {
        println!("got {}", v.next().unwrap());
    }
}
