fn main() {
  // let a = (359282..820401).map(|x| is_six_digits(x));
  let a: Vec<Vec<u32>> = (359282..820401)
                        .map(|x| x.to_string().chars()
                                  .map(|x| x.to_digit(10)
                                            .unwrap())
                                            .collect()
                        )
                        .filter(is_six_digits)
                        .filter(adjacent)
                        .filter(never_decrease)
                        .collect();

  println!("{:?}", a.len());
}

fn is_six_digits(num: &Vec<u32>) -> bool {
  num.len() == 6
}

fn adjacent(num: &Vec<u32>) -> bool {
  let mut unique = num.clone();
  unique.dedup();
  !is_six_digits(&unique)
}

fn never_decrease(num: &Vec<u32>) -> bool {
  let mut sorted = num.clone();
  sorted.sort();
  *num == sorted
}

#[test]
fn test_six_digits() {
  let yes = vec!(1,2,3,4,5,6);
  let no = vec!(1,2,3,4,5,6,7);
  assert!(is_six_digits(&yes));
  assert!(!is_six_digits(&no));
}

#[test]
fn test_adjacent() {
  let yes = vec!(1,2,3,3,5,6);
  let no = vec!(1,2,3,4,5,6);
  assert!(adjacent(&yes));
  assert!(!adjacent(&no));
}

#[test]
fn test_never_decrease() {
  let yes = vec!(1,2,3,4,5,6);
  let no = vec!(1,2,3,4,2,6);
  assert!(never_decrease(&yes));
  assert!(!never_decrease(&no));
}
