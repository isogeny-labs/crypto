#[allow(dead_code)]
mod good {
    include!("good.rs");
    include!("section_api.rs");
    pub fn diagnose(u: [u64; 4], y: [u64; 3]) {
        let f = curve();
        let mut fy = [0; 49];
        for (i, j) in monos(4) {
            if j > 0 {
                fy[7 * i + j - 1] = mul(j as u64, f[7 * i + j]);
            }
        }
        println!(
            "DIAG u={u:?} v={y:?} repeated_degree={} fy_nonunit_degree={} curve_residual={:?}",
            ug(u.to_vec(), ux(&u)).len() - 1,
            ug(eval_u(&fy, &y, &u), u.to_vec()).len() - 1,
            eval_u(&f, &y, &u)
        );
    }
}
#[allow(dead_code)]
mod target {
    include!("target.rs");
    include!("target_api.rs");
}
fn main() {
    let f = [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111];
    let pts = vec![
        [18, 52, 1],
        [28, 86, 1],
        [43, 125, 1],
        [101, 43, 1],
        [34, 113, 1],
    ];
    let j = target::QuarticJacobian::with_points(127, f, pts).unwrap();
    let s = good::Sections::new();
    good::verify_canonical_line();
    let o = [18, 52, 1];
    let mut h = j.zero();
    for p in j.d_inf {
        let d = target::difference(&j, p, o);
        h = j.add_raw(&h, &j.add_raw(&d, &d));
    }
    for p in [[101, 43, 1], [34, 113, 1]] {
        h = j.add_raw(&h, &j.neg_raw(&target::difference(&j, p, o)));
    }
    let beta = target::scalar(&j, h, 514447);
    assert_eq!(j.add_raw(&beta, &beta), h);
    let a = target::difference(&j, [43, 125, 1], [101, 43, 1]);
    let g = target::scalar(&j, j.add_raw(&a, &a), 1005372);
    let a = target::difference(&j, [28, 86, 1], [18, 52, 1]);
    let b = target::scalar(&j, j.add_raw(&a, &a), 203071);
    let x = j.add_raw(&j.add_raw(&b, &j.add_raw(&g, &g)), &j.neg_raw(&beta));
    println!("TARGET_CLASS {:?}", x.0);
    match target::graph(&j, x) {
        None => println!("DIAG affine_graph=false"),
        Some((u, v)) => {
            good::diagnose(u, v);
            println!("DIAG accepted={}", s.coords(u, v).is_some());
        }
    }
    println!("DIAGNOSTIC_COMPLETE same_fourth_holdout=true alignment_gate_claim=false");
}
