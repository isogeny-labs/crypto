#!/bin/sh
set -eu
base=research/hec_pairic13_20261115
pin=4e4c523ac3f126b08c92060a8e0332e5133637fb
git show "$pin":src/cryptanalysis/diem_quartic.rs > /tmp/hec-pairic-diem-source.rs
{
 printf '%s\n' 'use std::collections::{BinaryHeap,HashMap,HashSet};' 'use std::hash::{BuildHasherDefault,Hasher};' '#[derive(Default)]'
 awk '/^pub struct FxHasher/ {copy=1} copy {print} /^pub type FxSet/ {copy=0}' /tmp/hec-pairic-diem-source.rs
 awk '/^pub struct Mont/ {copy=1} copy && /^\/\/ ---/ {copy=0} copy {print}' /tmp/hec-pairic-diem-source.rs
 printf '%s\n' 'pub type SRow=Vec<(u32,u64)>;'
 awk '/^fn axpy_sub/ {copy=1} /^\/\/\/ Dense rank/ {copy=0} copy {print}' /tmp/hec-pairic-diem-source.rs
} > "$base/lp_reference.rs"
cp "$base/lp_reference.rs" "$base/lp_trace.rs"
# Preserve numerical operations/pivoting; record normalized rows for reverse reconstruction.
perl -0pi -e 's/pub fn eliminate_lp\(/pub fn eliminate_lp_trace(/;s/\) -> Vec<SRow> \{\n    use/\) -> (Vec<SRow>,Vec<(u32,SRow)>) {\n    use/;s/    let mut rows: Vec/    let mut trace=Vec::new();\n    let mut rows: Vec/;s/\*ops \+= prow.len\(\) as u64;/\*ops += prow.len() as u64;\n        trace.push((c,prow.clone()));/;s/    rows.into_iter\(\)/    let remaining=rows.into_iter()/;s/        \.collect\(\)\n\}/        .collect();\n    (remaining,trace)\n}/' "$base/lp_trace.rs"
"${RUSTFMT:-rustfmt}" "$base/lp_reference.rs" "$base/lp_trace.rs"
