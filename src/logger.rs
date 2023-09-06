use log::{Level, LevelFilter, Metadata, Record};

// use log::{debug, error, info, trace, warn};
pub struct SimpleLogger;

pub fn init() {
    log::set_logger(&SimpleLogger)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("Couldn't setup the logger");
}

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Trace => {
                    println!(
                        "\t{} {:<10}:{} {:<6}{}",
                        date(),
                        record.file().unwrap(),
                        record.line().unwrap(),
                        record.level(),
                        record.args()
                    );
                }
                _ => {
                    println!(
                        "{} {:<10}:{} {:<6}{}",
                        date(),
                        record.file().unwrap(),
                        record.line().unwrap(),
                        record.level(),
                        record.args()
                    );
                }
            }
        }
    }

    fn flush(&self) {}
}

fn date() -> String {
    let offset = chrono::FixedOffset::east_opt(7200).unwrap();
    let timezone: chrono::FixedOffset = chrono::offset::TimeZone::from_offset(&offset);
    let current_time = chrono::Utc::now().with_timezone(&timezone);
    current_time.format("%F %T").to_string()
}
