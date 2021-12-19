pub mod my_log{
    use log::{LevelFilter};
    use log4rs::Handle;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::config::{Appender, Config as LogConfig, Root};
    use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
    use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
    use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
    use log4rs::filter::threshold::ThresholdFilter;
    use log4rs::append::rolling_file::RollingFileAppender;


    pub fn create_logging(window_size: u32, size_limit: u64, filename: &str) -> Handle{
        
        let fixed_window_roller = FixedWindowRoller::builder().build("log.{}",window_size).unwrap();
        let size_trigger = SizeTrigger::new(size_limit);
        let compound_policy = CompoundPolicy::new(Box::new(size_trigger),Box::new(fixed_window_roller));
    
        let config = LogConfig::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Error)))
                .build(
                    filename,
                    Box::new(
                        RollingFileAppender::builder()
                            .encoder(Box::new(PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)(local)} - {l}: {m}{n})}")))
                            .build(filename, Box::new(compound_policy)).unwrap(),
                    ),
                ),
        )
        .build(
            Root::builder()
                .appender(filename)
                .build(LevelFilter::Error),
        ).unwrap();
    
        return log4rs::init_config(config).unwrap();
    }
}