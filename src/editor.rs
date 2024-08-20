mod terminal;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
use terminal::{Position, Size, Terminal};

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
            Self::draw_credits()?;
            Terminal::move_cursor_to(Position {x: 0, y: 0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        // versione del tutorial
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    // draw the application name centered at 1/3 height of screen
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