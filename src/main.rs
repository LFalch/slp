use pancurses::{initscr, endwin};

use std::{env::args, time::Duration, thread, sync::mpsc, fmt::{self, Debug}};

enum Error {
    InvalidNumberFormat,
    TooManyArgs,
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidNumberFormat => "Invalid number format",
            Error::TooManyArgs => "Too many args",
        })
    }
}

fn main() -> Result<(), Error> {
    let mut args = args().skip(1);

    let seconds = args.next().map(|n| n.parse::<u64>()).unwrap_or(Ok(5)).map_err(|_| Error::InvalidNumberFormat)?;
    if args.next().is_some() {
        return Err(Error::TooManyArgs);
    }

    let (tx, rx) = mpsc::channel();

    let s = format!("Wait {} seconds or press any key to continue...", seconds);

    thread::spawn(move || {
        let window = initscr();
        window.printw(&s);
        window.getch();
        tx.send(()).unwrap();
    });

    let _ = rx.recv_timeout(Duration::new(seconds, 0));

    endwin();

    Ok(())
}