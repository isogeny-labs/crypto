#!/usr/bin/env python3
"""Write src/bin/val_dump.rs into an extracted (scratch) tree: a copy of that
tree's frozen src/bin/ic_positive_control.rs whose row JSON additionally carries
the recovered logarithm, the instance's secret, f, G and T. Nothing else in the
job dispatch changes, so the counted fields must equal the frozen binary's.
Usage: make_dumper.py TREE"""
import sys

tree = sys.argv[1]
src = open(f"{tree}/src/bin/ic_positive_control.rs").read()
extra_inst = ('"val_t_secret": inst.t_secret, "val_f": inst.curve.f_coeffs(), '
              '"val_G": [u64::from(inst.g.deg), u64::from(inst.g.u[0]), u64::from(inst.g.u[1]), u64::from(inst.g.u[2]), u64::from(inst.g.v[0]), u64::from(inst.g.v[1]), u64::from(inst.g.v[2])], '
              '"val_T": [u64::from(inst.target.deg), u64::from(inst.target.u[0]), u64::from(inst.target.u[1]), u64::from(inst.target.u[2]), u64::from(inst.target.v[0]), u64::from(inst.target.v[1]), u64::from(inst.target.v[2])], ')
a = '"kind": "ic", '
b = '"kind": "rho", '
assert src.count(a) == 1 and src.count(b) == 1
src = src.replace(a, a + '"val_recovered": i.recovered, ' + extra_inst)
src = src.replace(b, b + '"val_recovered": o.recovered, ' + extra_inst)
open(f"{tree}/src/bin/val_dump.rs", "w").write(src)
print("wrote", f"{tree}/src/bin/val_dump.rs")

# Second pass: rho rows also carry the reference walk's own counters and its
# work_native (window word ops), which run_rho does not report.
src = open(f"{tree}/src/bin/val_dump.rs").read()
anchor = '"val_recovered": o.recovered, '
assert src.count(anchor) == 1
src = src.replace(anchor, anchor + '"val_rr": val_rr(inst, seed), ')
src += '''
fn val_rr(inst: &Instance, seed: u64) -> Value {
    let max_steps = (60.0 * (inst.r as f64).sqrt()) as u64 + 10_000;
    let rr = crypto_lib::cryptanalysis::ic_boundary::rho_reference_walk(
        &inst.curve, inst.g, inst.target, inst.r, seed, max_steps,
        crypto_lib::cryptanalysis::ic_boundary::RhoWalk::negation(),
    );
    json!({"work_native": rr.work_native, "counters": rr.counters, "steps": rr.steps,
           "recovered": rr.recovered, "group_ops": rr.group_ops.adds + rr.group_ops.doubles})
}
'''
open(f"{tree}/src/bin/val_dump.rs", "w").write(src)
print("added val_rr")

# Third pass: the extra json! fields exceed serde_json's default macro recursion.
src = open(f"{tree}/src/bin/val_dump.rs").read()
if not src.startswith("#![recursion_limit"):
    src = '#![recursion_limit = "512"]\n' + src
open(f"{tree}/src/bin/val_dump.rs", "w").write(src)
print("recursion_limit set")
