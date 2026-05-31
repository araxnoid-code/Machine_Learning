pub fn k_nearest_neighbor<const D: usize, const N: usize, const B: usize>(
    k: usize,
    datasets: [[f64; D]; N],
    labels: [usize; N],
    total_label: usize,
    batch: [[f64; D]; B],
) -> [Vec<i32>; B] {
    batch.map(|input| {
        let mut distance = datasets
            .iter()
            .enumerate()
            .map(|(idx, data)| {
                // euclidean distance
                let mut distance = 0.;
                for i in 0..D {
                    distance += (data[i] - input[i]).powi(2);
                }
                let result = (distance.sqrt(), labels[idx]);

                result
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
