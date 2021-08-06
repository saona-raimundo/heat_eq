use serde::{Deserialize, Serialize};

const SQRT_2PI: f64 = 2.5066282746310005024157652848110452530069867406099;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Kernel {
    Heat { time_step: f64, std_dev_inv: f64 }, // inverse of standard deviation
}

impl Default for Kernel {
    fn default() -> Self {
        Kernel::Heat {
            time_step: 1.,
            std_dev_inv: 1.,
        }
    }
}

impl Kernel {
    pub fn eval(&self, x: f64) -> f64 {
        match self {
            Kernel::Heat {
                time_step: _,
                std_dev_inv,
            } => (-0.5 * x * x * std_dev_inv * std_dev_inv).exp() * std_dev_inv / SQRT_2PI,
        }
    }
    pub fn effective_interval(&self) -> (f64, f64) {
        match self {
            Kernel::Heat {
                time_step: _,
                std_dev_inv,
            } => (-4. * std_dev_inv, 4. * std_dev_inv),
        }
    }

    pub fn set_time_step(&mut self, new_time_step: f64) -> &mut Self {
        match self {
            Kernel::Heat {
                time_step,
                std_dev_inv,
            } => {
                *time_step = new_time_step;
                *std_dev_inv = new_time_step.sqrt().recip();
            }
        }
        log::trace!("New Kernel: {:?}", self);
        self
    }
    pub fn time_step(&self) -> f64 {
        match self {
            Kernel::Heat {
                time_step,
                std_dev_inv: _,
            } => *time_step,
        }
    }
}
