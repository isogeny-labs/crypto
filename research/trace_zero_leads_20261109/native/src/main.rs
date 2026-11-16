use full_trace_zero_backend::binary_ecc::{TraceZeroAffineMode as Mode, TraceZeroVariety};
use full_trace_zero_backend::cryptanalysis::binary_semaev_chain_sat::ChainedS3Encoding;
use serde_json::json;
use trace_zero_leads::{fixture, window};
fn main() {
    let mut rows = Vec::new();
    for n in [83, 131] {
        let curve = fixture(n);
        let v = TraceZeroVariety::new(&curve).unwrap();
        for d in [8, 16, 32, 64.min(n as usize - 1)] {
            let basis = window(&curve, d);
            let profile = v.subspace_profile(&basis, 2_000_000).unwrap();
            let mut models = Vec::new();
            // A bounded exact construction, not a solver benchmark.
            for mode in [Mode::Raw, Mode::Compact, Mode::Factored] {
                let s = v
                    .affine_system_over_subspace_with_mode(&basis, mode, 2_000_000)
                    .unwrap();
                models.push(json!({"mode":format!("{mode:?}"),"variables":s.variables,"equations":s.equations.len(),
                    "degree":s.maximum_degree(),"terms":s.monomials(),"and_aux":s.auxiliary_variables()}));
            }
            rows.push(json!({"n":n,"basis":basis.iter().map(|b|b.to_biguint().to_str_radix(16)).collect::<Vec<_>>(),
                "dimension":d,"q_polar_rank":profile.polar_rank,"q_linear_terms":profile.linear_terms,
                "q_quadratic_terms":profile.quadratic_terms,"q_terms":profile.restricted_q.terms,
                "product_span_dimension":profile.product_span_dimension,"models":models}));
        }
    }
    let mut circuits = Vec::new();
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let full_trace_zero_backend::binary_ecc::BinaryPoint::Affine { x, .. } = &c.generator
        else {
            panic!()
        };
        for mask in [0, 1] {
            let mut e =
                ChainedS3Encoding::build(3, &c.irreducible, &c.b, Some(x), mask, 500_000).unwrap();
            e.constrain_summands(&[x.to_biguint()], 1_000_000).unwrap();
            let before = json!({"variables":e.solver.n_vars(),"clauses":e.solver.n_clauses(),"xors":e.solver.n_xors()});
            e.constrain_full_trace_zero_summands(&v, 500_000, 2_000_000)
                .unwrap();
            let after = json!({"variables":e.solver.n_vars(),"clauses":e.solver.n_clauses(),"xors":e.solver.n_xors()});
            circuits.push(
                json!({"n":n,"arity":3,"identity_mask":mask,"finite_x_domain_size":1,
                "certified_domain":before,"explicit_points":after}),
            );
        }
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"trace-zero-leads/v1",
        "scope":"exact polynomial and circuit construction; no solver timing, yield or scaling claim",
        "standing":"unverified","subspaces":rows,"circuits":circuits})).unwrap());
}
