use std::io;

fn get_lines() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    return n;
}

fn get_data() -> Vec<String>{
    let n: usize = get_lines();
    let mut input = String::new();
    let mut vec : Vec<String> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let word = input.trim();
        vec.push(String::from(word));
    }
    vec
}


fn process_data(data: Vec<String>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for string in &data {
        let length = string.len();
        if length > 10 {
            let new_string = format!(
                "{}{}{}",
                string.chars().next().unwrap(),
                length - 2,
                string.chars().last().unwrap()
            );
            vec.push(new_string);
        } else {
            vec.push(string.clone());
        }
    }
    vec
}


fn print_data(data : Vec<String>){
    for element in data{
        println!("{}", element)
    }
}
pub fn solve() {
    let data = get_data();
    let p_data = process_data(data);
    print_data(p_data);
}
