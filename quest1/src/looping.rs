use std::io;

fn main() {
    let mut tries = 0;

    let riddle = " I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";

    loop {
        println!("Riddle: {}", riddle);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        tries += 1;
		input = input.trim().to_string();
        if input.eq_ignore_ascii_case(answer) {
            println!("Number of trials: {}", tries);
            break;
        }
    }
}
