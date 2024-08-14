mod terminal;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::style::{Print};
use crossterm::execute;
use std::io::stdout;
use terminal::Terminal;

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
     
    fn repl(&mut self) -> Result<(), std::io::Error> {
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        if self.should_quit {
            Terminal::clear_screen()?;  
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows();
            Terminal::move_cursor_to(0, 0)?;
        }
        Terminal::show_cursor();
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;
        // versione del tutorial
        for current_row in 0..height {
            execute!(stdout(), Print("~".to_string()));
            if current_row + 1 < height {
                execute!(stdout(), Print("\r\n".to_string()));
            }
        }

        // versione nicola
        /*
        for current_row in 1..height {
            Terminal::move_cursor_to(0, current_row - 1)?;
            print!("~");
        }*/

        Ok(())
    }

    
}