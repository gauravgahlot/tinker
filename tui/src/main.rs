mod app;
mod ui;

use anyhow::Result;
use ratatui;

#[tokio::main]
async fn main() -> Result<()> {
    let tink = tlib::client::TinkerbellClient::connect().await?;
    let ns = tink.namespace();

    let mut terminal = ratatui::init();
    let app_result = app::App::new(ns.as_str()).run(&mut terminal);

    ratatui::restore();

    app_result
}
