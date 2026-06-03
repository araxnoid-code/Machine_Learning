use std::{array, default, fmt::Debug};

fn main() {
    let feature_type = [
        ConditionType::Boolean,
        ConditionType::Boolean,
        ConditionType::Boolean,
        ConditionType::Boolean,
        ConditionType::Boolean,
    ];

    let features = [
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
    ];

    let labels = [1, 1, 1, 1, 1, 0, 0, 0, 0, 0];

    build_tree(feature_type, &features, &labels);
}

#[derive(Debug)]
enum ConditionType {
    Boolean,
}

#[derive(Debug)]
enum ConditionArg {
    Boolean(bool),
}

struct Node {
    condition: ConditionArg,
    left: Box<Node>,
    right: Box<Node>,
}

fn build<const FEATURE_COUNT: usize>(
    feature_type: [(&ConditionType, usize); FEATURE_COUNT],
    feature_len: usize,
    features: &[[ConditionArg; FEATURE_COUNT]],
    labels: &[usize],
) {
    let scores: [f64; FEATURE_COUNT] = array::from_fn(|_| 0.);
    for scores_idx in 0..feature_len {
        let (con_type, column) = feature_type[scores_idx];

        if let ConditionType::Boolean = con_type {
            let mut result = [
                // label 0
                // (false, true)
                (0, 0),
                // label 1
                // (false, true)
                (0, 0),
            ];

            for (row_idx, feature_row) in features.iter().enumerate() {
                println!("{:?} => {:?}", feature_row[column], labels[row_idx]);

                let label_idx = labels[row_idx];
                if let ConditionArg::Boolean(status) = feature_row[column] {
                    if status {
                        result[label_idx].1 += 1;
                    } else {
                        result[label_idx].0 += 1;
                    }
                } else {
                    panic!()
                }
            }

            // left
            let prob_left = result[0].0 as f64 / (result[0].0 + result[0].1) as f64;
            let left_entropy = -(prob_left * prob_left.ln());

            // right
            let prob_right = result[1].0 as f64 / (result[1].0 + result[1].1) as f64;
            let right_entropy = -(prob_right * prob_right.ln());

            println!("data final:");
            println!("raw {:?}", result);
            println!("left entropy: {}", left_entropy);
            println!("right entropy: {}", right_entropy);
        }

        println!("====");
    }
}

fn build_tree<const FEATURE_COUNT: usize>(
    feature_type: [ConditionType; FEATURE_COUNT],
    features: &[[ConditionArg; FEATURE_COUNT]],
    labels: &[usize],
) {
    let feature_type: [(&ConditionType, usize); FEATURE_COUNT] =
        array::from_fn(|idx| (&feature_type[idx], idx));

    build(feature_type, FEATURE_COUNT, features, labels);
}
