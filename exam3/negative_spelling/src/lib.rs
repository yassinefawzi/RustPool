pub fn negative_spell(n: i64) -> String {
	if n > 0 {
        return "error: positive number".to_string();
    }

	if n == 0 {
        return "zero".to_string();
    }

    fn one_to_nineteen(n: i64) -> &'static str {
        match n {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => "",
        }
    }

    fn tens(n: i64) -> &'static str {
        match n {
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => "",
        }
    }

	fn spell_number(mut n: i64) -> String {
		let mut result = String::new();

		if n >= 1000000 {
			let millions = n / 1000000;
			result.push_str(&format!("{} million", spell_number(millions)));
			n %= 1000000;
			if n > 0 { result.push(' '); }
		}

		if n >= 1000 {
			let thousents = n / 1000;
			result.push_str(&format!("{} thousand", spell_number(thousents)));
			n %= 1000;
			if n > 0 {result.push(' '); }
		}

		if n >= 100 {
			let hundreds = n / 100;
			result.push_str(&format!("{} hundred", one_to_nineteen(hundreds)));
			n %= 100;
			if n > 0 { result.push(' '); }
		}

		if n >= 20 {
			let t = n / 10;
			result.push_str(tens(t));
			n %= 10;
			if n > 0 { result.push('-'); }
		}

		if n > 0 && n < 20 {
			result.push_str(one_to_nineteen(n))
		}
		result
	}
	format!("minus {}", spell_number(-n))
}