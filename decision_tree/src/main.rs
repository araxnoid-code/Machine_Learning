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

    let labels = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let features = [
        // KELAS 0 (25 sampel) - dengan noise dan overlap
        [18.5, 3.2, 5.1, 2.3, 7.8, 3.5, 2.4, 4.8, 1.9, 6.2],
        [22.1, 4.1, 4.5, 3.1, 6.9, 2.8, 1.7, 5.5, 2.6, 5.4],
        [15.3, 2.4, 6.2, 1.5, 8.9, 4.2, 3.1, 3.9, 1.1, 7.3],
        [35.2, 8.5, 12.1, 9.2, 15.3, 11.2, 10.5, 12.1, 9.5, 13.2], // NOISE: nilai tinggi tapi kelas 0
        [19.8, 3.5, 4.8, 2.6, 7.4, 3.1, 2.0, 5.0, 2.2, 5.8],
        [14.2, 2.1, 6.5, 1.2, 9.3, 4.5, 3.4, 3.5, 0.8, 7.8],
        [42.1, 10.2, 14.5, 11.3, 17.2, 12.8, 11.9, 14.2, 11.1, 15.5], // NOISE: nilai sangat tinggi
        [20.5, 3.8, 4.9, 2.8, 7.2, 3.0, 2.1, 5.2, 2.4, 5.6],
        [16.8, 2.9, 5.5, 1.9, 8.2, 3.7, 2.7, 4.4, 1.5, 6.6],
        [23.5, 4.5, 4.2, 3.4, 6.5, 2.5, 1.4, 5.8, 2.9, 5.0],
        [13.1, 1.8, 7.0, 0.9, 10.1, 4.8, 3.9, 3.1, 0.5, 8.5],
        [38.5, 9.2, 13.2, 10.5, 16.5, 11.8, 11.2, 13.5, 10.2, 14.8], // NOISE: overlap dengan kelas 1
        [21.2, 3.9, 4.6, 2.9, 7.0, 2.9, 1.9, 5.3, 2.5, 5.5],
        [17.5, 3.1, 5.3, 2.1, 7.9, 3.4, 2.5, 4.6, 1.8, 6.4],
        [24.8, 4.8, 4.0, 3.6, 6.2, 2.2, 1.2, 6.0, 3.1, 4.8],
        [12.5, 1.5, 7.2, 0.6, 10.5, 5.0, 4.2, 2.8, 0.3, 8.9],
        [28.5, 5.5, 8.5, 6.2, 11.5, 8.5, 7.5, 9.2, 6.5, 10.5], // NOISE: di zona tengah
        [19.2, 3.3, 5.0, 2.4, 7.5, 3.2, 2.2, 4.9, 2.1, 6.0],
        [15.8, 2.6, 5.8, 1.6, 8.6, 3.9, 2.9, 4.1, 1.3, 7.0],
        [22.5, 4.2, 4.4, 3.2, 6.7, 2.6, 1.6, 5.6, 2.7, 5.2],
        [45.0, 12.5, 16.5, 13.0, 19.0, 14.2, 13.5, 15.5, 12.5, 17.0], // OUTLIER ekstrim
        [18.2, 3.0, 5.2, 2.2, 7.6, 3.3, 2.3, 4.7, 2.0, 6.1],
        [14.5, 2.2, 6.3, 1.3, 9.0, 4.3, 3.2, 3.7, 1.0, 7.5],
        [21.8, 4.0, 4.7, 3.0, 6.8, 2.7, 1.8, 5.4, 2.6, 5.3],
        [16.2, 2.8, 5.4, 1.8, 8.1, 3.6, 2.6, 4.5, 1.6, 6.3],
        // KELAS 1 (25 sampel) - dengan noise dan overlap
        [48.5, 13.2, 15.8, 12.5, 18.5, 13.8, 12.8, 15.2, 12.5, 16.5],
        [52.1, 14.5, 14.2, 13.8, 16.8, 12.2, 11.5, 16.5, 13.8, 15.2],
        [45.3, 12.1, 16.5, 11.5, 19.2, 14.5, 13.5, 14.5, 11.5, 17.2],
        [19.5, 3.4, 5.2, 2.5, 7.7, 3.3, 2.3, 4.9, 2.1, 6.0], // NOISE: nilai rendah tapi kelas 1
        [50.2, 13.8, 15.1, 13.0, 17.5, 12.9, 12.1, 15.8, 13.1, 15.8],
        [44.1, 11.5, 16.8, 10.8, 19.5, 14.8, 14.0, 13.8, 10.9, 17.8],
        [55.0, 15.5, 13.5, 14.5, 16.0, 11.5, 10.8, 17.2, 14.2, 14.5],
        [25.5, 5.2, 8.2, 5.5, 10.5, 7.5, 6.5, 8.2, 5.5, 9.5], // NOISE: di zona tengah
        [47.2, 12.8, 15.5, 12.0, 18.0, 13.2, 12.5, 15.0, 12.2, 16.2],
        [42.5, 10.8, 17.0, 10.2, 19.8, 15.0, 14.2, 13.2, 10.5, 18.0],
        [53.5, 14.8, 13.8, 14.0, 16.2, 11.8, 11.2, 16.8, 13.5, 14.8],
        [30.2, 6.5, 9.5, 7.2, 12.5, 9.2, 8.5, 10.2, 7.5, 11.5], // OVERLAP dengan zona kelas 0
        [46.5, 12.5, 15.2, 11.8, 17.8, 13.0, 12.2, 14.8, 11.8, 15.5],
        [49.8, 13.5, 14.5, 12.8, 17.2, 12.5, 11.8, 15.5, 12.8, 15.0],
        [43.2, 11.2, 16.2, 11.0, 18.8, 14.2, 13.8, 14.0, 11.2, 17.0],
        [51.5, 14.2, 14.0, 13.5, 16.5, 12.0, 11.0, 16.0, 13.2, 14.5],
        [22.5, 4.5, 7.5, 4.5, 9.5, 6.5, 5.5, 7.5, 4.5, 8.5], // OVERLAP
        [48.0, 13.0, 15.6, 12.2, 18.2, 13.5, 12.6, 15.3, 12.3, 16.0],
        [41.5, 10.5, 17.2, 10.0, 20.0, 15.2, 14.5, 13.0, 10.0, 18.5],
        [54.2, 15.0, 13.2, 14.2, 15.8, 11.2, 10.5, 17.0, 14.0, 14.0],
        [28.5, 5.8, 8.8, 6.2, 11.8, 8.2, 7.2, 9.2, 6.2, 10.5], // OVERLAP
        [45.8, 12.2, 15.9, 11.5, 18.5, 13.6, 12.8, 14.9, 12.0, 16.3],
        [50.5, 14.0, 14.8, 13.2, 17.0, 12.4, 11.6, 15.9, 13.0, 15.3],
        [44.5, 11.8, 16.5, 11.2, 19.0, 14.4, 13.6, 14.2, 11.4, 17.5],
        [52.5, 14.8, 13.6, 14.0, 16.2, 11.8, 11.0, 16.5, 13.5, 14.8],
    ]
    .map(|row| row.map(|data| ConditionArg::Float(data)));

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
