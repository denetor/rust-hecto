mod terminal;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap()
    }
     
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('w') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;  
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position {x: 0, y: 0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{ height, .. } = Terminal::size()?;
        // versione del tutorial
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    // draw the application name centered at 1/3 height of screen
    // my solution, not used
    fn draw_credits() -> Result<(), Error> {
        let Size{height, width} = Terminal::size()?;
        let caption = "Hecto v0.1";
        let target_row: u16 = (height / 3) as u16;
        let target_column: u16 = ((width / 2) - (caption.len() as u16 / 2)) as u16;
        Terminal::move_cursor_to(Position {x: target_column, y: target_row})?;
        Terminal::print(caption)?;

        Ok(())
    }

    
}