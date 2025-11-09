mod add_expense;
mod analysis;
mod app;
mod commands;
mod components;
mod manage_limits;
mod overview;
mod state;
mod transaction_log;
mod view_reports;

use app::App;
use tracing_subscriber::{filter::Targets, prelude::*};
use tracing_web::MakeWebConsoleWriter;

fn main() {
    // Configure tracing to output to browser console
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true) // This might not work in all browsers
        .without_time() // std::time is not available in browsers
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(
            Targets::new()
                .with_target("yew", tracing::Level::DEBUG) // yew trace can be verbose
                .with_default(tracing::Level::TRACE),
        );

    tracing_subscriber::registry().with(fmt_layer).init();

    tracing::info!("Starting budget-rs application");
    yew::Renderer::<App>::new().render();
}
