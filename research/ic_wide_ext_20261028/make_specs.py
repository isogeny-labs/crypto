"""Specs of the wide-batching extension study (lead ICP-prime-ic-constant-engineering).

diag-<bits>   one curve, the incumbent walk, walk-prime-fast and batch-N for every width
              N in WIDTHS; run with ECBENCH_LANE_DIAG=1 for the per-stage lane timers.
grid-<bits>   joint tuning: (base size) x (oracle) x (targets) for the incumbent walk,
              the 16-lane default of round 3 and the candidate widths.
ladder-<bits> the extended size ladder (incumbent walk, 16 lanes, best width, rho.negation).
"""
import json, pathlib, sys

HERE = pathlib.Path(__file__).parent
SEEDS = {20: 59291, 24: 59293, 28: 59297, 32: 59297}   # earlier studies' curves
WIDTHS = [4, 8, 16, 24, 32, 48, 64, 128, 256, 512, 1024]
BASE = {24: 256, 28: 512, 32: 1024}


def arm(name, role, method):
    return {"name": name, "role": role, "method": method}


def ic(base, oracle, targets):
    return {"id": "ic.pipeline", "params": {"factor_base": f"prime-abscissa:size={base}",
                                            "oracle": oracle, "targets": targets}}


def spec(label, bits, arms, targets=4, rounds=3, warmup=1, seed=None):
    return {"schema": "ecbench.spec/v1", "label": label, "description": "lead ICP-prime-ic-constant-engineering, extension",
            "workloads": {"curves": [{"kind": "prime_search", "bits": bits, "seed": seed or SEEDS[bits]}],
                          "targets_per_curve": targets, "target_seed": 20261002},
            "arms": arms,
            "measurement": {"rounds": rounds, "warmup": warmup, "order": "alternate", "seed": 1,
                            "isolation_required": "L2", "timeout_seconds": 3600}}


def diag(bits):
    b = BASE[bits]
    arms = [arm("rho-neg", "reference", {"id": "rho.negation"}),
            arm("walk", "candidate", ic(b, "mitm-prime-batch:m=2", "walk")),
            arm("fast", "candidate", ic(b, "mitm-prime-batch:m=2", "walk-prime-fast"))]
    arms += [arm(f"b{n}", "candidate", ic(b, "mitm-prime-batch:m=2", f"walk-prime-batch-{n}")) for n in WIDTHS]
    return spec(f"lane width diagnosis, {bits}-bit", bits, arms)


if __name__ == "__main__":
    for bits in BASE:
        (HERE / "specs" / f"diag-{bits}.json").write_text(json.dumps(diag(bits), indent=1) + "\n")
        print("wrote", f"specs/diag-{bits}.json")
