#!/usr/bin/env python3
"""Validator: bindings, freeze timing, the section-4 amendment, analyze.py byte
reproduction and an accounting stress test for research/quartic_diem_la_20261123.

Usage:
  python3 -I checks_extra.py REPO SHA SNAP BIN OUT.json

  REPO  the crypto checkout (Git history and commit times are read from it)
  SHA   the validated commit
  SNAP  `git archive SHA research/quartic_diem_la_20261123 research/quartic_diem_ladder_20261121/raw`
        extracted (the study is SNAP/research/quartic_diem_la_20261123)
  BIN   the hash-checked copy of genus_ladder.qla taken at dispatch

Record keys are indexed with [] (lesson L-99ee952350b2).  analyze.py is executed in
a subprocess inside SNAP (never imported) only to test that analysis.json is its
byte-exact output; every number the validator reports comes from recompute.py.
"""
import glob
import hashlib
import json
import math
import os
import re
import subprocess
import sys
from datetime import datetime, timezone

STUDY = "research/quartic_diem_la_20261123"
SFB, SGE = "Q_diem_2lp_sfb", "Q_diem_2lp_sfb_sge"
NATIVE = ("rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts")
CURVES = {4099: [35, 205, 220, 257, 295], 16411: [93, 158, 252, 366, 382],
          65537: [13, 57, 156, 166, 369], 131101: [53, 177, 193, 228, 288]}


def sh(*a, cwd=None):
    return subprocess.run(list(a), check=True, capture_output=True, cwd=cwd).stdout


def sha(b):
    return hashlib.sha256(b).hexdigest()


def jl(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def ctime(repo, rev):
    return sh("git", "-C", repo, "show", "-s", "--format=%cI", rev).decode().strip()


def main():
    repo, rev, snap, binp, outp = sys.argv[1:6]
    st = os.path.join(snap, STUDY)
    full = sh("git", "-C", repo, "rev-parse", rev).decode().strip()
    out = {"schema": "validator-checks-extra/v1", "validated_commit": full}

    # ── manifests ──
    man = {}
    for mf in ("frozen.sha256", "data.sha256"):
        res = {}
        for line in open(os.path.join(st, mf)):
            h, f = line.split()
            res[f] = sha(open(os.path.join(st, f), "rb").read()) == h
        man[mf] = res
    src = {}
    for line in open(os.path.join(st, "source.sha256")):
        h, f = line.split()
        src[f] = {c: sha(sh("git", "-C", repo, "show", f"{c}:{f}")) == h
                  for c in ("4faca82a6f0", "612f8dee20e", full)}
    man["source.sha256 (vs Git blobs at 4faca82a6f0, 612f8dee20e, validated)"] = src
    bh = open(os.path.join(st, "binary.sha256")).read().split()[0]
    dbh = open(os.path.join(st, "dev/binary.sha256")).read().split()[0]
    got = sha(open(binp, "rb").read())
    man["binary"] = {"recorded": bh, "dev_recorded": dbh, "dispatch_copy": got,
                     "matches": got == bh == dbh}
    out["manifests"] = man

    # ── freeze timing ──
    freeze = "612f8dee20e"
    tz = lambda s: datetime.fromisoformat(s.replace("Z", "+00:00"))
    t_freeze = tz(ctime(repo, freeze))
    stamps = {f: open(os.path.join(st, "raw", f)).read().strip()
              for f in ("main_start.txt", "main_end.txt", "r131101_start.txt", "r131101_end.txt")}
    dev_start = open(os.path.join(st, "dev/start.txt")).read().strip()
    done = {f: [r for r in jl(os.path.join(st, "raw", f)) if r["kind"] == "done"][0]["wall_s"]
            for f in ("run.jsonl", "run131101.jsonl")}
    raw_commits = sh("git", "-C", repo, "log", "--all", "--format=%H %cI", "--",
                     f"{STUDY}/raw/").decode().split("\n")
    raw_commits = [x for x in raw_commits if x.strip()]
    first_raw = min(raw_commits, key=lambda x: x.split()[1])
    ana_hist = {c: sha(sh("git", "-C", repo, "show", f"{c}:{STUDY}/analyze.py"))
                for c in ("5299e594a29", "9781704897b", freeze)}
    pre = sh("git", "-C", repo, "show", f"{freeze}~1:{STUDY}/PROTOCOL.md").decode()
    post = sh("git", "-C", repo, "show", f"{freeze}:{STUDY}/PROTOCOL.md").decode()
    sec6 = lambda t: t[t.index("## 6."):t.index("## 7.")]
    out["timing"] = {
        "freeze_commit": freeze, "freeze_commit_time": t_freeze.isoformat(),
        "raw_start_stamps": stamps, "dev_start": dev_start,
        "freeze_before_main_start": t_freeze < tz(stamps["main_start.txt"]),
        "freeze_before_131101_start": t_freeze < tz(stamps["r131101_start.txt"]),
        "seconds_freeze_to_first_grid_start": (tz(stamps["main_start.txt"]) - t_freeze).total_seconds(),
        "done_wall_s": done,
        "main_end_predicted_minus_stamp_s": (tz(stamps["main_start.txt"]).timestamp() + done["run.jsonl"]
                                             - tz(stamps["main_end.txt"]).timestamp()),
        "r131101_end_predicted_minus_stamp_s": (tz(stamps["r131101_start.txt"]).timestamp()
                                                + done["run131101.jsonl"] - tz(stamps["r131101_end.txt"]).timestamp()),
        "commits_touching_raw": raw_commits,
        "first_commit_with_raw_after_freeze": tz(first_raw.split()[1]) > t_freeze,
        "analyze_py_sha_by_commit": ana_hist,
        "analyze_py_unchanged_since_draft": len(set(ana_hist.values())) == 1,
        "protocol_section6_unchanged_by_freeze": sec6(pre) == sec6(post),
        "note": "times are self-reported by the producer's host (commit dates, stamp files); "
                "they are mutually consistent but not tamper-proof",
    }

    # ── section-4 amendment against dev/ ──
    dev = {}
    for f in sorted(glob.glob(os.path.join(st, "dev", "dev_W*_F*_E*.jsonl"))):
        W, F, E = re.search(r"dev_W(\d+)_F([\d.]+)_E(\d+)\.jsonl$", f).groups()
        runs = [r for r in jl(f) if r["kind"] == "run"]
        a = {(r["p"], r["curve_seed"], r["seed"]): r for r in runs if r["arm"] == SFB}
        b = {(r["p"], r["curve_seed"], r["seed"]): r for r in runs if r["arm"] == SGE}
        assert set(a) == set(b) and len(a) == 18
        fbk = sorted(k for k, r in b.items() if r["la_fallbacks"] > 0)
        multi = sorted(k for k, r in a.items() if r["attempts"] > 1)
        big = [r for k, r in b.items() if k[0] == 32771]
        dev[f"{W},{F},{E}"] = {
            "W": int(W), "FILL": float(F), "EXCESS": int(E),
            "pairs": len(a), "n_32771": len(big),
            "la_fallbacks": sum(r["la_fallbacks"] for r in b.values()),
            "fallback_pairs": [list(k) for k in fbk],
            "fallbacks_equal_sfb_multi_attempt_pairs": fbk == multi,
            "all_certified_correct": all(r["certified"] is True and r["ok"] is True
                                         and r["recovered"] == r["t_secret"] for r in runs),
            "same_log": all(a[k]["recovered"] == b[k]["recovered"] for k in a),
            "gm_la_32771": math.exp(sum(math.log(r["la_add"] + r["la_mul"]) for r in big) / len(big)),
        }
    # FILL invariance: every run row (both arms), every field except wall_ns
    fill_arm = {}
    for W in (4, 8):
        for E in (8, 16, 64):
            r1 = [r for r in jl(os.path.join(st, "dev", f"dev_W{W}_F1.5_E{E}.jsonl")) if r["kind"] == "run"]
            r2 = [r for r in jl(os.path.join(st, "dev", f"dev_W{W}_F3_E{E}.jsonl")) if r["kind"] == "run"]
            k = lambda r: (r["arm"], r["p"], r["curve_seed"], r["seed"])
            m1 = {k(r): {f2: v for f2, v in r.items() if f2 != "wall_ns"} for r in r1}
            m2 = {k(r): {f2: v for f2, v in r.items() if f2 != "wall_ns"} for r in r2}
            fill_arm[f"W{W}_E{E}"] = len(m1) == len(r1) == len(r2) and m1 == m2
    elig = [k for k, v in dev.items() if v["fallbacks_equal_sfb_multi_attempt_pairs"]
            and v["all_certified_correct"] and v["n_32771"] == 10]
    pick = min(elig, key=lambda k: (dev[k]["gm_la_32771"], dev[k]["EXCESS"], dev[k]["FILL"]))
    by_gm = sorted(elig, key=lambda k: dev[k]["gm_la_32771"])
    choice = json.load(open(os.path.join(st, "dev/choice.json")))
    out["dev_amendment"] = {
        "settings": dev,
        "every_setting_has_4_fallbacks": all(v["la_fallbacks"] == 4 for v in dev.values()),
        "every_fallback_on_sfb_multi_attempt_pair": all(v["fallbacks_equal_sfb_multi_attempt_pairs"] for v in dev.values()),
        "fill_has_no_effect_on_any_field_except_wall_ns": all(fill_arm.values()),
        "fill_invariance_by_W_E": fill_arm,
        "amended_rule_eligible": len(elig),
        "pick_by_amended_rule": pick,
        "pick_matches_protocol": pick == "8,1.5,64",
        "ranking_by_gm_la_32771": [[k, dev[k]["gm_la_32771"]] for k in by_gm],
        "choice_json_pick_under_original_rule": choice["pick"],
        "measured_grid_disjoint_from_dev_primes": not ({4093, 8191, 32771} & set(CURVES)),
    }

    # ── analyze.py byte reproduction (subprocess, inside SNAP) ──
    ana = sh(sys.executable, "-I", "analyze.py", "raw/run.jsonl", "raw/run131101.jsonl", cwd=st)
    out["analyze_reproduction"] = {
        "rerun_sha256": sha(ana),
        "committed_sha256": sha(open(os.path.join(st, "analysis.json"), "rb").read()),
        "byte_identical": ana == open(os.path.join(st, "analysis.json"), "rb").read(),
        "python": sys.version.split()[0],
    }

    # ── accounting stress test: trim + SGE charges multiplied by k in the sge arm ──
    rows = [r for f in ("raw/run.jsonl", "raw/run131101.jsonl") for r in jl(os.path.join(st, f))
            if r["kind"] == "run"]
    by = {}
    for r in rows:
        by.setdefault((r["p"], r["curve_seed"], r["seed"]), {})[r["arm"]] = r

    def g(r, w, k=1):
        extra = (k - 1) * (r["la_sge_add"] + r["la_sge_mul"] + r["la_trim_add"] + r["la_trim_mul"])
        return r["group_ops"] + w * (sum(r[x] for x in NATIVE) + extra)

    stress = {}
    for k in (1, 10, 100):
        for w in (0.1, 1.0):
            lp = []
            for p in CURVES:
                cv = [math.exp(sum(math.log(g(by[(p, c, s)][SGE], w, k) / g(by[(p, c, s)][SFB], w))
                                   for s in (1, 2)) / 2) for c in CURVES[p]]
                lp.append(sum(math.log(x) for x in cv) / len(cv))
            stress[f"x{k}_w{w}"] = math.exp(sum(lp) / len(lp))
    out["stress_trim_sge_charge"] = stress
    json.dump(out, open(outp, "w"), indent=1)
    print(json.dumps({k: (v if not isinstance(v, dict) else {kk: vv for kk, vv in v.items()
                                                             if not isinstance(vv, (dict, list))})
                      for k, v in out.items()}, indent=1)[:4000])


if __name__ == "__main__":
    main()
