fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .without_time()
        .with_writer(
            tracing_subscriber_wasm::MakeConsoleWriter::default()
                .map_info_level_to(tracing::Level::DEBUG),
        )
        .init();

    console_error_panic_hook::set_once();

    leptos::prelude::mount_to_body(portfolio::App);
}
