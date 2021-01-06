use flexi_logger::{DeferredNow, Record};

pub fn setup() {
    flexi_logger::Logger::with_env_or_str("debug")
        .log_to_file()
        .directory(".logs")
        .format(log_format)
        .start()
        .unwrap();
}

fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} {} [{}] {}",
        now.now().to_rfc3339_opts(chrono::SecondsFormat::Millis, false),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.args()
    )
}
