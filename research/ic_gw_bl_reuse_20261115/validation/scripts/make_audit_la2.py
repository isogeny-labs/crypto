#!/usr/bin/env python3
"""Make the validator's oracle-audit la2.rs from the BLR la2.rs (scratch build only, never committed as source).

usage: make_audit_la2.py BLR_LA2_RS OUT_LA2_RS

In block_lanczos, wherever BLR uses a reused value, the audit build also computes the direct gram it
replaced (gram(mvpr, mv) for identity 1, gram(v, v0) for identity 2) with the LMUL/LADD counters saved
before and restored after, so the run's counted fields are unchanged; it compares the two values exactly
and accumulates the counted cost of the skipped grams and of the recurrence. One stderr line per call:
BLAUDIT {json}. Mutants for negative controls: --cfg mut_c2 drops the c2^T (V_{i-2}^T V0) term of the
recurrence; --cfg mut_pm replaces the reused MV_{i-1}^T MV_i by zeros.
"""
import sys

src = open(sys.argv[1]).read()

def rep(old, new, count=1):
    global src
    n = src.count(old)
    if n != count:
        sys.exit(f"anchor found {n} times, expected {count}: {old[:80]!r}")
    src = src.replace(old, new)

rep("    let mut last_c2: Option<(Vec<u64>, usize)> = None; // k_{i-2} x k_{i-1}\n",
    "    let mut last_c2: Option<(Vec<u64>, usize)> = None; // k_{i-2} x k_{i-1}\n"
    "    let mut a_narrow_at: Vec<u64> = Vec::new(); let mut a_narrow_k: Vec<(usize, usize)> = Vec::new();\n"
    "    let (mut a_id1_used, mut a_id1_bad, mut a_id1_direct) = (0u64, 0u64, 0u64);\n"
    "    let (mut a_id2_used, mut a_id2_bad, mut a_id2_direct) = (0u64, 0u64, 0u64);\n"
    "    let (mut a_skip_lmul, mut a_skip_ladd, mut a_rec_lmul, mut a_rec_ladd) = (0u64, 0u64, 0u64, 0u64);\n"
    "    let mut a_first_bad2: i64 = -1;\n")
rep("                st.narrowed += 1;\n",
    "                st.narrowed += 1;\n                a_narrow_at.push(iters);\n")
rep("                sel_used = sel.clone();\n",
    "                sel_used = sel.clone();\n                a_narrow_k.push((k, sel.len()));\n")
rep("        // X += V Winv (V^T V0)\n",
    "        // X += V Winv (V^T V0)\n        let a_s0 = crate::common::snap();\n")
rep("        } else { gram(&v, &v0, ell) }; // k x b\n",
    "        } else { gram(&v, &v0, ell) }; // k x b\n"
    "        {\n"
    "            let a_s1 = crate::common::snap();\n"
    "            let used2 = reuse2 && !narrowed0 && iters >= 2 && h_vt0.len() == 2 && last_c1.is_some();\n"
    "            if used2 {\n"
    "                a_id2_used += 1; a_rec_lmul += a_s1.lmul - a_s0.lmul; a_rec_ladd += a_s1.ladd - a_s0.ladd;\n"
    "                let (l0, d0) = unsafe { (crate::common::LMUL, crate::common::LADD) };\n"
    "                let direct = gram(&v, &v0, ell);\n"
    "                let (l1, d1) = unsafe { (crate::common::LMUL, crate::common::LADD) };\n"
    "                a_skip_lmul += l1 - l0; a_skip_ladd += d1 - d0;\n"
    "                unsafe { crate::common::LMUL = l0; crate::common::LADD = d0; }\n"
    "                if direct != vt0 { a_id2_bad += 1; if a_first_bad2 < 0 { a_first_bad2 = iters as i64; } }\n"
    "            } else { a_id2_direct += 1; }\n"
    "        }\n")
rep("                if let Some((c2, kpp)) = &last_c2 { for p in 0..*kpp {",
    "                if let (false, Some((c2, kpp))) = (cfg!(mut_c2), &last_c2) { for p in 0..*kpp {")
rep("                for p in 0..vpr.k { for (r, &c) in sel_used.iter().enumerate() { t[p * k + r] = w_full[p * k_full + c]; } }\n",
    "                for p in 0..vpr.k { for (r, &c) in sel_used.iter().enumerate() { t[p * k + r] = if cfg!(mut_pm) { 0 } else { w_full[p * k_full + c] }; } }\n")
rep("            } else { gram(mvpr, &mv, ell) }; // kp x k\n",
    "            } else { gram(mvpr, &mv, ell) }; // kp x k\n"
    "            if vpr.k == k_full {\n"
    "                a_id1_used += 1;\n"
    "                let (l0, d0) = unsafe { (crate::common::LMUL, crate::common::LADD) };\n"
    "                let direct = gram(mvpr, &mv, ell);\n"
    "                let (l1, d1) = unsafe { (crate::common::LMUL, crate::common::LADD) };\n"
    "                a_skip_lmul += l1 - l0; a_skip_ladd += d1 - d0;\n"
    "                unsafe { crate::common::LMUL = l0; crate::common::LADD = d0; }\n"
    "                if direct != pm { a_id1_bad += 1; }\n"
    "            } else { a_id1_direct += 1; }\n")
rep("    st.iters += iters;\n",
    "    eprintln!(\"BLAUDIT {{\\\"b\\\":{},\\\"n\\\":{},\\\"iters\\\":{},\\\"narrowed0\\\":{},\\\"narrow_at\\\":{:?},\\\"narrow_k\\\":{:?},\\\"id1_used\\\":{},\\\"id1_bad\\\":{},\\\"id1_direct\\\":{},\\\"id2_used\\\":{},\\\"id2_bad\\\":{},\\\"id2_direct\\\":{},\\\"first_bad2\\\":{},\\\"skip_lmul\\\":{},\\\"skip_ladd\\\":{},\\\"rec_lmul\\\":{},\\\"rec_ladd\\\":{}}}\",\n"
    "        b, n, iters, narrowed0, a_narrow_at, a_narrow_k.iter().map(|&(x, y)| vec![x, y]).collect::<Vec<_>>(), a_id1_used, a_id1_bad, a_id1_direct, a_id2_used, a_id2_bad, a_id2_direct, a_first_bad2, a_skip_lmul, a_skip_ladd, a_rec_lmul, a_rec_ladd);\n"
    "    st.iters += iters;\n")
open(sys.argv[2], "w").write(src)
