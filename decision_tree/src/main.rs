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

fn build<'a, const FEATURE_COUNT: usize>(
    mut feature_type: [(&'a ConditionType, usize); FEATURE_COUNT],
    feature_len: usize,
    features: &[[ConditionArg; FEATURE_COUNT]],
    labels: &[usize],
) -> Child<'a> {
    if feature_len == 0 {
        for feature in features {
            println!("{:?}", feature);
        }
        println!("=================");
        return Child::Class(0);
    }

    let mut max: Option<(f64, &ConditionType, usize)> = None;
    for scores_idx in 0..feature_len {
        let (con_type, column) = feature_type[scores_idx];

        if let ConditionType::Boolean = con_type {
            // false [0, 1]
            let mut false_result = [0, 0];

            // true [0, 1]
            let mut true_result = [0, 0];

            for (row_idx, feature_row) in features.iter().enumerate() {
                // println!("{:?} => {:?}", feature_row[column], labels[row_idx]);

                let label_idx = labels[row_idx];
                if let ConditionArg::Boolean(status) = feature_row[column] {
                    if status {
                        true_result[label_idx] += 1;
                    } else {
                        false_result[label_idx] += 1;
                    }
                } else {
                    panic!()
                }
            }

            // entropy
            // parent
            let parent_total = features.len() as f64;

            let category_zero = (false_result[0] + true_result[0]) as f64;
            let category_one = (false_result[1] + true_result[1]) as f64;
            let parent_entropy = -(category_zero / parent_total)
                * (category_zero / parent_total).log2()
                - (category_one / parent_total) * (category_one / parent_total).log2();

            // left
            let left_total = (false_result[0] + false_result[1]) as f64;
            let left_entropy = -(false_result[0] as f64 / left_total)
                * (false_result[0] as f64 / left_total).log2()
                - (false_result[1] as f64 / left_total)
                    * (false_result[1] as f64 / left_total).log2();

            // right
            let right_total = (true_result[0] + true_result[1]) as f64;
            let right_entropy = -(true_result[0] as f64 / right_total)
                * (true_result[0] as f64 / right_total).log2()
                - (true_result[1] as f64 / right_total)
                    * (true_result[1] as f64 / right_total).log2();

            // information gain
            let ig = parent_entropy
                - (left_total / parent_total) * left_entropy
                - (right_total / parent_total) * right_entropy;

            if let Some(max) = &mut max {
                if max.0 < ig {
                    max.0 = ig;
                    max.1 = con_type;
                    max.2 = column;
                }
            } else {
                max = Some((ig, con_type, column));
            }

            // println!("raw result:");
            // println!("false compunent: {:?}", false_result);
            // println!("true compunent: {:?}", true_result);
            // println!("entropy:");
            // println!("parent entropy: {}", parent_entropy);
            // println!("left entropy: {}", left_entropy);
            // println!("right entropy: {}", right_entropy);
            // println!("information gain: {}", ig);
        }
        // println!("====");
    }

    // println!("column choice for condition is {:?}", max);
    // feature_type.swap(max.unwrap().2, feature_len - 1);
    // println!("features status now: {:?}", feature_type);

    // println!("features: ");
    // for feature in features {
    //     println!("{:?}", feature);
    // }

    // SPLITTING
    let max_idx = max.unwrap().2;
    let mut features_left = vec![];
    let mut labels_left = vec![];

    let mut features_right = vec![];
    let mut labels_right = vec![];

    for (idx, feature) in features.iter().enumerate() {
        if let ConditionArg::Boolean(status) = feature[max_idx] {
            if status {
                features_right.push(*feature);
                labels_right.push(labels[idx]);
            } else {
                features_left.push(*feature);
                labels_left.push(labels[idx]);
            }
        }
    }

    // println!("after splitting");
    // println!("left");
    // for feature in features_left {
    //     println!("{:?}", feature);
    // }
    // println!("right");
    // for feature in features_right {
    //     println!("{:?}", feature);
    // }

    // return Child::Class(1);
    let node = Node {
        condition: max.unwrap().1,
        left: Box::new(build(
            feature_type,
            feature_len - 1,
            &features_left,
            &labels_left,
        )),
        right: Box::new(build(
            feature_type,
            feature_len - 1,
            &features_right,
            &labels_right,
        )),
    };

    Child::Node(node)
}

fn build_tree<const FEATURE_COUNT: usize>(
    feature_type: [ConditionType; FEATURE_COUNT],
    features: &[[ConditionArg; FEATURE_COUNT]],
    labels: &[usize],
) {
    let feature_type: [(&ConditionType, usize); FEATURE_COUNT] =
        array::from_fn(|idx| (&feature_type[idx], idx));

    let node = build(feature_type, FEATURE_COUNT, features, labels);

    // println!("{:#?}", node);
}
