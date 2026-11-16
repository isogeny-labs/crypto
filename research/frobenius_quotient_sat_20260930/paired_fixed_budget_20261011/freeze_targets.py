#!/usr/bin/env python3
"""Freeze the target manifest for the paired fixed-budget comparison.

Writes, once and deterministically:
  manifest.json        what the solver arms may read: curve, factor-space basis,
                       representatives, three target sets with full coordinates
  witness_sealed.json  construction witnesses and brute-force answers; NEVER
                       read by run_paired.py (only by verify.py after the run)
  manifest.sha256      SHA-256 of manifest.json

The script refuses to overwrite an existing file whose bytes differ, so the
manifest cannot be regenerated after solver runs.  Identical regeneration is a
no-op.  Same seeds always give byte-identical output.

Target sets (4 targets each, n = 13, s = 4):
  a  planted, 4 distinct representatives, random signs, random Frobenius phase
     per summand (redrawn if all four phases are 0)
  b  planted, 4 distinct representatives, random signs, all phases 0
  c  uniform in H \\ {O}, H = 4E, as [k]g for a random odd-subgroup g

Each set draws from its own stream random.Random(target_seed*10000 + n*10 + i),
i = 1, 2, 3 for a, b, c.  The factor space is the one of ../README.md:
run.factor_space with random.Random(factor_seed*1000 + n).
"""
import argparse
import os
import random
import sys

from paired_common import (FACTOR_SEED, HERE, MANIFEST, MANIFEST_SHA, N,
                           PER_SET, S, SEALED, SET_NAMES, TARGET_SEED, Curve,
                           dumps, frob_pt, hx, sha256_bytes,
                           subgroup_generator)

from brute_check import admissible, decomposable  # parent harness
from run import factor_space  # parent harness

STREAM = {"a": 1, "b": 2, "c": 3}


def _planted(E, reps_pt, rng, random_phase):
    n = E.f.n
    while True:
        idx = rng.sample(range(len(reps_pt)), 4)
        signs = [rng.getrandbits(1) for _ in range(4)]
        phases = [rng.randrange(n) for _ in range(4)] if random_phase else [0] * 4
        if random_phase and not any(phases):
            continue
        pts = []
        for j, sg, k in zip(idx, signs, phases):
            P = frob_pt(E, reps_pt[j], k)
            pts.append(E.neg(P) if sg else P)
        if len({P[0] for P in pts}) < 4:
            continue
        R = None
        for P in pts:
            R = E.add(R, P)
        if R is None:
            continue
        summands = [{"rep_index": j, "phase": k, "sign": sg, "x": hx(P[0]),
                     "point": [hx(P[0]), hx(P[1])]}
                    for j, sg, k, P in zip(idx, signs, phases, pts)]
        return R, {"kind": "planted", "summands": summands}


def _uniform(E, rng):
    while True:
        g = E.mul(4, E.random_point(rng))
        if g is None:
            continue
        k = rng.randrange(1, E.h)
        return E.mul(k, g), {"kind": "uniform", "multiplier": k,
                             "base": [hx(g[0]), hx(g[1])]}


def build_artifacts(factor_seed=FACTOR_SEED, target_seed=TARGET_SEED, n=N, s=S,
                    per_set=PER_SET):
    """Return (manifest_bytes, sealed_bytes); pure function of the seeds."""
    E = Curve(n)
    basis, valid = factor_space(E, s, random.Random(factor_seed * 1000 + n))
    reps_pt = [E.lift(x) for x in valid]
    g = subgroup_generator(E)
    reps_all, xs_all = admissible(E, basis, True)
    assert reps_all == valid
    manifest = {
        "schema": "frobenius_quotient_sat/paired_fixed_budget/manifest/v1",
        "curve": {
            "equation": "y^2 + x*y = x^3 + 1",
            "n": n,
            "modulus": hx(E.f.m),
            "group_order": E.N,
            "odd_subgroup_order": E.h,
            "trace_t": -1,
            "generator": [hx(g[0]), hx(g[1])],
            "lambda_relation": "pi(P) = [lambda]P on H, lambda^2 - t*lambda + 2 = 0 mod r",
        },
        "factor_space": {
            "seed": factor_seed,
            "rng": "random.Random(seed*1000 + n) into run.factor_space",
            "s": s,
            "basis": [hx(b) for b in basis],
            "representatives": [{"index": j, "x": hx(P[0]),
                                 "point": [hx(P[0]), hx(P[1])]}
                                for j, P in enumerate(reps_pt)],
            "admissible_x_phases_known_zero": len(reps_all),
            "admissible_x_phases_free": len(xs_all),
        },
        "target_seed": target_seed,
        "target_rng": "random.Random(target_seed*10000 + n*10 + i), i=1,2,3 for sets a,b,c",
        "target_sets": {},
    }
    sealed_targets = {}
    seen = set()
    for key in ("a", "b", "c"):
        rng = random.Random(target_seed * 10000 + n * 10 + STREAM[key])
        items = []
        while len(items) < per_set:
            if key == "c":
                R, wit = _uniform(E, rng)
            else:
                R, wit = _planted(E, reps_pt, rng, key == "a")
            if R in seen:
                continue
            seen.add(R)
            tid = f"{key}{len(items)}"
            items.append({"id": tid, "R": [hx(R[0]), hx(R[1])]})
            sealed_targets[tid] = {
                "set": key,
                "construction": wit,
                "brute_force": {
                    "decomposable_reps_only": decomposable(E, reps_all, R),
                    "decomposable_phases_free": decomposable(E, xs_all, R),
                },
            }
        manifest["target_sets"][key] = {"name": SET_NAMES[key], "targets": items}
    mbytes = dumps(manifest)
    sealed = {
        "schema": "frobenius_quotient_sat/paired_fixed_budget/witness_sealed/v1",
        "warning": "construction witnesses; run_paired.py must never read this file",
        "manifest_sha256": sha256_bytes(mbytes),
        "targets": sealed_targets,
    }
    return mbytes, dumps(sealed)


def _write_once(path, data):
    if os.path.exists(path):
        with open(path, "rb") as fh:
            if fh.read() == data:
                return "unchanged"
        sys.exit(f"REFUSING to overwrite {path}: frozen bytes differ from regeneration")
    with open(path, "wb") as fh:
        fh.write(data)
    return "written"


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--factor-seed", type=int, default=FACTOR_SEED)
    ap.add_argument("--target-seed", type=int, default=TARGET_SEED)
    ap.add_argument("--check", action="store_true",
                    help="verify the committed files equal a regeneration; write nothing")
    a = ap.parse_args()
    mbytes, sbytes = build_artifacts(a.factor_seed, a.target_seed)
    sha = sha256_bytes(mbytes)
    if a.check:
        ok = all(os.path.exists(p) and open(p, "rb").read() == b
                 for p, b in ((MANIFEST, mbytes), (SEALED, sbytes)))
        print("manifest_sha256", sha, "MATCH" if ok else "MISMATCH")
        sys.exit(0 if ok else 1)
    for path, data in ((MANIFEST, mbytes), (SEALED, sbytes),
                       (MANIFEST_SHA, (sha + "  manifest.json\n").encode())):
        print(os.path.relpath(path, HERE), _write_once(path, data))
    print("manifest_sha256", sha)


if __name__ == "__main__":
    main()
