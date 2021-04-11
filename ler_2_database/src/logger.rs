use env_logger::Env;

pub fn init_logger() {
    let log = Env::new().default_filter_or("warn");
    env_logger::init_from_env(log);
    debug!("Succesfully initalized logger");
}
