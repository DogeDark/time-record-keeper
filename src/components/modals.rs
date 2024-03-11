use dioxus::prelude::*;

#[component]
pub fn SaveRecordModal(on_record_save: EventHandler<String>) -> Element {
    let mut description_text = use_signal(String::new);

    let textarea_class = if !description_text().is_empty() {
        "active"
    } else {
        ""
    };

    rsx! {
        div {
            id: "saveRecordModal",
            div {
                id: "content",

                p {
                    id: "header",
                    "Save Record"
                }

                div {
                    class: "formField",
                    p { "Description"}
                    textarea {
                        class: "{textarea_class}",
                        placeholder: "(Optional)",
                        value: "{description_text}",
                        oninput: move |data| {
                            let text = data.value();
                            description_text.set(text);
                        },
                    }
                }

                div {
                    id: "footer",
                    button {
                        onclick: move |_| on_record_save.call(description_text()),
                        "Save"
                    }
                }
            }
        }
    }
}
