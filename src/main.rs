#![allow(non_snake_case)]

use chrono::{Local, NaiveDate};
use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};

mod components;
mod util;
mod model;

use model::SavedRecord;
use components::{modals, Navbar, Records};

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

    rsx! {
        if show_modal() {
            modals::SaveRecordModal { on_record_save }
        }

        Navbar {
            timer_running: timer_running(),
            seconds_elapsed: seconds_elapsed(),
            on_timer_button_clicked,
        }

        Records { records }
    }
}
