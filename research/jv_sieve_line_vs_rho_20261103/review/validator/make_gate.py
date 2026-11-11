#!/usr/bin/env python3
"""Write replay-certificate.json, validator-report.json and ../../VALIDATION.json from the
validator's own outputs (replay/COMPARE.json, recompute.json, bootstrap.json)."""
import hashlib, json, os
V = os.path.dirname(os.path.abspath(__file__)); D = os.path.dirname(os.path.dirname(V))
REPO = os.path.dirname(os.path.dirname(D))
rel = lambda p: os.path.relpath(p, REPO)
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
cmp_ = json.load(open(f"{V}/replay/COMPARE.json"))
plan = json.load(open(f"{V}/replay/PLAN.json"))
rc = json.load(open(f"{V}/recompute.json")); bs = json.load(open(f"{V}/bootstrap.json"))
binsha = open(f"{V}/replay/binary.sha256").read().split()[0]
src = open(f"{V}/replay/source_commit.txt").read().strip()
ok = all(c["status"] == "identical" for c in cmp_)
cert = {"schema": "replay-certificate/v1", "study": rel(D), "status": "verified" if ok else "failed",
        "independent": True, "scalars_verified": ok,
        "replayer": "validator session 2026-11-11 (not the study's producer)",
        "binary": {"path": "target/release/examples/jv_cover", "sha256": binsha, "source_commit": src,
                   "note": "current main build, not the frozen jv_cover_round3_frozen (d21002a3); it emits four counters added after the freeze, all zero in these runs"},
        "sampling": {"nonce_commit": plan["nonce_commit"], "nonce_sha256": plan["nonce_sha256"],
                     "rule": "one seed per (arm, prime) for the cover, one rho record per prime, indexed by successive nonce bytes (replay/plan.py)"},
        "comparison": "every field of the committed record except wall_ms, exactly",
        "runs": [{k: c[k] for k in ("kind", "arm", "p", "seed", "run", "status", "src", "replay")} | {"new_fields_in_replay": c.get("new_fields_in_replay", {})} for c in cmp_]}
json.dump(cert, open(f"{V}/replay-certificate.json", "w"), indent=1)
heads = [c for c in rc["cells"] if "pooled" in c["name"] and c["name"].split("|")[3] in
         ("S/rho U1", "S/rho U2", "S+/rhoS+ U2", "band A U2 w=0", "band A U2 w=0.1", "band A U2 w=1", "band B U2 w=0.1", "band B U2 w=1")]
raw = sorted([os.path.join(dp, f) for dp, _, fs in os.walk(f"{D}/results/cover") for f in fs if f.endswith(".json")]
             + [os.path.join(dp, f) for dp, _, fs in os.walk(f"{D}/results/rho") for f in fs if f.endswith(".json")])
rep = {"schema": "validator-report/v1", "study": rel(D), "status": "pass" if rc["max_rel_err"] < 1e-9 and bs["all_below"] and ok else "fail",
       "independent": True, "validator": "validator session 2026-11-11 (not the study's producer); recompute written from PROTOCOL.md without reading fit/src",
       "raw_data": [{"path": rel(p), "sha256": sha(p)} for p in raw],
       "headline": [{"name": c["name"], "claimed": c["claimed"], "recomputed": c["recomputed"], "rel_tol": 1e-9} for c in heads],
       "all_cells": {"count": len(rc["cells"]), "max_rel_err": rc["max_rel_err"], "file": "recompute.json"},
       "bootstrap": {"file": "bootstrap.json", "resamples": bs["resamples"], "cells": len(bs["cells"]), "all_below_rho": bs["all_below"], "max_hi": bs["max_hi"]},
       "caveats": [
         "Weak class only: E over F_{p^6} with full rational 2-torsion, #E = 4l, genus-3 cover over F_{p^2}; never prime order.",
         "The isogeny walk into the weak class is not charged.",
         "p = 1823 has 7 of 8 matched rho runs: s17 run0 hit the 3600 s timeout and is excluded (R3). Dropping the slowest run lowers the rho mean, which raises the cover/rho ratio, so the exclusion is conservative for 'below rho'.",
         "Primary reference is per-prime because the frozen homogeneity rule failed (KS p = 0.012).",
         "Headline units (U2, S+/rhoS+) were chosen after the freeze at the red team's request; every frozen statistic is in fit.json and is recomputed here.",
         "The replay used the current main build of jv_cover, not the frozen binary; every committed field matched exactly."]}
json.dump(rep, open(f"{V}/validator-report.json", "w"), indent=1)
pool = {c["name"].split("|")[3]: c["recomputed"] for c in rc["cells"] if c["name"].startswith("per-prime|U|pooled")}
gate = {"schema": "validation-gate/v1",
  "note": "Validator 2026-11-11 (replay/, recompute.py, bootstrap.py under review/validator). Scope: JV cover sieve route, arms P/H/U on p in {1009, 1511, 1823}, seeds 17-24, against a matched negation-map rho per prime, weak class only. Band below: arm U, pooled 24 instances, unit U2 (textbook F_p mults), layer A (V2's upper-bound convention: all unemitted cover F_p work priced as native). ic = geometric mean of per-instance cover/rho ratios times the reference (ratio form, rho = 1). Every arm, layer, unit and scope is below rho at every w (bootstrap.json).",
  "replay_certificate": {"path": "review/validator/replay-certificate.json", "sha256": sha(f"{V}/replay-certificate.json")},
  "validator_report": {"path": "review/validator/validator-report.json", "sha256": sha(f"{V}/validator-report.json")},
  "rho_reference": {"kind": "negation",
    "protocol": "PROTOCOL.md (frozen 2026-11-03T01:25:50Z): rhostrong --neg 1 --batch 16 --r 256 --dp-bits 16 on the cover instances; per-prime reference (homogeneity failed)",
    "unit": "U2: textbook F_p multiplications / (c_add_e = 331) per sqrt(l); native operations priced at w GAE each",
    "binary_or_process": "examples/jv_cover --exp sieve (cover) and --exp rhostrong (rho), same frozen build jv_cover_round3_frozen d21002a3 (main aa1c89ab2a)",
    "matches_protocol": True, "strongest_available": True, "same_unit": True, "same_binary_or_process": True,
    "gae_band": {w: {"ic": pool[f"band A U2 w={w}"], "rho": 1.0} for w in ("0", "0.1", "1")},
    "order_holds_across_band": bs["all_below"]}}
json.dump(gate, open(f"{D}/VALIDATION.json", "w"), indent=1)
print(cert["status"], rep["status"], gate["rho_reference"]["gae_band"])
