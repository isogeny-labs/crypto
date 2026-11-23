"""Writes the frozen ecbench specs for PROTOCOL.md (pair-table l growing with n vs fixed l = 7, K1 n = 17, 19, 23, 29).
L_STAR is frozen from the disclosed sizing in sizing/ (2 targets per n, seeds 9999xxxx, never used below)."""
import json, pathlib
HERE = pathlib.Path(__file__).resolve().parents[1]
L_STAR = {17: 7, 19: 7, 23: 9, 29: 10}
def pt(l): return {"id": "ic.pipeline", "params": {"factor_base": f"koblitz-standard-subspace:dimension={l}",
                   "oracle": "pdp3-koblitz:m=3,engine=rootfind-negfold", "max_trials": "10000000"}}
for n in (17, 19, 23, 29):
    for j in (0, 1):
        seed = 20262400 + 10 * n + j
        arms = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
                {"name": "ic-pt-l7", "role": "baseline", "method": pt(7)}]
        if L_STAR[n] != 7: arms.append({"name": f"ic-pt-l{L_STAR[n]}", "role": "candidate", "method": pt(L_STAR[n])})
        spec = {"schema": "ecbench.spec/v1", "label": f"fb-pt-lgrow K1 n={n} job {j}",
                "description": "Frozen by experiments/fb-pt-lgrow-k1/PROTOCOL.md. Pair-table at l*(n) vs fixed l = 7 vs strong rho. No security claim.",
                "arms": arms,
                "workloads": {"curves": [{"kind": "koblitz", "a": 1, "n": n}], "targets_per_curve": 4, "target_seed": seed},
                "measurement": {"rounds": 5, "warmup": 1, "order": "alternate", "seed": seed, "isolation_required": "L2", "timeout_seconds": 1800}}
        (HERE / "specs" / f"spec-n{n}-j{j}.json").write_text(json.dumps(spec, indent=1) + "\n")
print("ok")
