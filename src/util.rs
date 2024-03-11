/// Formats seconds into a time format with hours, minutes, and seconds.
/// Example: 61 seconds would be 00:01:01
pub fn seconds_to_formatted(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds_final = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds_final)
}
