#!/usr/bin/env python3
"""Writes the pilot specs (pilot/) and, once the per-curve rule is fixed (RULE below), the frozen confirmatory specs
(3 curves x 3 replicate sessions, specs/). Run from anywhere: make_specs.py pilot | confirm"""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
OR = "mitm-frobenius-layout-counted:m=2,sib=1"
FBX = "raw_x_cap=100000000,generator_orbit=1"
A = {23: 1, 29: 1, 37: 0}
# Incumbent per curve: lean_layout + sib on the validated column choice (koblitz_lean_sib_confirm_20261118).
INC = {23: ("sp-c4-d5", 4), 29: ("gen-c1", 1), 37: ("sp-c8-d5", 8)}
def fb(k, extra=""):
    base = f"compact-orbit-scan:columns={k},{FBX}" + (f",generator_multiples={k},multiple_doublings=5" if k > 1 else "")
    return base + ",lean_layout=1" + extra
def ic(name, role, fbs, ors):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
        "factor_base": fbs, "oracle": ors, "max_trials": "1000000000", "targets": "stride-generator"}}}
def cand(k, m):
    # walk_sib=m on every curve; ladder_verify and orbit_key_reuse only where the base has a doubling chain / several keys.
    return fb(k, ",orbit_key_reuse=1" if k > 1 else ""), OR + f",walk_sib={m}" + (",ladder_verify=1" if k > 1 else "")
def spec(n, label, desc, arms, ts, rounds):
    return {"schema": "ecbench.spec/v1", "label": label, "description": desc,
            "workloads": {"curves": [{"kind": "koblitz", "a": A[n], "n": n}], "targets_per_curve": 48, "target_seed": ts, "target_kind": "public"},
            "arms": arms,
            "measurement": {"rounds": rounds, "warmup": 1, "order": "alternate", "seed": ts + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
# Pilot (separate seeds 20261118471-473): column count k and block half-width m.
PILOT_K = {23: (4, 5, 6), 29: (1,), 37: (7, 8, 9)}
PILOT_M = (1, 2, 4, 8)
PILOT_SEED = {23: 20261118471, 29: 20261118472, 37: 20261118473}
# Confirmatory rule (k, m) per curve: filled from the pilot by the selection rule in PROTOCOL.md section 0 before
# registration; None until then.
RULE = None
CONF_SEED = {23: 20261118481, 29: 20261118484, 37: 20261118487}
def main(which):
    if which == "pilot":
        for n in (23, 29, 37):
            inc, k0 = INC[n]
            arms = [ic("ls", "baseline", fb(k0), OR)]
            for k in PILOT_K[n]:
                for m in PILOT_M:
                    f, o = cand(k, m)
                    arms.append(ic(f"c{k}-m{m}", "candidate", f, o))
            s = spec(n, f"K1 sibling walk pilot, a={A[n]} n={n}", "Pilot (not confirmatory) for koblitz_k1_sibwalk_20261118: k and m per curve; separate seeds.", arms, PILOT_SEED[n], 2)
            p = os.path.join(HERE, "pilot", f"pilot-n{n}.json"); open(p, "w").write(json.dumps(s, indent=1) + "\n"); print(p)
    else:
        assert RULE is not None, "fix RULE from the pilot first"
        for n in (23, 29, 37):
            inc, k0 = INC[n]; k, m = RULE[n]
            f, o = cand(k, m)
            arms = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
                    {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
                    ic("ls", "baseline", fb(k0), OR),
                    ic("ls-aa", "control", fb(k0), OR),
                    ic("ls-off", "control", fb(k0, ",orbit_key_reuse=0"), OR + ",walk_sib=0,ladder_verify=0"),
                    ic("sw", "candidate", f, o)]
            for rep in (1, 2, 3):
                ts = CONF_SEED[n] + rep - 1
                s = spec(n, f"Sibling giant-step walk + reuse of formed points vs lean+sib incumbent, K1 a={A[n]} n={n}, replicate {rep}",
                         "Frozen in research/koblitz_k1_sibwalk_20261118/PROTOCOL.md before its data. Candidate sw = rule (k, m) of the protocol: walk_sib=m (+ ladder_verify=1, orbit_key_reuse=1 where k > 1); rho strong and v5 references in the same session.",
                         arms, ts, 3)
                p = os.path.join(HERE, "specs", f"n{n}-r{rep}.json"); open(p, "w").write(json.dumps(s, indent=1) + "\n"); print(p)
if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "pilot")
