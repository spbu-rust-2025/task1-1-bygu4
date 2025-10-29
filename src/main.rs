use std::io;

fn main() {
    let mut buf = String::new();
    let res = io::stdin().read_line(&mut buf);

    match res {
        Ok(_) => {
            let mut it = buf.as_str().trim().split(' ');
            match (it.next(), it.next()) {
                (Some(first), Some(second)) if it.count() == 0 => {
                    let first = first.parse::<i32>();
                    let second = second.parse::<i32>();
                    match (first, second) {
                        (Ok(first), Ok(second)) => println!("{}", first + second),
                        _ => println!("failed to parse line"),
                    }
                }
                (_, _) => println!("expected two numbers"),
            }
        }
        _ => println!("failed to read line"),
    }
}
