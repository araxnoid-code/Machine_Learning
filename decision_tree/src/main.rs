use decision_tree::{ConditionArg as Arg, FeatureType, build_tree};

fn main() {
    let feature_type = [FeatureType::Float, FeatureType::Float, FeatureType::Boolean];

    let datasets = [
        [Arg::Float(25.5), Arg::Float(10.2), Arg::Boolean(false)],
        [Arg::Float(30.1), Arg::Float(15.7), Arg::Boolean(true)],
        [Arg::Float(45.8), Arg::Float(25.3), Arg::Boolean(false)],
        [Arg::Float(52.3), Arg::Float(30.1), Arg::Boolean(true)],
        [Arg::Float(38.7), Arg::Float(18.5), Arg::Boolean(true)],
    ];

    let labels = [0, 0, 1, 1, 0];

    let tree = build_tree(feature_type, &datasets, &labels);

    for data in datasets {
        let prediction = tree.input(&data).unwrap();
    }
}
