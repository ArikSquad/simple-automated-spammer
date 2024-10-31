use enigo::Direction::Click;
use enigo::{Enigo, Key, Keyboard, Settings};
use std::{io, thread, time::Duration};

fn main() {
    println!("Text to spam? > ");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read line");

    println!("How many times? > ");
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Failed to read line");
    let times: u32 = count.trim().parse().expect("Please enter a number");

    println!("Starting in 5 seconds...");
    thread::sleep(Duration::from_secs(5));

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    for i in 1..=times {
        for c in text.trim().chars() {
            enigo.text(&c.to_string()).unwrap();
        }

        enigo.key(Key::Return, Click).unwrap();
        println!("Sent {} times", i);
        thread::sleep(Duration::from_millis(150));
    }

    println!("Done! Sent {} times.", times);
}