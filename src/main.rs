// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::{Path, PathBuf};

use pico_args;
use rustyline;

const HELP: &str = r#"Usage: hello [options]

Options:
    -h, --help      Print this help information"#;

enum Task {
    /// The default, which is an REPL shell.
    Shell,
    /// Print the help text.
    Help,
}

struct CmdLine {
    /// The path to the binary being executed, including the executable file.
    #[allow(dead_code)]
    binpath: std::path::PathBuf,
    /// The name of the executable, without the leading path or file extension.
    prog: String,
    /// The task to perform.
    task: Task,
}

impl CmdLine {
    fn from_env() -> Self {
        // get the command line executable name from the environment.
        let current_exe = std::env::current_exe().unwrap_or(PathBuf::from(env!("CARGO_BIN_NAME")));

        // Remove the executable name from the path
        let binpath = current_exe.parent().unwrap_or(Path::new("")).to_owned();

        // Get just the name of the executable, without the path or extension.
        // Used in error reporting.
        let prog = current_exe
            .file_stem()
            .expect("error: unable to determine executable name")
            .to_owned()
            .into_string()
            .expect("error: executable name OsString is not valid UTF-8");

        // Parse command line arguments using the pico-args crate.
        let mut pargs = pico_args::Arguments::from_env();

        // Help has a higher priority than any other option, and further
        // command-line processing is terminated.
        if pargs.contains(["-h", "--help"]) {
            return Self {
                binpath,
                prog,
                task: Task::Help,
            };
        }

        // Create a new CmdLine struct, and populate it with the parsed
        // command line arguments & options.
        let args = Self {
            binpath,
            prog,
            task: Task::Shell,
        };

        // Parse the remaining options.
        let remaining = pargs.finish();
        if !remaining.is_empty() {
            // Print an error message for each unused option.
            for arg in remaining.iter() {
                if let Some(arg) = arg.to_str() {
                    if !arg.is_empty() && arg.as_bytes()[0] == '-' as u8 {
                        eprintln!("{}: error: unrecognized option: {}", args.prog, arg);
                    } else {
                        eprintln!("{}: error: unused argument: {}", args.prog, arg);
                    }
                } else {
                    eprintln!(
                        "{}: error: unused/unrecognized command line parameter: {:?}",
                        args.prog, arg
                    );
                }
            }
            // "Incorrect usage, such as invalid options or missing arguments."
            eprintln!("{}: error: invalid command line arguments.", args.prog);
            eprintln!("{}: Try 'hello --help' for more information.", args.prog);
            std::process::exit(2);
        };

        // Return the parsed command line.
        return args;
    }
}

fn shell(cmdline: CmdLine) {
    // Create a readline interface.
    let mut rl = rustyline::DefaultEditor::new().unwrap_or_else(|err| {
        eprintln!(
            "{}: error: unable to create readline interface: {}",
            cmdline.prog, err
        );
        std::process::exit(1);
    });

    loop {
        // Read a line from the user.
        let input = rl.readline(">>> ").unwrap_or_else(|err| {
            eprintln!(
                "{}: error: unable to read from stdin: {}",
                cmdline.prog, err
            );
            std::process::exit(1);
        });

        // If the user entered a blank line, then exit the REPL.
        if input.trim().is_empty() {
            break;
        }
    }
}

fn main() {
    let cmdline = CmdLine::from_env();

    match cmdline.task {
        Task::Help => {
            println!("{}", HELP);
        }
        Task::Shell => {
            shell(cmdline);
        }
    }
}

// End of File
