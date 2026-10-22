use fern::colors::{Color, ColoredLevelConfig};

pub fn setup_logger(level: Option<log::LevelFilter>) -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let datetime = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S");
            let colors = ColoredLevelConfig::new()
                .debug(Color::Green)
                .info(Color::Blue)
                .warn(Color::TrueColor { r: 0xE0u8, g: 0x68u8, b: 0x26u8 });

            out.finish(format_args!(
                "[{}] {} {} {}",
                colors.color(record.level()),
                record.target(),
                datetime,
                message
            ))
        })
        .level(level.or(log::LevelFilter::Debug))
        .chain(std::io::stderr())
        // .chain(fern::log_file("mp3-organizer.log"))
        .apply()?;
    Ok(())
}
