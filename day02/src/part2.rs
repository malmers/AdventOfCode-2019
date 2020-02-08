fn main() {
    let input = include_str!("../input");
    let opcodes: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    find(opcodes);
}

fn find(mut opcodes: Vec<usize>) {
  for i in 0..100 {
    for j in 0..100 {
      opcodes[1] = i;
      opcodes[2] = j;
      let value = run(opcodes.to_owned(), 0);
      if value == 19690720 {
        println!("100 * i + j = {}", 100*i + j);
      }
    }
  }
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
