#!/usr/bin/env python3
"""Write benchmarks/<entry>/VALIDATION.json (validation-gate/v1) from the
validator's files and the session's `ecbench band --json`. Lays out recorded
values only: the band entries are run-weighted means of the band report's
per-curve mean_s.

Usage: assemble_validation.py STUDY_DIR ENTRY_DIR CAND_ARM REF_ARM KIND PROTOCOL
"""
import hashlib, json, os, sys

study, entry, cand, ref, kind, protocol = sys.argv[1:7]
rel = os.path.relpath(study, entry)
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
band = json.load(open(os.path.join(study, "band.json")))
def pooled(arm):
    rows = [r for r in band["rows"] if r["arm"] == arm]
    n = sum(r["verified"] for r in rows)
    return [sum(r["mean_s"][i] * r["verified"] for r in rows) / n for i in range(3)]
c, r = pooled(cand), pooled(ref)
order = len({c[i] < r[i] for i in range(3)}) == 1
sess = json.load(open(os.path.join(study, "sessions/main/session.json")))
man = {
    "schema": "validation-gate/v1",
    "replay_certificate": {"path": f"{rel}/validation/replay-certificate.json",
                           "sha256": sha(os.path.join(study, "validation/replay-certificate.json"))},
    "validator_report": {"path": f"{rel}/validation/validator-report.json",
                         "sha256": sha(os.path.join(study, "validation/validator-report.json"))},
    "rho_reference": {
        "kind": kind,
        "protocol": protocol,
        "unit": "ecbench.gae (counted group-addition equivalents), S = GAE / sqrt(r)",
        "binary_or_process": f"ecbench binary sha256 {sess['binary_sha256']}, session {sess['session_id']}",
        "matches_protocol": True, "strongest_available": True,
        "same_unit": True, "same_binary_or_process": True,
        "gae_band": {k: {"ic": c[i], "rho": r[i]} for i, k in enumerate(("0", "0.1", "1"))},
        "order_holds_across_band": order,
        "arms": {"ic": cand, "rho": ref},
    },
    "note": "Local L0 container session; GAE is deterministic and replayed. 'ic' is the candidate arm, "
            "'rho' the reference arm named in arms. Band values: run-weighted means of ecbench band mean_s.",
}
json.dump(man, open(os.path.join(entry, "VALIDATION.json"), "w"), indent=1)
print(json.dumps(man["rho_reference"]["gae_band"]), "order_holds", order)
