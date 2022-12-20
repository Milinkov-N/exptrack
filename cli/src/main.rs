use cli::*;

fn main() -> std::io::Result<()> {
    App::new("Nikita $ ").run(|cmd, mut shutdown| match cmd {
        Command::New(expense) => println!("{expense:?}"),
        Command::Unknown(unknown) => println!("Error: unknown command `{unknown}`"),
        Command::Exit => shutdown(),
    })
}
