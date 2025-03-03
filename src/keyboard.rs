use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
    cursor::{MoveLeft, MoveRight, MoveUp},
    terminal::{ClearType, Clear},
    ExecutableCommand,
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor},
};
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::env;

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
                    KeyCode::Tab => {
                        let _ = match env::current_dir() {
                            Ok(path) => list_directory(Path::new(&path)),
                            Err(_) => list_directory(Path::new("/"))
                        };
                    }
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
                                let _ = io::stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine)); // 清除光标到行尾
                                print!("{}", &inputed[input_buffer..]); // 重新绘制剩余字符
                                let _ = io::stdout().execute(MoveLeft((inputed.len() - input_buffer) as u16)); // 光标移动到正确位置
                            } else {
                                input_buffer -= 1;
                                inputed.remove(inputed.len() - 1);
                                let _ = io::stdout().execute(MoveLeft(1)); // 光标左移
                                let _ = io::stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine)); // 清除光标到行尾
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

fn list_directory<P: AsRef<Path>>(path: P) -> io::Result<()> {
    // 获取目录条目
    let entries = fs::read_dir(path)?;

    // 移动到下一行以开始打印
    // execute!(io::stdout(), MoveToNextLine(1))?;
    println!("");
    print!("\r   \r");
    let _ = io::stdout().flush();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // 获取文件名并过滤隐藏文件
        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            if file_name.starts_with('.') {
                continue; // 跳过隐藏文件
            }

            // 根据条目类型设置颜色并打印
            if path.is_dir() {
                execute!(
                    io::stdout(),
                    SetForegroundColor(Color::Blue),
                    Print(file_name),
                    Print(" "),
                    ResetColor
                )?;
            } else if path.is_file() {
                execute!(io::stdout(), Print(file_name), Print(" "))?;
            }
        }
    }
    // 返回上一行
    execute!(io::stdout(), MoveUp(1))?;
    
    print!("\r$ ");
    let _ = io::stdout().flush();
    Ok(())
}
