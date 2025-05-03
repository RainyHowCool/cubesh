use crossterm::{
    ExecutableCommand,
    cursor::{MoveLeft, MoveRight, MoveUp},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

pub fn get_line(inputed: &mut String) -> io::Result<()> {
    let mut input_buffer: usize = 0; // usize type needed
    // enable raw key mode
    enable_raw_mode()?;

    loop {
        // Read Keyboard
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    // Tab
                    // TODO: Improve Tab
                    /*
                    KeyCode::Tab => {
                        let current_input = inputed.clone();
                        let _ = match env::current_dir() {
                            Ok(path) => list_directory(Path::new(&path)),
                            Err(_) => list_directory(Path::new("/")),
                        };
                        // Recover the input line
                        print!("\r$ {} ", current_input);

                        let _ = io::stdout().execute(MoveLeft(1));
                        let _ = io::stdout().flush();
                    } */

                    // Enter
                    KeyCode::Enter => {
                        println!("");
                        execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
                        execute!(io::stdout(), MoveUp(1))?;
                        break;
                    }

                    // Backspace
                    KeyCode::Backspace => {
                        if input_buffer > 0 {
                            if input_buffer < inputed.len() {
                                input_buffer -= 1;
                                inputed.remove(input_buffer);
                                let _ = io::stdout().execute(MoveLeft(1)); // 光标左移
                                let _ = io::stdout().execute(crossterm::terminal::Clear(
                                    crossterm::terminal::ClearType::UntilNewLine,
                                )); // Clean curser to the end
                                print!("{}", &inputed[input_buffer..]); // Redraw the char
                                let _ = io::stdout()
                                    .execute(MoveLeft((inputed.len() - input_buffer) as u16)); // Move curser to right way
                            } else {
                                input_buffer -= 1;
                                inputed.remove(inputed.len() - 1);
                                let _ = io::stdout().execute(MoveLeft(1)); // Curser left move
                                let _ = io::stdout().execute(crossterm::terminal::Clear(
                                    crossterm::terminal::ClearType::UntilNewLine,
                                )); // Clear curser to the end
                            }
                        }
                    }
                    // Left Move
                    KeyCode::Left => {
                        if input_buffer > 0 {
                            input_buffer -= 1;
                            let _ = io::stdout().execute(MoveLeft(1)); // Move cursor
                        }
                    }
                    // Right Move
                    KeyCode::Right => {
                        if input_buffer < inputed.len() {
                            input_buffer += 1;
                            let _ = io::stdout().execute(MoveRight(1)); // Move cursor
                        }
                    }
                    // Normal Char
                    KeyCode::Char(ch) => {
                        print!("{}", ch);
                        inputed.insert(input_buffer, ch);
                        input_buffer += 1;
                        // Move the char's position
                        if input_buffer < inputed.len() {
                            print!("{}", &inputed[input_buffer..]);
                            let move_len = (inputed.len() - input_buffer) as u16;
                            let _ = io::stdout().execute(MoveLeft(move_len));
                        }
                    }
                    _ => {}
                }
                let _ = io::stdout().flush();
            }
        }
    }

    // Restore and exit
    disable_raw_mode()?;
    println!("");
    Ok(())
}
