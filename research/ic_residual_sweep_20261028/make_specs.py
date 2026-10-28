"""Specs of the residual-sweep ladder (lead ICP-residual-partial-relations).

Arms per curve size: the matched `rho.negation` reference, the incumbent pair-table
recipe (`mitm-prime-batch:m=2`, `targets=walk-prime-fast`, base ~ r^(1/3)), and the
single-large-prime residual sweep (`residual-sweep:m=2`) at base sizes B = 2 .. 128.
`rho.negation_lanes` (round 3) is a second reference: it is the fastest generic search
in wall time, and the sweep at small B is a birthday collision of the same kind.
"""
import json, pathlib

HERE = pathlib.Path(__file__).parent
INCUMBENT_BASE = {20: 128, 24: 256, 28: 512, 32: 1024}
SWEEP_B = [2, 4, 8, 16, 32, 64, 128]
SEEDS = {20: 59291, 24: 59293, 28: 59297, 32: 59297}


def spec(bits):
    arms = [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
            {"name": "rho-lanes16", "role": "candidate",
             "method": {"id": "rho.negation_lanes", "params": {"lanes": 16}}},
            {"name": f"pair-table-b{INCUMBENT_BASE[bits]}", "role": "candidate",
             "method": {"id": "ic.pipeline",
                        "params": {"factor_base": f"prime-abscissa:size={INCUMBENT_BASE[bits]}",
                                   "oracle": "mitm-prime-batch:m=2", "targets": "walk-prime-fast"}}}]
    for b in SWEEP_B:
        arms.append({"name": f"sweep-b{b}", "role": "candidate",
                     "method": {"id": "ic.pipeline",
                                "params": {"factor_base": f"prime-abscissa:size={b}",
                                           "oracle": "residual-sweep:m=2", "targets": "walk"}}})
    return {"schema": "ecbench.spec/v1",
            "label": f"single-large-prime residual sweep, {bits}-bit prime curve",
            "description": "lead ICP-residual-partial-relations: sweep + large-prime pairing vs the pair-table recipe",
            "workloads": {"curves": [{"kind": "prime_search", "bits": bits, "seed": SEEDS[bits]}],
                          "targets_per_curve": 4, "target_seed": 20261002},
            "arms": arms,
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": 1,
                            "isolation_required": "L2", "timeout_seconds": 1800}}


for bits in INCUMBENT_BASE:
    (HERE / "specs" / f"sweep-{bits}.json").write_text(json.dumps(spec(bits), indent=1) + "\n")
    print("wrote", f"specs/sweep-{bits}.json")
