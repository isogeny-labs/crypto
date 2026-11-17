// W1 / V4 dumper. Reuses research/ghs_cubic_quartic_ic_20261012/quartic_ic.rs
// UNMODIFIED (textually included into a module so its private items are
// reachable). For the first split GHS instance at q (the instance quartic_ic.rs
// itself builds), for every valid transfer phi(nG) (n = 1..ell-1) it writes:
//   * n, the deg-6 key quartic_ic.rs uses (div_key), and
//   * the 4-dimensional F_q space of cubics through the degree-6 transfer
//     divisor (via quartic_ic.rs's own cubic_matrix / nullspace_basis_fq),
// plus the quartic over F_q. Build: rustc --edition 2021 -O dump_extra.rs
// Run:   ./dump_extra 5 > results/v4_dump_q5.jsonl
#![allow(dead_code, unused_imports, unused_variables)]

mod qic {
    include!("../ghs_cubic_quartic_ic_20261012/quartic_ic.rs");

    pub fn dump(q: u32) {
        let mut cost = Cost::new(q);
        let inst = Inst::build_first_split(q, &mut cost).expect("no split instance");
        let e = E0::build(inst.clone(), &mut cost).expect("no E0");
        let g = e.find_generator().expect("no generator");
        let f1 = &inst.f1;
        let ell = inst.ell;
        let p0 = inst.fb[inst.p0];
        let curve_k = inst.enum_curve_k(&mut cost);
        // quartic over F_q as (i,j,k,coefficient) with coefficients as integers mod q
        let mut terms = Vec::new();
        for ((i, j, k), c) in inst.gwn.iter() {
            terms.push(format!("[{},{},{},{}]", i, j, k, f1.idx_of[*c as usize]));
        }
        println!(
            "{{\"rec\":\"header\",\"q\":{},\"ell\":{},\"ord\":{},\"quartic\":[{}],\"cubic_monomials\":[{}]}}",
            q,
            ell,
            inst.ord,
            terms.join(","),
            cubic_monomials()
                .iter()
                .map(|m| format!("[{},{},{}]", m.0, m.1, m.2))
                .collect::<Vec<_>>()
                .join(",")
        );
        let mut n_valid = 0;
        let mut n_not_deg6 = 0;
        let mut n_bad_dim = 0;
        for n in 1..ell {
            let p = e.mul(g, n, &mut cost);
            if !e.valid_transfer(p) {
                continue;
            }
            n_valid += 1;
            let d6 = e.transfer_div(p, &mut cost);
            if d6.deg() != 6 {
                n_not_deg6 += 1;
                continue;
            }
            let (mons, mat) = cubic_matrix(&inst, &d6, &mut cost);
            let basis = nullspace_basis_fq(f1, mat);
            if basis.len() != 4 {
                n_bad_dim += 1;
                println!(
                    "{{\"rec\":\"skip\",\"n\":{},\"reason\":\"cubic space dim {}\"}}",
                    n,
                    basis.len()
                );
                continue;
            }
            let rows: Vec<String> = basis
                .iter()
                .map(|r| {
                    let v: Vec<String> = r.iter().map(|&x| f1.idx_of[x as usize].to_string()).collect();
                    format!("[{}]", v.join(","))
                })
                .collect();
            let key = div_key(&inst, &d6);
            // the degree-3 representative quartic_ic.rs's own reduction (pic_canon) gives
            let pic3 = pic_canon(&inst, &d6, p0, &curve_k, &mut cost);
            let key3 = div_key(&inst, &pic3);
            println!(
                "{{\"rec\":\"transfer\",\"n\":{},\"deg6_key\":\"{:?}\",\"pic3_deg\":{},\"pic3_key\":\"{:?}\",\"cubics\":[{}]}}",
                n,
                key,
                pic3.deg(),
                key3,
                rows.join(",")
            );
        }
        println!(
            "{{\"rec\":\"summary\",\"q\":{},\"ell\":{},\"valid_transfers\":{},\"not_degree6\":{},\"cubic_dim_not_4\":{}}}",
            q, ell, n_valid, n_not_deg6, n_bad_dim
        );
    }
}

fn main() {
    let q: u32 = std::env::args().nth(1).expect("q").parse().expect("q");
    qic::dump(q);
}
