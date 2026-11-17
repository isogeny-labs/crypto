#!/usr/bin/env python3
"""Independent validator for the wallfix RunPod session-level ABAB benchmark.

Written from the raw ecbench session records only. It does not import, read or
reuse the producer's analysis code (analysis/analyze.py, analysis/plot.py).
Standard library only.

Inputs (relative to research/wallfix_runpod_20261114/):
  sessions/run2/pr<PR>-<spec>-r<round>-<arm>/{records.jsonl,session.json,spec.json}
  sessions/run1-aa/...                      (A/A control: byte-identical binaries)
  reports/run-20261114T2240Z/data/runlog-run2.tsv, binaries-run2.sha256,
      runlog-run1-aa.tsv, results-ab.json (producer claims), results-aa.json
  specs/<spec>.json, tarballs.txt

Metric (as dispatched): per session, sum over records with warmup == false of
time.schedstat_solve.cpu_ns (solve_cpu) and time.solve_wall_ns (solve_wall),
all arms. Per (PR, spec, round): B/A of the sums. Per (PR, spec): geometric
mean, min, max of the three per-round ratios; aa_spread = max/min of A sums.

Output: validator/validator-report.json (validation-gate validator report).
Usage: python3 -I validator.py [--base DIR] [--out FILE]
"""

import argparse
import collections
import hashlib
import json
import math
import os
import re
import subprocess
import sys

REL_TOL = 1e-9
DESIGN = [("3850", "koblitz"), ("3850", "prime"), ("3850", "tensor"),
          ("3991", "prime"), ("4009", "koblitz"), ("4009", "tensor")]
ROUNDS = (1, 2, 3)
ARMS = ("A", "B")
SESSION_RE = re.compile(r"^pr(\d+)-(prime|koblitz|tensor)-r(\d+)-([AB])$")
DATA_REL = "reports/run-20261114T2240Z/data"
EVIDENCE_PREFIX = "../../research/wallfix_runpod_20261114/"  # from benchmarks/20261114-wallfix-runpod-abab/


# ---------------------------------------------------------------- helpers

def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def close(a, b, tol=REL_TOL):
    if a is None or b is None:
        return False
    if a == b:
        return True
    return abs(a - b) <= tol * max(abs(a), abs(b))


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def read_jsonl(path):
    out = []
    with open(path, "r", encoding="utf-8") as f:
        for i, line in enumerate(f):
            line = line.strip()
            if not line:
                continue
            out.append(json.loads(line))
    return out


def dig(obj, *keys):
    for k in keys:
        if not isinstance(obj, dict) or k not in obj:
            return None
        obj = obj[k]
    return obj


# ---------------------------------------------------------------- identity

DROP_TOP = {"time", "isolation", "binary_sha256", "record_id", "session_id"}


def is_timing_key(k):
    return (k.endswith("_ns") or k.endswith("_ms") or k.startswith("ns_per_")
            or k in ("started_at", "finished_at"))


def strip_timing(obj, path=()):
    if isinstance(obj, dict):
        out = {}
        for k, v in obj.items():
            if not path and k in DROP_TOP:
                continue
            if path == ("online",) and k == "phases_ns":
                continue
            if is_timing_key(k):
                continue
            out[k] = strip_timing(v, path + (k,))
        return out
    if isinstance(obj, list):
        return [strip_timing(x, path + ("[]",)) for x in obj]
    return obj


def canon(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def leaf_diffs(a, b, path=""):
    """Yield paths where two stripped records differ."""
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                yield path + "." + k + " (missing on one side)"
            else:
                yield from leaf_diffs(a[k], b[k], path + "." + k)
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            yield path + " (list length %d vs %d)" % (len(a), len(b))
        else:
            for i, (x, y) in enumerate(zip(a, b)):
                yield from leaf_diffs(x, y, path + "[%d]" % i)
    elif a != b or type(a) is not type(b):
        yield path


def compare_multisets(recs_a, recs_b):
    ca = collections.Counter(canon(strip_timing(r)) for r in recs_a)
    cb = collections.Counter(canon(strip_timing(r)) for r in recs_b)
    res = {"n_a": len(recs_a), "n_b": len(recs_b), "identical": ca == cb}
    if ca != cb:
        only_a = sum((ca - cb).values())
        only_b = sum((cb - ca).values())
        res["records_only_in_a"] = only_a
        res["records_only_in_b"] = only_b
        # locate differing fields by pairing on seq (same plan => same seq order)
        by_seq_b = {r.get("seq"): r for r in recs_b}
        paths = collections.Counter()
        examples = {}
        for r in recs_a:
            rb = by_seq_b.get(r.get("seq"))
            if rb is None:
                paths["<seq missing in b>"] += 1
                continue
            sa, sb = strip_timing(r), strip_timing(rb)
            for p in leaf_diffs(sa, sb):
                paths[p] += 1
                examples.setdefault(p, {"seq": r.get("seq"), "arm": r.get("arm")})
        res["differing_paths"] = dict(paths.most_common())
        res["examples"] = examples
    return res


# ---------------------------------------------------------------- loading

def load_runlog(path):
    rows, order = {}, []
    with open(path) as f:
        for line in f:
            parts = line.rstrip("\n").split("\t")
            if len(parts) != 8:
                continue
            pr, spec, rnd, arm, sha, rc, ms, load = parts
            key = (pr, spec, int(rnd), arm)
            rows[key] = {"sha": sha, "rc": int(rc), "wall_ms": int(ms), "load1": float(load)}
            order.append(key)
    return rows, order


def load_binaries(path):
    out = {}
    with open(path) as f:
        for line in f:
            parts = line.split()
            if len(parts) == 2 and parts[1].startswith("bin/ecbench-"):
                out[parts[1][len("bin/ecbench-"):]] = parts[0]
    return out


def load_sessions(run_dir):
    sessions = {}
    for name in sorted(os.listdir(run_dir)):
        m = SESSION_RE.match(name)
        full = os.path.join(run_dir, name)
        if not m or not os.path.isdir(full):
            continue
        pr, spec, rnd, arm = m.group(1), m.group(2), int(m.group(3)), m.group(4)
        rec_path = os.path.join(full, "records.jsonl")
        sess = {
            "name": name, "dir": full, "records_path": rec_path,
            "records_sha256": sha256_file(rec_path),
            "records": read_jsonl(rec_path),
            "session": json.load(open(os.path.join(full, "session.json"))),
            "spec_bytes": open(os.path.join(full, "spec.json"), "rb").read(),
        }
        sessions[(pr, spec, rnd, arm)] = sess
    return sessions


def session_sums(records):
    s = {"cpu": 0, "wall": 0, "cpu_all": 0, "wall_all": 0, "n": 0, "n_warmup": 0,
         "cpu_missing": 0, "wall_missing": 0, "cpu_zero": 0, "wall_in_cpu_zero": 0,
         "warmup_round_mismatch": 0, "warmup_not_bool": 0,
         "per_arm_cpu": collections.Counter(), "per_arm_wall": collections.Counter()}
    for r in records:
        w = r.get("warmup")
        if not isinstance(w, bool):
            s["warmup_not_bool"] += 1
        if (w is True) != (r.get("round") == 0):
            s["warmup_round_mismatch"] += 1
        cpu = dig(r, "time", "schedstat_solve", "cpu_ns")
        wall = dig(r, "time", "solve_wall_ns")
        if not isinstance(cpu, int) or isinstance(cpu, bool):
            s["cpu_missing"] += 1
            cpu = None
        if not isinstance(wall, int) or isinstance(wall, bool):
            s["wall_missing"] += 1
            wall = None
        s["cpu_all"] += cpu or 0
        s["wall_all"] += wall or 0
        if w is True:
            s["n_warmup"] += 1
            continue
        s["n"] += 1
        if cpu is not None:
            s["cpu"] += cpu
            s["per_arm_cpu"][r["arm"]] += cpu
            if cpu == 0:
                s["cpu_zero"] += 1
                s["wall_in_cpu_zero"] += wall or 0
        if wall is not None:
            s["wall"] += wall
            s["per_arm_wall"][r["arm"]] += wall
    return s


def row_stats(sums, metric):
    a = [sums[(r, "A")][metric] for r in ROUNDS]
    b = [sums[(r, "B")][metric] for r in ROUNDS]
    ratios = [bb / aa for aa, bb in zip(a, b)]
    return {"A_ns": a, "B_ns": b, "ba_per_round": ratios,
            "ba_geomean": geomean(ratios), "ba_min": min(ratios), "ba_max": max(ratios),
            "aa_spread": max(a) / min(a), "bb_spread": max(b) / min(b)}


# ---------------------------------------------------------------- main

def main():
    here = os.path.dirname(os.path.abspath(__file__))
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", default=os.path.dirname(here))
    ap.add_argument("--out", default=os.path.join(here, "validator-report.json"))
    args = ap.parse_args()
    base = os.path.abspath(args.base)
    repo = os.path.abspath(os.path.join(base, "..", ".."))
    data = os.path.join(base, DATA_REL)

    findings = []  # human-readable problems, any severity

    # ---- inputs
    runlog, runlog_order = load_runlog(os.path.join(data, "runlog-run2.tsv"))
    runlog_aa, runlog_aa_order = load_runlog(os.path.join(data, "runlog-run1-aa.tsv"))
    binaries = load_binaries(os.path.join(data, "binaries-run2.sha256"))
    claims = json.load(open(os.path.join(data, "results-ab.json")))
    claims_aa = json.load(open(os.path.join(data, "results-aa.json")))
    tarballs = {}
    for line in open(os.path.join(base, "tarballs.txt")):
        p = line.split()
        if len(p) == 2:
            tarballs[p[0][:10]] = {"commit": p[0], "tarball_sha256": p[1]}

    run2 = load_sessions(os.path.join(base, "sessions", "run2"))
    run1 = load_sessions(os.path.join(base, "sessions", "run1-aa"))

    expected_keys = {(pr, spec, r, a) for pr, spec in DESIGN for r in ROUNDS for a in ARMS}
    design_check = {
        "expected_sessions": len(expected_keys),
        "run2_sessions_found": len(run2),
        "run2_missing": sorted("-".join(map(str, k)) for k in expected_keys - set(run2)),
        "run2_unexpected": sorted("-".join(map(str, k)) for k in set(run2) - expected_keys),
        "run1_aa_sessions_found": len(run1),
        "run1_aa_missing": sorted("-".join(map(str, k)) for k in expected_keys - set(run1)),
    }
    design_ok = not design_check["run2_missing"] and not design_check["run2_unexpected"]
    if not design_ok:
        findings.append("run2 session set does not match the 6x3x2 design")

    # ---- session integrity (records hash bound in session.json, spec identity)
    integrity = {"ok": True, "problems": []}
    spec_sha_by_spec = collections.defaultdict(set)
    for key, s in sorted(run2.items()):
        ss = s["session"]
        if ss.get("records_sha256") != s["records_sha256"]:
            integrity["problems"].append("%s: records.jsonl sha256 != session.json records_sha256" % s["name"])
        if ss.get("records_written") != len(s["records"]):
            integrity["problems"].append("%s: records_written %s != %d lines" % (s["name"], ss.get("records_written"), len(s["records"])))
        if ss.get("status") != "complete":
            integrity["problems"].append("%s: session status %s" % (s["name"], ss.get("status")))
        ref_spec = open(os.path.join(base, "specs", key[1] + ".json"), "rb").read()
        if s["spec_bytes"] != ref_spec:
            integrity["problems"].append("%s: spec.json differs from specs/%s.json" % (s["name"], key[1]))
        spec_sha_by_spec[key[1]].add(ss.get("spec_sha256"))
        sids = {r.get("session_id") for r in s["records"]}
        if sids != {ss.get("session_id")}:
            integrity["problems"].append("%s: record session_id mismatch %s" % (s["name"], sids))
        man_path = os.path.join(s["dir"], ".artifacts.json")
        if os.path.exists(man_path):
            ent = dig(json.load(open(man_path)), "files", "records.jsonl") or {}
            integrity.setdefault("artifact_manifests_checked", 0)
            integrity["artifact_manifests_checked"] += 1
            if ent.get("sha256") != s["records_sha256"] or ent.get("bytes") != os.path.getsize(s["records_path"]):
                integrity["problems"].append("%s: records.jsonl does not match committed .artifacts.json" % s["name"])
        else:
            integrity.setdefault("artifact_manifests_missing", []).append(s["name"])
    for spec, shas in spec_sha_by_spec.items():
        if len(shas) != 1:
            integrity["problems"].append("spec %s: %d distinct spec_sha256 across sessions" % (spec, len(shas)))
    integrity["git_commit_in_sessions"] = sorted({str(s["session"].get("git_commit")) for s in run2.values()})
    integrity["git_dirty_in_sessions"] = sorted({str(s["session"].get("git_dirty")) for s in run2.values()})
    integrity["ok"] = not integrity["problems"]

    # ---- metric sums
    sums2 = {k: session_sums(s["records"]) for k, s in run2.items()}
    sums1 = {k: session_sums(s["records"]) for k, s in run1.items()}

    recomputed = {}
    for pr, spec in DESIGN:
        per = {(r, a): sums2[(pr, spec, r, a)] for r in ROUNDS for a in ARMS}
        recomputed[(pr, spec)] = {m: row_stats(per, key) for m, key in (("solve_cpu", "cpu"), ("solve_wall", "wall"))}

    # ---- compare with producer
    claim_rows = {(str(r["pr"]), r["spec"]): r for r in claims["rows"]}
    headline = []
    table_cmp = []
    all_table_ok = True
    for pr, spec in DESIGN:
        c = claim_rows.get((pr, spec))
        for metric in ("solve_cpu", "solve_wall"):
            mine = recomputed[(pr, spec)][metric]
            claimed = dig(c, metric, "ba_geomean") if c else None
            headline.append({"name": "pr%s/%s/%s_ba_geomean" % (pr, spec, metric),
                             "claimed": claimed, "recomputed": mine["ba_geomean"], "rel_tol": REL_TOL,
                             "agrees": close(claimed, mine["ba_geomean"])})
            for field in ("ba_min", "ba_max", "aa_spread", "bb_spread"):
                cv = dig(c, metric, field) if c else None
                ok = close(cv, mine[field])
                all_table_ok &= ok
                table_cmp.append({"name": "pr%s/%s/%s_%s" % (pr, spec, metric, field),
                                  "claimed": cv, "recomputed": mine[field], "agrees": ok})
            for field in ("A_ns", "B_ns"):
                cv = dig(c, metric, field) if c else None
                ok = cv == mine[field]
                all_table_ok &= ok
                table_cmp.append({"name": "pr%s/%s/%s_%s" % (pr, spec, metric, field),
                                  "claimed": cv, "recomputed": mine[field], "agrees": ok, "exact_integer": True})
            cv = dig(c, metric, "ba_per_round") if c else None
            ok = cv is not None and len(cv) == 3 and all(close(x, y) for x, y in zip(cv, mine["ba_per_round"]))
            all_table_ok &= ok
            table_cmp.append({"name": "pr%s/%s/%s_ba_per_round" % (pr, spec, metric),
                              "claimed": cv, "recomputed": mine["ba_per_round"], "agrees": ok})
    headline_ok = all(h["agrees"] for h in headline)
    if not headline_ok:
        findings.append("headline disagreement: " + ", ".join(h["name"] for h in headline if not h["agrees"]))
    if not all_table_ok:
        findings.append("secondary table disagreement: " + ", ".join(t["name"] for t in table_cmp if not t["agrees"]))
    dispatch_rounded = {("3850", "koblitz"): 0.977, ("3850", "prime"): 0.905, ("3850", "tensor"): 0.964,
                        ("3991", "prime"): 1.013, ("4009", "koblitz"): 0.926, ("4009", "tensor"): 0.654}
    dispatch_round_ok = all(round(recomputed[k]["solve_cpu"]["ba_geomean"], 3) == v for k, v in dispatch_rounded.items())

    # ---- check 1: output identity A vs B per (PR, spec, round)
    identity = {"pass": True, "per_pair": [], "cross_round": [], "run1_aa": []}
    for pr, spec in DESIGN:
        for r in ROUNDS:
            res = compare_multisets(run2[(pr, spec, r, "A")]["records"], run2[(pr, spec, r, "B")]["records"])
            res["pair"] = "pr%s-%s-r%d" % (pr, spec, r)
            identity["per_pair"].append(res)
            if not res["identical"]:
                identity["pass"] = False
                findings.append("output identity failed for %s: %s" % (res["pair"], list(res.get("differing_paths", {}))[:5]))
        # determinism across rounds (informational): every session of the row vs A r1
        ref = run2[(pr, spec, 1, "A")]["records"]
        for r in ROUNDS:
            for a in ARMS:
                if (r, a) == (1, "A"):
                    continue
                res = compare_multisets(ref, run2[(pr, spec, r, a)]["records"])
                identity["cross_round"].append({"pair": "pr%s-%s r1A vs r%d%s" % (pr, spec, r, a), "identical": res["identical"],
                                                "differing_paths": res.get("differing_paths")})
        # run1-aa outputs vs run2 arm A (informational: same source commits for A)
        if (pr, spec, 1, "A") in run1:
            res = compare_multisets(run1[(pr, spec, 1, "A")]["records"], ref)
            identity["run1_aa"].append({"pair": "pr%s-%s run1-aa r1A vs run2 r1A" % (pr, spec), "identical": res["identical"],
                                        "differing_paths": res.get("differing_paths")})
    identity["pairs_identical"] = sum(1 for p in identity["per_pair"] if p["identical"])
    identity["pairs_total"] = len(identity["per_pair"])
    identity["cross_round_identical"] = sum(1 for p in identity["cross_round"] if p["identical"])
    identity["cross_round_total"] = len(identity["cross_round"])
    identity["dropped_fields"] = ("keys ending _ns or _ms at any depth; keys starting ns_per_; started_at; finished_at; "
                                  "top-level time, isolation, binary_sha256, record_id, session_id; online.phases_ns")

    # ---- check 2: outcome status
    outcome = {"pass": True, "run2": {}, "run1_aa": {}}
    for label, sess in (("run2", run2), ("run1_aa", run1)):
        n = bad_status = bad_target = bad_planted = 0
        examples = []
        for key, s in sess.items():
            for rr in s["records"]:
                n += 1
                st = dig(rr, "outcome", "status")
                mt = dig(rr, "outcome", "matches_target")
                mp = dig(rr, "outcome", "matches_planted")
                if st != "verified":
                    bad_status += 1
                if mt is not True:
                    bad_target += 1
                if mp is not True:
                    bad_planted += 1
                if (st != "verified" or mt is not True) and len(examples) < 5:
                    examples.append({"session": s["name"], "seq": rr.get("seq"), "status": st, "matches_target": mt})
        outcome[label] = {"records": n, "status_not_verified": bad_status, "matches_target_not_true": bad_target,
                          "matches_planted_not_true": bad_planted, "examples": examples}
    outcome["pass"] = (outcome["run2"]["records"] == 5184 and outcome["run2"]["status_not_verified"] == 0
                       and outcome["run2"]["matches_target_not_true"] == 0)
    if not outcome["pass"]:
        findings.append("check 2 failed: %s" % outcome["run2"])

    # ---- check 3: binary identity
    def first_parent(commit):
        try:
            out = subprocess.run(["git", "-C", repo, "rev-parse", commit + "^1"], capture_output=True, text=True, timeout=30)
            return out.stdout.strip() if out.returncode == 0 else None
        except Exception:
            return None

    def full_sha(commit):
        try:
            out = subprocess.run(["git", "-C", repo, "rev-parse", commit + "^{commit}"], capture_output=True, text=True, timeout=30)
            return out.stdout.strip() if out.returncode == 0 else None
        except Exception:
            return None

    binary = {"pass": True, "per_session": [], "per_pr": [], "problems": []}
    pr_commits = collections.defaultdict(lambda: {"A": set(), "B": set(), "binA": set(), "binB": set()})
    for key in sorted(expected_keys):
        if key not in run2:
            continue
        s = run2[key]
        pr, spec, r, a = key
        rl = runlog.get(key)
        rec_bins = sorted({rr.get("binary_sha256") for rr in s["records"]})
        listed = binaries.get(rl["sha"]) if rl else None
        ok = (rl is not None and rl["rc"] == 0 and len(rec_bins) == 1 and rec_bins[0] == s["session"].get("binary_sha256")
              and listed is not None and rec_bins[0] == listed)
        binary["per_session"].append({"session": s["name"], "runlog_commit": rl and rl["sha"], "runlog_rc": rl and rl["rc"],
                                      "record_binary_sha256": rec_bins, "session_binary_sha256": s["session"].get("binary_sha256"),
                                      "listed_binary_sha256": listed, "ok": ok})
        if not ok:
            binary["problems"].append("%s: binary mismatch or missing runlog row" % s["name"])
        if rl:
            pr_commits[pr][a].add(rl["sha"])
        pr_commits[pr]["bin" + a].update(rec_bins)
    for pr in sorted(pr_commits):
        pc = pr_commits[pr]
        ca, cb = sorted(pc["A"]), sorted(pc["B"])
        entry = {"pr": pr, "A_commits": ca, "B_commits": cb, "A_binaries": sorted(pc["binA"]), "B_binaries": sorted(pc["binB"])}
        entry["one_commit_per_arm"] = len(ca) == 1 and len(cb) == 1
        entry["one_binary_per_arm"] = len(pc["binA"]) == 1 and len(pc["binB"]) == 1
        entry["A_binary_differs_from_B"] = entry["one_binary_per_arm"] and pc["binA"] != pc["binB"]
        if entry["one_commit_per_arm"]:
            fa, fb = full_sha(ca[0]), full_sha(cb[0])
            fp = first_parent(cb[0])
            entry["A_full"], entry["B_full"], entry["B_first_parent"] = fa, fb, fp
            entry["A_is_first_parent_of_B"] = (fa is not None and fa == fp)
            entry["tarball_commit_A"] = tarballs.get(ca[0], {}).get("commit")
            entry["tarball_commit_B"] = tarballs.get(cb[0], {}).get("commit")
            entry["tarball_matches_git"] = entry["tarball_commit_A"] == fa and entry["tarball_commit_B"] == fb
        ok = (entry["one_commit_per_arm"] and entry["one_binary_per_arm"] and entry["A_binary_differs_from_B"]
              and entry.get("A_is_first_parent_of_B") is True)
        entry["ok"] = ok
        if not ok:
            binary["problems"].append("PR %s: arm binding failed %s" % (pr, entry))
        binary["per_pr"].append(entry)
    # every commit listed in binaries-run2.sha256 maps to one distinct binary
    binary["listed_binaries_distinct"] = len(set(binaries.values())) == len(binaries)
    binary["pass"] = not binary["problems"] and binary["listed_binaries_distinct"]
    if not binary["pass"]:
        findings.extend(binary["problems"])

    # run1-aa: confirm A and B binaries really are byte-identical (A/A control)
    aa_bin = []
    for pr, spec in DESIGN:
        for r in ROUNDS:
            ka, kb = (pr, spec, r, "A"), (pr, spec, r, "B")
            if ka in run1 and kb in run1:
                ba = {x.get("binary_sha256") for x in run1[ka]["records"]}
                bb = {x.get("binary_sha256") for x in run1[kb]["records"]}
                aa_bin.append({"pair": "pr%s-%s-r%d" % (pr, spec, r), "A": sorted(ba), "B": sorted(bb), "identical": ba == bb and len(ba) == 1})

    # ---- check 4: hazards
    hazards = {}

    # 4a warmup filtering
    warm = {"warmup_round_mismatch": sum(s["warmup_round_mismatch"] for s in sums2.values()),
            "warmup_not_bool": sum(s["warmup_not_bool"] for s in sums2.values()),
            "warmup_records": sum(s["n_warmup"] for s in sums2.values()),
            "measured_records": sum(s["n"] for s in sums2.values()),
            "sensitivity_including_warmup": {}}
    for pr, spec in DESIGN:
        per = {(r, a): sums2[(pr, spec, r, a)] for r in ROUNDS for a in ARMS}
        warm["sensitivity_including_warmup"]["pr%s/%s" % (pr, spec)] = {
            "solve_cpu_ba_geomean": row_stats(per, "cpu_all")["ba_geomean"],
            "solve_wall_ba_geomean": row_stats(per, "wall_all")["ba_geomean"]}
    hazards["warmup_filtering"] = warm

    # 4b cpu_ns availability and quantisation
    cpuq = {"cpu_ns_missing_or_null": sum(s["cpu_missing"] for s in sums2.values()),
            "solve_wall_ns_missing_or_null": sum(s["wall_missing"] for s in sums2.values()),
            "measured_records_with_cpu_ns_zero": sum(s["cpu_zero"] for s in sums2.values()),
            "measured_records": sum(s["n"] for s in sums2.values()),
            "wall_share_of_cpu_zero_records": (sum(s["wall_in_cpu_zero"] for s in sums2.values())
                                               / sum(s["wall"] for s in sums2.values())),
            "per_arm_cpu_over_wall": {}}
    arm_cpu = collections.Counter()
    arm_wall = collections.Counter()
    for (pr, spec, r, a), s in sums2.items():
        for arm, v in s["per_arm_cpu"].items():
            arm_cpu[(spec, arm)] += v
        for arm, v in s["per_arm_wall"].items():
            arm_wall[(spec, arm)] += v
    for k in sorted(arm_wall):
        cpuq["per_arm_cpu_over_wall"]["%s/%s" % k] = arm_cpu[k] / arm_wall[k] if arm_wall[k] else None
    hazards["cpu_ns"] = cpuq

    # 4c dominance: one arm carries the session sum
    dom = {}
    for pr, spec in DESIGN:
        tot_c = collections.Counter()
        for r in ROUNDS:
            for a in ARMS:
                tot_c.update(sums2[(pr, spec, r, a)]["per_arm_cpu"])
        top_arm, top_v = tot_c.most_common(1)[0]
        share = top_v / sum(tot_c.values())
        ratios_top, ratios_rest = [], []
        for r in ROUNDS:
            sa, sb = sums2[(pr, spec, r, "A")]["per_arm_cpu"], sums2[(pr, spec, r, "B")]["per_arm_cpu"]
            ratios_top.append(sb[top_arm] / sa[top_arm])
            ra = sum(v for k, v in sa.items() if k != top_arm)
            rb = sum(v for k, v in sb.items() if k != top_arm)
            ratios_rest.append(rb / ra if ra else None)
        dom["pr%s/%s" % (pr, spec)] = {
            "dominant_arm": top_arm, "dominant_cpu_share": share,
            "dominant_arm_cpu_ba_per_round": ratios_top, "dominant_arm_cpu_ba_geomean": geomean(ratios_top),
            "rest_cpu_ba_per_round": ratios_rest}
    hazards["dominance"] = dom

    # 4d order effects (rounds 1, 3 A first; round 2 B first)
    order = {"runlog_order_counterbalanced": True, "session_start_order_counterbalanced": True,
             "adjacent_pairs": True, "per_row": {}}
    pos = {k: i for i, k in enumerate(runlog_order)}
    for pr, spec in DESIGN:
        for r in ROUNDS:
            ka, kb = (pr, spec, r, "A"), (pr, spec, r, "B")
            a_first_expected = (r % 2 == 1)
            if (pos[ka] < pos[kb]) != a_first_expected:
                order["runlog_order_counterbalanced"] = False
            if abs(pos[ka] - pos[kb]) != 1:
                order["adjacent_pairs"] = False
            ta = run2[ka]["session"].get("started_unix_ms")
            tb = run2[kb]["session"].get("started_unix_ms")
            if (ta < tb) != a_first_expected:
                order["session_start_order_counterbalanced"] = False
        for metric in ("solve_cpu", "solve_wall"):
            rr = recomputed[(pr, spec)][metric]["ba_per_round"]
            la = (math.log(rr[0]) + math.log(rr[2])) / 2.0   # A-first rounds
            lb = math.log(rr[1])                             # B-first round
            d = la - lb
            order["per_row"]["pr%s/%s/%s" % (pr, spec, metric)] = {
                "a_first_ratio_geomean": math.exp(la), "b_first_ratio": math.exp(lb),
                "second_slot_factor_est": math.exp(d / 2.0),
                "order_balanced_ratio": math.exp((la + lb) / 2.0),
                "residual_bias_of_3round_geomean": math.exp(d / 6.0)}
    hazards["order_effects"] = order

    # 4e A/A control (run1-aa, byte-identical binaries)
    aa = {"binaries_identical_per_pair": all(x["identical"] for x in aa_bin), "binary_pairs": aa_bin, "per_row": {}}
    for pr, spec in DESIGN:
        if not all((pr, spec, r, a) in sums1 for r in ROUNDS for a in ARMS):
            continue
        per = {(r, a): sums1[(pr, spec, r, a)] for r in ROUNDS for a in ARMS}
        row = {}
        crow = next((x for x in claims_aa["rows"] if (str(x["pr"]), x["spec"]) == (pr, spec)), None)
        for metric, key in (("solve_cpu", "cpu"), ("solve_wall", "wall")):
            st = row_stats(per, key)
            ab = recomputed[(pr, spec)][metric]
            max_dev = max(abs(math.log(x)) for x in st["ba_per_round"])
            row[metric] = {"aa_ba_per_round": st["ba_per_round"], "aa_ba_geomean": st["ba_geomean"],
                           "aa_max_abs_log_dev": max_dev,
                           "ab_ba_geomean": ab["ba_geomean"],
                           "ab_outside_aa_band": abs(math.log(ab["ba_geomean"])) > max_dev,
                           "ab_all_rounds_outside_aa_band": all(abs(math.log(x)) > max_dev for x in ab["ba_per_round"]),
                           "matches_results_aa_json": crow is not None and close(dig(crow, metric, "ba_geomean"), st["ba_geomean"])}
        aa["per_row"]["pr%s/%s" % (pr, spec)] = row
    aa["run1_load1_range"] = [min(v["load1"] for v in runlog_aa.values()), max(v["load1"] for v in runlog_aa.values())]
    aa["run2_load1_range"] = [min(v["load1"] for v in runlog.values()), max(v["load1"] for v in runlog.values())]
    hazards["aa_control"] = aa

    # 4f isolation level and preflight
    iso = collections.Counter()
    quiet = collections.Counter()
    for s in run2.values():
        for rr in s["records"]:
            iso[dig(rr, "isolation", "level")] += 1
        quiet[str(dig(s["session"], "preflight", "quiet"))] += 1
    hazards["isolation"] = {"record_isolation_levels": dict(iso), "session_preflight_quiet": dict(quiet),
                            "allow_busy": sorted({str(dig(s["session"], "options", "allow_busy")) for s in run2.values()})}

    # 4g producer per_arm_cpu internal consistency (not a headline, informational)
    pac = []
    for c in claims["rows"]:
        for arm, v in (c.get("per_arm_cpu") or {}).items():
            gm, lo, hi = v.get("ba_geomean"), v.get("ba_min"), v.get("ba_max")
            if gm is not None and lo is not None and hi is not None and not (lo - 1e-12 <= gm <= hi + 1e-12 and lo > 0):
                key_pr, key_spec = str(c["pr"]), c["spec"]
                mine = {}
                if all((key_pr, key_spec, r, a) in sums2 for r in ROUNDS for a in ARMS):
                    mine = {"A_cpu_ns_per_round": [sums2[(key_pr, key_spec, r, "A")]["per_arm_cpu"][arm] for r in ROUNDS],
                            "B_cpu_ns_per_round": [sums2[(key_pr, key_spec, r, "B")]["per_arm_cpu"][arm] for r in ROUNDS]}
                pac.append({"row": "pr%s/%s" % (c["pr"], c["spec"]), "arm": arm, "ba_geomean": gm, "ba_min": lo, "ba_max": hi,
                            "aa_spread": v.get("aa_spread"), "validator_recomputed": mine,
                            "note": "a geometric mean cannot lie outside [min, max], and a zero ratio forces it to zero; "
                                    "short-arm cpu_ns is tick-quantised (mostly 0), so these per-arm figures carry no signal"})
    hazards["producer_per_arm_cpu_inconsistent"] = pac

    # 4h replay receipts are same-binary self-audits
    selfaudit = collections.Counter()
    for key, s in run2.items():
        vpath = s["dir"] + ".verify.json"
        try:
            txt = open(vpath).read()
            obj, _ = json.JSONDecoder().raw_decode(txt)
            selfaudit["ok" if obj.get("ok") else "not_ok"] += 1
            if obj.get("auditor_binary_sha256") == s["session"].get("binary_sha256"):
                selfaudit["auditor_binary_equals_producer_binary"] += 1
        except Exception:
            selfaudit["unreadable"] += 1
    hazards["on_pod_verify"] = dict(selfaudit)

    # 4i process-level wall from the runlog (cross-check of direction only)
    rl_ratio = {}
    for pr, spec in DESIGN:
        rs = [runlog[(pr, spec, r, "B")]["wall_ms"] / runlog[(pr, spec, r, "A")]["wall_ms"] for r in ROUNDS]
        rl_ratio["pr%s/%s" % (pr, spec)] = {"runlog_wall_ba_per_round": rs, "geomean": geomean(rs)}
    hazards["runlog_process_wall_ba"] = rl_ratio

    # ---- status

    try:
        head = subprocess.run(["git", "-C", repo, "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
        tracked = subprocess.run(["git", "-C", repo, "ls-files", "--error-unmatch",
                                  os.path.relpath(run2[DESIGN[0] + (1, "A")]["records_path"], repo)],
                                 capture_output=True, text=True).returncode == 0
        rel_base = os.path.relpath(base, repo)
        input_paths = [os.path.join(rel_base, p) for p in ("sessions", "reports", "specs", "tarballs.txt", "run.sh", "build.sh")]
        dirty = subprocess.run(["git", "-C", repo, "status", "--porcelain", "--"] + input_paths,
                               capture_output=True, text=True).stdout.strip().splitlines()
        tracked_inputs_missing = []
        for p in [os.path.join(rel_base, DATA_REL, x) for x in ("runlog-run2.tsv", "binaries-run2.sha256", "results-ab.json",
                                                                 "runlog-run1-aa.tsv", "results-aa.json")]:
            if subprocess.run(["git", "-C", repo, "ls-files", "--error-unmatch", p], capture_output=True).returncode != 0:
                tracked_inputs_missing.append(p)
        for s in list(run2.values()) + list(run1.values()):
            for fn in ("session.json", "spec.json", ".artifacts.json"):
                p = os.path.relpath(os.path.join(s["dir"], fn), repo)
                if subprocess.run(["git", "-C", repo, "ls-files", "--error-unmatch", p], capture_output=True).returncode != 0:
                    tracked_inputs_missing.append(p)
    except Exception:
        head, tracked, dirty, tracked_inputs_missing = None, None, None, None
    binding_ok = (dirty == [] and tracked_inputs_missing == [] and integrity.get("artifact_manifests_checked") == len(run2)
                  and not integrity.get("artifact_manifests_missing"))
    if not binding_ok:
        findings.append("artifact binding to a commit failed (dirty, untracked or manifest mismatch)")

    # status: every headline agrees, checks 1-3 pass, the design is complete, and the inputs are bound to a commit
    status = "pass" if (headline_ok and design_ok and integrity["ok"] and identity["pass"]
                        and outcome["pass"] and binary["pass"] and binding_ok) else "fail"

    raw_data = [{"path": EVIDENCE_PREFIX + "sessions/run2/%s/records.jsonl" % run2[k]["name"],
                 "sha256": run2[k]["records_sha256"]} for k in sorted(run2)]
    control_data = [{"path": EVIDENCE_PREFIX + "sessions/run1-aa/%s/records.jsonl" % run1[k]["name"],
                     "sha256": run1[k]["records_sha256"]} for k in sorted(run1)]
    inputs = []
    for rel in ("runlog-run2.tsv", "binaries-run2.sha256", "results-ab.json", "runlog-run1-aa.tsv", "results-aa.json"):
        inputs.append({"path": EVIDENCE_PREFIX + DATA_REL + "/" + rel, "sha256": sha256_file(os.path.join(data, rel))})
    for spec in ("koblitz", "prime", "tensor"):
        inputs.append({"path": EVIDENCE_PREFIX + "specs/%s.json" % spec, "sha256": sha256_file(os.path.join(base, "specs", spec + ".json"))})

    report = {
        "schema": "validation-gate.validator-report/v1",
        "status": status,
        "independent": True,
        "validator": {"role": "Validator", "model": "claude-opus-5-5",
                      "code": "research/wallfix_runpod_20261114/validator/validator.py",
                      "code_sha256": sha256_file(os.path.abspath(__file__)),
                      "producer_code_read": False,
                      "python": sys.version.split()[0]},
        "subject": "wallfix RunPod session-level ABAB (PRs 3850, 3991, 4009), solve_cpu and solve_wall B/A",
        "method": ("Independent stdlib recompute from the 36 run2 records.jsonl files: per session, sum over records with "
                   "warmup == false of time.schedstat_solve.cpu_ns and time.solve_wall_ns over all arms; per (PR, spec, round) "
                   "B/A; per (PR, spec) geometric mean (exp of mean log), min, max; aa_spread = max/min of A sums. Compared "
                   "with results-ab.json at rel_tol 1e-9 (sums exactly). Checks: output identity of A vs B after dropping timing "
                   "fields; every record verified with matches_target true; binary_sha256 bound to runlog-run2.tsv and "
                   "binaries-run2.sha256 and A = first parent of merge B in git; hazards (warmup, cpu_ns quantisation, dominance, "
                   "order, A/A control, isolation, self-audit). The producer's analysis code was not read."),
        "artifact_binding": {"repo_head_at_validation": head, "records_jsonl_tracked_in_git": tracked,
                             "records_bound_by_committed_artifacts_manifest": integrity.get("artifact_manifests_checked"),
                             "tracked_inputs_missing": tracked_inputs_missing,
                             "input_paths_dirty_vs_head": dirty, "ok": binding_ok,
                             "note": ("session.json, spec.json, .artifacts.json, runlog, binaries list, results-*.json and specs "
                                      "are tracked at repo_head_at_validation and clean; records.jsonl live in the artifact "
                                      "store and are bound by the committed .artifacts.json sha256, re-checked here.")},
        "raw_data": raw_data,
        "control_data": control_data,
        "inputs": inputs,
        "headline": [{k: h[k] for k in ("name", "claimed", "recomputed", "rel_tol")} for h in headline],
        "checks": {
            "design": dict(design_check, ok=design_ok),
            "session_integrity": integrity,
            "producer_table_recompute": {"all_agree": all_table_ok, "dispatch_rounded_values_agree": dispatch_round_ok,
                                         "fields": table_cmp},
            "1_output_identity": identity,
            "2_outcome_verified": outcome,
            "3_binary_identity": binary,
            "4_hazards": hazards,
        },
        "findings": findings,
    }
    with open(args.out, "w") as f:
        json.dump(report, f, indent=1, sort_keys=False)
        f.write("\n")

    # ---- console summary
    print("status:", status)
    print("%-26s %10s %14s %14s %8s" % ("headline", "metric", "claimed", "recomputed", "agree"))
    for h in headline:
        print("%-40s %.12f %.12f %s" % (h["name"], h["claimed"] or float("nan"), h["recomputed"], h["agrees"]))
    print("secondary table agrees:", all_table_ok, " dispatch rounded agree:", dispatch_round_ok)
    print("check1 identity: %d/%d pairs identical; cross-round %d/%d" % (identity["pairs_identical"], identity["pairs_total"],
          identity["cross_round_identical"], identity["cross_round_total"]))
    print("check2 outcome:", outcome["run2"])
    print("check3 binary:", binary["pass"], [(p["pr"], p["A_is_first_parent_of_B"] if "A_is_first_parent_of_B" in p else None,
                                              p["A_binary_differs_from_B"]) for p in binary["per_pr"]])
    print("integrity:", integrity["ok"], "manifests checked:", integrity.get("artifact_manifests_checked"), integrity["problems"][:5])
    print("binding: head", head, "ok", binding_ok, "dirty", dirty, "untracked inputs", tracked_inputs_missing)
    print("findings:", findings)
    return 0 if status == "pass" else 1


if __name__ == "__main__":
    sys.exit(main())
