use std::{array, default, fmt::Debug};

fn main() {
    let feature_type = [
        FeatureType::Float,
        FeatureType::Float,
        FeatureType::Float,
        FeatureType::Float,
        FeatureType::Float,
        FeatureType::Float,
        FeatureType::Float,
        FeatureType::Float,
        FeatureType::Float,
        FeatureType::Float,
    ];

    let features = [
        [15.2, 2.1, 5.3, 1.2, 8.1, 3.4, 2.1, 4.5, 1.2, 6.7],
        [18.5, 3.2, 4.8, 2.1, 7.3, 2.8, 1.9, 5.1, 2.3, 5.4],
        [12.8, 1.9, 6.1, 0.8, 9.2, 4.1, 3.2, 3.8, 0.9, 7.2],
        [21.3, 4.5, 3.9, 3.2, 6.8, 2.2, 1.5, 6.2, 3.1, 4.8],
        [14.1, 2.8, 5.7, 1.5, 8.5, 3.7, 2.8, 4.9, 1.8, 6.1],
        [19.2, 3.7, 4.2, 2.8, 7.1, 2.5, 1.2, 5.5, 2.7, 5.2],
        [11.5, 1.5, 6.8, 0.5, 9.8, 4.5, 3.8, 3.2, 0.5, 8.1],
        [22.1, 5.2, 3.5, 3.8, 6.2, 1.9, 1.1, 6.8, 3.5, 4.2],
        [16.8, 3.1, 5.1, 1.9, 7.9, 3.1, 2.4, 5.2, 2.1, 5.9],
        [13.5, 2.4, 6.2, 1.1, 8.8, 3.9, 3.1, 4.1, 1.3, 7.5],
        [20.5, 4.1, 4.0, 3.0, 7.0, 2.4, 1.8, 5.8, 2.9, 5.0],
        [17.2, 3.4, 4.9, 2.2, 7.6, 2.9, 2.2, 5.0, 2.4, 5.7],
        [14.8, 2.5, 5.5, 1.3, 8.3, 3.5, 2.6, 4.6, 1.6, 6.5],
        [23.0, 5.5, 3.2, 4.0, 5.9, 1.5, 0.8, 7.0, 3.8, 4.0],
        [16.2, 3.0, 5.3, 1.8, 8.0, 3.2, 2.3, 5.1, 2.0, 6.0],
        [45.2, 12.1, 15.3, 11.2, 18.1, 13.4, 12.1, 14.5, 11.2, 16.7],
        [48.5, 13.2, 14.8, 12.1, 17.3, 12.8, 11.9, 15.1, 12.3, 15.4],
        [42.8, 11.9, 16.1, 10.8, 19.2, 14.1, 13.2, 13.8, 10.9, 17.2],
        [51.3, 14.5, 13.9, 13.2, 16.8, 12.2, 11.5, 16.2, 13.1, 14.8],
        [44.1, 12.8, 15.7, 11.5, 18.5, 13.7, 12.8, 14.9, 11.8, 16.1],
        [49.2, 13.7, 14.2, 12.8, 17.1, 12.5, 11.2, 15.5, 12.7, 15.2],
        [41.5, 11.5, 16.8, 10.5, 19.8, 14.5, 13.8, 13.2, 10.5, 18.1],
        [52.1, 15.2, 13.5, 13.8, 16.2, 11.9, 11.1, 16.8, 13.5, 14.2],
        [46.8, 13.1, 15.1, 11.9, 17.9, 13.1, 12.4, 15.2, 12.1, 15.9],
        [43.5, 12.4, 16.2, 11.1, 18.8, 13.9, 13.1, 14.1, 11.3, 17.5],
        [50.5, 14.1, 14.0, 13.0, 17.0, 12.4, 11.8, 15.8, 12.9, 15.0],
        [47.2, 13.4, 14.9, 12.2, 17.6, 12.9, 12.2, 15.0, 12.4, 15.7],
        [44.8, 12.5, 15.5, 11.3, 18.3, 13.5, 12.6, 14.6, 11.6, 16.5],
        [53.0, 15.5, 13.2, 14.0, 15.9, 11.5, 10.8, 17.0, 13.8, 14.0],
        [46.2, 13.0, 15.3, 11.8, 18.0, 13.2, 12.3, 15.1, 12.0, 16.0],
    ]
    .map(|data| data.map(|data| ConditionArg::Float(data)));

    let labels = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let tree = build_tree(feature_type, &features, &labels);
    println!("{:#?}", tree);
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
    left_entropy: f64,
    right_entropy: f64,
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
                        left_entropy,
                        right_entropy,
                        feature_idx,
                        able_idx,
                        left_empty,
                        right_empty,
                        condition_node: ConditionNode::Boolean(Feature(feature_idx)),
                    });
                }

                if weighted_entropy == 0. {
                    break;
                }
            }
            FeatureType::Float => {
                let mut minimum: Option<(
                    f64,
                    f64,
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
                                left_entropy,
                                right_entropy,
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
                            left_entropy,
                            right_entropy,
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
                            left_entropy: minimum.1,
                            right_entropy: minimum.2,
                            condition_node: minimum.3,
                            able_idx: minimum.4,
                            feature_idx: minimum.5,
                            left_empty: minimum.6,
                            right_empty: minimum.7,
                        };
                    }
                } else {
                    minimum_score = Some(MinimumScore {
                        weighted_entropy: minimum.0,
                        left_entropy: minimum.1,
                        right_entropy: minimum.2,
                        condition_node: minimum.3,
                        able_idx: minimum.4,
                        feature_idx: minimum.5,
                        left_empty: minimum.6,
                        right_empty: minimum.7,
                    });
                }

                //  immediately select homogenous
                if minimum.0 == 0. {
                    break;
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

    let mut left_sample = [0, 0];
    let mut right_sample = [0, 0];
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
                    right_sample[*label] += 1;
                } else {
                    left_features.push((*feature, *label));
                    left_sample[*label] += 1;
                }
            }
            (ConditionArg::Float(value), ConditionNode::Float(_, compare)) => {
                if value <= *compare {
                    left_features.push((*feature, *label));
                    left_sample[*label] += 1;
                } else {
                    right_features.push((*feature, *label));
                    right_sample[*label] += 1;
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

    // println!("{:?} | {:?}", left_sample, right_sample);

    let left_early_stop = if minimum_score.left_entropy == 0. {
        if left_sample[0] > left_sample[1] {
            Some(Box::new(Child::Class(0)))
        } else {
            Some(Box::new(Child::Class(1)))
        }
    } else {
        None
    };

    let right_early_stop = if minimum_score.right_entropy == 0. {
        if right_sample[0] > right_sample[1] {
            Some(Box::new(Child::Class(0)))
        } else {
            Some(Box::new(Child::Class(1)))
        }
    } else {
        None
    };

    let (left, right) = if (able_len - 1) == 0 {
        let left_klasifikasi = if let Some(left_empty) = minimum_score.left_empty {
            left_empty
        } else if left_sample[0] > left_sample[1] {
            0
        } else {
            1
        };

        let right_klasifikasi = if let Some(right_empty) = minimum_score.right_empty {
            right_empty
        } else if right_sample[0] > right_sample[1] {
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
            if let Some(_) = left_early_stop {
                Box::new(Child::Class(0))
            } else if let Some(left_empty) = minimum_score.left_empty {
                Box::new(Child::Class(left_empty))
            } else {
                Box::new(build(&left_features, indexed_feature_type, able_len - 1))
            },
            if let Some(_) = right_early_stop {
                Box::new(Child::Class(0))
            } else if let Some(right_empty) = minimum_score.right_empty {
                Box::new(Child::Class(right_empty))
            } else {
                Box::new(build(&right_features, indexed_feature_type, able_len - 1))
            },
        )
    };

    let node = Node {
        condition: minimum_score.condition_node,
        left: left_early_stop.unwrap_or(left),
        right: right_early_stop.unwrap_or(right),
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
