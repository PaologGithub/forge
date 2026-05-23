use forge::{forgefile::{check_availability::check_availability, check_validity::check_validity}, forgerhai::engine::ForgeRhaiEngine};

/// Returns Some(String) if there's a function to run,
/// or None if there's none.
fn parse_args() -> Option<String> {
    let args = std::env::args();

    if args.len() > 1 {
        let args: Vec<String> = args.collect();
        Some(args[1].clone())
    } else {
        None
    }
}

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

    let command = match parse_args() {
        Some(arg) => arg,
        None => "run".into()
    };

    if !engine.has_function(command.clone(), 0) {
        eprintln!("Function defined with name {} doesn't exist.", command);

        std::process::exit(3);
    }

    let command_result = match engine.run_main_function(command) {
        Ok(status) => status,
        Err(e) => {
            eprintln!("ForgeRhai couldn't run function.");
            eprintln!("Error: {}", e);

            std::process::exit(4);
        }
    };

    println!("{}", command_result);
}
