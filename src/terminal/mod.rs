use core::fmt;
use core::fmt::Write;

use limine::LimineTerminalResponse;

use crate::TERMINAL_REQUEST;

struct GrapefruitTerminal;

impl core::fmt::Write for GrapefruitTerminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        static mut CACHED: Option<&'static LimineTerminalResponse> = None;
        

        unsafe {
            if let Some(writer) = CACHED {
                let terminal = writer.terminals().unwrap().first().unwrap();

                writer.write().unwrap()(terminal, s);
            }
            else {
                let response = TERMINAL_REQUEST.get_response().get().unwrap();
                let terminal = response.terminals().unwrap().first().unwrap();
                let writer = response.write().unwrap();

                writer(&terminal, s);

                CACHED = Some(response);
            }
        }


        Ok(())
        
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::terminal::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut term = GrapefruitTerminal;

    term.write_fmt(args).unwrap();
}