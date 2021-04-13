mod method_table;
mod driver;

fn main() {

    let mut program = driver::Program::new();

    println!("¡Bienvenido al manejador de métodos de Luis!");

    println!("  - Powered by rust 🦀\n\n");

    while program.should_run() {
        program.run()
    }

}
