use opentelemetry::sdk::export::trace::stdout;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Registry};

pub fn _setup_optl() {
    let tracer = stdout::new_pipeline().install_simple();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tracing::instrument]
fn doing_work() {
    for i in 0..10 {
        println!("doing work {}", i);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_requires_role() {
        _setup_optl();
        doing_work();
    }
}
