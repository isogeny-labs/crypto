"""Specs of the pair-table / target-guard layout study (lead ICP-prime-ic-constant-engineering).

ab-<bits>   the incumbent walk and the 16/32-lane walks on the tuned base of each size, with
            rho.negation as the reference arm; run once by the old binary and once by the new
            one, alternated, so each pair of sessions differs in the table layout only.
"""
import json
import pathlib

HERE = pathlib.Path(__file__).parent
SEEDS = {24: 59293, 28: 59297, 32: 59297}   # the curves of the earlier prime IC studies
BASE = {24: 256, 28: 512, 32: 1024}          # the tuned base of each size (research/ic_wide_ext_20261028)


def arm(name, role, method):
    return {"name": name, "role": role, "method": method}


def ic(base, targets):
    return {"id": "ic.pipeline", "params": {"factor_base": f"prime-abscissa:size={base}",
                                            "oracle": "mitm-prime-batch:m=2", "targets": targets}}


def spec(bits):
    b = BASE[bits]
    arms = [arm("rho-neg", "reference", {"id": "rho.negation"}),
            arm("walk", "candidate", ic(b, "walk")),
            arm("L16", "candidate", ic(b, "walk-prime-batch-16")),
            arm("L32", "candidate", ic(b, "walk-prime-batch-32"))]
    return {"schema": "ecbench.spec/v1", "label": f"table layout old/new, {bits}-bit",
            "description": "lead ICP-prime-ic-constant-engineering: flat pair table and open-addressed target guard",
            "workloads": {"curves": [{"kind": "prime_search", "bits": bits, "seed": SEEDS[bits]}],
                          "targets_per_curve": 4, "target_seed": 20261002},
            "arms": arms,
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": 1,
                            "isolation_required": "L2", "timeout_seconds": 3600}}


if __name__ == "__main__":
    (HERE / "specs").mkdir(exist_ok=True)
    for bits in BASE:
        (HERE / "specs" / f"ab-{bits}.json").write_text(json.dumps(spec(bits), indent=1) + "\n")
        print("wrote", f"specs/ab-{bits}.json")
