#![allow(non_snake_case)]

use chrono::{Local, NaiveDate};
use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};

#[derive(Clone, Copy)]
struct SavedRecord {
    date: NaiveDate,
    time: u32,
}

fn main() {
    let cfg = Config::new()
        .with_default_menu_bar(false)
        .with_custom_head("<link rel=\"stylesheet\" href=\"public/styles/index.css\">".to_string())
        .with_window(WindowBuilder::default().with_title("Time Tracker"));

    let builder = LaunchBuilder::desktop().with_cfg(cfg);
    builder.launch(App);
}

fn App() -> Element {
    let records = use_signal(Vec::<SavedRecord>::new);

    let mut records_reversed = records();
    records_reversed.reverse();

    rsx! {
        Navbar { records }
        div { class: "title", "Recorded Time" }
        div {
            id: "recordsContainer",

            for record in records_reversed {
                Record { date: record.date, seconds: record.time }
            }
        }
    }
}

#[component]
fn Record(date: NaiveDate, seconds: u32) -> Element {
    let time_formatted = seconds_to_formatted(seconds);
    rsx! {
        div {
            class: "record",
            p { class: "date", "{date}" }
            p { class: "time", "{time_formatted}" }
        }
    }
}

fn seconds_to_formatted(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds_final = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds_final)
}

#[component]
fn Navbar(records: Signal<Vec<SavedRecord>>) -> Element {
    let mut timer_running = use_signal(|| false);
    let mut timer_start_date: Signal<Option<NaiveDate>> = use_signal(|| None);
    let mut seconds_elapsed = use_signal(|| 0 as u32);

    use_effect(move || {
        spawn_forever(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                if timer_running() {
                    seconds_elapsed += 1;
                }
            }
        });
    });

    // Format timer button text
    let (timer_text, timer_button_id) = match timer_running() {
        true => ("STOP", "stopButton"),
        false => ("START", "startButton"),
    };

    // Format time
    let time_elapsed = seconds_to_formatted(seconds_elapsed());

    rsx! {
        div {
            id: "navbar",
            button {
                class: "timerButton navItem",
                id: "{timer_button_id}",
                onclick: move |_| {
                    if timer_running() {
                        // Reset timer and save data

                        records.push(SavedRecord {
                            date: timer_start_date().unwrap_or(Local::now().date_naive()),
                            time: seconds_elapsed(),
                        });

                        seconds_elapsed.set(0);
                        timer_start_date.set(None);
                    } else {
                        // Set timer start date
                        let date = Local::now().date_naive();
                        timer_start_date.set(Some(date));
                    }

                    // Flip timer bool
                    timer_running.toggle();
                },
                "{timer_text}"
            }

            if timer_running() {
                p {
                    class: "navItem",
                    "{time_elapsed}"
                }
            }

            button {
                class: "navItem",
                id: "exportButton",
                "EXPORT"
            }
        }
    }
}
