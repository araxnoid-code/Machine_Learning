use std::{array, default, fmt::Debug};

const E: f64 = 0.000000000001;

fn main() {
    let feature_type = [
        ConditionType::Boolean,
        ConditionType::Boolean,
        ConditionType::Boolean,
    ];

    let features = [
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
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

#[derive(Debug, Clone, Copy)]
enum ConditionArg {
    Boolean(bool),
}

#[derive(Debug)]
enum Child<'a> {
    Node(Node<'a>),
    Class(usize),
}

#[derive(Debug)]
struct Node<'a> {
    condition: &'a ConditionType,
    left: Box<Child<'a>>,
    right: Box<Child<'a>>,
}

fn build<const FEATURES_COUNT: usize>(
    labeled_features: &[(&[ConditionArg; FEATURES_COUNT], usize)],
    mut indexed_feature_type: [(&ConditionType, usize); FEATURES_COUNT],
    able_len: usize,
) {
    let mut minimum_score = None;
    for column in 0..able_len {
        let (column_type, column_idx) = indexed_feature_type[column];
        println!("column index: {}", column_idx);

        if let ConditionType::Boolean = column_type {
            // left (false)
            let mut left_result = [0, 0];
            let mut right_result = [0, 0];

            for (feature, label) in labeled_features {
                if let ConditionArg::Boolean(status) = feature[column] {
                    if status {
                        right_result[*label] += 1;
                    } else {
                        left_result[*label] += 1;
                    }
                }
            }

            // left entropy
            let left_total = (left_result[0] + left_result[1]) as f64;
            let left_entropy = -(left_result[0] as f64 / left_total)
                * (left_result[0] as f64 / left_total).log2()
                - (left_result[1] as f64 / left_total)
                    * (left_result[1] as f64 / left_total).log2();

            // right entropy
            let right_total = (right_result[0] + right_result[1]) as f64;
            let right_entropy = -(right_result[0] as f64 / right_total)
                * (right_result[0] as f64 / right_total).log2()
                - (right_result[1] as f64 / right_total)
                    * (right_result[1] as f64 / right_total).log2();

            // score
            let parent_total = labeled_features.len() as f64;
            let score = (left_total / parent_total) * left_entropy
                + (right_total / parent_total) * right_entropy;

            println!("raw result:");
            println!("left result: {:?}", left_result);
            println!("right result: {:?}", right_result);
            println!("entropy:");
            println!("left_entropy : {:?}", left_entropy);
            println!("right_entropy : {:?}", right_entropy);
            println!("score : {:?}", score);

            if let Some((min_score, min_column)) = &mut minimum_score {
                if *min_score > score {
                    *min_score = score;
                    *min_column = column;
                }
            } else {
                minimum_score = Some((score, column));
            }
        }

        println!("===========================");
    }
    println!("minimum features is {:?}", minimum_score.unwrap());

    // swap
    indexed_feature_type.swap(minimum_score.unwrap().1, able_len - 1);

    // split
    let mut left_features = vec![];
    let mut right_features = vec![];

    let column = minimum_score.unwrap().1;
    for (feature, label) in labeled_features {
        if let ConditionArg::Boolean(status) = feature[column] {
            if status {
                right_features.push((*feature, *label));
            } else {
                left_features.push((*feature, *label));
            }
        }
    }

    println!("before splitting");
    for feature in labeled_features {
        println!("{:?}", feature);
    }

    println!("after splitting");
    println!("left:");
    for feature in left_features {
        println!("{:?}", feature);
    }

    println!("right:");
    for feature in right_features {
        println!("{:?}", feature);
    }
}

fn build_tree<const FEATURES_COUNT: usize>(
    feature_type: [ConditionType; FEATURES_COUNT],
    features: &[[ConditionArg; FEATURES_COUNT]],
    labels: &[usize],
) {
    let labeled_features = features
        .iter()
        .enumerate()
        .map(|(idx, feature)| (feature, labels[idx]))
        .collect::<Vec<(&[ConditionArg; FEATURES_COUNT], usize)>>();

    let indexed_feature_type: [(&ConditionType, usize); FEATURES_COUNT] =
        array::from_fn(|idx| (&feature_type[idx], idx));

    build(&labeled_features, indexed_feature_type, FEATURES_COUNT);
}
