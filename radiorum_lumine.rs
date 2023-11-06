extern crate nalgebra as na;
use na::Complex;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create list of complex numbers
    let complex_numbers = vec![
        Complex::new(1.0, 2.0),
        Complex::new(3.0, 4.0),
        Complex::new(5.0, 6.0),
        Complex::new(7.0, 8.0),
    ];

    // Plot
    let root = BitMapBackend::new("complex_plot.png", (400, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    // Extract real + imaginary parts of complex numbers for visualization
    let real_parts: Vec<f64> = complex_numbers.iter().map(|c| c.re).collect();
    let imag_parts: Vec<f64> = complex_numbers.iter().map(|c| c.im).collect();

    // Plot real vs. imaginary parts
    let x_range = -10.0..10.0;
    let y_range = -10.0..10.0;
    let mut chart = ChartBuilder::on(&root)
        .caption("Complex Numbers Visualization", ("sans-serif", 20))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(x_range, y_range)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(real_parts.iter().zip(imag_parts.iter()).map(|(x, y)| {
        Circle::new((*x, *y), 5, BLACK.filled())
    }))?;

    Ok(())
}
