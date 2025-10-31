use tracing_subscriber::fmt;

pub fn setup_logging(debug: bool) {
    if debug {
        // Debug Mode: Structured JSON für Entwicklung
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_target(true)
            .json()
            .init();
    } else {
        // Production Mode: DJB-Style minimal
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_timer(Tai64nTimer::default())
            .with_target(false)
            .with_level(false)
            .compact()
            .init();
    }
}

// TAI64N Timer Implementation
#[derive(Default)]
struct Tai64nTimer;

impl fmt::time::FormatTime for Tai64nTimer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();

        // TAI64N format: @4000000067129f3c1a2b3c5c
        let tai64n = 0x4000000000000000u64 + now.as_secs();
        let nano_part = now.subsec_nanos();

        write!(w, "@{:016x}{:08x}", tai64n, nano_part)
    }
}