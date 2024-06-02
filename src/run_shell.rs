use nix::{
    errno::Errno,
    sys::{
        signal::{self, SaFlags, SigAction, SigHandler, SigSet, Signal},
        wait::waitpid
    },
    unistd::{
        close, execvp, fork, getpgrp, pipe, read, setpgid, tcsetpgrp,
        ForkResult}
};
use std::{
    env,
    ffi::CString,
    io::{self, Write},
    os::fd::{AsFd, AsRawFd, BorrowedFd, IntoRawFd}
};
use whoami;
use crate::commands::{cd, time_manage, exit};
use colored::*;
use once_cell::sync::Lazy;

#[derive(Debug)]
enum Action {
    SimpleCommand(Vec<String>)
}

static BUILTIN_COMMANDS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec!["cd", "work","exit"]
});
unsafe fn stdin_fd() -> impl AsFd {
    unsafe { BorrowedFd::borrow_raw(nix::libc::STDIN_FILENO) }
}

pub fn shell_loop() {
    ignore_tty_signals();
    while let Some(line) = shell_read_line() {
        let action = match shell_parse_line(&line) {
            None => continue,
            Some(action) => action,
        };

        match action {
            Action::SimpleCommand(command) => shell_exec_simple_command(command),
        }
    }
}

fn shell_read_line() -> Option<String> {
    print!("{}{}{} {}\n> ",
           "@".bright_cyan().bold(),
           whoami::username().bright_cyan().bold(),
           ":".bright_magenta().bold(),
           env::current_dir().unwrap().display().to_string().bright_magenta().bold()
    );
    io::stdout().flush().unwrap();

    let mut result = String::new();
    match io::stdin().read_line(&mut result) {
        Ok(size) => {
            if size == 0 {
                None
            } else {
                Some(result.trim_end().to_string())
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

fn shell_parse_line(line: &str) -> Option<Action> {
    match line.is_empty() {
        true => None,
        false => {
            let commands: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            Some(Action::SimpleCommand(commands))
        }
    }
}

fn shell_exec_simple_command(command: Vec<String>) {
    let (pipe_read, pipe_write) = pipe().unwrap();

    if BUILTIN_COMMANDS.contains(&command[0].as_str()) {
        match command[0].as_str() {
            "cd" => cd::chdir(command.clone()),
            "exit" => exit::exit(),
            "work" => time_manage::time_manage(command.clone()),
            _ => unimplemented!()
        }
    } else {

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                setpgid(child, child).unwrap();

                tcsetpgrp(unsafe { stdin_fd() }, getpgrp()).unwrap();

                close(pipe_read.into_raw_fd()).unwrap();
                close(pipe_write.into_raw_fd()).unwrap();
                waitpid(child, None).ok();

                tcsetpgrp(unsafe { stdin_fd() }, getpgrp()).unwrap();
            },
            Ok(ForkResult::Child) => {
                restore_tty_signals();

                close(pipe_write.into_raw_fd()).unwrap();

                loop {
                    let mut buf = [0];
                    match read(pipe_read.as_raw_fd(), &mut buf) {
                        Err(e) if e == Errno::EINTR => (),
                        _ => break
                    }
                }
                close(pipe_read.into_raw_fd()).unwrap();

                let args = command.into_iter().map(|c| CString::new(c).unwrap()).collect::<Vec<_>>();
                execvp(&args[0], &args).unwrap();

            },
            Err(e) => eprintln!("fork error: {}", e)
        }
    }

}

fn ignore_tty_signals() {
    let sa = SigAction::new(SigHandler::SigIgn, SaFlags::empty(), SigSet::empty());
    unsafe {
        signal::sigaction(Signal::SIGTSTP, &sa).unwrap();
        signal::sigaction(Signal::SIGTTIN, &sa).unwrap();
        signal::sigaction(Signal::SIGTTOU, &sa).unwrap();
    }
}

fn restore_tty_signals() {
    let sa = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
    unsafe {
        signal::sigaction(Signal::SIGTSTP, &sa).unwrap();
        signal::sigaction(Signal::SIGTTIN, &sa).unwrap();
        signal::sigaction(Signal::SIGTTOU, &sa).unwrap();
    }
}
