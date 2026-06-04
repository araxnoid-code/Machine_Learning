use std::{array, default, fmt::Debug};

const EPSILON: f64 = 0.000000000001;

fn main() {
    let feature_type = [
        ConditionType::Boolean,
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
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
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
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
        [
            ConditionArg::Boolean(false),
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
            ConditionArg::Boolean(true),
        ],
        [
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(false),
            ConditionArg::Boolean(true),
            ConditionArg::Boolean(false),
        ],
    ];

    let labels = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let tree = build_tree(feature_type, &features, &labels);

    for (feature, label) in features.iter().zip(labels.iter()) {
        let pred = tree.input(*feature);
        println!("pred: {} | actual: {}", pred, label);
    }

    // let feature_type = [
    //     ConditionType::Boolean,
    //     ConditionType::Boolean,
    //     ConditionType::Boolean,
    // ];

    // let features = [
    //     [
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(true),
    //     ],
    //     [
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(true),
    //     ],
    //     [
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(true),
    //     ],
    //     [
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(true),
    //     ],
    //     [
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(false),
    //     ],
    //     [
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(false),
    //     ],
    //     [
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(true),
    //     ],
    //     [
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(false),
    //     ],
    //     [
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(true),
    //         ConditionArg::Boolean(false),
    //     ],
    //     [
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(false),
    //         ConditionArg::Boolean(false),
    //     ],
    // ];

    // let labels = [1, 1, 1, 1, 1, 0, 0, 0, 0, 0];

    // build_tree(feature_type, &features, &labels);
}

#[derive(Debug)]
enum ConditionType {
    Boolean,
    Float,
}

impl ConditionType {
    pub fn create_condition_node(
        &self,
        feature_idx: usize,
        float_value: Option<f64>,
    ) -> ConditionNode {
        match self {
            ConditionType::Boolean => ConditionNode::Boolean(Feature(feature_idx)),
            ConditionType::Float => {
                ConditionNode::Float(Feature(feature_idx), float_value.unwrap_or(0.))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ConditionArg {
    Boolean(bool),
    Float(f64),
}

#[derive(Debug)]
struct Feature(usize);

#[derive(Debug)]
enum ConditionNode {
    Boolean(Feature),
    Float(Feature, f64),
}

#[derive(Debug)]
enum Child<const FEATURES_COUNT: usize> {
    Node(Node<FEATURES_COUNT>),
    Class(usize),
}

impl<const FEATURES_COUNT: usize> Child<FEATURES_COUNT> {
    pub fn input(&self, input: [ConditionArg; FEATURES_COUNT]) -> usize {
        match self {
            Child::Node(node) => {
                //
                if let ConditionNode::Boolean(Feature(idx)) = &node.condition {
                    if let ConditionArg::Boolean(status) = input[*idx] {
                        if status {
                            node.right.input(input)
                        } else {
                            node.left.input(input)
                        }
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            Child::Class(class) => *class,
        }
    }
}

#[derive(Debug)]
struct Node<const FEATURES_COUNT: usize> {
    condition: ConditionNode,
    left: Box<Child<FEATURES_COUNT>>,
    right: Box<Child<FEATURES_COUNT>>,
}

fn build<const FEATURES_COUNT: usize>(
    labeled_features: &[(&[ConditionArg; FEATURES_COUNT], usize)],
    mut indexed_feature_type: [(&ConditionType, usize); FEATURES_COUNT],
    able_len: usize,
) -> Child<FEATURES_COUNT> {
    // println!();
    // println!("ITER!!!!!!!!!!!!!!!!!! ITER!!!!!!!!!!!!!!!!!!");
    // println!();

    let mut minimum_score = None;
    for able_idx in 0..able_len {
        let (column_type, column_idx) = indexed_feature_type[able_idx];
        // println!("column index: {}", column_idx);

        match column_type {
            ConditionType::Boolean => {
                // left (false)
                let mut left_result = [0, 0];
                let mut right_result = [0, 0];

                for (row_idx, (feature, label)) in labeled_features.iter().enumerate() {
                    // println!("{:?}", feature[column_idx]);

                    if let ConditionArg::Boolean(status) = feature[column_idx] {
                        if status {
                            right_result[*label] += 1;
                        } else {
                            left_result[*label] += 1;
                        }
                    } else {
                        panic!(
                            "Error, data type in row {} is Boolean but found Float in data sequence {}",
                            column_idx, row_idx
                        );
                    }
                }

                // left entropy
                let mut left_empty = None;
                let left_total = (left_result[0] + left_result[1]) as f64;
                if left_total == 0. {
                    let classification =
                        if right_result[0] + left_result[0] > left_result[1] + right_result[1] {
                            0
                        } else {
                            1
                        };
                    left_empty = Some(classification);
                }

                let left_entropy = if left_empty.is_none() {
                    -(left_result[0] as f64 / left_total)
                        * (left_result[0] as f64 / left_total).log2()
                        - (left_result[1] as f64 / left_total)
                            * (left_result[1] as f64 / left_total).log2()
                } else {
                    0.
                };

                // right entropy
                let mut right_empty = None;
                let right_total = (right_result[0] + right_result[1]) as f64;
                if right_total == 0. {
                    let classification =
                        if right_result[0] + left_result[0] > right_result[1] + left_result[1] {
                            0
                        } else {
                            1
                        };
                    right_empty = Some(classification);
                }

                let right_entropy = if right_empty.is_none() {
                    -(right_result[0] as f64 / right_total)
                        * (right_result[0] as f64 / right_total).log2()
                        - (right_result[1] as f64 / right_total)
                            * (right_result[1] as f64 / right_total).log2()
                } else {
                    0.
                };

                // score
                let parent_total = labeled_features.len() as f64;
                let score = (left_total / parent_total) * left_entropy
                    + (right_total / parent_total) * right_entropy;

                // println!("raw result:");
                // println!("left total: {:?}", left_total);
                // println!("left result: {:?}", left_result);
                // println!("right total: {:?}", right_total);
                // println!("right result: {:?}", right_result);
                // println!("entropy:");
                // println!("left_entropy : {:?}", left_entropy);
                // println!("right_entropy : {:?}", right_entropy);
                // println!("score : {:?}", score);

                if let Some((min_score, min_column, idx, min_left_empty, min_right_empty)) =
                    &mut minimum_score
                {
                    if *min_score > score {
                        *min_score = score;
                        *min_column = column_idx;
                        *idx = able_idx;
                        *min_left_empty = left_empty;
                        *min_right_empty = right_empty;
                    }
                } else {
                    minimum_score = Some((score, column_idx, able_idx, left_empty, right_empty));
                }
            }
            ConditionType::Float => {}
        }

        // println!("===========================");
    }
    // println!("minimum features is {:?}", minimum_score.unwrap());

    // swap
    let (condition_type, idx_feature) = indexed_feature_type[minimum_score.unwrap().2];
    indexed_feature_type.swap(minimum_score.unwrap().2, able_len - 1);

    // split
    let mut left_features = vec![];
    let mut right_features = vec![];

    let column = minimum_score.unwrap().1;

    let mut left_result = [1, 1];
    let mut right_result = [1, 1];
    for (feature, label) in labeled_features {
        if let ConditionArg::Boolean(status) = feature[column] {
            if status {
                right_features.push((*feature, *label));
                right_result[*label] += 1;
            } else {
                left_features.push((*feature, *label));
                left_result[*label] += 1;
            }
        }
    }

    // println!("before splitting");
    // for feature in labeled_features {
    //     println!("{:?}", feature);
    // }

    // println!("after splitting");
    // println!("left:");
    // for feature in &left_features {
    //     println!("{:?}", feature);
    // }

    // println!("right:");
    // for feature in &right_features {
    //     println!("{:?}", feature);
    // }

    let (left, right) = if (able_len - 1) == 0 {
        let left_klasifikasi = if let Some(left_empty) = minimum_score.unwrap().3 {
            left_empty
        } else if left_result[0] > left_result[1] {
            0
        } else {
            1
        };

        let right_klasifikasi = if let Some(right_empty) = minimum_score.unwrap().4 {
            right_empty
        } else if right_result[0] > right_result[1] {
            0
        } else {
            1
        };

        (
            Box::new(Child::Class(left_klasifikasi)),
            Box::new(Child::Class(right_klasifikasi)),
        )
    } else {
        (
            if let Some(left_empty) = minimum_score.unwrap().3 {
                Box::new(Child::Class(left_empty))
            } else {
                Box::new(build(&left_features, indexed_feature_type, able_len - 1))
            },
            if let Some(right_empty) = minimum_score.unwrap().4 {
                Box::new(Child::Class(right_empty))
            } else {
                Box::new(build(&right_features, indexed_feature_type, able_len - 1))
            },
        )
    };

    let node = Node {
        condition: condition_type.create_condition_node(idx_feature, None),
        left: left,
        right: right,
    };

    return Child::Node(node);
}

fn build_tree<const FEATURES_COUNT: usize>(
    feature_type: [ConditionType; FEATURES_COUNT],
    features: &[[ConditionArg; FEATURES_COUNT]],
    labels: &[usize],
) -> Child<FEATURES_COUNT> {
    let labeled_features = features
        .iter()
        .enumerate()
        .map(|(idx, feature)| (feature, labels[idx]))
        .collect::<Vec<(&[ConditionArg; FEATURES_COUNT], usize)>>();

    let indexed_feature_type: [(&ConditionType, usize); FEATURES_COUNT] =
        array::from_fn(|idx| (&feature_type[idx], idx));

    let node = build(&labeled_features, indexed_feature_type, FEATURES_COUNT);
    node
}
