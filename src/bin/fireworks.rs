use crossterm::{
    cursor::{Hide, Show},
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};
use fireworks_erikh::{firework::Firework, screen::Screen};
use std::{
    cell::RefCell,
    io::stdout,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

fn handle_signal(term: Arc<AtomicBool>) {
    loop {
        if term.load(Ordering::Relaxed) {
            stdout()
                .execute(Show)
                .unwrap()
                .execute(Clear(ClearType::All))
                .unwrap();
            std::process::exit(0);
        }
        std::thread::sleep(std::time::Duration::new(0, 500))
    }
}

fn main() -> std::io::Result<()> {
    let term = Arc::new(AtomicBool::new(false));
    let winch = Arc::new(AtomicBool::new(true));
    let t2 = term.clone();

    std::thread::spawn(move || handle_signal(t2));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
    signal_hook::flag::register(signal_hook::consts::SIGWINCH, Arc::clone(&winch))?;
    stdout().execute(Hide)?.execute(Clear(ClearType::All))?;

    let mut s = size()?;
    let mut screen = Screen::new(s.1.into(), s.0.into());
    let mut iterations = 0;

    loop {
        if iterations == 5 {
            screen.add_turtle(Rc::new(RefCell::new(Firework::new(s.1.into(), s.0.into()))));
            iterations = 0;
        }

        if winch.load(Ordering::Relaxed) {
            s = size()?;
            screen = Screen::new(s.1.into(), s.0.into());
            winch.store(false, Ordering::Relaxed);
        }
        screen.draw()?;
        screen.tick();
        std::thread::sleep(std::time::Duration::new(0, 50000000));
        iterations += 1;
    }
}
