use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use splines::Spline;
use yew::prelude::*;

/// Function at a current time
#[derive(Debug)]
pub struct Current {
    pub canvas_ref: NodeRef,
}

impl Current {
    pub fn new(canvas_ref: NodeRef) -> Self {
        Self { canvas_ref }
    }

    pub fn plot(
        &self,
        backend: CanvasBackend,
        current_time: usize,
        domain: (f64, f64),
        value_limits: (f64, f64),
        spline: &Spline<f64, f64>,
    ) {
        // Pre-computations
        let (mut min, mut max) = value_limits;
        log::trace!("min/max values of the plot: ({}, {})", min, max,);
        if !min.is_finite() {
            log::error!("min value is not real!");
            min = -1.;
            log::warn!("min value changed to {}", min);
        }
        if !max.is_finite() {
            log::error!("max value is not real!");
            max = 1.;
            log::warn!("max value changed to {}", max);
        }

        // Plot spline
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();

        // Plottings

        let keys = spline.keys();
        let mut chart_builder = ChartBuilder::on(&root);
        chart_builder.set_label_area_size(LabelAreaPosition::Bottom, 40);
        chart_builder.set_label_area_size(LabelAreaPosition::Left, 40);
        let title = format!("Time {}", current_time);
        chart_builder.caption(title, ("Arial", 30));

        let delta = max - min;
        let mut chart = chart_builder
            .build_cartesian_2d(
                domain.0..domain.1,
                (min - delta / 100.)..(max + delta / 100.),
            )
            .unwrap();

        let mut mesh_style = chart.configure_mesh();
        mesh_style.draw().unwrap();
        chart
            .draw_series(LineSeries::new(keys.iter().map(|k| (k.t, k.value)), &BLACK))
            .unwrap();
    }
}
