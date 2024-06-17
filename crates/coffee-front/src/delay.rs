use std::time::Duration;

async fn a() {
    wasm_timer::Delay::new(Duration::from_secs(10)).await.unwrap();

}