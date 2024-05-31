use std::env;

pub fn chdir(command: Vec<String>) {
    if let Some(path) = command.get(1) {
        env::set_current_dir(path).expect("Failed to change directory.");
    } else {
        env::set_current_dir("/Users/noharu").expect("Failed to change directory.");
    }
}
