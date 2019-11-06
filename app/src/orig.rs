#[derive(Debug)]
struct Patch {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    heigth: usize,
}

impl Patch {
    pub fn from_str(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }

        RE.captures(s).map(|cap| Self { 
            id: cap[1].parse().unwrap(),
            left: cap[2].parse().unwrap(),
            top: cap[3].parse().unwrap(),
            width: cap[4].parse().unwrap(),
            heigth: cap[5].parse().unwrap(),
        })
    }
}