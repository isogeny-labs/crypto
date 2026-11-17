mod confluent;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use confluent::{gaussian, jets, normalize, reference, Node};
use hec_native_ic::{
    instance::{factor_base, pick_instance, thread_curve},
    jac::Curve,
    poly::Poly,
};
fn compositions(total: usize, n: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if n == 1 {
        prefix.push(total);
        out.push(prefix.clone());
        prefix.pop();
        return;
    }
    for k in 1..=total - (n - 1) {
        prefix.push(k);
        compositions(total - k, n - 1, prefix, out);
        prefix.pop();
    }
}
fn check(c: &Curve, nodes: &[Node], d: usize, reference_check: bool) -> (usize, bool, bool, bool) {
    let (a, b, nullity) = gaussian(c, nodes, d);
    let (a, b, q, kept, common) = normalize(c, nodes, a, b);
    assert!(q.deg() <= c.g as isize);
    if reference_check {
        let (ar, br) = reference(c, nodes, d);
        assert_eq!((a, b), (ar, br), "independent_CRT_Euclidean_primitive_pair");
    }
    let table = table_factor::Tables::for_degree(c.fp.p, c.g);
    let split = if let Some(roots) = table.split(&q.monic(&c.fp)) {
        let mut sum = c.zero();
        for (n, &m) in nodes.iter().zip(&kept) {
            let d = c.mul(&c.place(n.x, n.y), m as u64);
            sum = c.add(&sum, &if n.neg { c.neg(&d) } else { d });
        }
        for &(x, m) in roots.as_slice() {
            let by = b.eval(x, &c.fp);
            assert_ne!(by, 0);
            let y = c.fp.mul(a.eval(x, &c.fp), c.fp.inv(by));
            assert_eq!(c.fp.mul(y, y), c.f.eval(x, &c.fp));
            sum = c.add(&sum, &c.mul(&c.place(x, y), m as u64));
        }
        assert!(c.is_zero(&sum), "principal_confluent_divisor");
        true
    } else {
        false
    };
    (nullity, common > 0, q.deg() < c.g as isize, split)
}
fn main() {
    let (mut cases, mut common, mut drops, mut splits, mut higher) = (0, 0, 0, 0, 0);
    for (g, p) in [(2, 101), (3, 61), (4, 31)] {
        let inst = pick_instance(p, g).unwrap();
        let c = &inst.curve;
        let points: Vec<_> = factor_base(c)
            .into_iter()
            .filter(|p| p.1 != 0)
            .take(3)
            .collect();
        assert_eq!(points.len(), 3);
        for d in 0..=2 {
            let total = if d == 0 { g + 1 } else { g + 2 * d };
            for mask in 1usize..8 {
                let support: Vec<_> = (0..3).filter(|i| mask & (1 << i) != 0).collect();
                if support.len() > total {
                    continue;
                }
                let mut profiles = vec![];
                compositions(total, support.len(), &mut vec![], &mut profiles);
                for ms in profiles {
                    for signs in 0..1usize << (support.len() - 1) {
                        let nodes: Vec<_> = support
                            .iter()
                            .enumerate()
                            .map(|(i, &j)| Node {
                                x: points[j].0,
                                y: points[j].1,
                                m: ms[i],
                                neg: i > 0 && signs & (1 << (i - 1)) != 0,
                            })
                            .collect();
                        let (n, h, drop, split) = check(c, &nodes, d, true);
                        cases += 1;
                        common += h as usize;
                        drops += drop as usize;
                        splits += split as usize;
                        higher += (n > 1) as usize;
                    }
                }
            }
            println!("NONBRANCH g={g} p={p} d={d} total_conditions={total} cumulative_cases={cases} independent_CRT_EEA=true");
        }
    }
    let mut branchfixtures = 0;
    for g in 2..=4 {
        let (c, points) = (1..60)
            .find_map(|v| {
                let c = thread_curve(101, v, g)?;
                let fb = factor_base(&c);
                let branch = fb.iter().find(|p| p.1 == 0).copied()?;
                let regular = fb.iter().find(|p| p.1 != 0).copied()?;
                Some((c, [branch, regular]))
            })
            .expect("fixed_branch_search");
        for d in 0..=2 {
            let total = if d == 0 { g + 1 } else { g + 2 * d };
            for m in 1..total {
                let nodes = [
                    Node {
                        x: points[0].0,
                        y: 0,
                        m,
                        neg: false,
                    },
                    Node {
                        x: points[1].0,
                        y: points[1].1,
                        m: total - m,
                        neg: false,
                    },
                ];
                let (n, h, drop, split) = check(&c, &nodes, d, false);
                cases += 1;
                common += h as usize;
                drops += drop as usize;
                splits += split as usize;
                higher += (n > 1) as usize;
            }
        }
        let branch = Node {
            x: points[0].0,
            y: 0,
            m: 2,
            neg: false,
        };
        let (a, b, q, kept, _) =
            normalize(&c, &[branch], Poly::x_minus(branch.x, &c.fp), Poly::zero());
        assert_eq!(
            (a, b, q, kept),
            (Poly::one(), Poly::zero(), Poly::one(), vec![0])
        );
        branchfixtures += 1;
        println!("BRANCH g={g} p=101 local_uniformizer=y forced_vertical_control=true");
    }
    let inst = pick_instance(101, 2).unwrap();
    let (x, y) = factor_base(&inst.curve)
        .into_iter()
        .find(|p| p.1 != 0)
        .unwrap();
    let node = Node {
        x,
        y,
        m: 2,
        neg: false,
    };
    let h = Poly::x_minus(x, &inst.curve.fp);
    let (_, _, _, kept, _) = normalize(&inst.curve, &[node], h.scale(y, &inst.curve.fp), h);
    assert_eq!(kept, vec![1], "nonbranch_cancellation_retains_one_order");
    let (xx, mut yy) = jets(&inst.curve, node);
    yy[1] = inst.curve.fp.add(yy[1], 1);
    let expected = confluent::compose(&inst.curve.f, &xx, &inst.curve.fp);
    let corrupt = inst.curve.fp.mul(inst.curve.fp.add(yy[0], yy[0]), yy[1]);
    assert_ne!(corrupt, expected[1], "corrupted_jet_detected");
    assert_eq!(branchfixtures, 3);
    assert!(common > 0 && splits > 0);
    println!("PASS cases={cases} common_factor_cases={common} degree_drops={drops} split_principal_checks={splits} higher_nullity={higher} branch_fixtures=3 corrupted_jet_rejected=true nonbranch_vs_branch_cancellation=true timing_claim=false");
}
