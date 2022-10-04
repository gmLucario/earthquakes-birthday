use crate::models::Earthquake;

pub fn get_mean_magnitude(earthquakes: &Vec<Earthquake>) -> f32 {
    let sum = earthquakes
        .iter()
        .map(|eq| eq.magnitude)
        .collect::<Vec<f32>>()
        .iter()
        .sum::<f32>();
    let count = earthquakes.len() as f32;

    sum / count
}
