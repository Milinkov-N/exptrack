use std::{cell::Cell, io::Write, rc::Rc};

use tracker::prelude::*;

pub enum Command {
    Exit,
    Unknown(String),
    New(Expense),
}

#[derive(Debug)]
pub enum AppError {
    NoCommand,
    ExpenseErr(ExpenseParseError),
}

pub struct App {
    promt: &'static str,
}

impl App {
    pub fn new(promt: &'static str) -> Self {
        Self { promt }
    }
    pub fn run<F>(&mut self, cb: F) -> std::io::Result<()>
    where
        F: Fn(Command, Box<&mut dyn FnMut() -> ()>),
    {
        let running = Rc::new(Cell::new(true));
        let mut shutdown = || running.set(false);

        while running.get() {
            let mut input = String::new();

            self.print_promt()?;

            match std::io::stdin().read_line(&mut input) {
                Ok(_) => match Self::get_command(&input) {
                    Ok(cmd) => cb(cmd, Box::new(&mut shutdown)),
                    Err(e) => eprintln!("Error: Couldn't parse the command (Cause: {e:?})"),
                },

                Err(e) => eprintln!("Failed to read the input (Cause: {e})"),
            }
        }

        Ok(())
    }

    fn get_command(input: &str) -> Result<Command, AppError> {
        let mut tokens = input.split_whitespace();
        let command = tokens.next().ok_or(AppError::NoCommand)?;

        match command {
            "exit" => Ok(Command::Exit),

            // new <date> <store> <item[:amount]> <price>
            "new" => Ok(Command::New(
                Expense::parse_iter(&mut tokens).map_err(|e| AppError::ExpenseErr(e))?,
            )),
            unknown => Ok(Command::Unknown(unknown.to_owned())),
        }
    }

    pub fn print_promt(&mut self) -> std::io::Result<()> {
        print!("{}", self.promt);
        std::io::stdout().flush()?;

        Ok(())
    }
}
