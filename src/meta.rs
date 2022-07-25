pub fn metacmd(command: &str) {
    match command {
        "quit" => {
            println!("Bye!");
            std::process::exit(0);
        }
        "help" => {
            println!("Available commands:");
            println!("  .quit");
            println!("  .help");
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}