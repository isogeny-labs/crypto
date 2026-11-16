//! Hand-authored synthetic costs and dummy equality keys; never measurements.
use compact_speed_analysis::{Arm, Case, Sample};
pub fn synthetic() -> Case {
    let sample = |arm| Sample {
        arm,
        complete: true,
        input_sha256: "12".repeat(32),
        worker_sha256: "34".repeat(32),
        validation_input_sha256: "12".repeat(32),
        worker_wall_ns: match arm {
            Arm::F4 => 1998,
            Arm::Certified => 2398,
            Arm::Compact => 98,
        },
        validation_wall_ns: 1,
        basis_sha256: "ab".repeat(32),
        external_basis_sha256: "ab".repeat(32),
    };
    // Independent literal specification of the frozen order cycle.
    let orders = ["FEC", "ECF", "CFE", "CEF", "EFC", "FCE"];
    let arm = |c| match c {
        'F' => Arm::F4,
        'E' => Arm::Certified,
        'C' => Arm::Compact,
        _ => unreachable!(),
    };
    Case {
        schema: "compact-speed-transcript/v1".into(),
        prime: 2147483647,
        variables: 6,
        seed: 101,
        input_sha256: "12".repeat(32),
        worker_sha256: "34".repeat(32),
        reference_basis_sha256: "ab".repeat(32),
        setup_wall_ns: 1,
        warmups: "FEC".chars().map(|c| sample(arm(c))).collect(),
        aa_pairs: [Arm::F4, Arm::Certified, Arm::Compact].map(|a| vec![[sample(a), sample(a)]; 7]),
        blocks: (0..21)
            .map(|i| orders[i % 6].chars().map(|c| sample(arm(c))).collect())
            .collect(),
    }
}
