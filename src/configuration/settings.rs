use crate::kernel::Kernel;
use serde::{Deserialize, Serialize};
use splines::{Interpolation, Key, Spline};
use yew::prelude::*;

mod border_conditions;
mod fn_input;
mod storage;

pub use border_conditions::BorderConditions;
pub use fn_input::{kind::FnInputKind, FnInput};

#[derive(Debug)]
pub enum Set {
    InitialConditions(ChangeData),
    BorderConditions(ChangeData),
    Quality(ChangeData),
    Default,
    TimeStep(ChangeData),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub initial_conditions: FnInput,
    pub domain: (f64, f64), // TODO
    pub border_conditions: BorderConditions,
    pub quality: usize,
    pub kernel: Kernel,
    pub canvas_size: (u32, u32),
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            initial_conditions: FnInput::default(),
            domain: (-1., 1.),
            border_conditions: BorderConditions::default(),
            quality: 100,
            kernel: Kernel::default(),
            canvas_size: (360, 360),
        }
    }
}

impl Settings {
    pub fn update(&mut self, set: Set) -> ShouldRender {
        match set {
            Set::InitialConditions(data) => {
                if let ChangeData::Value(s) = data {
                    log::trace!("Trying to change initial conditions to {}", s);
                    let proposal = s.parse().unwrap();
                    self.initial_conditions = proposal;
                    true
                } else {
                    log::error!("Tried to change initial conditions to {:?}", data);
                    false
                }
            }
            Set::BorderConditions(data) => {
                if let ChangeData::Select(select_element) = data {
                    log::trace!(
                        "Trying to change border conditions to {:?}",
                        select_element.value()
                    );
                    let proposal = select_element.value().parse().unwrap();
                    self.border_conditions = proposal;
                    true
                } else {
                    log::error!("Tried to change border conditions to {:?}", data);
                    false
                }
            }
            Set::Quality(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change quality to {}", x);
                    let proposal: usize = x.parse().unwrap();
                    self.quality = proposal;
                    true
                } else {
                    log::error!("Tried to change quality to {:?}", data);
                    false
                }
            }
            Set::Default => {
                *self = Settings::remove_and_default();
                true
            }
            Set::TimeStep(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change time step to {}", x);
                    let proposal: f64 = x.parse().unwrap();
                    self.kernel.set_time_step(proposal);
                    true
                } else {
                    log::error!("Tried to change time step to {:?}", data);
                    false
                }
            }
        }
    }

    pub fn compute_initial_spline(&self) -> Spline<f64, f64> {
        let grid = itertools_num::linspace(self.domain.0, self.domain.1, self.quality);

        Spline::from_iter(grid.map(|x| {
            let y = self.initial_conditions.eval(x);
            Key::new(x, y, Interpolation::Cosine)
        }))
    }
}
