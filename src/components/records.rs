use dioxus::prelude::*;
use std::collections::HashMap;

use crate::{model::SavedRecord, util::seconds_to_formatted};

#[component]
pub fn Records(records: Signal<Vec<SavedRecord>>) -> Element {
    let mut records_copy = records();
    records_copy.sort_by(|x, y| x.date.cmp(&y.date));

    let mut dates_and_records: HashMap<String, Vec<SavedRecord>> = HashMap::new();

    for record in records_copy {
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

            if final_records.is_empty() {
                p { 
                    id: "noRecordsText", 
                    "You don't have any saved records. 😕"
                }
            }

            for data in final_records {
                {data}
            }
        }
    }
}
