#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use enigo::Direction::Click;
use enigo::{Enigo, Key, Keyboard, Settings};
use std::{io, thread, time::Duration};
use std::ops::Add;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "cli" {
        run_cli();
    } else {
        run_gui();
    }
}

struct SimpleApp {
    text: String,
    count: String,
    status: String,
    is_running: bool,
    time_wait: u32,
}

impl Default for SimpleApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            count: String::new(),
            time_wait: 5,
            status: String::from(""),
            is_running: false,
        }
    }
}

impl SimpleApp {
    fn spam_text(&mut self) {
        if let Ok(times) = self.count.trim().parse::<u32>() {
            self.status = String::from("Spamming in ".to_string().add(&self.time_wait.to_string()).add(" seconds..."));  // you can probably see im new to rust
            self.is_running = true;

            let text = self.text.clone();

            thread::spawn(move || {
                thread::sleep(Duration::from_secs(5));
                let mut enigo = Enigo::new(&Settings::default()).unwrap();

                for _ in 1..=times {
                    for c in text.trim().chars() {
                        enigo.text(&c.to_string()).unwrap();
                    }
                    enigo.key(Key::Return, Click).unwrap();
                    thread::sleep(Duration::from_millis(150));
                }
            });
        }
    }
}

impl eframe::App for SimpleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("simple-automated-spammer");

            ui.horizontal(|ui| {
                ui.label("Text:");
                ui.text_edit_singleline(&mut self.text);
            });

            // We don't want a slider here.
            ui.horizontal(|ui| {
                ui.label("Number of times:");
                ui.text_edit_singleline(&mut self.count);
            });
            ui.add(egui::Slider::new(&mut self.time_wait, 1..=30).text("Time to wait"));
            if ui.button("Start").clicked() && !self.is_running {
                self.spam_text();
            }

            ui.label(&self.status);
        });
    }
}

fn run_gui() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "simple-automated-spammer",
        options,
        Box::new(|_cc| Ok(Box::<SimpleApp>::default())),
    ).unwrap();
}

fn run_cli() {
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