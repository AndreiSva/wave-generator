use rand::Rng;
use std::io::stdout;
use std::{thread, time};

use crossterm::{
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal
};

fn draw(amplitude : i32, wavelength : i32) {
    let selected_color : crossterm::style::Color;
    match rand::thread_rng().gen_range(0..8) {
        0 => {selected_color = Color::Red;}
        1 => {selected_color = Color::Green;}
        2 => {selected_color = Color::Blue;}
        3 => {selected_color = Color::Yellow;}
        4 => {selected_color = Color::Cyan;}
        5 => {selected_color = Color::Grey;}
        6 => {selected_color = Color::Magenta;}
        7 => {selected_color = Color::DarkBlue;}
        8 => {selected_color = Color::White;}
        _ => {
            selected_color = Color::White;
        }
    }
    for x in 0..terminal::size().unwrap().0 {
        let y = f32::sin(x as f32 / wavelength as f32) * amplitude as f32 + (terminal::size().unwrap().1 / 2) as f32;
        execute!(
            stdout(),
            SetForegroundColor(selected_color),
            crossterm::cursor::MoveTo(x, y as u16),
            Print("*")
        ).ok();
    } 
}

fn main() {
    ctrlc::set_handler(move || {
        execute!(
            stdout(),
            crossterm::cursor::Show,
            terminal::Clear(terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Reset),
        ).ok();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    execute!(
        stdout(),
        crossterm::cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    ).ok();
    let mut amplitude : i32 = 2;
    let mut wavelength : i32 = 2;
    let mut operator : i8 = 1;
    loop {
        draw(wavelength, amplitude);
        if amplitude >= 10 || amplitude <= 1 {
            operator*=-1;
        }

        thread::sleep(time::Duration::from_millis(100));
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All) 
        ).ok();
        amplitude+=operator as i32;
        wavelength+=operator as i32;
    }
}
