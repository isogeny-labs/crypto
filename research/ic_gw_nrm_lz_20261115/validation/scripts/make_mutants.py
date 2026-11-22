#!/usr/bin/env python3
"""Negative-control mutants of CAND (validator): each makes one reuse/skip inexact.
MUT_LZWY: w_{i+1}^T y recurrence drops the b1 term. MUT_LZVV: v^T v_prev taken as v^T v (wrong identity).
MUT_NRM: g3 also skips the product by b[1] (operand not 1). usage: make_mutants.py CAND_SRC OUT_PARENT"""
import shutil, sys, pathlib
src, parent = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
muts = {
 "mut_lzwy": ("la2.rs", "let nw = ls2(t, lm2(b1, wy_prev, ell), ell);", "let nw = t; let _ = wy_prev;"),
 "mut_lzvv": ("la2.rs", "let vvp = if lz { s } else { dot(&v, &v_prev, ell) };", "let vvp = if lz { dot(&v, &v, ell) } else { dot(&v, &v_prev, ell) };"),
 "mut_nrm": ("ic2.rs", "f.mul(ga[1], b[1])), f.mul(ga[2], b[2]));", "if nb { ga[1] } else { f.mul(ga[1], b[1]) }), f.mul(ga[2], b[2]));"),
}
for name, (f, old, new) in muts.items():
    d = parent / name / "src"
    if d.exists():
        shutil.rmtree(d)
    shutil.copytree(src, d)
    t = (d / f).read_text()
    assert t.count(old) == 1, (name, t.count(old))
    (d / f).write_text(t.replace(old, new))
    print(name, "ok")
