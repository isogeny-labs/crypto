"""Write the frozen claim specs (layout only; computes no result).

    python3 research/gaudry_k4_char2_replication_20261026/make_specs.py
One spec per (m, target, arm) so a container restart loses at most one run.
"""
import hashlib
import json
import pathlib

HERE = pathlib.Path(__file__).resolve().parent
IC_PARAMS = {  # the claim's ic-wied arm, unchanged
    "factor_base": "bin-t-line",
    "oracle": "gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1,charge_unsolved=1",
    "max_trials": "50000000",
    "linalg": "wiedemann",
}
ARMS = {
    "ic": {"role": "candidate", "method": {"id": "ic.pipeline_f2m4_refmm", "params": IC_PARAMS}},
    "rho-batched": {"role": "reference", "method": {"id": "rho.negation_batched_refmm"}},
    "rho-neg": {"role": "reference", "method": {"id": "rho.negation_f2m4_refmm"}},
}


def seed(tag, m, i):
    h = hashlib.sha256(f"gaudry-k4-redteam-v2|{tag}|{m}|{i}".encode()).digest()
    return int.from_bytes(h[:8], "big") >> 1  # 63 bits, fits every seed field


for m in (14, 15, 16, 17):
    for i in range(8):
        curve_seed = 1 if i < 4 else 2
        for arm, a in ARMS.items():
            spec = {
                "schema": "ecbench.spec/v1",
                "label": f"k=4 char-2 replication: m={m} target {i} arm {arm}",
                "description": "PROTOCOL.md (research/gaudry_k4_char2_replication_20261026), frozen before data.",
                "workloads": {
                    "curves": [{"kind": "f2m4_torsion", "m": m, "seed": curve_seed, "max_cofactor": 2}],
                    "targets_per_curve": 1,
                    "target_seed": seed("target", m, i),
                },
                "arms": [{"name": arm, **a}],
                "measurement": {
                    "rounds": 1, "warmup": 0, "order": "alternate",
                    "seed": seed(f"alg-{arm}", m, i),
                    "isolation_required": "L0",
                    "timeout_seconds": 172800,
                },
            }
            out = HERE / "specs" / f"m{m}-t{i}-{arm}.json"
            out.write_text(json.dumps(spec, indent=2) + "\n")
