pub fn rpn(input: &str) {
    let mut stack: Vec<i64> = Vec::new();

    for token in input.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" | "%" => {
                // Need at least 2 numbers to operate
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let res = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0 {
                            println!("Error");
                            return;
                        }
                        a / b
                    }
                    "%" => {
                        if b == 0 {
                            println!("Error");
                            return;
                        }
                        a % b
                    }
                    _ => unreachable!(),
                };
                stack.push(res);
            }
            number => {
                // Try to parse number
                match number.parse::<i64>() {
                    Ok(n) => stack.push(n),
                    Err(_) => {
                        println!("Error");
                        return;
                    }
                }
            }
        }
    }

    if stack.len() != 1 {
        println!("Error");
    } else {
        println!("{}", stack[0]);
    }
}
