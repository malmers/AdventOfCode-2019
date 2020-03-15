fn string_to_relation(relation: String) -> Option<(String, String)> {
    let a: Vec<&str> = relation.split_terminator(')').collect();
    match a.len() {
        2 => Some((a[0].to_string(), a[1].to_string())),
        _ => None
    }
}

fn orbits_object(objects: &Vec<(String, String)>, object: &String) -> Vec<String> {
    objects.iter().filter(|x| x.0 == *object).map(|x| x.1.clone()).collect()
}

fn main() {
    let input = include_str!("../input");
    let objects: Vec<(String, String)> = input.lines().map(|x| string_to_relation(x.parse().unwrap()).unwrap()).collect();
    let init = "COM".to_string();
    let mut step = 0;
    let mut vectors = vec!(init.clone());
    let mut value = 0;

    loop {
        step += 1;
        vectors = vectors.iter().map(|x| orbits_object(&objects, &x)).flatten().collect();

        println!("{:?}", vectors);
        match vectors.len() {
            0 => break,
            len => {
                value += len * step;
            }
        }
    }

    println!("{}", value);
}
