use pat_parse::parse;

#[parse(r"#{} @ {},{}: {}x{}")]
#[derive(Debug)]
struct Patch {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    heigth: usize,
}

#[parse(r"{},{}: {}")]
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    value: f64,
}

#[parse(r"{}\n{},+{}\nFlag: {}")]
#[derive(Debug)]
struct Types {
    name: char,
    int: usize,
    float: f64,
    flag: bool
}

fn main(){
    println!("{:?}", Patch::from_str("#1 @ 2,3: 4x5"));
    println!("{:#?}", Patch::from_str_multiple("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    println!("{:?}", Point::from_str("2,3: 123.456"));
    println!("{:?}", Types::from_str("X\n2,,,123.456\nFlag: true"));
}
