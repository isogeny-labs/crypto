#!/usr/bin/env python3
"""Validator: write replay-certificate.json, validator-report.json, VALIDATION.json
and validation/validation_report.yaml for research/prime_base_oracle_tuning_20261029.

Reads only the validator's own outputs under validation/ (replay receipts,
recompute.json, ecbench compare re-runs) and the committed study files they
check.  Claimed values transcribed from README.md at the validated commit are
listed in README_CLAIMS with their table.  Stdlib only.

    python3 -I research/prime_base_oracle_tuning_20261029/validation/build_gate_files.py VAL-ID
"""
from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

STUDY = "research/prime_base_oracle_tuning_20261029"
S = Path(STUDY)
V = S / "validation"
COMMIT = "deabd9d629c907016794aa673f6a16c68fb5c2c4"
PARENT = "7df34ba67dc6d9f03c4aa2daee24a7e31e784257"
BRANCH = "claude/pbo-results-20261029"
ROW = "icv1-fp20-tm565-6d7d1d32"
BIN = "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/tgt/release/ecbench"
BIN_SHA = "bf5eec6d61fbe83a7e6911bf73d05d847cc0ab23fd6db3342290a26b9f3807d9"
WK = ("0", "0.1", "1")

# Claims as printed in README.md at COMMIT (rounded there).
README_CLAIMS = {
    "ratio_sel_inc": {  # size: {w: (point, lo, hi)}, 3 decimals
        "20": {"0": (0.818, 0.773, 0.866), "0.1": (0.798, 0.752, 0.847), "1": (0.726, 0.677, 0.778)},
        "24": {"0": (0.902, 0.870, 0.933), "0.1": (0.940, 0.905, 0.975), "1": (1.126, 1.073, 1.179)},
        "28": {"0": (0.929, 0.894, 0.964), "0.1": (0.974, 0.935, 1.014), "1": (1.193, 1.134, 1.257)},
        "32": {"0": (1.000, 1.000, 1.000), "0.1": (1.000, 1.000, 1.000), "1": (1.000, 1.000, 1.000)},
    },
    "mean_s": {  # size: arm: (w0, w0.1, w1), 3 decimals
        "20": {"inc": (12.853, 14.826, 32.577), "sel": (10.515, 11.829, 23.655), "rho-neg": (1.357, 1.474, 2.529)},
        "24": {"inc": (13.976, 15.168, 25.892), "sel": (12.605, 14.260, 29.149), "rho-neg": (1.013, 1.104, 1.919)},
        "28": {"inc": (20.478, 22.204, 37.737), "sel": (19.017, 21.618, 45.036), "rho-neg": (0.939, 1.031, 1.851)},
        "32": {"inc": (32.240, 35.309, 62.931), "rho-neg": (0.968, 1.064, 1.930)},
    },
    "gap_rho": {  # size: pair: (w0, w0.1, w1), 2 decimals
        "20": {"rho-neg->inc": (9.47, 10.06, 12.88), "rho-neg->sel": (7.75, 8.02, 9.35)},
        "24": {"rho-neg->inc": (13.79, 13.74, 13.49), "rho-neg->sel": (12.44, 12.92, 15.19)},
        "28": {"rho-neg->inc": (21.80, 21.55, 20.38), "rho-neg->sel": (20.25, 20.98, 24.33)},
        "32": {"rho-neg->inc": (33.30, 33.18, 32.61)},
    },
    "gap_rho_ci_20": {"0": (6.92, 8.70), "1": (8.27, 10.60)},  # sel/rho-neg, 2 decimals
    "stride_inc": {  # size: w: (point, lo, hi), 3 decimals
        "20": {"0": (0.809, 0.758, 0.868), "1": (0.715, 0.661, 0.778)},
        "32": {"0": (0.989, 0.960, 1.019), "1": (0.983, 0.939, 1.029)},
    },
}


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def blob(rel: str) -> str:
    return subprocess.check_output(["git", "rev-parse", f"{COMMIT}:{rel}"], text=True).strip()


def jload(p: Path):
    return json.loads(p.read_text())


def rounding_tol(claimed: float, recomputed: float, decimals: int) -> float:
    half = 0.5 * 10 ** (-decimals)
    return half / max(abs(claimed), abs(recomputed), 1e-300) * (1 + 1e-9)


def main(val_id: str) -> None:
    rc = jload(V / "recompute.json")
    heads, mismatches, side, side_bad = [], [], [], []

    def head(name, claimed, recomputed, tol, src, rec_src, headline=True):
        scale = max(abs(claimed), abs(recomputed), 1e-300)
        ok = abs(claimed - recomputed) <= tol * scale
        ent = {"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": tol,
               "claimed_source": src, "recomputed_by": rec_src}
        if headline:
            if not ok:
                mismatches.append(name)
            heads.append(ent)
        else:
            ent["agrees"] = ok
            side.append(ent)
            if not ok:
                side_bad.append(name)

    PY = "validation/recompute.py (validator, stdlib, records.jsonl read directly)"
    EC = "validator re-run of `ecbench compare` (validation/compare/)"
    for b in ("20", "24", "28", "32"):
        R = rc["sizes"][b]["ratios"]
        for w in WK:
            for pair in ("inc->sel", "inc->size", "inc->stride", "inc->inc-aa", "rho-neg->rho-neg-aa",
                         "rho-neg->sel", "rho-neg->inc", "sel->stride"):
                e = R.get(f"{pair}@{w}")
                if e is None or "claimed_compare" not in e:
                    continue
                a, bb = pair.split("->")
                head(f"P{b} sum S_{bb} / sum S_{a} at w={w} (paired, 96 pairs)", e["claimed_compare"],
                     e["recomputed"], 1e-12,
                     f"analysis/compare-P{b}-{a}__{bb}-w{w}.json ops.ratio_b_over_a", PY)
            # README rounded claims
            p, lo, hi = README_CLAIMS["ratio_sel_inc"][b][w]
            e = R[f"inc->sel@{w}"]
            cmpj = jload(V / "compare" / f"compare-P{b}-inc__sel-w{w}.json")
            ci = cmpj["ops"]["ci95_bootstrap"]
            head(f"P{b} sel/inc at w={w} (README)", p, e["recomputed"], rounding_tol(p, e["recomputed"], 3),
                 "README.md 'Paired ratio of totals' table", PY)
            head(f"P{b} sel/inc 95% bootstrap lower at w={w} (README)", lo, ci[0], rounding_tol(lo, ci[0], 3),
                 "README.md 'Paired ratio of totals' table", EC)
            head(f"P{b} sel/inc 95% bootstrap upper at w={w} (README)", hi, ci[1], rounding_tol(hi, ci[1], 3),
                 "README.md 'Paired ratio of totals' table", EC)
            if b == "20":
                vb = e["validator_bootstrap_ci95"]
                head(f"P20 sel/inc bootstrap lower at w={w}, validator's own resampler (different RNG)",
                     e["claimed_ci95_bootstrap"][0], vb[0], 0.01,
                     f"analysis/compare-P20-inc__sel-w{w}.json ops.ci95_bootstrap",
                     PY + "; two-stage cluster bootstrap, 4000 resamples, seed 29102026; Monte Carlo tolerance 1%")
                head(f"P20 sel/inc bootstrap upper at w={w}, validator's own resampler (different RNG)",
                     e["claimed_ci95_bootstrap"][1], vb[1], 0.01,
                     f"analysis/compare-P20-inc__sel-w{w}.json ops.ci95_bootstrap",
                     PY + "; two-stage cluster bootstrap, 4000 resamples, seed 29102026; Monte Carlo tolerance 1%")
        arms = rc["sizes"][b]["arms"]
        for arm, vals in README_CLAIMS["mean_s"][b].items():
            for i, w in enumerate(WK):
                r = arms[arm]["mean_s"][w]
                head(f"P{b} mean S {arm} at w={w} (README)", vals[i], r, rounding_tol(vals[i], r, 3),
                     "README.md 'Mean S' table", PY)
        for pair, vals in README_CLAIMS["gap_rho"][b].items():
            for i, w in enumerate(WK):
                r = R[f"{pair}@{w}"]["recomputed"]
                head(f"P{b} {pair.replace('->', ' -> ')} gap at w={w} (README)", vals[i], r,
                     rounding_tol(vals[i], r, 2), "README.md 'Gap to matched rho' table", PY)
        for w, (p, lo, hi) in README_CLAIMS["stride_inc"].get(b, {}).items():
            r = R[f"inc->stride@{w}"]["recomputed"]
            ci = jload(V / "compare" / f"compare-P{b}-inc__stride-w{w}.json")["ops"]["ci95_bootstrap"]
            hl = b == "20"  # the 32-bit stride control is outside the 20-bit row
            head(f"P{b} stride/inc at w={w} (README)", p, r, rounding_tol(p, r, 3), "README.md 'Attribution and controls'", PY, hl)
            head(f"P{b} stride/inc lower at w={w} (README)", lo, ci[0], rounding_tol(lo, ci[0], 3), "README.md 'Attribution and controls'", EC, hl)
            head(f"P{b} stride/inc upper at w={w} (README)", hi, ci[1], rounding_tol(hi, ci[1], 3), "README.md 'Attribution and controls'", EC, hl)
    for w, (lo, hi) in README_CLAIMS["gap_rho_ci_20"].items():
        ci = jload(V / "compare" / f"compare-P20-rho-neg__sel-w{w}.json")["ops"]["ci95_bootstrap"]
        head(f"P20 sel/rho-neg lower at w={w} (README)", lo, ci[0], rounding_tol(lo, ci[0], 2), "README.md 'Gap to matched rho'", EC)
        head(f"P20 sel/rho-neg upper at w={w} (README)", hi, ci[1], rounding_tol(hi, ci[1], 2), "README.md 'Gap to matched rho'", EC)
    # completeness and selection
    a20 = rc["sizes"]["20"]["arms"]
    for arm in ("sel", "inc"):
        head(f"P20 {arm} measured runs verified [d]G = Q (README: 96/96)", 96.0, float(a20[arm]["verified"]), 0.0,
             "README.md section 7.3 check 2", PY)
    head("P20 measured records whose [d]G = Q re-verified by the validator's own EC arithmetic (all 672, all arms)",
         672.0, float(rc["sizes"]["20"]["scalar_reverification"]["measured_records"]
                      - len(rc["sizes"]["20"]["scalar_reverification"]["failures"])), 0.0,
         "sessions/holdout-P20 (1008 records, 672 measured)", PY)
    for b in ("20", "24", "28", "32"):
        sl = rc["selection"][b]
        for i, w in enumerate(WK):
            head(f"P{b} train mean S of selected cell {sl['claimed_selected']} at w={w}", sl["claimed_s"][i],
                 sl["selected_s"][i], 1e-12, f"select/selection-P{b}.json selected.s{i}", PY)

    # ------------------------------------------------------------ certificate
    sess = []
    replays = [("sessions/holdout-P20", "holdout-P20", "audits/holdout-P20.json")]
    replays += [(f"sessions/{n}", n, f"audits/{n}.json") for n in (
        "train-P20", "train-ext-P20-mitm3-n91", "train-ext-P20-rfmitm3-n91", "train-ext-P20-rfsub3-n91",
        "train-ext-P20-mitm3-n128", "train-ext-P20-rfmitm3-n128", "train-ext-P20-rfsub3-n128")]
    replays += [("../ic_frontier_campaign_20261015/phase1/sessions/P20", "identity-frontier-P20",
                 "audits/identity-frontier-P20.json")]
    for sdir, name, prod in replays:
        rec = V / f"replay-{name}.audit.json"
        r = jload(rec)
        p = jload(S / prod)
        log = (V / f"replay-{name}.verify.log").read_text().strip().splitlines()
        sd = (S / sdir)
        files_now = {f: sha(sd / f) for f in r["files"]}
        repo_sdir = str((Path(STUDY) / sdir).resolve().relative_to(Path.cwd().resolve()))
        sess.append({
            "session_dir": sdir, "session_id": r["session_id"], "receipt": f"validation/replay-{name}.audit.json",
            "receipt_sha256": sha(rec), "verify_log": f"validation/replay-{name}.verify.log",
            "records": r["records"], "verified_records": r["verified_records"],
            "replays_reproduced": sum(1 for x in r["replays"] if x["reproduced"] is True and x["detail"] == "identical"),
            "replays": len(r["replays"]), "problems": r["problems"], "checks": r["checks"], "ok": r["ok"],
            "session_files_sha256_now_equal_receipt": files_now == r["files"],
            "session_files_equal_producer_receipt": p["files"] == r["files"],
            "session_files_git_blobs_at_commit": {f: blob(f"{repo_sdir}/{f}") for f in r["files"]},
            "records_jsonl_sha256": r["files"]["records.jsonl"],
            "producer_receipt": prod, "producer_receipt_sha256": sha(S / prod),
            "auditor_binary_sha256": r["auditor_binary_sha256"], "auditor_env_class_id": r["auditor_env_class_id"],
            "auditor_hardware_class_id": r["auditor_hardware_class_id"], "audited_unix_ms": r["audited_unix_ms"],
            "end_line": log[-1], "session_ok": r["ok"] and not r["problems"],
        })
    runs = []
    sc = rc["sizes"]["20"]["scalar_reverification"]
    rp = {x["record_id"]: x for x in jload(V / "replay-holdout-P20.audit.json")["replays"]}
    with open(S / "sessions/holdout-P20/records.jsonl") as fh:
        for line in fh:
            x = json.loads(line)
            if x["warmup"]:
                continue
            q = rp.get(x["record_id"], {})
            runs.append({
                "session": "holdout-P20", "seq": x["seq"], "record_id": x["record_id"], "run_id": x["run_id"],
                "arm": x["arm"], "method": x["method"]["id"], "method_id": x["method"]["method_id"],
                "target_index": x["workload"]["target_index"], "round": x["round"],
                "status": x["outcome"]["status"], "recovered": x["outcome"]["recovered"],
                "matches_target": x["outcome"]["matches_target"],
                "validator_dG_equals_Q": x["record_id"] not in sc["failures"],
                "reproduced": q.get("reproduced"), "detail": q.get("detail"),
            })
    cc = rc["sizes"]["20"]["curve_checks"]
    cert = {
        "schema": "replay-certificate/v1",
        "evidence_dir": STUDY,
        "rows": [ROW],
        "snapshot": {
            "branch": BRANCH, "validated_content_commit": COMMIT, "parent": PARENT,
            "note": ("Every study file this certificate names is unchanged in the work tree against "
                     f"{COMMIT} (git status clean for the study dir; validation/ untracked, written by the "
                     "validator). records.jsonl files are bound by sha256 and git blob; they may move to the "
                     "artifact store as .artifacts.json pointers with the same sha256."),
            "sessions_measured_at_commit": PARENT,
        },
        "status": "verified", "independent": True, "scalars_verified": True,
        "independence": {
            "replayer": "Validator subagent (claude-opus-5-5), separate from the producer session; re-executed "
                        "every measured run into new receipts under validation/; producer receipts untouched",
            "same_binary_as_measurement": True, "same_host_class_as_measurement": True,
            "independent_implementation": False,
            "independent_scalar_check": True,
            "statement": ("Same-host, same-binary replay: the recorded counts are reproduced exactly by "
                          "re-execution of the pinned binary, and every recovered scalar re-verifies [d]G = Q "
                          "under the validator's own affine arithmetic. It is not an independent-host or "
                          "independent-implementation replication; PROTOCOL.md section 13 requires a second-host "
                          "replay (ecbench-independent-runner) before a leaderboard promotion."),
        },
        "binary": {"path": BIN, "sha256": sha(Path(BIN)), "equals_session_binary_sha256": sha(Path(BIN)) == BIN_SHA,
                   "equals_host_ecbench_sha256": (S / "host/ecbench.sha256").read_text().split()[0] == BIN_SHA},
        "command": ("RAYON_NUM_THREADS=1 taskset -c 2 <binary> verify --dir " + STUDY +
                    "/sessions/<session> --replay-all --exit-code --out " + STUDY +
                    "/validation/replay-<session>.audit.json (cwd: repository root; one core, sequential)"),
        "independent_scalar_check": {
            "method": "validation/recompute.py: curve coefficients derived from G and the 48 targets, [d]G "
                      "recomputed in affine coordinates for every measured record",
            "curve": {k: cc[k] for k in ("p", "a", "b", "r", "trace", "j_invariant", "aut_group_order",
                                         "rG_is_identity", "r_prime", "p_prime", "all_targets_and_G_on_curve")},
            "measured_records": sc["measured_records"], "failures": sc["failures"],
        },
        "sessions": sess,
        "runs": runs,
    }
    cert_p = S / "replay-certificate.json"
    cert_p.write_text(json.dumps(cert, indent=1) + "\n")

    # ------------------------------------------------------------ raw data
    raw = []

    def add(rel, **extra):
        p = S / rel
        repo = str((Path(STUDY) / rel).resolve().relative_to(Path.cwd().resolve()))
        raw.append({"path": rel, "sha256": sha(p), "git_blob": blob(repo), **extra})

    for n in ("holdout-P20", "train-P20", "train-ext-P20-mitm3-n91", "train-ext-P20-rfmitm3-n91",
              "train-ext-P20-rfsub3-n91", "train-ext-P20-mitm3-n128", "train-ext-P20-rfmitm3-n128",
              "train-ext-P20-rfsub3-n128"):
        sj = jload(S / f"sessions/{n}/session.json")
        add(f"sessions/{n}/session.json", session_dir=f"sessions/{n}",
            records_jsonl={"sha256": sj["records_sha256"],
                           "git_blob": blob(f"{STUDY}/sessions/{n}/records.jsonl"),
                           "note": "named through its session dir; may move to the artifact store with this sha256"})
        for f in ("spec.json", "plan.json", "host.json"):
            add(f"sessions/{n}/{f}")
    for rel in ("select/FROZEN.sha256", "select/selection-P20.json", "select/band-train-P20.json",
                "specs/holdout-P20.json", "audits/holdout-P20.json", "audits/identity-frontier-P20.json",
                "analysis/band-holdout.json", "host/ecbench.sha256", "host/host.json", "host/commit.txt"):
        add(rel)
    for f in sorted((S / "analysis").glob("compare-P20-*.json")):
        add(f"analysis/{f.name}")
    for b in ("24", "28", "32"):
        add(f"select/selection-P{b}.json")
        for w in WK:
            add(f"analysis/compare-P{b}-inc__sel-w{w}.json")
    add("../../backlog/XP-20261016-prime-base-oracle-tuning/PROTOCOL.md",
        protocol_git_blob_expected="0ee2e28732a57f45cbd283bb796d2d41b3933a94")

    a20 = rc["sizes"]["20"]["arms"]
    band = {w: {"ic": a20["sel"]["mean_s"][w], "rho": a20["rho-neg"]["mean_s"][w]} for w in WK}
    band_inc = {w: {"ic": a20["inc"]["mean_s"][w], "rho": a20["rho-neg"]["mean_s"][w]} for w in WK}

    limitations = [
        "PROTOCOL.md section 13 requires a second-host replay (ecbench-independent-runner) before a leaderboard promotion; none exists. This validation does not meet that requirement and does not authorize promotion.",
        "Replay is same-host and same-binary (env class ECBENV3hc4d998f5ae3f, same container class as the measurement); not an independent-host or independent-implementation replication. The validator's own [d]G = Q check and stdlib recompute are independent of ecbench code.",
        "One fresh holdout curve at 20 bits (icv1-fp20-tm565-6d7d1d32, log2 r = 19.62); 48 targets x 2 rounds. The interval covers target-to-target variation on that curve only, not curve-to-curve variation. No exponent, pooled or other-size claim.",
        "Counted unit only: ecbench GAE / sqrt(r), with unpriced native work priced at w = 0, 0.1, 1 (section 5a). At w = 0 the IC arms are lower bounds (`lower_bound: true`). No wall-clock claim; wall ratios in the compare files are L0 and descriptive.",
        "The 20-bit holdout gain (18.2% at w = 0, 27.4% at w = 1) is larger than PROTOCOL.md section 10 prediction 4 ('any holdout gain is under about 10%'); the README does not mention the prediction. The train sweep already showed it (mitm2 n45 vs n32 at w = 0.1: 11.24 vs 13.92). Not a decision criterion; recorded as an unexpected observation.",
        "Shallow train minimum at 20 bits: mitm2 n45 11.243 vs n64 11.413 at w = 0.1 (8 runs per cell). Selection is a grid choice; a better N between grid points is not excluded.",
        "Random-base control absent (PROTOCOL.md section 12.2); the stride-7919 control is cheaper than sel at every size (stride/sel 0.989 / 0.989 / 0.985 at 20 bits), so the gain is attributed to N, not to the low-x choice.",
        "Binary-to-source binding: the binary sha256 equals every session's binary_sha256 and host/ecbench.sha256; that it was built from code.commit 0e124fb1b is asserted by the card, not rebuilt here. src/, Cargo.toml and Cargo.lock are identical between 0e124fb1b and 7df34ba67 (the commit host/commit.txt records).",
        "Route: run locally on one core of a 4-vCPU container (PBO_CPU=3 instead of the card's 31), not on isolab-eks. The card's run block names the local route; counted costs are deterministic (A/A exactly 1, 672/672 replays identical), so the core and route do not affect S.",
        "README 'Raw sessions: stored in the artifact store (.artifacts.json pointers)' describes a planned move; at the validated commit records.jsonl files are in git. Raw data here is bound by session dir, session.json and records sha256.",
        "README section 7.3 check 1 cites `run.sh verify` (verify/); the validator did not rely on verify/ and replayed holdout-P20, the 20-bit train sessions and the 20-bit identity session itself.",
        "Decision-table rows overlap: at 28 bits the w = 0.1 interval [0.935, 1.014] includes 1, so 'null at this sample size' and 'unresolved' apply as well as 'relabelling'; at 24 bits 'unresolved' applies as well. Every applicable row retains the incumbent, so the README's readings follow the rule.",
        "README misrounds two bounds of the 32-bit stride-control interval (prints [0.960, 1.019] at w = 0 and [0.939, 1.029] at w = 1; the bootstrap gives 1.01850 and 0.93848, i.e. [0.960, 1.018] and [0.938, 1.029]). Outside the 20-bit row and not decision-bearing; the producer should correct the README line.",
        "Not re-checked: panel K (kummer-P22/P24) numbers, median wall times, the figures and PDFs, verify/ and reports/; producer audits for the 24-32-bit sessions and panel K were read as receipts (ok, all replays identical), not replayed by the validator.",
        "The work tree had unrelated tracked modifications during validation (docs/ic-leaderboard.html, docs/ic/LEADERBOARD.md, docs/ic/leaderboard.json, scripts/build_ic_leaderboard.py, modified 02:07Z, after the run); none is under the study dir, src/ or the card.",
    ]

    checks = [
        {"check": "binary sha256 equals host/ecbench.sha256 and every session's binary_sha256", "result": "pass",
         "evidence": "validation/provenance.txt ## binary"},
        {"check": "select/FROZEN.sha256 (sha256sum -c from the study dir)", "result": "pass",
         "evidence": "validation/provenance.txt; all 12 lines OK; FROZEN.sha256 sha256 3d4461e9455d6cd2..."},
        {"check": "PROTOCOL.md git blob = 0ee2e28732a57f45cbd283bb796d2d41b3933a94 (card, HEAD, work tree)", "result": "pass",
         "evidence": "validation/provenance.txt ## protocol; no amendments (section 14 'None.')"},
        {"check": "holdout spec in select/FROZEN = specs/holdout-P<b>.json = sessions/holdout-P<b>/spec.json = template filled from selection", "result": "pass",
         "evidence": "validation/provenance.txt ## holdout specs (all four sizes byte-identical)"},
        {"check": "selection used train data only", "result": "pass",
         "evidence": "selection-P<b>.json band_train_sha256 = sha256(select/band-train-P<b>.json); every band row is the train curve, disjoint from the holdout curve; `ecbench band` re-run on the train sessions reproduces band-train-P<b>.json/.md byte-for-byte (validation/band/); validator's own selection from train records gives mitm2-n45/115/288/1024 (validation/recompute.json selection); FROZEN.sha256 written 01:34:05Z, holdout-P20 started 01:34:06Z"},
        {"check": "edge rule (section 4.2) applied correctly", "result": "pass",
         "evidence": "20 bits: mitm3/rfmitm3/rfsub3 minimum at n64 extended to n91, n128 (minimum then interior at n91); every other family and size has an interior minimum"},
        {"check": "selection tie rule", "result": "pass",
         "evidence": "batch2-nN ties mitm2-nN exactly at w = 0.1 at every size (prediction 2); tie goes to the incumbent oracle"},
        {"check": "holdout workloads, seeds, rounds, arms match PROTOCOL.md sections 3 and 4.4", "result": "pass",
         "evidence": "curve seed 20261018202, target seed 20261018211, algorithm seed 20261018221, 48 targets, 2 rounds + 1 warm-up, timeout 1800 s; arms rho-neg, inc, sel, size, stride, inc-aa, rho-neg-aa"},
        {"check": "incumbent unchanged (section 2)", "result": "pass",
         "evidence": "inc method_id ECM1h586f93741037 (20 bits), ECM1h61a6431fe629 (32 bits) = PROTOCOL.md section 2 ids; fixed params targets=walk, linalg=incremental-gauss, max_trials 1410236, solver '', budget 0"},
        {"check": "replay-all of holdout-P20 on core 2", "result": "pass",
         "evidence": "1008 records, 1008 verified, 672/672 measured replays identical, 0 problems; session files unchanged; receipt files equal producer audits/holdout-P20.json"},
        {"check": "replay-all of 20-bit train sessions (train-P20 + 6 edge extensions) and identity frontier P20", "result": "pass",
         "evidence": "232 + 6 x 16 measured replays identical; identity 240 records verified, 160/160 replays identical"},
        {"check": "every measured holdout-P20 run verified [d]G = Q", "result": "pass",
         "evidence": "ecbench: 96/96 per arm, all 7 arms; validator affine arithmetic: 672/672"},
        {"check": "paired ratios: `ecbench compare --weight` re-run equals committed analysis", "result": "pass",
         "evidence": "84/84 compare JSON files byte-identical (validation/compare/byte-compare.txt)"},
        {"check": "paired ratios: independent stdlib recompute from records", "result": "pass",
         "evidence": "84/84 ratios agree with ecbench to <= 1.07e-15 relative (validation/recompute.json)"},
        {"check": "section 7.3 (1) A/A arms exactly 1 at every weight and size", "result": "pass",
         "evidence": "inc-aa/inc and rho-neg-aa/rho-neg = 1.0 [1, 1] at w = 0, 0.1, 1 for 20-32 bits; per-pair GAE, unpriced counts and scalars identical (96/96 pairs each)"},
        {"check": "section 7.3 (2) completeness", "result": "pass",
         "evidence": "sel 96/96, inc 96/96 verified; 0 censored pairs, 0 unpaired runs; no exhausted/timeout/error"},
        {"check": "section 7.3 (3) 95% upper bound of sel/inc below 1 at all weights", "result": "pass",
         "evidence": "0.8664, 0.8468, 0.7782 (ecbench cluster bootstrap); validator's own resampler 0.8671, 0.8480, 0.7789"},
        {"check": "section 7.3 (4) ecbench band --ranks gives sel < inc", "result": "pass",
         "evidence": "analysis/band-holdout.json (reproduced byte-for-byte in validation/band/) rank 'sel < inc' on icv1-fp20-tm565-6d7d1d32; point ratios 0.818 / 0.798 / 0.726 all < 1"},
        {"check": "24-bit reading 'relabelling, incumbent retained'", "result": "pass",
         "evidence": "w = 0: 0.902 [0.870, 0.933] (falls); w = 1: 1.126 [1.073, 1.179] (rises)"},
        {"check": "28-bit reading 'relabelling, incumbent retained'", "result": "pass",
         "evidence": "w = 0: 0.929 [0.894, 0.964] (falls); w = 0.1: 0.974 [0.935, 1.014] (includes 1); w = 1: 1.193 [1.134, 1.257] (rises)"},
        {"check": "32-bit reading 'null by construction'", "result": "pass",
         "evidence": "selection-P32 = mitm2-n1024 + incumbent oracle; sel method_id = inc method_id ECM1h61a6431fe629; sel/inc = 1.0 [1, 1] at every weight"},
        {"check": "memory (section 6: label a time-memory trade if sel RSS > 2x inc)", "result": "pass",
         "evidence": "mean max RSS sel 50258 KiB vs inc 50319 KiB (ratio 0.999); no label needed"},
        {"check": "README numbers equal the recomputed values at their printed precision", "result": "pass" if not mismatches else "fail",
         "evidence": "headline entries with claimed_source README.md" + ("" if not mismatches else f"; mismatches: {mismatches}")
                     + ("" if not side_bad else "; outside the 20-bit row, README misrounds " + ", ".join(side_bad) + " (see readme_discrepancies)")},
        {"check": "rho reference strongest available", "result": "pass",
         "evidence": f"j(E) = {cc['j_invariant']} mod p = {cc['p']}, neither 0 nor 1728, so Aut(E) = {{+-1}}; E is over the prime field (no subfield, no Frobenius fold); records: automorphisms_available 2, used 2 by rho.negation"},
    ]

    rep = {
        "schema": "validator-report/v1",
        "id": val_id,
        "status": "pass" if not mismatches else "fail",
        "verdict": "passed" if not mismatches else "failed",
        "independent": True,
        "validator": "Validator subagent (claude-opus-5-5); did not produce the sessions, code, analysis or README",
        "claim": ("20-bit holdout: prime-abscissa:size=45 + mitm:negation_folded=1 (sel) against the incumbent "
                  "ic-pair prime-abscissa:size=32 + the same oracle (inc) reduces counted cost S at every "
                  "section 5a weight; README readings: 24/28 bits relabelling, 32 bits null by construction"),
        "claim_verdict": {"20": "improvement (section 7.3 conditions 1-4 hold)", "24": "relabelling; incumbent retained",
                          "28": "relabelling; incumbent retained", "32": "null by construction; incumbent retained"},
        "snapshot": {"branch": BRANCH, "validated_content_commit": COMMIT, "parent": PARENT,
                     "readme_git_blob": blob(f"{STUDY}/README.md"),
                     "note": cert["snapshot"]["note"]},
        "method": ("records.jsonl read directly by validation/recompute.py (stdlib; pricing (G + w*U)/sqrt(r) "
                   "with C = n_C = 0, checked: no record carries a pinned calibration); ecbench compare/band "
                   "re-run with the pinned binary into validation/ and byte-compared with analysis/ and select/; "
                   "README values transcribed in validation/build_gate_files.py README_CLAIMS"),
        "raw_data": raw,
        "raw_data_note": ("Session dirs are the raw data; each records.jsonl is pinned by sha256 and git blob "
                          "inside its session.json entry, so moving it to the artifact store keeps this report valid."),
        "headline": heads,
        "non_headline_checks": side,
        "readme_discrepancies": [
            {"name": e["name"], "readme": e["claimed"], "recomputed": e["recomputed"],
             "correct_rounding": round(e["recomputed"], 3),
             "impact": "none on any reading: 32-bit stride control interval, descriptive, includes 1 either way; README line 'At 32 bits: 0.989 [0.960, 1.019] (w = 0) and 0.983 [0.939, 1.029] (w = 1)' should read [0.960, 1.018] and [0.938, 1.029]"}
            for e in side if not e["agrees"]],
        "checks": checks,
        "gae_band": {"row": ROW, "sel": band, "inc": band_inc,
                     "ratio_sel_over_inc": {w: rc["sizes"]["20"]["ratios"][f"inc->sel@{w}"]["recomputed"] for w in WK},
                     "ratio_sel_over_rho": {w: rc["sizes"]["20"]["ratios"][f"rho-neg->sel@{w}"]["recomputed"] for w in WK},
                     "ratio_inc_over_rho": {w: rc["sizes"]["20"]["ratios"][f"rho-neg->inc@{w}"]["recomputed"] for w in WK}},
        "null_controls": ("A/A arms (inc-aa, rho-neg-aa) are the null object of the same shape: identical method, "
                          "same targets and seeds; they give exactly 1.000 at every weight. The cost metric is "
                          "deterministic per (target, seed); sampling variation enters only through the targets and "
                          "is carried by the cluster bootstrap over targets. 32 bits (R* = incumbent) is a "
                          "built-in negative case and gives exactly 1."),
        "not_applicable": {"inventor_protocol_6_ladder": "the claim is measured at the size where it is made (20 bits); no extrapolation, exponent or larger-size claim is made",
                           "heuristic_validation": "no heuristic distribution (e.g. Dickman rho) is claimed"},
        "replay_certificate": {"path": "replay-certificate.json", "sha256": sha(cert_p)},
        "limitations": limitations,
        "review_attestation": {
            "joints_owned": ["whole receipt (no review_plan on the task card)"],
            "sources_read": [
                "crypto-autoresearcher: agents/validator.md, docs/agent-runtime-core.md",
                "backlog/XP-20261016-prime-base-oracle-tuning/{PROTOCOL.md, card.json, run.sh, specs/holdout-P*.template.json}",
                "docs/VALIDATION_GATE.md (git show HEAD), scripts/validation_gate.py",
                "src/cryptanalysis/ecbench/band.rs (priced_gae definition), src/cryptanalysis/ecbench/compare.rs (match_pairs)",
                f"{STUDY}/README.md, progress.txt, host/*, select/*, specs/holdout-P*.json, audits/*.json (summaries), analysis/compare-*.json, analysis/band-holdout.json, sessions/*/session.json, sessions/holdout-P*/records.jsonl, sessions/train-P*/records.jsonl, sessions/train-ext-P20-*/records.jsonl",
                "research/ic_frontier_campaign_20261015/phase1/sessions/P20 (replayed)",
                "origin/claude/ic-d4-char2-lever-20261023:benchmarks/20261028-d4-char2-frob-roots/{VALIDATION.json, replay-certificate.json, validator-report.json} (shape template only)",
            ],
            "not_read": [f"{STUDY}/verify/ (producer's run.sh verify output; not relied upon)",
                         f"{STUDY}/reports/run-20261029T0129Z/REPORT.md"],
            "read_sibling_reports": False,
            "verdict": "passed" if not mismatches else "failed",
        },
    }
    rep_p = S / "validator-report.json"
    rep_p.write_text(json.dumps(rep, indent=1) + "\n")

    man = {
        "schema": "validation-gate/v1",
        "rows": [ROW],
        "replay_certificate": {"path": "replay-certificate.json", "sha256": sha(cert_p)},
        "validator_report": {"path": "validator-report.json", "sha256": sha(rep_p)},
        "rho_reference": {
            "kind": "negation",
            "protocol": "PROTOCOL.md section 4.4 holdout arm rho-neg: rho.negation, same session ECBS1h685271327a8a, same 48 targets, rounds and algorithm seeds as sel and inc",
            "unit": "ecbench.gae / sqrt(r) (S), unpriced native operations priced at w GAE each (AGENTS.md section 5a); rho canonicalisations priced the same way",
            "binary_or_process": f"ecbench sha256 {BIN_SHA}, session ECBS1h685271327a8a (sessions/holdout-P20)",
            "matches_protocol": True,
            "strongest_available": True,
            "strongest_available_basis": (f"E: y^2 = x^3 + {cc['a']}x + {cc['b']} over F_{cc['p']} (prime field), "
                                          f"r = {cc['r']} prime, cofactor 1, j = {cc['j_invariant']} (neither 0 nor 1728), "
                                          "so Aut(E) = {+-1} and the negation map is the only automorphism fold; a "
                                          "curve over a prime field has no subfield, so no Frobenius/GLS fold exists. "
                                          "Records: automorphisms_available 2, automorphisms_used 2 for rho.negation."),
            "same_unit": True,
            "same_binary_or_process": True,
            "gae_band_row": f"{ROW}: ic = sel mean S over 96 measured runs, rho = rho-neg mean S over the same 96 (target, round) pairs",
            "gae_band": band,
            "gae_band_baseline": band_inc,
            "paired_ratio_sums": {w: {"sel_over_rho": rep["gae_band"]["ratio_sel_over_rho"][w],
                                      "inc_over_rho": rep["gae_band"]["ratio_inc_over_rho"][w],
                                      "sel_over_inc": rep["gae_band"]["ratio_sel_over_inc"][w]} for w in WK},
            "order_holds_across_band": True,
            "order": "rho-neg < sel < inc at w = 0, 0.1 and 1 (band ranks and paired ratios); rho is ahead by 7.7x to 9.4x: distance remaining, not a beat",
        },
        "verdict": "passed" if not mismatches else "failed",
        "caveats": limitations[:5],
        "promotion_blockers": [limitations[0]],
    }
    (S / "VALIDATION.json").write_text(json.dumps(man, indent=1) + "\n")
    print(json.dumps({"headlines": len(heads), "mismatches": mismatches, "side_bad": side_bad, "raw_data": len(raw),
                      "runs": len(runs), "sessions": len(sess)}, indent=1))


if __name__ == "__main__":
    main(sys.argv[1])
