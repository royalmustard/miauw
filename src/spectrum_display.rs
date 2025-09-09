use std::sync::{Arc, Mutex};

use num_complex::Complex32;
use plotters_iced::{Chart, ChartWidget, DrawingBackend, ChartBuilder};
use plotters::prelude::*;

struct SpectrumPlot;

#[derive(Default)]
struct SpectrumPlotState
{
    data: Arc<Mutex<Vec<Complex32>>>
}

impl<Message> Chart<Message> for SpectrumPlot
{
    type State = SpectrumPlotState;

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder.build_cartesian_2d(0..24000, -100..100).expect("Error building chart");

        chart.configure_mesh().draw().expect("Error drawing chart mesh");

        
    }
}