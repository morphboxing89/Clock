use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use figlet_rs::FIGlet;
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let standard_font = FIGlet::standard().unwrap();

    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let total_secs = now.as_secs() % 86400;

        let hour = (total_secs / 3600) % 24;

        let local_hour = (hour + 3) % 24;

        let minute = (total_secs % 3600) / 60;

        let second = total_secs % 60;

        let time_str = format!("{:02} : {:02} : {:02}", local_hour, minute, second);

        let figure = standard_font.convert(&time_str).unwrap();

        clear_screen();

        if let Some((Width(w), Height(h))) = terminal_size() {
            let figure_str = figure.to_string();
            let lines: Vec<&str> = figure_str.lines().collect();

            let vertical_padding = h.saturating_sub(lines.len() as u16) / 2;
            for _ in 0..vertical_padding {
                println!();
            }

            for line in lines {
                let left_padding = (w as usize / 2).saturating_sub(line.len() / 2);
                println!(
                    "{:width$}{}{}{}",
                    "",
                    SetForegroundColor(Color::Yellow),
                    line,
                    ResetColor,
                    width = left_padding
                );
            }

            std::io::stdout().flush().unwrap();
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn clear_screen() {
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0),).unwrap();
}
