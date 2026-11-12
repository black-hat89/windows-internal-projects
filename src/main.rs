mod open_process;

fn main() {
    match open_process::get_open_process() {
        Ok(()) => println!("Process snapshot created successfully!"),
        Err(mesage) => println!("Error: {}", mesage),
    }
}
