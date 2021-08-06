use crate::Analysis;
use yew::prelude::*;

pub mod settings;
use settings::{Set, Settings};

#[derive(Debug)]
pub enum Msg {
    SetReceiver(ComponentLink<Analysis>),
    UpdateReceiver,
    Auxiliary(Set),
}

#[derive(Debug)]
pub struct Configuration {
    link: ComponentLink<Self>,
    settings: Settings,
    analysis: Option<ComponentLink<Analysis>>,
}

impl Component for Configuration {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            settings: Settings::restore_or_default(),
            analysis: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetReceiver(receiver) => {
                log::trace!("Connecting to a receiver");
                self.analysis = Some(receiver);
                false
            }
            Msg::UpdateReceiver => {
                log::trace!("Start to update receiver");
                log::trace!("Storing settings");
                self.settings.store().unwrap();
                log::trace!("Updating receiver");
                self.analysis
                    .as_mut()
                    .unwrap()
                    .send_message(crate::analysis::Msg::RestartFrom(self.settings.clone()));
                false
            }
            Msg::Auxiliary(set) => {
                log::trace!("Changing a seeting");
                self.settings.update(set)
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>
                { "Configuration" }
                <br/>

                <div>
                    { "Initial conditions" }
                    <input
                        type="text"
                        id="initial_conditions"
                        name="initial_conditions"
                        value=self.settings.initial_conditions.string.clone()
                        onchange=self.link.callback(move |f| Msg::Auxiliary(Set::InitialConditions(f)))
                    />
                    <div class="tooltip">{ "Available fomats?" }
                        <span class="tooltiptext">{ "analytical: sin({x})\npoints: [(0, 2), (1, 3.5)]" }</span>
                    </div>
                </div>
                <div>
                    { "Time step" }
                    <input type="number" id="time_step" name="time_step" min="0" max="100" value=self.settings.kernel.time_step().to_string() onchange=self.link.callback(|x| Msg::Auxiliary(Set::TimeStep(x)))/>
                </div>
                <div>
                    { "Border conditions" }
                    <select
                        name="border_conditions"
                        id="border_conditions"
                        onchange=self.link.callback(|x| Msg::Auxiliary(Set::BorderConditions(x)))
                    >
                        <option value="Fixed">{ "Fixed" }</option>
                        <option value="Periodic">{ "Periodic" }</option>
                    </select>
                </div>
                <div>
                    { "Quality" }
                    <input type="range" id="quality" name="quality" min="2" max="100" value=self.settings.quality.to_string() class="slider" onchange=self.link.callback(|x| Msg::Auxiliary(Set::Quality(x)))/>
                </div>
                <div>
                    <button type="button" id="update_receiver" name="update_receiver" onclick=self.link.callback(|_| Msg::UpdateReceiver)>{ "Update" }</button>
                    <button type="button" id="default" name="default" onclick=self.link.callback(|_| Msg::Auxiliary(Set::Default))>{ "Default" }</button>
                </div>
            </p>
        }
    }
}
