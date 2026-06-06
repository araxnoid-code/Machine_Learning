use std::{array, default, fmt::Debug};

const EPSILON: f64 = 0.000000000001;

fn main() {
    let feature_type = [FeatureType::Float, FeatureType::Float, FeatureType::Float];

    let features = [
        [
            ConditionArg::Float(25.),
            ConditionArg::Float(5.),
            ConditionArg::Float(3.),
        ],
        [
            ConditionArg::Float(32.),
            ConditionArg::Float(8.),
            ConditionArg::Float(5.),
        ],
        [
            ConditionArg::Float(45.),
            ConditionArg::Float(15.),
            ConditionArg::Float(8.),
        ],
        [
            ConditionArg::Float(28.),
            ConditionArg::Float(6.),
            ConditionArg::Float(4.),
        ],
        [
            ConditionArg::Float(50.),
            ConditionArg::Float(20.),
            ConditionArg::Float(9.),
        ],
        [
            ConditionArg::Float(35.),
            ConditionArg::Float(10.),
            ConditionArg::Float(6.),
        ],
        [
            ConditionArg::Float(22.),
            ConditionArg::Float(4.),
            ConditionArg::Float(2.),
        ],
        [
            ConditionArg::Float(48.),
            ConditionArg::Float(18.),
            ConditionArg::Float(7.),
        ],
        [
            ConditionArg::Float(30.),
            ConditionArg::Float(7.),
            ConditionArg::Float(4.),
        ],
        [
            ConditionArg::Float(40.),
            ConditionArg::Float(12.),
            ConditionArg::Float(7.),
        ],
    ];

    let labels = [0, 0, 1, 0, 1, 1, 0, 1, 0, 1];

    let node = build_tree(feature_type, &features, &labels);
    println!("DONEEEE");
    println!("{:#?}", node);
}

#[derive(Debug)]
enum FeatureType {
    Boolean,
    Float,
}

impl FeatureType {
    pub fn create_condition_node(
        &self,
        feature_idx: usize,
        float_value: Option<f64>,
    ) -> ConditionNode {
        match self {
            FeatureType::Boolean => ConditionNode::Boolean(Feature(feature_idx)),
            FeatureType::Float => {
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

#[derive(Debug)]
struct MinimumScore {
    weighted_entropy: f64,
    feature_idx: usize,
    able_idx: usize,
    left_empty: Option<usize>,
    right_empty: Option<usize>,
    condition_node: ConditionNode,
}

fn build<const FEATURES_COUNT: usize>(
    labeled_features: &[(&[ConditionArg; FEATURES_COUNT], usize)],
    mut indexed_feature_type: [(&FeatureType, usize); FEATURES_COUNT],
    able_len: usize,
) -> Child<FEATURES_COUNT> {
    // println!();
    // println!("ITER!!!!!!!!!!!!!!!!!! ITER!!!!!!!!!!!!!!!!!!");
    // println!();

    let mut minimum_score: Option<MinimumScore> = None;
    for able_idx in 0..able_len {
        let (feature_type, feature_idx) = indexed_feature_type[able_idx];
        // println!("column index: {}", column_idx);

        match feature_type {
            FeatureType::Boolean => {
                // left (false)
                let mut left_result = [0, 0];
                let mut right_result = [0, 0];

                for (row, (feature, label)) in labeled_features.iter().enumerate() {
                    // println!("{:?}", feature[column_idx]);

                    if let ConditionArg::Boolean(status) = feature[feature_idx] {
                        if status {
                            right_result[*label] += 1;
                        } else {
                            left_result[*label] += 1;
                        }
                    } else {
                        panic!(
                            "Error, the data type in column {} is Float but a Boolean was found in the row {} in the column",
                            feature_idx, row
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

                // weighted entropy
                let parent_total = labeled_features.len() as f64;
                let weighted_entropy = (left_total / parent_total) * left_entropy
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

                if let Some(minimum_score) = &mut minimum_score {
                    if minimum_score.weighted_entropy > weighted_entropy {
                        minimum_score.weighted_entropy = weighted_entropy;
                        minimum_score.feature_idx = feature_idx;
                        minimum_score.able_idx = able_idx;
                        minimum_score.left_empty = left_empty;
                        minimum_score.right_empty = right_empty;
                    }
                } else {
                    minimum_score = Some(MinimumScore {
                        weighted_entropy,
                        feature_idx,
                        able_idx,
                        left_empty,
                        right_empty,
                        condition_node: ConditionNode::Boolean(Feature(feature_idx)),
                    });
                }
            }
            FeatureType::Float => {
                let mut minimum: Option<(
                    f64,
                    ConditionNode,
                    usize,
                    usize,
                    Option<usize>,
                    Option<usize>,
                )> = None;
                for (row, (candidate_features, _)) in labeled_features.iter().enumerate() {
                    let mut left_sample = [0, 0];
                    let mut right_sample = [0, 0];

                    let candidate_feature = if let ConditionArg::Float(value) =
                        candidate_features[feature_idx]
                    {
                        value
                    } else {
                        panic!(
                            "Error, the data type in column {} is Float but a Boolean was found in the row {} in the column",
                            feature_idx, row
                        );
                    };

                    // println!(
                    //     "candidate feature\n{:?} in column {}\n",
                    //     candidate_feature, feature_idx
                    // );
                    // println!("will be compared by");
                    for (row, (features, label)) in labeled_features.iter().enumerate() {
                        let feature = if let ConditionArg::Float(value) = features[feature_idx] {
                            value
                        } else {
                            panic!(
                                "Error, the data type in column {} is Float but a Boolean was found in the row {} in the column",
                                feature_idx, row
                            );
                        };

                        if feature <= candidate_feature {
                            left_sample[*label] += 1;
                        } else {
                            right_sample[*label] += 1;
                        }

                        // println!("{:?} with label {:?}", feature, label);
                    }

                    // left entropy
                    let left_total = (left_sample[0] + left_sample[1]) as f64;

                    let left_empty = if left_sample[0] == 0 && left_sample[1] == 0 {
                        let total_of_zero = left_sample[0] + right_sample[0];
                        let total_of_one = left_sample[1] + right_sample[1];
                        if total_of_zero >= total_of_one {
                            Some(0)
                        } else {
                            Some(1)
                        }
                    } else {
                        None
                    };

                    let left_entropy = if left_sample[0] == 0 || left_sample[1] == 0 {
                        0.
                    } else {
                        let prop_0: f64 = left_sample[0] as f64 / left_total;
                        let prop_1: f64 = left_sample[1] as f64 / left_total;
                        -prop_0 * prop_0.log2() - prop_1 * prop_1.log2()
                    };

                    // right entropy
                    let right_total = (right_sample[0] + right_sample[1]) as f64;

                    let right_empty = if right_sample[0] == 0 && right_sample[1] == 0 {
                        let total_of_zero = left_sample[0] + right_sample[0];
                        let total_of_one = left_sample[1] + right_sample[1];
                        if total_of_zero >= total_of_one {
                            Some(0)
                        } else {
                            Some(1)
                        }
                    } else {
                        None
                    };

                    let right_entropy = if right_sample[0] == 0 || right_sample[1] == 0 {
                        0.
                    } else {
                        let prop_0: f64 = right_sample[0] as f64 / right_total;
                        let prop_1: f64 = right_sample[1] as f64 / right_total;
                        -prop_0 * prop_0.log2() - prop_1 * prop_1.log2()
                    };

                    let parent_total = left_total + right_total;
                    let w_entropy = (left_total / parent_total) * left_entropy
                        + (right_total / parent_total) * right_entropy;

                    // println!("compare samples left: {:?}", left_sample);
                    // println!("compare samples right: {:?}", right_sample);
                    // println!("left entropy: {:?}", left_entropy);
                    // println!("right entropy: {:?}", right_entropy);
                    // println!("weighted entropy: {:?}", w_entropy);
                    // println!("----------------------");

                    if let Some(minimum) = &mut minimum {
                        if minimum.0 > w_entropy {
                            *minimum = (
                                w_entropy,
                                ConditionNode::Float(Feature(feature_idx), candidate_feature),
                                able_idx,
                                feature_idx,
                                left_empty,
                                right_empty,
                            );
                        }
                    } else {
                        minimum = Some((
                            w_entropy,
                            ConditionNode::Float(Feature(feature_idx), candidate_feature),
                            able_idx,
                            feature_idx,
                            left_empty,
                            right_empty,
                        ));
                    }
                }
                let minimum = minimum.unwrap();
                // println!("minimum value is {:?}\n", minimum);

                if let Some(minimum_score) = &mut minimum_score {
                    if minimum_score.weighted_entropy > minimum.0 {
                        *minimum_score = MinimumScore {
                            weighted_entropy: minimum.0,
                            condition_node: minimum.1,
                            able_idx: minimum.2,
                            feature_idx: minimum.3,
                            left_empty: minimum.4,
                            right_empty: minimum.5,
                        };
                    }
                } else {
                    minimum_score = Some(MinimumScore {
                        weighted_entropy: minimum.0,
                        condition_node: minimum.1,
                        able_idx: minimum.2,
                        feature_idx: minimum.3,
                        left_empty: minimum.4,
                        right_empty: minimum.5,
                    });
                }
            }
        }

        // println!("===========================");
    }
    // println!("minimum features is {:?}", minimum_score.unwrap());

    // return Child::Class(0);

    let minimum_score = minimum_score.unwrap();
    // println!("minimum score: {:?}", minimum_score);

    // return Child::Class(0);
    // swap
    // let (feature_type, feature_idx) = indexed_feature_type[minimum_score.able_idx];
    indexed_feature_type.swap(minimum_score.able_idx, able_len - 1);

    // split
    let mut left_features = vec![];
    let mut right_features = vec![];

    // minimum_scoreome((score, feature_idx, able_idx, left_empty, right_empty));
    let column = minimum_score.feature_idx;

    let mut left_result = [0, 0];
    let mut right_result = [0, 0];
    for (feature, label) in labeled_features {
        // if let ConditionArg::Boolean(status) = feature[column] {
        //     if status {
        //         right_features.push((*feature, *label));
        //         right_result[*label] += 1;
        //     } else {
        //         left_features.push((*feature, *label));
        //         left_result[*label] += 1;
        //     }
        // }

        match (feature[column], &minimum_score.condition_node) {
            (ConditionArg::Boolean(status), _) => {
                if status {
                    right_features.push((*feature, *label));
                    right_result[*label] += 1;
                } else {
                    left_features.push((*feature, *label));
                    left_result[*label] += 1;
                }
            }
            (ConditionArg::Float(value), ConditionNode::Float(_, compare)) => {
                if value <= *compare {
                    left_features.push((*feature, *label));
                    left_result[*label] += 1;
                } else {
                    right_features.push((*feature, *label));
                    right_result[*label] += 1;
                }
            }
            _ => (),
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
        let left_klasifikasi = if let Some(left_empty) = minimum_score.left_empty {
            left_empty
        } else if left_result[0] > left_result[1] {
            0
        } else {
            1
        };

        let right_klasifikasi = if let Some(right_empty) = minimum_score.right_empty {
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
            if let Some(left_empty) = minimum_score.left_empty {
                Box::new(Child::Class(left_empty))
            } else {
                Box::new(build(&left_features, indexed_feature_type, able_len - 1))
            },
            if let Some(right_empty) = minimum_score.right_empty {
                Box::new(Child::Class(right_empty))
            } else {
                Box::new(build(&right_features, indexed_feature_type, able_len - 1))
            },
        )
    };

    let node = Node {
        condition: minimum_score.condition_node,
        left: left,
        right: right,
    };

    return Child::Node(node);
}

fn build_tree<const FEATURES_COUNT: usize>(
    feature_type: [FeatureType; FEATURES_COUNT],
    features: &[[ConditionArg; FEATURES_COUNT]],
    labels: &[usize],
) -> Child<FEATURES_COUNT> {
    let labeled_features = features
        .iter()
        .enumerate()
        .map(|(idx, feature)| (feature, labels[idx]))
        .collect::<Vec<(&[ConditionArg; FEATURES_COUNT], usize)>>();

    let indexed_feature_type: [(&FeatureType, usize); FEATURES_COUNT] =
        array::from_fn(|idx| (&feature_type[idx], idx));

    let node = build(&labeled_features, indexed_feature_type, FEATURES_COUNT);
    node
}
