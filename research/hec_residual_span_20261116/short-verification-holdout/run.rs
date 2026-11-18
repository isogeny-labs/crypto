type Outcome = (Option<u64>, usize, u64, u64, u64, bool);
fn run(arm: usize, c: &Curve, d: &Div, t: &Div, ell: u64, case: usize) -> Outcome {
    match arm {
        0 => {
            let r = batch::solve(c, d, t, ell);
            (
                r.k,
                r.relations,
                0,
                r.anchors,
                r.parameter,
                r.source_normalized,
            )
        }
        1 => {
            let r = pencil::solve(c, d, t, ell);
            (
                r.k,
                r.relations,
                0,
                r.anchors,
                r.parameter,
                r.source_normalized,
            )
        }
        a => {
            let r = walk::solve(
                c,
                d,
                t,
                ell,
                2026111700 + case as u64,
                if a == 2 {
                    walk::Mode::Folded
                } else {
                    walk::Mode::Expanded
                },
            );
            (r.k, r.relations, r.trials, 0, 0, r.source_normalized)
        }
    }
}
