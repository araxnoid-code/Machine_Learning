use knn::{Distance, k_nearest_neighbor};

fn main() {
    let datasets = [
        [85., 90.],
        [88., 85.],
        [82., 88.],
        [60., 55.],
        [58., 62.],
        [65., 58.],
    ];
    let labels = [0, 0, 0, 1, 1, 1];
    let input = [[75., 80.], [55., 63.], [65., 58.]];

    k_nearest_neighbor(3, datasets, labels, 2, Distance::Euclidean, input);
}
