"""Write the ecbench specs of the wide-batching ladder (lead ICP-prime-ic-constant-engineering).

One spec per curve size.  Every arm is `ic.pipeline` with the same factor base and
oracle (`mitm-prime-batch:m=2`, negation-folded x-only pair table); only `targets`
changes:

  walk                    the incumbent guarded r-adding walk, one target at a time
  walk-prime-fast         the same walk on native Montgomery additions
  walk-prime-serial-1024  1024 synchronized walks, additions done one by one (control)
  walk-prime-batch-N      N synchronized walks, one shared inversion per round

`rho.negation` is the matched reference arm of the section 5a band.
"""
import json, pathlib

HERE = pathlib.Path(__file__).parent
SIZES = {20: 128, 24: 256, 28: 512, 32: 1024}
TARGETS = ["walk", "walk-prime-fast", "walk-prime-batch-16", "walk-prime-batch-64",
           "walk-prime-batch-256", "walk-prime-batch-1024", "walk-prime-serial-1024"]
SEEDS = {20: 59291, 24: 59293, 28: 59297, 32: 59297}


def spec(bits, base):
    arms = [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}]
    for t in TARGETS:
        arms.append({"name": t.replace("walk-prime-", "").replace("walk", "walk"), "role": "candidate",
                     "method": {"id": "ic.pipeline",
                                "params": {"factor_base": f"prime-abscissa:size={base}",
                                           "oracle": "mitm-prime-batch:m=2", "targets": t}}})
    return {"schema": "ecbench.spec/v1",
            "label": f"wide batching ladder, {bits}-bit prime curve",
            "description": "lead ICP-prime-ic-constant-engineering: two-level target batching, widths 16..1024",
            "workloads": {"curves": [{"kind": "prime_search", "bits": bits, "seed": SEEDS[bits]}],
                          "targets_per_curve": 4, "target_seed": 20261002},
            "arms": arms,
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": 1,
                            "isolation_required": "L2", "timeout_seconds": 1800}}


for bits, base in SIZES.items():
    (HERE / "specs" / f"wide-{bits}.json").write_text(json.dumps(spec(bits, base), indent=1) + "\n")
    print("wrote", f"specs/wide-{bits}.json")
