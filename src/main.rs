// open a file with today's date, mm-dd-yyyy, the US locale
// in the $NOTES directory, using their $EDITOR
// if file doesn't exist, create it.
// will source your env var for $DIR to set the directory.
// Equivalent flag for setting $DIR is "-d|--dir", e.g. `today.sh -d ~/writing`

// Thanks, Github Copilot.

fn main() {
    let mut args = std::env::args();
    let mut dir = std::env::var("NOTES").unwrap_or(".".to_string());
    let mut editor = std::env::var("EDITOR").unwrap_or("vim".to_string());

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "-d" | "--dir" => {
                dir = args.next().expect("Missing argument");
            }
            "-e" | "--editor" => {
                editor = args.next().expect("Missing argument");
            }
            _ => (),
        }
    }

    let date = chrono::Local::now().format("%m-%d-%Y");
    let file = format!("{}/{}.md", dir, date);
    let mut command = std::process::Command::new(&editor);
    command.arg(&file);
    command.status().expect("Failed to execute command");
}
