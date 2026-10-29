"""Specs of the large-prime ladder (atlas lead LEAD-ICB-large-prime-variants).

Every arm is `ic.pipeline` on the Koblitz curve K_0 over F_2^n with the compact-orbit-scan
base and the counted three-summand Frobenius pair table (`mitm-frobenius-counted:m=3`),
differing only in the oracle's large-prime parameters and the number of columns.  The
matched reference is `rho.signed_frobenius_strong` (the strong signed-Frobenius rho of the
IC measurement rules).  Counted S = GAE / sqrt(r) is the metric; native work is priced at
0, 0.1 and 1 GAE per operation by `ecbench band` (AGENTS.md section 5a).
"""
import json, pathlib, sys

HERE = pathlib.Path(__file__).parent
CAP = {53: 400_000_000, 61: 400_000_000}
RHO = {"id": "rho.signed_frobenius_strong", "params": {"lanes": "32", "dp_bits": "8", "step_cap_factor": "2000"}}


def ic(n, cols, lp=None, c=None):
    o = "mitm-frobenius-counted:m=3"
    if lp is not None:
        o += f",large_primes={lp},lp_domain_factor={c},lp_raw_x_cap={CAP[n]}"
    return {"id": "ic.pipeline", "params": {"factor_base": f"compact-orbit-scan:columns={cols},raw_x_cap={CAP[n]}",
                                               "oracle": o, "targets": "walk", "max_trials": "40000000"}}


# (arm name, role, columns, lp, c).  Plain arms bracket the best plain table; each LP arm has an
# equal-envelope plain control (columns = k (1 + c)) in the plain list.
ARMS = {
    53: [("plain-k80", 80, None, None), ("plain-k220", 220, None, None), ("plain-k320", 320, None, None),
         ("plain-k440", 440, None, None), ("plain-k660", 660, None, None),
         ("lp1-k20-c10", 20, 1, 10), ("lp1-k40-c10", 40, 1, 10),
         ("lp2-k20-c10", 20, 2, 10), ("lp2-k20-c30", 20, 2, 30), ("lp2-k40-c10", 40, 2, 10),
         ("lp2-k80-c3", 80, 2, 3), ("lp2-k40-c3", 40, 2, 3)],
}


def spec(n, target_seed):
    arms = [{"name": "rho-strong", "role": "reference", "method": RHO}]
    for name, cols, lp, c in ARMS[n]:
        arms.append({"name": name, "role": "candidate", "method": ic(n, cols, lp, c)})
    # controls: A/A duplicate of the best plain table, and the large-prime collector switched off
    arms.append({"name": "plain-k440-aa", "role": "control", "method": ic(n, 440)})
    arms.append({"name": "lp0-k440", "role": "control", "method": ic(n, 440, 0, 10)})
    return {"schema": "ecbench.spec/v1",
            "label": f"large-prime collector on the Koblitz compact-orbit m=3 pipeline, n={n}",
            "description": "lead LEAD-ICB-large-prime-variants: 1LP/2LP partial relations against the plain pair table at equal and optimal envelope",
            "workloads": {"curves": [{"kind": "koblitz", "a": 0, "n": n}], "targets_per_curve": 8,
                          "target_seed": target_seed, "target_kind": "public"},
            "arms": arms,
            "measurement": {"rounds": 2, "warmup": 0, "order": "alternate", "seed": 20261029,
                            "isolation_required": "L2", "timeout_seconds": 3600}}


if __name__ == "__main__":
    for n in ARMS:
        for i, ts in enumerate((202610291, 202610292, 202610293, 202610294), 1):
            (HERE / "specs" / f"n{n}-s{i}.json").write_text(json.dumps(spec(n, ts), indent=1) + "\n")
            print("wrote", f"specs/n{n}-s{i}.json")
