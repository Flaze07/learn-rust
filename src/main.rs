pub trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct Filter<T: Logger> {
    logger: T,
    filter_func: fn(u8, &str) -> bool,
}

impl <T: Logger> Filter<T> {
    fn new(logger: T, filter_func: fn(u8, &str) -> bool) -> Filter<T> {
        Filter {
            logger: logger,
            filter_func: filter_func,
        }
    }
}

impl <T: Logger> Logger for Filter<T> {
    fn log(&self, verbosity: u8, message: &str) {
        if ((self.filter_func)(verbosity, message)) {
            self.logger.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, Something went wrong");
    logger.log(2, "uhoh");
}