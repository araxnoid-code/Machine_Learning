fn main() {
    let features = [
        [true, true, true, true, true],
        [true, true, true, false, false],
        [true, false, true, true, false],
        [false, true, true, true, false],
        [true, true, false, false, true],
        [false, false, false, false, false],
        [false, false, true, false, false],
        [true, false, false, false, false],
        [false, true, false, false, true],
        [false, false, false, true, false],
    ];

    let lebels = [
        "Yes", "Yes", "Yes", "Yes", "Yes", "No", "No", "No", "No", "No",
    ];
}

enum Condition {
    Boolean(bool),
}

struct Node {
    condition: Condition,
    left: Box<Node>,
    right: Box<Node>,
}
