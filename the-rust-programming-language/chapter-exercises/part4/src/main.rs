fn main() {
    let mut city_names = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];


    // the match is needed here because Rust does not have nulls and pop returns an Option<&str> (None if the vec is empty)
    // this is similar to null checking in other languages
    let last_city: &str = match city_names.pop() {
        Some(city) => city, // if the vec is not empty, return the last city
        None => "No city found", // if the vec is empty, return "No city found"
    };

    if last_city.starts_with("R") {
        println!("“{}” starts with an R!", last_city);
    } else {
        println!("“{}” doesn't start with R", last_city);
    }

    city_names.push(last_city); // push the last city "Rustville" back to the vec

    println!("Here is the full list of cities:");
    for city in city_names.iter() {
        println!("* {}", city);
    }
}
