pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut obj = json::JsonValue::new_object();
    let mut tot_calories = 0.0;
    let mut tot_fats = 0.0;
    let mut tot_carbs = 0.0;
    let mut tot_proteins = 0.0;

    for food in foods {
        tot_calories +=
            food.calories.1.trim_end_matches("kcal").parse::<f64>().unwrap() * food.nbr_of_portions;
        tot_fats += food.fats * food.nbr_of_portions;
        tot_carbs += food.carbs * food.nbr_of_portions;
        tot_proteins += food.proteins * food.nbr_of_portions;
    }

    obj["cals"] = format!("{:.2}", tot_calories).parse::<f64>().ok().into();
    obj["carbs"] = format!("{:.2}", tot_carbs).parse::<f64>().ok().into();
    obj["proteins"] = format!("{:.2}", tot_proteins).parse::<f64>().ok().into();
    obj["fats"] = format!("{:.2}", tot_fats).parse::<f64>().ok().into();

    obj
}
