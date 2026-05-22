use forge::forgerhai::engine::ForgeRhaiEngine;

fn main() {
    let engine: ForgeRhaiEngine = ForgeRhaiEngine::new();

    engine.run_file();

    println!("Hello, world!");
}
