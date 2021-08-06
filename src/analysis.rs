use crate::configuration::settings::Settings;
use itertools::Itertools;
use plotters_canvas::CanvasBackend;
use splines::{Key, Spline};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

mod cummulative;
mod current;

use cummulative::Cummulative;
use current::Current;

#[derive(Debug)]
pub enum Msg {
    RestartFrom(Settings),
    Time(ChangeData),
    Advance,
}

#[derive(Debug)]
pub struct Analysis {
    link: ComponentLink<Self>,
    current: Current,
    cummulative: Cummulative,
    values: Vec<Spline<f64, f64>>,
    current_time: usize,
    max_time: usize,
    limit_values: (f64, f64),
    settings: Settings,
}

impl Component for Analysis {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let current = Current::new(NodeRef::default());
        let cummulative = Cummulative::new(NodeRef::default());
        Self {
            link,
            current,
            cummulative,
            values: vec![],
            current_time: 0,
            max_time: 0,
            limit_values: (0., 0.),
            settings: Settings::default(), // they are updated anyway
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Time(data) => {
                if let ChangeData::Value(s) = data {
                    log::trace!("Trying to change time to {}", s);
                    let proposal: usize = s.parse().unwrap();
                    self.current_time = proposal;
                    // TODO: Update current plot
                    true
                } else {
                    log::error!("Tried to change time to {:?}", data);
                    false
                }
            }
            Msg::Advance => {
                log::trace!("Advancing one step");
                if self.current_time < self.max_time {
                    self.current_time += 1;
                } else {
                    log::trace!("Computing a new time point");
                    self.compute_next();
                    self.current_time += 1;
                    self.max_time = self.current_time.max(self.max_time);
                }
                true
            }
            Msg::RestartFrom(settings) => {
                log::trace!("Restarting from new settings");
                let spline = settings.compute_initial_spline();
                self.limit_values = spline
                    .keys()
                    .iter()
                    .map(|k| k.value)
                    .minmax()
                    .into_option()
                    .unwrap();
                self.values = vec![spline];
                self.settings = settings;
                self.current_time = 0;
                self.max_time = 0;

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>
                { "Analysis" }
                <div>
                    <canvas ref={self.cummulative.canvas_ref.clone()} />
                    <canvas ref={self.current.canvas_ref.clone()} />
                </div>
                <div>
                    { "Time" }
                    <input type="range" id="time" name="time" min="0" max=self.max_time.to_string() value=self.current_time.to_string() class="slider" onchange=self.link.callback(|x| Msg::Time(x))/>
                    <button onclick=self.link.callback(|_| Msg::Advance)>{ "Advance" }</button>
                </div>
            </p>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let current_canvas: HtmlCanvasElement =
            self.current.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let cummulative_canvas: HtmlCanvasElement = self
            .cummulative
            .canvas_ref
            .cast::<HtmlCanvasElement>()
            .unwrap();

        if first_render {
            log::trace!("First render of Analysis");
            current_canvas.set_width(self.settings.canvas_size.0);
            current_canvas.set_height(self.settings.canvas_size.1);
            cummulative_canvas.set_width(self.settings.canvas_size.0);
            cummulative_canvas.set_height(self.settings.canvas_size.1);
        } else {
            log::trace!("Rerendering Analysis");
            let current_backend: CanvasBackend =
                CanvasBackend::with_canvas_object(current_canvas).unwrap();
            self.current.plot(
                current_backend,
                self.current_time,
                self.settings.domain,
                self.limit_values,
                &self.values[self.current_time],
            );
            let cummulative_backend: CanvasBackend =
                CanvasBackend::with_canvas_object(cummulative_canvas).unwrap();
            self.cummulative
                .plot(
                    cummulative_backend,
                    self.current_time,
                    self.limit_values,
                    &self.values,
                )
                .unwrap();
        }
    }
}

impl Analysis {
    /// Computes the next time point and saves the result.
    fn compute_next(&mut self) -> &mut Self {
        let kernel = &self.settings.kernel;
        log::trace!("Retrieving current spline");
        let last_spline = self.values.last().unwrap();

        let border_conditions = &self.settings.border_conditions;

        let mut new_keys = vec![];
        log::trace!("Computing new spline");
        // match border_conditions {

        // }
        for key in last_spline.keys() {
            let Key { t, .. } = key;
            let x = t;

            let integrand =
                |z: f64| -> f64 { kernel.eval(x - z) * last_spline.clamped_sample(z).unwrap() };
            let effective_interval = kernel.effective_interval();

            let new_value =
                quadrature::integrate(integrand, effective_interval.0, effective_interval.1, 1e-7)
                    .integral;

            new_keys.push(splines::Key::new(
                *x,
                new_value,
                splines::Interpolation::Cosine,
            ));
        }
        self.values.push(Spline::from_vec(new_keys));
        self
    }
}
