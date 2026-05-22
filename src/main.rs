use forge::{forgefile::{check_availability::check_availability, check_validity::check_validity}, forgerhai::engine::ForgeRhaiEngine};

fn main() {
    let file = match check_availability() {
        Some(file) => file,
        None => {
            eprintln!("Hasn't found any `Forgefile` in current directory.");

            std::process::exit(1);
        }
    };

    let mut engine: ForgeRhaiEngine = ForgeRhaiEngine::new();

    match check_validity(&mut engine, &file) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Compilation of `Forgefile` failed.");
            eprintln!("Error: {}", e);

            std::process::exit(2);
        }
    }

    println!("Hello, world!");
}
