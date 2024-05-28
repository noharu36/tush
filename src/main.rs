use tush::run_shell::shell_loop;
use colored::*;

fn main() {
    println!(
        "
                        ╭╯         ╭╯
                        ╰╮Smoking ╭╯    {}
                        ╭╯ TIME ╭╯
        ▓▓██████████▒ ╭━╯               {} {} {} {} {}🚬
        ",
        "Hello tush!".bright_cyan().bold(),
        "tupakka".bright_red().bold(),
        "+".bright_white().bold(),
        "shell".bright_magenta().bold(),
        "=".bright_white().bold(),
        "tush".bright_cyan().bold()
        );
    shell_loop()
}
