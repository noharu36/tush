use colored::*;

pub fn render() {
    println!(
        "
                        ╭╯         ╭╯
                        ╰╮Smoking ╭╯    {}
                        ╭╯ TIME ╭╯
        ▓▓██████████▒ ╭━╯               {} {} {} {} {}🚬
        ",
        "Welcome to tush!".bright_cyan().bold(),
        "tupakka".bright_red().bold(),
        "+".bright_white().bold(),
        "shell".bright_magenta().bold(),
        "=".bright_white().bold(),
        "tush".bright_cyan().bold()
    );
}
