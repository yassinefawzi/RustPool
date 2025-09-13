pub struct Food {
    pub name: String,
	pub calories: (String, String),
	pub fats: f64,
	pub carbs: f64,
	pub proteins: f64,
	pub nbr_of_portions: f64,
}

fn round_it(value: f64) -> f64 {
	let round_it = (value * 100.0).round() / 100.0;
	if round_it.fract() == 0.0 {
		(round_it * 10.0).round() / 10.0
	} else {
		round_it
	}
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

	for food in foods {
		let cal_splt = food.calories.1.trim_end_matches("kcal");
		let cal: f64 = cal_splt.parse().unwrap_or(0.0);
		let portions = food.nbr_of_portions;

		cals += cal * portions;
		carbs += food.carbs * portions;
		proteins += food.proteins * portions;
		fats += food.fats * portions;
	}

	json::object! {
		"cals" => round_it(cals),
		"carbs" => round_it(carbs),
		"proteins" => round_it(proteins),
		"fats" => round_it(fats),
	}
}
