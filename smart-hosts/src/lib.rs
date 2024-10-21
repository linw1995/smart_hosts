#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

mod core;
pub mod monitor;
