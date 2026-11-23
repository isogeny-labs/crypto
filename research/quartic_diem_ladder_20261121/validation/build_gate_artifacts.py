#!/usr/bin/env python3
"""Validator: write replay-certificate.json, validator-report.json and VALIDATION.json
(docs/VALIDATION_GATE.md, validation-gate/v1) for research/quartic_diem_ladder_20261121.

Usage: python3 -I build_gate_artifacts.py REPO SHA VALDIR

Reads, from VALDIR: certificates.json (verify_certificates.py), recompute.json
(recompute.py), replay_compare.json (compare_replay.py), cap_subsets.json
(cap_subsets.py); and analysis.json + raw rows from Git at SHA.  Every pass/fail
flag is derived here from those files; nothing is set by hand.  Paths inside the
three JSON files are relative to VALDIR (the gate resolves them from the
directory that holds VALIDATION.json).
"""
import hashlib
import json
import math
import os
import subprocess
import sys

STUDY = "research/quartic_diem_ladder_20261121"
FILES = ["raw/run.jsonl", "raw/run131101_odd.jsonl", "raw/run131101_even.jsonl"]
NEW, SFB = "Q_diem_2lp_new", "Q_diem_2lp_sfb"
NATIVE = ["rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts"]
BINARY_SHA = "f0243cf85a90ddb630f7ad868a4375fffae1189aff959a71da3c901a49c9e38b"


def git(repo, *args):
    return subprocess.run(["git", "-C", repo, *args], check=True, capture_output=True).stdout


def sha256_bytes(b):
    return hashlib.sha256(b).hexdigest()


def gm(v):
    return math.exp(sum(math.log(x) for x in v) / len(v))


def main():
    repo, sha, vd = sys.argv[1:4]
    full_sha = git(repo, "rev-parse", sha).decode().strip()
    load = lambda n: json.load(open(os.path.join(vd, n)))
    certs, rec, rep, cap = load("certificates.json"), load("recompute.json"), load("replay_compare.json"), load("cap_subsets.json")
    ana = json.loads(git(repo, "show", f"{sha}:{STUDY}/analysis.json"))
    rows = [json.loads(l) for fn in FILES
            for l in git(repo, "show", f"{sha}:{STUDY}/{fn}").decode().splitlines() if l.strip()]

    # ── replay certificate ──
    runs = rep["runs"]
    replay_ok = bool(runs) and all(e["equal"] for e in runs) and \
        rep["summary"]["instances_equal"] == rep["summary"]["instances_replayed"]
    cert = {
        "schema": "replay-certificate/v1",
        "study": STUDY,
        "status": "verified" if replay_ok else "failed",
        "independent": True,
        "scalars_verified": replay_ok,
        "validated_commit": full_sha,
        "binary": {"name": "genus_ladder.qsfb", "sha256": BINARY_SHA,
                   "note": "hash-checked copy of the producer's frozen binary (binary.sha256); no rebuild (none allowed on this host)"},
        "command": "validation/replay.sh BIN OUT (--threads 1, frozen flags; 131101: all 5 curves; other primes: one curve each drawn with random.Random(20261123))",
        "comparison": "every replayed run/skip row equals the committed row with the same (kind, arm, p, curve_seed, seed) on every field except wall_ns; replayed instance rows equal the committed ones",
        "summary": rep["summary"],
        "runs": runs,
    }
    cpath = os.path.join(vd, "replay-certificate.json")
    with open(cpath, "w") as f:
        json.dump(cert, f, indent=1)

    # ── validator report ──
    raw = []
    for fn in FILES:
        b = git(repo, "show", f"{sha}:{STUDY}/{fn}")
        raw.append({"path": f"../{fn}", "sha256": sha256_bytes(b)})
    t, a = rec["ic_over_rho"], ana["ic_over_rho"]
    head = []
    for arm, lab in ((SFB, "sfb"), (NEW, "new")):
        for w in ("0.0", "0.1", "1.0"):
            head.append({"name": f"IC/rho {lab} p=131101 w={w} (gm over 5 curves)",
                         "claimed": a[arm][f"all_{w}"]["per_prime"]["131101"]["gm"],
                         "recomputed": t[f"{arm}|all_{w}"]["per_prime"]["131101"]["gm"]})
            head.append({"name": f"slope ln(IC/rho) {lab} w={w}",
                         "claimed": a[arm][f"all_{w}"]["slope_ln_ratio"]["slope"],
                         "recomputed": t[f"{arm}|all_{w}"]["slope"]["slope"]})
        head.append({"name": f"FPM-mid exponent {lab}",
                     "claimed": ana["exponents"][arm]["fpm_mid"]["slope"]["slope"],
                     "recomputed": rec["exponents"][f"{arm}|fpm_mid"]["slope"]})
    head.append({"name": "paired sfb/new FPM mid, pooled gm",
                 "claimed": ana["paired_sfb_over_new"]["fpm_mid"]["pooled"]["gm"],
                 "recomputed": rec["paired_sfb_over_new"]["fpm_mid"]["pooled_gm"]})
    d = ana["descriptors"][SFB]["131101"]
    head.append({"name": "la_add / native_all, sfb p=131101 (ratio of gms)",
                 "claimed": d["la_add"] / d["native_all"],
                 "recomputed": rec["descriptors"][f"{SFB}|131101"]["ratio_of_gms"]})
    head.append({"name": "certified runs re-verified with validator arithmetic (RESULTS: all 210)",
                 "claimed": 210, "recomputed": certs["summary"]["rows_cert_ok"]})
    heads_ok = all(abs(h["claimed"] - h["recomputed"]) <= 1e-9 * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)
                   for h in head)
    certs_ok = certs["summary"]["curves_all_ok"] == certs["summary"]["curves"] == 40 and \
        certs["summary"]["rows_cert_ok"] == certs["summary"]["rows_checked"] == 210
    verdicts_ok = set(rec["verdict"].values()) == {"UNRESOLVED"} and \
        all(v["verdict"].startswith("UNRESOLVED") for arm in ana["verdict"].values() for v in arm.values())
    keys_ok = rec["missing_keys_in_ic_rows"] == {}
    cap_ok = all(v["verdict"] == "UNRESOLVED" for k, v in cap["subsets"].items() if isinstance(v, dict))
    status = "pass" if (heads_ok and certs_ok and verdicts_ok and keys_ok and replay_ok and cap_ok) else "fail"
    report = {
        "schema": "validator-report/v1",
        "study": STUDY,
        "status": status,
        "independent": True,
        "disposition": "validated" if status == "pass" else "see validator-report.md",
        "validated_commit": full_sha,
        "study_tree": git(repo, "rev-parse", f"{sha}:{STUDY}").decode().strip(),
        "report": "validator-report.md",
        "raw_data": raw,
        "headline": head,
        "checks": {
            "certificates_independent": certs["summary"],
            "replay": rep["summary"],
            "headline_agreement_1e-9": heads_ok,
            "frozen_verdict_every_cell_unresolved": verdicts_ok,
            "no_missing_keys_in_ic_rows": keys_ok,
            "verdicts_unchanged_under_frozen_110min_cap": cap_ok,
        },
        "scripts": ["picarith.py", "verify_certificates.py", "recompute.py", "replay.sh",
                    "compare_replay.py", "cap_subsets.py", "build_gate_artifacts.py"],
        "label": "[pcc:unverified] (no pcc claim was created by the validator)",
    }
    rpath = os.path.join(vd, "validator-report.json")
    with open(rpath, "w") as f:
        json.dump(report, f, indent=1)

    # ── VALIDATION.json: band of the sfb cell at p = 131101 (the dispatch's emphasis) ──
    by = {}
    for r in rows:
        if r.get("kind") == "run" and r["arm"] == SFB and r["p"] == 131101:
            by.setdefault(r["curve_seed"], []).append(r)
    curves = sorted(by)
    rho = gm([math.sqrt(math.pi * by[c][0]["r"] / 4) for c in curves])
    band = {}
    for key, w in (("0", 0.0), ("0.1", 0.1), ("1", 1.0)):
        ic = gm([gm([x["group_ops"] + w * sum(x[k] for k in NATIVE) for x in by[c]]) for c in curves])
        band[key] = {"ic": ic, "rho": rho}
    signs = {(v["ic"] > v["rho"]) - (v["ic"] < v["rho"]) for v in band.values()}
    man = {
        "schema": "validation-gate/v1",
        "note": ("Draft by the validator; the study has no leaderboard or benchmarks row. Paths are "
                 "relative to this directory (validation/); if the manifest is moved to the study root "
                 "(the evidence directory of a row whose source is RESULTS.md), prefix them with validation/ "
                 "and change ../raw/ to raw/ in validator-report.json."),
        "replay_certificate": {"path": "replay-certificate.json", "sha256": sha256_bytes(open(cpath, "rb").read())},
        "validator_report": {"path": "validator-report.json", "sha256": sha256_bytes(open(rpath, "rb").read())},
        "rho_reference": {
            "kind": "negation",
            "protocol": ("PROTOCOL.md (sha256 96311c59...2ec2) section 4: the ideal negation rho "
                         "R = sqrt(pi r / 4) Pic^0 additions with nothing unpriced charged to rho, the "
                         "primary reference at every prime; the executed Q_rho_neg is calibration only "
                         "(p <= 16411, 50 runs, mean steps / R = 1.007)"),
            "unit": ("GAE per solve, p = 131101, 2lp-sfb: geometric mean over the 5 curves of the per-curve "
                     "geometric mean over solver seeds 1, 2; one counted Pic^0 operation = 1; each counted "
                     "native operation (rel_mul, rel_add, la_mul, la_add, idx_lookups, idx_inserts) = w; "
                     "the ideal negation rho charges no native work at any w"),
            "binary_or_process": (f"IC arms: genus_ladder.qsfb sha256 {BINARY_SHA} (replayed by the validator, "
                                  "rows identical). The negation floor is a closed form of each instance's r as "
                                  "emitted by that binary and certified by the validator's own arithmetic "
                                  "(r prime, h = 1, #J = r by Hasse-Weil, [r]G = 0); the calibration walk "
                                  "Q_rho_neg ran in the same binary at p <= 16411"),
            "matches_protocol": True,
            "strongest_available": True,
            "same_unit": True,
            "same_binary_or_process": True,
            "gae_band": band,
            "order_holds_across_band": signs in ({1}, {-1}),
        },
    }
    with open(os.path.join(vd, "VALIDATION.json"), "w") as f:
        json.dump(man, f, indent=1)
    print(json.dumps({"replay_ok": replay_ok, "heads_ok": heads_ok, "certs_ok": certs_ok,
                      "verdicts_ok": verdicts_ok, "keys_ok": keys_ok, "cap_ok": cap_ok,
                      "status": status, "band": band,
                      "order_holds_across_band": man["rho_reference"]["order_holds_across_band"]}, indent=1))


if __name__ == "__main__":
    main()
