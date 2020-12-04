pub fn setup() {
    flexi_logger::Logger::with_env_or_str("debug")
        .log_to_file()
        .directory(".logs")
        .start()
        .unwrap();
}
