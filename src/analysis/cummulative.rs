use itertools::Itertools;
use nalgebra::DMatrix;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use splines::Spline;
use yew::prelude::*;

/// Function at a current time
#[derive(Debug)]
pub struct Cummulative {
    pub canvas_ref: NodeRef,
}

impl Cummulative {
    pub fn new(canvas_ref: NodeRef) -> Self {
        Self { canvas_ref }
    }

    /// Assumes that all Spline has the same uniform grid
    pub fn plot(
        &self,
        backend: CanvasBackend,
        current_time: usize,
        limit_values: (f64, f64),
        splines: &Vec<Spline<f64, f64>>,
    ) -> anyhow::Result<()> {
        // Pre-computations
        let (mut min, mut max) = limit_values;
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

        let rows = splines[0].keys().len();
        let cols = splines.len();
        log::trace!("Plotting a {}x{} matrix", rows, cols);
        let mut matrix = DMatrix::from_element(rows, cols, 0.0); // , 0.0);
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = splines[j].get(i).unwrap().value;
            }
        }

        plot_matrix(backend, matrix, limit_values, current_time)?;

        Ok(())
    }
}

/// Plot a matrix when a index highlighted by using a different color scheme.
///
/// The value of the matrix corresponds to the color value, which are rescaled by `limit_values`.
fn plot_matrix(
    backend: CanvasBackend,
    matrix: DMatrix<f64>,
    limit_values: (f64, f64),
    highlighted_index: usize,
) -> anyhow::Result<()> {
    let gradient = colorous::VIRIDIS;
    let highlight_gradient = colorous::MAGMA;

    let values_recip = (limit_values.1 - limit_values.0).recip();
    let value_scaling = |v| (v - limit_values.0) * values_recip;

    let (rows, columns) = matrix.shape();

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root).build_cartesian_2d(0..rows, 0..columns)?;
    chart.configure_mesh().draw()?;

    chart.draw_series(
        (0..rows)
            .cartesian_product(0..columns)
            .map(|(i, j)| (i, j, value_scaling(matrix[(i, j)])))
            .map(|(i, j, v)| {
                Rectangle::new([(i, j), (i + 1, j + 1)], {
                    let color = if j == highlighted_index {
                        highlight_gradient.eval_continuous(v)
                    } else {
                        gradient.eval_continuous(v)
                    };
                    RGBColor(color.r, color.g, color.b).filled()
                })
            }),
    )?;

    root.present()?;

    Ok(())
}
