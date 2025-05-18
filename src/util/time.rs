pub fn format_time(ms: u32) -> String {
    let seconds = ms / 1000;
    let milliseconds = ms % 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let seconds = seconds % 60;
    let minutes = minutes % 60;
    format!(
        "{}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    )
}
