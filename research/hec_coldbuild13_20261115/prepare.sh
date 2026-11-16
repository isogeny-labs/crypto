#!/bin/sh
set -eu
b=research/hec_coldbuild13_20261115
# Existing arithmetic unchanged; copied adapters replace precomputed QF/HC with live outputs.
sed '1,2d' research/hec_sourcejet13_20261115/source.rs | perl -pe 's/\bHC\b/cold_hc()/g; s/\bQF\b/cold_qf()/g; s/base\(19\)/base(cold_zeta())/g' > "$b/source_dynamic.rs"
perl -pe 's/\bQF\b/cold_qf()/g' research/hec_publicmap13_20261115/map_v2.rs > "$b/map_dynamic.rs"
perl -0pi -e 's/struct Mapper<\x27a> \{/struct Mapper<\x27a> {\n    lambda: std::cell::Cell<Option<u64>>,/;s/let pair = oriented\(&d, 10\);/let lambda=self.lambda.get().unwrap_or(d.r);\n            self.lambda.set(Some(lambda));\n            let pair=oriented(\&d,lambda);/;s/tj: &tj,/tj: \&tj,\n        lambda: std::cell::Cell::new(None),/' "$b/map_dynamic.rs"
perl -pe 's/\bQF\b/cold_qf()/g' research/hec_mappedic13_20261115/pipeline.rs > "$b/pipeline_dynamic.rs"
perl -pe 's/\bQF\b/cold_qf()/g' research/hec_pairic13_20261115/pair.rs > "$b/pair_dynamic.rs"
# Remove reference-only validation from the candidate collector; retain residuals and actual solve.
perl -0pi -e 's/    assert_eq!\(mdot\(&row, reference\), 0\);/    \/\/ Global reference is not read by the cold candidate./;s/    let rm = super::lp_reference::Mont::new\(ELL\);.*?assert_eq!\(\(ops, entries\), \(rops, rentries\)\);/    \/\/ Reference eliminator is excluded from the cold candidate./s;s/    let anchor = \(1\.\.m\).*?let mut checked = 0;/    let scale = 0u64; \/\/ no reference normalization\n    let mut checked = 0;/s;s/trace_matches_reference=true/trace_reference_check=not_executed/' "$b/pair_dynamic.rs"
# Compute extension order from the freshly derived polynomial, not a stored L.
sed '/^pub const L:/d' research/hec_torsion13_20261115/orders.rs > "$b/orders_dynamic.rs"
perl -0pi -e 's/pub fn extension_order\(\)/#[allow(non_snake_case)]\npub fn extension_order(L: [i64;7], ell:u64)/;s/    assert_eq!\(L.iter\(\).sum::<i64>\(\), 2057786\);\n//;s/1028893u32/ell/g' "$b/orders_dynamic.rs"
# Main wrapper keeps all old studies immutable.
perl -pe 's@../hec_sourcejet13_20261115/source.rs@source_dynamic.rs@;s@../hec_publicmap13_20261115/map_v2.rs@map_dynamic.rs@;s@../hec_mappedic13_20261115/pipeline.rs@pipeline_dynamic.rs@;s@include!\("pair.rs"\);@include!("pair_dynamic.rs");\n    include!("builder.rs");@;s@previous::run_pair\(\)@previous::run_coldbuild()@;s@include!\("lp_reference.rs"\)@include!("../hec_pairic13_20261115/lp_reference.rs")@;s@include!\("lp_trace.rs"\)@include!("../hec_pairic13_20261115/lp_trace.rs")@' research/hec_pairic13_20261115/main.rs > "$b/main.rs"
perl -0pi -e 's/\n}\nfn main\(\)/\n    include!("source_counts.rs");\n}\nfn main()/;' "$b/main.rs"
cat >> "$b/main.rs" <<'END'
#[allow(dead_code)] mod orders_dynamic { include!("orders_dynamic.rs"); }
END
"${RUSTFMT:-rustfmt}" "$b/"*.rs
# Compile-only historical entry points also need the new live calibration field.
perl -0pi -e 's/tj: &tj,/tj: \&tj,\n        lambda: std::cell::Cell::new(None),/' "$b/pipeline_dynamic.rs" "$b/pair_dynamic.rs"
perl -0pi -e 's/reference: &\[u64\]/_reference: \&[u64]/;s/    let \(mut rops, mut rentries\) = \(0, 0\);\n//;s/sparse.clone\(\)/sparse/;s/    let scale = 0u64; \/\/ no reference normalization\n//;s/reference_scale=\{scale\}/reference_comparison=not_executed/' "$b/pair_dynamic.rs"
"${RUSTFMT:-rustfmt}" "$b/"*.rs
