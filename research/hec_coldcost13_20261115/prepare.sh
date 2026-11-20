#!/bin/sh
set -eu
b=research/hec_coldcost13_20261115
perl -pe 's@include!\("(source_dynamic|map_dynamic|pipeline_dynamic|pair_dynamic|orders_dynamic|source_counts)\.rs"\)@include!("../hec_coldbuild13_20261115/$1.rs")@g; s@include!\("builder.rs"\)@include!("builder_bench.rs")@' research/hec_coldbuild13_20261115/main.rs > "$b/worker.rs"
cat >> "$b/worker.rs" <<'END'
#[allow(dead_code)]
#[path="../hec_anchor_relations_20261114/anchor.rs"] mod anchor;
#[allow(dead_code)]
#[path="../../crates/hec-native-ic/src/table_factor.rs"] mod table_factor;
END
cp research/hec_coldbuild13_20261115/builder.rs "$b/builder_bench.rs"
perl -0pi -e 's/pub fn run_coldbuild\(\) \{/pub fn run_coldbuild() {\n    let mut meter=Meter::new();/;s/    if args.get\(2\).is_some_and.*?    let l = cold_weil/    meter.tick("input");\n    let mode=args.get(2).map(|s|s.as_str()).unwrap_or("cold");\n    if mode!="cold" {\n        let k=if mode=="compact" {super::anchor::solve_compact(\&refc,\&d1,\&b,ell,20260916,false).k.expect("compact_cap")}else{\n            let mut p=hec_native_ic::ic::IcParams::default();p.max_trials=200000;p.reference_oracle=false;\n            p.solver=if mode=="hybrid"{hec_native_ic::solve::Solver::Hybrid}else{assert_eq!(mode,"fast");hec_native_ic::solve::Solver::Fast};\n            hec_native_ic::ic::solve_dlp(\&refc,\&d1,\&b,ell,\&p).k.expect("native_cap")\n        };\n        meter.tick("native_solve");assert_eq!(refc.mul(\&d1,k),b);meter.tick("verification");meter.finish(mode,k);return\n    }\n    let l = cold_weil/s; s/    let field = Field::new\(\);/    meter.tick("source_weil");\n    let field = Field::new();/;s/    let ks = &all\[1\];/    meter.tick("field_kernels");\n    let ks = \&all[1];/;s/    let root = refc.fp.sqrt/    meter.tick("weil_normalization");\n    let root = refc.fp.sqrt/;s/    let pts = fast_points\(\);/    meter.tick("geometry");\n    let pts = fast_points();/;s/    let mapper = Mapper \{/    meter.tick("points_jacobian");\n    let mapper = Mapper {/;s/    let g = mapper.map/    meter.tick("relations");\n    let g = mapper.map/;s/    let q = mapper.map/    meter.tick("map_generator");\n    let q = mapper.map/;s/    let gl = descend_known/    meter.tick("map_target");\n    let gl = descend_known/;s/    assert_eq!\(refc.mul\(&d1, k\), b\);/    meter.tick("descent");\n    assert_eq!(refc.mul(\&d1, k), b);/;s/    cold_emit\(&all, &alpha\);/    meter.tick("verification");meter.finish("cold",k);\n    cold_emit(\&all,\&alpha);/;s/timing_comparison=false/instrumented_measurement=true/g' "$b/builder_bench.rs"
cat >> "$b/builder_bench.rs" <<'END'
struct Meter{start:std::time::Instant,last:std::time::Instant,phases:Vec<(&'static str,u128)>}
impl Meter{
 fn new()->Self{let n=std::time::Instant::now();Self{start:n,last:n,phases:Vec::new()}}
 fn tick(&mut self,p:&'static str){let n=std::time::Instant::now();self.phases.push((p,n.duration_since(self.last).as_nanos()));self.last=n}
 fn finish(&self,arm:&str,k:u64){let total=self.start.elapsed().as_nanos();for(p,n)in &self.phases{println!("PHASE arm={arm} name={p} ns={n}")}println!("MEASURE arm={arm} k={k} total_ns={total} source_verified=true")}
}
END
"${RUSTFMT:-rustfmt}" "$b/worker.rs" "$b/builder_bench.rs" "$b/collector.rs"
