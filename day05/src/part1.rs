use std::io;

fn main() {
    let input = include_str!("../input");
    let opcodes: Vec<isize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    println!("{:?}", opcodes);
    run(opcodes, 0);
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
            "04" => output(&mut codes, &mut pos, modes),
            "05" => jump_true(&mut codes, &mut pos, modes),
            "06" => jump_false(&mut codes, &mut pos, modes),
            "07" => less_than(&mut codes, &mut pos, modes),
            "08" => equals(&mut codes, &mut pos, modes),
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
    let mode_c = modes & 0b100;
    let a: isize = match mode_a == 0 {
        true => codes[codes[*pos + 1] as usize],
        false => codes[*pos + 1],
    };

    let b: isize = match mode_b == 0 {
        true => codes[codes[*pos + 2] as usize],
        false => codes[*pos + 2],
    };

    let c = match mode_c == 0 {
        true => codes[*pos + 3] as usize,
        false => *pos + 3,
    };

    *pos += 4;
    codes[c] = a + b;
}

fn mul(codes: &mut Vec<isize>, pos: &mut usize, modes_string: &str) {
    let modes = usize::from_str_radix(modes_string, 2).unwrap();
    let mode_a = modes & 0b1;
    let mode_b = modes & 0b10;
    let mode_c = modes & 0b100;
    let a: isize = match mode_a == 0 {
        true => codes[codes[*pos + 1] as usize],
        false => codes[*pos + 1],
    };

    let b: isize = match mode_b == 0 {
        true => codes[codes[*pos + 2] as usize],
        false => codes[*pos + 2],
    };

    let c = match mode_c == 0 {
        true => codes[*pos + 3] as usize,
        false => *pos + 3,
    };

    *pos += 4;
    codes[c] = a * b;
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

fn output(codes: &mut Vec<isize>, pos: &mut usize, modes_string: &str) {
    let modes = usize::from_str_radix(modes_string, 2).unwrap();
    let mode_a = modes & 0b1;

    let a = match mode_a == 0{
        true => codes[codes[*pos +1] as usize],
        false => codes[*pos +1]
    };

    println!("Output: {}", a);
    *pos += 2;
}

fn jump_true(codes: &mut Vec<isize>, pos: &mut usize, modes_string: &str) {
    let modes = usize::from_str_radix(modes_string, 2).unwrap();
    let mode_a = modes & 0b1;
    let mode_b = modes & 0b10;

    let a = match mode_a == 0 {
        true => codes[codes[*pos + 1] as usize],
        false => codes[*pos + 1]
    };

    let b = match mode_b == 0 {
        true => codes[codes[*pos + 2] as usize],
        false => codes[*pos + 2]
    };

    match a {
        0 => *pos += 3,
        _ => *pos = b as usize
    }
}

fn jump_false(codes: &mut Vec<isize>, pos: &mut usize, modes_string: &str) {
    let modes = usize::from_str_radix(modes_string, 2).unwrap();
    let mode_a = modes & 0b1;
    let mode_b = modes & 0b10;

    let a = match mode_a == 0 {
        true => codes[codes[*pos + 1] as usize],
        false => codes[*pos + 1]
    };

    let b = match mode_b == 0 {
        true => codes[codes[*pos + 2] as usize],
        false => codes[*pos + 2]
    };

    match a {
        0 => *pos = b as usize,
        _ => *pos += 3
    }
}

fn less_than(codes: &mut Vec<isize>, pos: &mut usize, modes_string: &str) {
    let modes = usize::from_str_radix(modes_string, 2).unwrap();
    let mode_a = modes & 0b1;
    let mode_b = modes & 0b10;
    let mode_c = modes & 0b100;

    let a = match mode_a == 0 {
        true => codes[codes[*pos + 1] as usize],
        false => codes[*pos + 1]
    };

    let b = match mode_b == 0 {
        true => codes[codes[*pos + 2] as usize],
        false => codes[*pos + 2]
    };

    let c = match mode_c == 0 {
        true => codes[*pos + 3] as usize,
        false => *pos + 3,
    };

    codes[c] = match a < b {
        true => 1,
        false => 0
    };

    *pos += 4;
}

fn equals(codes: &mut Vec<isize>, pos: &mut usize, modes_string: &str) {
    let modes = usize::from_str_radix(modes_string, 2).unwrap();
    let mode_a = modes & 0b1;
    let mode_b = modes & 0b10;
    let mode_c = modes & 0b100;

    let a = match mode_a == 0 {
        true => codes[codes[*pos + 1] as usize],
        false => codes[*pos + 1]
    };

    let b = match mode_b == 0 {
        true => codes[codes[*pos + 2] as usize],
        false => codes[*pos + 2]
    };

    let c = match mode_c == 0 {
        true => codes[*pos + 3] as usize,
        false => *pos + 3,
    };

    codes[c] = match a == b {
        true => 1,
        false => 0
    };

    *pos += 4;
}

#[allow(dead_code)]
fn str_to_ints(input: &str) -> Vec<isize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
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
    run(s1, 0);
    let s2 = str_to_ints("2,0,2,0,99");
    assert_eq!(run(s2, 0), 4);
}

#[test]
fn test_jmp_true() {
    let mut s1 = str_to_ints("3,12,6,12,15,1,13,14,13,4,13,99,5,0,1,9");
    let mut s2 = str_to_ints("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
    let mut pos = 2;
    jump_true(&mut s1, &mut pos, "00");
    pos = 2;
    jump_true(&mut s2, &mut pos, "11");
}
