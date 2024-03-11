#![allow(non_snake_case)]

use std::collections::HashMap;

use chrono::{Local, NaiveDate};
use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};

mod components;
mod util;

use components::{modals, Navbar};

use crate::util::seconds_to_formatted;

#[derive(Debug, Clone, PartialEq)]
struct SavedRecord {
    date: NaiveDate,
    time: u32,
    description: String,
}

fn main() {
    let cfg = Config::new()
        .with_default_menu_bar(false)
        .with_custom_head("<link rel=\"stylesheet\" href=\"public/styles/index.css\">".to_string())
        .with_window(WindowBuilder::default().with_title("Time Record Keeper"));

    let builder = LaunchBuilder::desktop().with_cfg(cfg);
    builder.launch(App);
}

fn App() -> Element {
    // Record data
    let mut records = use_signal(Vec::<SavedRecord>::new);

    // Timer data
    let mut timer_running = use_signal(|| false);
    let mut timer_start_date: Signal<Option<NaiveDate>> = use_signal(|| None);
    let mut seconds_elapsed = use_signal(|| 0 as u32);

    // Modal
    let mut show_modal = use_signal(|| false);

    // Start the timer
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

    // Event Handlers
    let on_timer_button_clicked = move |_| {
        if timer_running() {
            // Reset timer and save data
            show_modal.set(true);
        } else {
            // Set timer start date
            let date = Local::now().date_naive();
            timer_start_date.set(Some(date));
        }

        // Flip timer bool
        timer_running.toggle();
    };

    // Called when the save modal's save button is pressed.
    let on_record_save = move |description: String| {
        // Format
        let mut final_desc = description;
        if final_desc.is_empty() {
            final_desc = String::from("No description.");
        }

        // Save
        records.push(SavedRecord {
            date: timer_start_date().unwrap_or(Local::now().date_naive()),
            time: seconds_elapsed(),
            description: final_desc,
        });

        // Reset timer
        seconds_elapsed.set(0);
        timer_start_date.set(None);
        show_modal.set(false);
    };

    // Display logic
    let mut records_reversed = records();
    records_reversed.reverse();

    rsx! {
        if show_modal() {
            modals::SaveRecordModal { on_record_save }
        }

        Navbar {
            timer_running: timer_running(),
            seconds_elapsed: seconds_elapsed(),
            on_timer_button_clicked,
        }

        Records { records: records() }
    }
}

#[component]
fn Records(records: Vec<SavedRecord>) -> Element {
    let mut dates_and_records: HashMap<String, Vec<SavedRecord>>= HashMap::new();

    //records[0].date.to_string();

    for record in records {
        if let Some(data) = dates_and_records.get_mut(&record.date.to_string()) {
            data.push(record);
        } else {
            let mut data = Vec::new();
            data.push(record.clone());
            dates_and_records.insert(record.date.to_string(), data);
        }
    }

    let mut final_records: Vec<Element> = Vec::new();

    let mut last_key = String::new();
    for (key, value) in dates_and_records {
        if key != last_key {
            last_key = key.clone();
            final_records.push(rsx! {
                p { class: "recordDate", "{key}" }
            });
        }


        for record in value {
            let time_formatted = seconds_to_formatted(record.time);

            final_records.push(rsx! {
                div {
                    class: "record",
                    p { class: "time", "{time_formatted}" }
                    p { class: "description", "{record.description}" }
               }
            });
        }
    }

    rsx! {
        div {
            id: "records",
            p {
                class: "title",
                "Records"
            }
            for data in final_records {
                {data}
            }
        }
    }
}
