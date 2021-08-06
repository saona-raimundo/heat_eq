#![recursion_limit = "2048"]
use yew::html::Scope;
use yew::prelude::*;

mod analysis;
mod configuration;
mod kernel;

use analysis::Analysis;
use configuration::{Configuration, Msg};

fn main() -> anyhow::Result<()> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    log::trace!("Mounting two applications");
    let (configuration, analysis) = mount()?;
    log::trace!("Enabling configuration -> analysis communication");
    configuration.send_message(Msg::SetReceiver(analysis.clone()));
    configuration.send_message(Msg::UpdateReceiver);

    Ok(())
}

fn mount() -> anyhow::Result<(Scope<Configuration>, Scope<Analysis>)> {
    let document = yew::utils::document();
    let element = document.query_selector(".configuration").unwrap().unwrap();
    let configuration = App::<Configuration>::new().mount(element);
    let element = document.query_selector(".analysis").unwrap().unwrap();
    let analysis = App::<Analysis>::new().mount(element);

    Ok((configuration, analysis))
}
