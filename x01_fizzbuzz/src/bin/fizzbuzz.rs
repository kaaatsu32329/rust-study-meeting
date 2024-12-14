fn main() {
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).unwrap();

    let input_num = input_str.trim().parse::<isize>().unwrap();

    for i in 0..input_num + 1 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(n: isize) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}
