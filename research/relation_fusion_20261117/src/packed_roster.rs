// Declared immutable roster. 12 controls; 6 candidates. AA duplicates every control.
#[derive(Clone)]
struct Arm {
    name: String,
    kind: &'static str,
    kernel: &'static str,
    worker: usize,
}
fn controls() -> Vec<Arm> {
    let mut out = Vec::new();
    for (prefix, worker, forms) in [
        ("frozen85", 0, vec![("sparse", "compact-sparse")]),
        ("frozen96", 1, vec![("dag", "compact-dag")]),
        (
            "current",
            2,
            vec![("sparse", "compact-sparse"), ("dag", "compact-dag")],
        ),
    ] {
        for (form, kind) in forms {
            for (k, kernel) in [
                ("103", "frontier"),
                ("231", "packed-products"),
                ("487", "packed-batch"),
            ] {
                out.push(Arm {
                    name: format!("{prefix}-{form}-{k}"),
                    kind,
                    kernel,
                    worker,
                });
            }
        }
    }
    out
}
fn roster(stage: &str) -> Vec<Arm> {
    let mut out = Vec::new();
    if stage == "AA" {
        for arm in controls() {
            let mut copy = arm.clone();
            copy.name += "-copy";
            out.push(arm);
            out.push(copy);
        }
    } else {
        out = controls();
        for (form, kind) in [
            ("sparse", "packed-constructor-sparse"),
            ("dag", "packed-constructor-dag"),
        ] {
            for (k, kernel) in [
                ("103", "frontier"),
                ("231", "packed-products"),
                ("487", "packed-batch"),
            ] {
                out.push(Arm {
                    name: format!("packed-{form}-{k}"),
                    kind,
                    kernel,
                    worker: 2,
                });
            }
        }
    }
    out
}
