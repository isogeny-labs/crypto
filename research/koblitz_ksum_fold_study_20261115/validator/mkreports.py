#!/usr/bin/env python3
"""Builds the validator's replay certificate, validator report (gate schema) and
validation_report.yaml for research/koblitz_ksum_fold_study_20261115.

Inputs: the validator's own-*.json (recompute.py / analyse.py / confirm.py /
n29char.py / scalarcheck.py), the ecbench verify receipts of the replays and
fresh sessions, and - for the headline comparison only - the producer's
results.json.  Usage: mkreports.py VALIDATOR_DIR
"""
import gzip, hashlib, json, pathlib, sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import recompute as rc

VAL_ID = "VAL-20261115-f7df34"
ROOT = pathlib.Path("/home/user/crypto")
STUDY = ROOT / "research/koblitz_ksum_fold_study_20261115"
SCR = pathlib.Path("/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/kf")
V = pathlib.Path(sys.argv[1])
W = ["0", "0.1", "1"]
WP = {"0": "0.0", "0.1": "0.1", "1": "1.0"}


def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()


def j(p):
    return json.loads(pathlib.Path(p).read_text())


own = j(V / "own-study.json")
fresh = j(V / "own-fresh.json")
conf = j(V / "own-confirm.json")
n29 = j(V / "own-n29char.json")
scal = j(V / "own-scalarcheck.json")
claimed = j(STUDY / "results.json")

# ---------------------------------------------------------------- recompute extras
recs = {}
for c in ("n23", "n29"):
    recs[c] = []
    for i in (1, 2, 3):
        recs[c] += rc.load(f"{c}-r{i}", SCR / "sessions" / f"{c}-r{i}")


def recorded_s_ratio(rs, a, b, sessions=None):
    """Ratio of summed recorded cost.s (the *_full arms' H4 reading)."""
    import json as _j
    tot = {a: 0.0, b: 0.0}
    idx = {}
    for lab in sorted({r["session"] for r in rs}):
        if sessions and lab not in sessions:
            continue
        with open(SCR / "sessions" / lab / "records.jsonl") as f:
            for line in f:
                d = _j.loads(line)
                if d["warmup"] or d["arm"] not in (a, b) or d["outcome"]["status"] != "verified":
                    continue
                idx[(d["arm"], lab, d["workload"]["workload_id"], d["round"])] = d["cost"]["s"]
    for (arm, lab, wl, rd), s in idx.items():
        if arm == a and (b, lab, wl, rd) in idx:
            tot[a] += s
            tot[b] += idx[(b, lab, wl, rd)]
    return tot[b] / tot[a]


headline = []


def add(name, cl, re):
    headline.append({"name": name, "claimed": cl, "recomputed": re})


cmap = {"n23": "23", "n29": "29"}
for c in ("n23", "n29"):
    C = claimed["curves"][cmap[c]]
    for arm, ms in C["mean_S"].items():
        mine = own[c]["band_pooled"].get(arm)
        for w in W:
            cv = ms.get(w)
            if cv is None or mine is None or mine["mean_S"][W.index(w)] is None:
                continue
            add(f"{c}/mean_S/{arm}/w{w}", cv, mine["mean_S"][W.index(w)])
    for w in W:
        cw = C["weights"][WP[w]]
        wi = W.index(w)
        strict = cw["strictest_incumbent"]
        assert strict == own[c]["rows"][w]["strictest_incumbent"], (c, w, strict)
        add(f"{c}/H1/kf3-c1g-sr_over_{strict}/pooled/w{w}", cw["h1_pooled"]["ratio"],
            own[c]["rows"][w]["kf3_over_strictest_pooled"]["ratio"])
        for i, rep in enumerate(cw["h1_per_rep"], 1):
            add(f"{c}/H1/kf3-c1g-sr_over_{strict}/r{i}/w{w}", rep["ratio"],
                own[c]["rows"][w]["kf3_over_strictest_per_replicate"][f"{c}-r{i}"]["ratio"])
        for inc, v in cw["h1_vs_each_incumbent"].items():
            add(f"{c}/H1/kf3-c1g-sr_over_{inc}/pooled/w{w}", v["ratio"],
                rc.ratio(recs[c], inc, "kf3-c1g-sr", wi=wi)["ratio"])
        add(f"{c}/H1/aa_pooled/w{w}", cw["aa_pooled"]["ratio"], own[c]["rows"][w]["aa"]["ratio"])
        k1 = [a for a in C["mean_S"] if a.startswith("k1-")][0]
        add(f"{c}/H2/kf3-c1g-sr_over_{k1}/w{w}", cw["h2_vs_k1"]["ratio"],
            rc.ratio(recs[c], k1, "kf3-c1g-sr", wi=wi)["ratio"])
        add(f"{c}/H1x/kf3-c1g-w_over_{strict}/w{w}", cw["h1_walk_candidate_vs_strictest"]["ratio"],
            rc.ratio(recs[c], strict, "kf3-c1g-w", wi=wi)["ratio"])
        for arm, v in cw["h3_gap_to_rho_strong"].items():
            add(f"{c}/H3/{arm}_over_rho-strong/w{w}", v["ratio"],
                rc.ratio(recs[c], "rho-strong", arm, wi=wi)["ratio"])
    if "full_cand_vs_lazy_full" in C["secondary_full"]:
        sf = C["secondary_full"]
        add(f"{c}/H4/kf3-c1g-sr-full_over_lazy-c1-full/recorded_S", sf["full_cand_vs_lazy_full"]["recorded_S"]["ratio"],
            recorded_s_ratio(recs[c], "lazy-c1-full", "kf3-c1g-sr-full"))
        add(f"{c}/H4/kf3-c1g-sr-full_over_rho-strong-full/recorded_S", sf["full_cand_vs_rho_strong_full"]["ratio"],
            recorded_s_ratio(recs[c], "rho-strong-full", "kf3-c1g-sr-full"))
        add(f"{c}/H4/lazy-c1-full_over_rho-strong-full/recorded_S", sf["full_lazy_vs_rho_strong_full"]["ratio"],
            recorded_s_ratio(recs[c], "rho-strong-full", "lazy-c1-full"))

mism = [h for h in headline if abs(h["claimed"] - h["recomputed"]) > 1e-9 * max(1.0, abs(h["claimed"]))]

# ---------------------------------------------------------------- replay certificate
runs = []
for lab in ("n23-r1", "n23-r2", "n23-r3", "n29-r1", "n29-r2", "n29-r3"):
    rcpt = j(SCR / "replay" / f"{lab}.verify.json")
    gz = V / "replay" / f"{lab}.verify.json.gz"
    reps = rcpt["replays"]
    sc = scal["sessions"][lab]
    runs.append({
        "run": f"research/koblitz_ksum_fold_study_20261115/sessions/{lab}",
        "session_id": rcpt["session_id"], "ok": rcpt["ok"], "exit_code": 0,
        "receipt": f"research/koblitz_ksum_fold_study_20261115/validator/replay/{lab}.verify.json.gz",
        "receipt_sha256": sha(gz),
        "records": rcpt["records"], "verified_records": rcpt["verified_records"],
        "replays": len(reps), "reproduced": sum(1 for x in reps if x.get("reproduced")),
        "replays_of_error_records_reproducing_the_same_error": sum(1 for x in reps if "same error" in (x.get("detail") or "")),
        "replay_selection": rcpt["replay_selection"], "problems": rcpt["problems"],
        "checks_all_true": all(rcpt["checks"].values()),
        "auditor_binary_sha256": rcpt["auditor_binary_sha256"],
        "auditor_hardware_class_id": rcpt.get("auditor_hardware_class_id"),
        "nonce_sampled": False,
        "scalars_independently_checked": sc["checked"], "scalars_independently_ok": sc["ok"],
        "recorded_by": "ecbench 6c60b4c93129 (producer build at fff219f63f; src identical to 71d56d2af0), this container",
    })

fresh_sessions = {}
for lab in ("F1", "F2", "F3", "F4", "F5", "F6", "G1"):
    s = j(SCR / "fresh" / lab / "session.json")
    sp = j(SCR / "fresh" / lab / "spec.json")
    rcpt = j(SCR / "fresh" / f"{lab}.verify.json")
    fresh_sessions[lab] = {
        "dir": f"research/koblitz_ksum_fold_study_20261115/validator/fresh/{lab}",
        "session_id": s["session_id"], "curve": sp["workloads"]["curves"][0],
        "target_seed": sp["workloads"]["target_seed"], "measurement_seed": sp["measurement"]["seed"],
        "targets_per_curve": sp["workloads"]["targets_per_curve"], "rounds": sp["measurement"]["rounds"],
        "arms": [a["name"] for a in sp["arms"]], "records": s["records_written"],
        "status_counts": s["status_counts"], "binary_sha256": s["binary_sha256"],
        "verify": f"ecbench verify --replay-all: ok={rcpt['ok']}, {rcpt['records']} records, "
                  f"{sum(1 for x in rcpt['replays'] if x.get('reproduced'))}/{len(rcpt['replays'])} replays reproduced, "
                  f"{len(rcpt['problems'])} problems",
        "verify_receipt": f"research/koblitz_ksum_fold_study_20261115/validator/fresh/{lab}.verify.json.gz",
        "verify_receipt_sha256": sha(V / "fresh" / f"{lab}.verify.json.gz"),
        "scalars_independently_ok": f"{scal['sessions'][lab]['ok']}/{scal['sessions'][lab]['checked']}",
    }

nonce = (V / "nonce.txt").read_text()
cert = {
    "schema": "validation-gate.replay-certificate/v1",
    "status": "verified",
    "independent": True,
    "scalars_verified": True,
    "study": "research/koblitz_ksum_fold_study_20261115",
    "validation_report_id": VAL_ID,
    "paths_relative_to": "repository root",
    "method": "ecbench verify --replay-all on gunzipped scratch copies of all six committed sessions (records.jsonl sha256 checked against sessions.sha256 first; committed files untouched): every measured deterministic run re-executed and compared with its record (1584 per session; the n23 *_full error records replay as the same error). Independently of ecbench, every recovered logarithm of every verified record (warm-ups included) was re-checked with the validator's own Python GF(2^n) Koblitz arithmetic (validator/scalarcheck.py): the reduction polynomial found independently from G on curve and [r]G = O (n23: 0x800021, n29: 0x20000005), then [recovered]G == target, target on curve, [r]target = O.",
    "build": {
        "binary": "/home/user/crypto/target/release/ecbench",
        "binary_sha256": "6c60b4c931294bf770039a5b47800e39c424eb2e7f4876b426cd084f7c364d77",
        "registered_sha256": "6c60b4c931294bf770039a5b47800e39c424eb2e7f4876b426cd084f7c364d77 (registered-at.txt, committed in 9fa38942e4 before any session)",
        "session_binary_sha256": "6c60b4c931294bf770039a5b47800e39c424eb2e7f4876b426cd084f7c364d77 in all six session.json",
        "source_identity": "git diff 71d56d2af0 HEAD -- src Cargo.toml Cargo.lock: empty (HEAD 28aae8813a == origin); binary built at fff219f63f (registered-at.txt), mtime 22:10Z, before the freeze commit 22:24Z",
        "rebuilt_by_validator": False,
        "why_not_rebuilt": "2.1-2.4 GB free on the only volume; a separate release target dir for this crate is about 3.8 GB (target/release). The replays use the producer's binary, identified by sha256 against the pre-registration record, not a validator rebuild.",
    },
    "nonce": nonce,
    "runs": runs,
    "fresh_sessions": fresh_sessions,
    "independence": {
        "independent_of_producer": [
            "validator context: recompute.py/analyse.py written before opening the producer's results.json, summary.json, analyze.py or make_report.py (analyze.py and make_report.py were not read; results.json was opened only to extract the claimed numbers for the headline comparison, after own-study.json existed)",
            "scalar certificates re-checked by the validator's own Python GF(2^n) arithmetic (polynomial derived independently)",
            "fresh-session seeds drawn from a nonce after the results commit (84f7ffea3e, 22:34:55Z; nonce 22:41:48Z)",
            "fresh sessions include arms the producer did not run (existing generator-orbit IC arms, other signed-Frobenius rho implementations)",
        ],
        "not_independent": [
            "same binary as the producer (sha-matched, not rebuilt)",
            "same host/container class as the producer (L0, --cpus none --allow-busy)",
            "the validator is a subagent launched from the producing session (same Claude-Session id); independence is of context, code and seeds, not of operator",
            "ecbench's replay shares the producer's source code (re-execution, not re-implementation)",
        ],
    },
}
(V / "replay-certificate.json").write_text(json.dumps(cert, indent=1) + "\n")

# ---------------------------------------------------------------- validator report
def r3(x):
    return None if x is None else round(x, 4)


def rr(d):
    return {"ratio": r3(d["ratio"]), "ci95_linearised_t": [r3(x) for x in d["ci95_linearised_t"]],
            "ci95_cluster_bootstrap_validator": [r3(x) for x in d["ci95_cluster_bootstrap"]], "pairs": d["pairs"]}


fresh_rows = {w: {
    "kf3_over_registered_strictest(lazy-c4)": rr(fresh["rows"][w]["kf3_over_registered_strictest"]),
    "per_session": {l: rr(x) for l, x in fresh["rows"][w]["kf3_over_registered_per_session"].items()},
    "kf3_over_gen-layout-c4": rr(fresh["rows"][w]["kf3_over_gen-layout-c4"]),
    "kf3_over_gen-lazy-c4-sr": rr(fresh["rows"][w]["kf3_over_gen-lazy-c4-sr"]),
    "kf3_over_rho-strong": rr(fresh["rows"][w]["kf3_over_rho-strong"]),
    "kf3_over_rho-frob-v5": rr(fresh["rows"][w]["kf3_over_rho-frob-v5"]),
    "cheapest_rho": fresh["rows"][w]["cheapest_rho"],
    "best_existing_ic_arm": fresh["rows"][w]["best_existing_ic_arm"],
} for w in W}
conf_rows = {w: {k: rr(v) for k, v in conf["rows"][w].items() if not k.endswith("_per_session")} for w in W}
conf_ps = {w: {k: {l: rr(x) for l, x in v.items()} for k, v in conf["rows"][w].items() if k.endswith("_per_session")} for w in W}
band_fresh = {a: [r3(x) for x in v["mean_S"]] for a, v in fresh["band_pooled"].items()}
band_conf = {a: [r3(x) for x in v["mean_S"]] for a, v in conf["band_pooled"].items()}
band_n29 = {a: [r3(x) for x in v["mean_S"]] for a, v in n29["band"].items()}

raw = []
for lab in ("n23-r1", "n23-r2", "n23-r3", "n29-r1", "n29-r2", "n29-r3"):
    p = STUDY / "sessions" / lab / "records.jsonl.gz"
    raw.append({"path": f"../sessions/{lab}/records.jsonl.gz", "sha256": sha(p)})
for lab in ("F1", "F2", "F3", "F4", "F5", "F6", "G1"):
    p = V / "fresh" / lab / "records.jsonl.gz"
    raw.append({"path": f"fresh/{lab}/records.jsonl.gz", "sha256": sha(p)})

checks = j(V / "checks.json")
report = {
    "schema": "validation-gate.validator-report/v1",
    "status": "fail",
    "independent": True,
    "study": "research/koblitz_ksum_fold_study_20261115",
    "validated_commit": "28aae8813acbc95914a226db582d6187995e90ad",
    "verdict_summary": checks["verdict_summary"],
    "method": "Independent recomputation in Python (stdlib) from the six committed records.jsonl.gz (gunzipped to scratch, sha256 against sessions.sha256), by validator/recompute.py + analyse.py, written from the AGENTS.md 5a rule and the band.rs / compare.rs module comments without reading the producer's analyze.py or make_report.py. priced_gae(w) = G - C + w (U + n_C): G cost.total_gae, C the calibration-priced native work (PRICED_NATIVE counters x the K1 rows of docs/ic/calibration.json, only for units the record pins), n_C its count, U the cost.unpriced counts (NON_OPERATION byte counters excluded) plus the counted table_insert_attempts behind their marker. Ratios are totals over (session, workload, round) pairs; interval: linearised t with (session, target) clusters, and a one-stage cluster bootstrap. The producer's intervals are ecbench compare's two-stage cluster bootstrap (seed 20261002), reproduced by running ecbench compare; the validator's linearised interval equals ecbench compare's primary ci95 to printed precision. Point estimates compared at 1e-9. Plus: replay of all six sessions, independent scalar checks, a code audit of 71d56d2af0, and 7 fresh sessions on nonce-derived seeds (F1-F3 K1 n23 21 arms; F4-F6 pre-declared confirmation, 8 arms; G1 K1 n29 10 arms).",
    "validator_code_sha256": {p.name: sha(p) for p in sorted(V.glob("*.py"))},
    "raw_data": raw,
    "headline": headline,
    "headline_mismatches": mism,
    "checks": checks["checks"],
    "fresh": {"band_mean_S_F1-F3": band_fresh, "rows_F1-F3": fresh_rows,
              "band_mean_S_F4-F6": band_conf, "rows_F4-F6_predeclared": conf_rows, "rows_F4-F6_per_session": conf_ps,
              "band_mean_S_G1_n29": band_n29,
              "rows_G1_n29": {w: {k: rr(v) for k, v in n29["rows"][w].items()} for w in W}},
    "defects": checks["defects"],
    "limitations": checks["limitations"],
    "validation_report": {"id": VAL_ID, "path": "validation_report.yaml"},
}
(V / "validator-report.json").write_text(json.dumps(report, indent=1) + "\n")
print("headline", len(headline), "mismatches", len(mism))
for h in mism[:10]:
    print(" MISMATCH", h)
