pub enum Distance {
    Euclidean,
    Manhattan,
    Minkowski(i32),
}

impl Default for Distance {
    fn default() -> Self {
        Self::Euclidean
    }
}

pub fn k_nearest_neighbor<const D: usize, const N: usize, const B: usize>(
    k: usize,
    datasets: [[f64; D]; N],
    labels: [usize; N],
    total_label: usize,
    distance: Distance,
    batch: [[f64; D]; B],
) -> [Vec<i32>; B] {
    batch.map(|input| {
        let mut distance = datasets
            .iter()
            .enumerate()
            .map(|(idx, data)| match distance {
                Distance::Euclidean => (
                    data.iter()
                        .zip(input.iter())
                        .map(|(a, b)| (a - b).powi(2))
                        .sum::<f64>()
                        .sqrt(),
                    labels[idx],
                ),

                Distance::Manhattan => (
                    data.iter()
                        .zip(input.iter())
                        .map(|(a, b)| (a - b).abs())
                        .sum::<f64>(),
                    labels[idx],
                ),

                Distance::Minkowski(p) => (
                    data.iter()
                        .zip(input.iter())
                        .map(|(a, b)| (a - b).powi(p))
                        .sum::<f64>()
                        .powf(1. / p as f64),
                    labels[idx],
                ),
            })
            .collect::<Vec<(f64, usize)>>();
        distance.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut class = vec![0; total_label];
        if k < N { &distance[..k] } else { &distance[..] }
            .iter()
            .for_each(|&(_, i)| class[i] += 1);

        class
    })
}
