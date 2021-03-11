use env_logger::Builder;
use log::LevelFilter;

pub fn init_logger() {
    let mut log_builder = Builder::from_default_env();
    log_builder.filter_level(LevelFilter::Debug);
    log_builder.init();
    debug!("Succesfully initalized logger");
}
