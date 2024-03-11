use crate::util::seconds_to_formatted;
use dioxus::prelude::*;

#[component]
pub fn Navbar(
    on_timer_button_clicked: EventHandler,
    timer_running: bool,
    seconds_elapsed: u32,
) -> Element {
    // Format timer button text
    let (timer_text, timer_button_id) = match timer_running {
        true => ("STOP", "stopButton"),
        false => ("START", "startButton"),
    };

    // Format time
    let time_elapsed = seconds_to_formatted(seconds_elapsed);

    rsx! {
        div {
            id: "navbar",
            button {
                class: "timerButton navItem",
                id: "{timer_button_id}",
                onclick: move |_| on_timer_button_clicked.call(()),
                "{timer_text}"
            }

            // Show time if timer is running
            if timer_running {
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
