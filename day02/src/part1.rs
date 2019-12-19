fn main() {
    let input = include_str!("../input");
    let mut opcodes: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    opcodes[1] = 12;
    opcodes[2] = 2;
    println!("{:?}", run(opcodes, 0));
}

fn run(mut codes: Vec<usize>, mut pos: usize) -> usize {
    loop {
        match codes[pos] {
            1 => add(&mut codes, pos),
            2 => mul(&mut codes, pos),
            99 => return codes[0],
            _ => panic!("That opcode is not implemented!")
        }
        pos += 4;
    }
}

fn add(codes: &mut Vec<usize>, pos: usize) {
    let a = codes[pos+1];
    let b = codes[pos+2];
    let target = codes[pos+3];
    codes[target] = codes[a] + codes[b];
}

fn mul(codes: &mut Vec<usize>, pos: usize) {
    let a = codes[pos+1];
    let b = codes[pos+2];
    let target = codes[pos+3];
    codes[target] = codes[a] * codes[b];
}

#[test]
fn test_part1() {
    let s = "1,1,1,4,99,5,6,0,99".split(',').map(|x| x.parse().unwrap()).collect();
    assert_eq!(run(s, 0), 30);
}
