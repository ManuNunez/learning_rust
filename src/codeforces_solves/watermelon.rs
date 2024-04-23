use std::io;

pub fn watermelon_solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fallo al leer la línea");
    let input = input.trim();
    let numero: Result<i32, _> = input.parse();
    match numero {
        Ok(n) => {
            if (n - 2) % 2 == 0  && (n - 2) > 0{
                println!("YES");
            } else {
                println!("NO");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
