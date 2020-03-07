use std::io;

fn main() {
    let input = include_str!("../input");
    let opcodes: Vec<isize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    println!("{:?}", opcodes);
    // find(opcodes);
    run(opcodes, 0);
}

fn find(mut opcodes: Vec<isize>) {
    for i in 0..100 {
        for j in 0..100 {
            opcodes[1] = i;
            opcodes[2] = j;
            let value = run(opcodes.to_owned(), 0);
            if value == 19690720 {
                println!("100 * i + j = {}", 100 * i + j);
            }
        }
    }
}

fn run(mut codes: Vec<isize>, position: usize) -> isize {
    let mut pos = position;
    loop {
        let code_string = format!("{:0>4}", codes[pos].to_string());
        let (modes, opcode) = code_string.split_at(code_string.len() - 2);
        match opcode {
            "01" => add(&mut codes, &mut pos, modes),
            "02" => mul(&mut codes, &mut pos, modes),
            "03" => input(&mut codes, &mut pos),
            "04" => output(&mut codes, &mut pos),
            "99" => return codes[0],
            _ => {
                println!("{:?}", opcode);
                panic!("That opcode is not implemented!");
            }
        }
    }
}

fn add(codes: &mut Vec<isize>, pos: &mut usize, modes_string: &str) {
    let modes: usize = modes_string.parse().unwrap();
    let mode_a = modes & 0b1;
    let mode_b = modes & 0b10;
    let a: isize = match mode_a {
        0 => codes[codes[*pos + 1] as usize],
        _ => codes[*pos + 1],
    };

    let b: isize = match mode_b {
        0 => codes[codes[*pos + 2] as usize],
        _ => codes[*pos + 2],
    };
    let target = codes[*pos + 3] as usize;
    *pos += 4;
    codes[target] = a + b;
}

fn mul(codes: &mut Vec<isize>, pos: &mut usize, modes_string: &str) {
    let modes = usize::from_str_radix(modes_string, 2).unwrap();
    let mode_a = modes & 0b1;
    let mode_b = modes & 0b10;
    let a: isize = match mode_a {
        0 => codes[codes[*pos + 1] as usize],
        _ => codes[*pos + 1],
    };

    let b: isize = match mode_b {
        0 => codes[codes[*pos + 2] as usize],
        _ => codes[*pos + 2],
    };
    let target = codes[*pos + 3] as usize;
    *pos += 4;
    codes[target] = a * b;
}

fn input(codes: &mut Vec<isize>, pos: &mut usize) {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("{} was inputed", input.trim());
        }
        Err(_error) => {
            panic!("Could not read from stdin");
        }
    }

    let position = codes[*pos+1] as usize;
    codes[position] = input.trim().parse().unwrap();
    *pos += 2;
}

fn output(codes: &mut Vec<isize>, pos: &mut usize) {
    println!("Output: {}", codes[codes[*pos+1] as usize]);
    *pos += 2;
}

#[allow(dead_code)]
fn str_to_ints(input: &str) -> Vec<isize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[test]
fn test_part1() {
    let s = str_to_ints("1,1,1,4,99,5,6,0,99");
    assert_eq!(run(s, 0), 30);
}

#[test]
fn test_add() {
    let s1 = str_to_ints("1,1,1,0,99");
    assert_eq!(run(s1, 0), 2);

    let s2 = str_to_ints("1001,1,4,0,99");
    assert_eq!(run(s2, 0), 5);

    let s3 = str_to_ints("101,1,4,0,99");
    assert_eq!(run(s3, 0), 100);

    let s4 = str_to_ints("1101,1,4,0,99");
    assert_eq!(run(s4, 0), 5);
}

#[test]
fn test_mul() {
    let s1 = str_to_ints("1002,4,3,4,33");
    assert_eq!(run(s1, 0), 1002)
}
