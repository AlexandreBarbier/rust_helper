use chrono::{offset::Utc, DateTime};
use std::io::Write;
use std::time::SystemTime;

pub fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            let datetime: DateTime<Utc> = SystemTime::now().into();
            let file_line = match (record.file(), record.line()) {
                (Some(file), Some(line)) if record.level() == log::Level::Error => {
                    format!(" -{} {}-", file, line)
                }
                _ => String::new(),
            };
            writeln!(
                buf,
                "{} {}:{} {}",
                datetime.format("%T %D"),
                record.level(),
                file_line,
                record.args()
            )
        })
        .init();
}

pub fn get_log_format() -> &'static str {
    "%s %r - %{r}a %Dms"
}
