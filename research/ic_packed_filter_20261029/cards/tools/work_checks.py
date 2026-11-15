"""bitop/v1 facts of XP-20261029-packed-filter: PROTOCOL.md section 6.1 condition 8 (the bitop clause) and section 5 "bitop/v1".
Runs `ecbench work` (the image's binary) on a session and reads, from its output, only work.bitops (and the per-phase bitops of each record); from
records.jsonl it reads arm, round, warmup, outcome.status, workload and the oracle_setup phase's work_native.table_filter_bit_sets. It reads no wall figure.
It decides nothing and exits 0 whatever the values are.

usage: work_checks.py FB_PATH WALL_EKS COUNTED_EKS WALL_AN COUNTED_AN
  COUNTED_EKS may be a session directory (the counted session); both sessions are processed.
Writes WALL_AN/work_checks.json and COUNTED_AN/work_checks.json; prints the line R_COND8_BITOPS= (wall session) and the sums.
Exits 1 with a line beginning STOP: only when a file is absent or `ecbench work` fails.

For every dgbf and dgbf-edge record (warm-ups included), matched by (curve slug, target_index, round) with the dgb record: the difference
bitops(dgbf) - bitops(dgb) must equal 384 x table_filter_bit_sets of the dgbf record, and the phases whose bitops differ must be exactly [oracle_setup]."""
import collections, json, os, subprocess, sys

fb, eks_w, eks_c, an_w, an_c = sys.argv[1:6]
TFBS = "table_filter_bit_sets"


def stop(msg):
    print("STOP: " + msg)
    sys.exit(1)


def tf(x):
    return "true" if x else "false"


def key(r):
    w = r.get("workload") or {}
    return ((w.get("curve") or {}).get("slug"), w.get("target_index"), r.get("round"))


def session(eks, an, label, required):
    sdir = os.path.join(eks, "session")
    try:
        recs = [json.loads(l) for l in open(os.path.join(sdir, "records.jsonl")) if l.strip()]
    except Exception as e:
        stop("cannot read records.jsonl of %s: %s" % (label, type(e).__name__))
    r = subprocess.run([fb, "work", sdir], capture_output=True, text=True)
    if r.returncode != 0:
        stop("ecbench work failed on %s: %s" % (sdir, r.stderr.strip()[:200]))
    rows = {}
    for line in r.stdout.splitlines():
        if line.strip():
            d = json.loads(line)
            rows[d["record_id"]] = d
    info = {}
    notok = 0
    for rec in recs:
        w = rows.get(rec.get("record_id"))
        ok = isinstance(w, dict) and isinstance(w.get("work"), dict) and "Ok" in w["work"]
        if not ok:
            notok += 1
            info[rec["record_id"]] = None
            continue
        okd = w["work"]["Ok"]
        tfv = None
        for p in rec.get("phases") or []:
            if p.get("name") == "oracle_setup":
                tfv = (p.get("work_native") or {}).get(TFBS)
        info[rec["record_id"]] = {
            "arm": rec.get("arm"), "key": key(rec), "warmup": bool(rec.get("warmup")),
            "verified": (rec.get("outcome") or {}).get("status") == "verified",
            "bitops": int(okd["bitops"]), "phases": {p["name"]: int(p["bitops"]) for p in okd.get("phases") or []},
            "tfbs": tfv,
        }
    out = {"label": label, "records": len(recs), "work_rows": len(rows), "records_without_Ok_work": notok}
    sums_all = collections.Counter()
    sums_counted = collections.Counter()
    n_all = collections.Counter()
    n_counted = collections.Counter()
    for rid, v in info.items():
        if v is None:
            continue
        sums_all[v["arm"]] += v["bitops"]
        n_all[v["arm"]] += 1
        if (not v["warmup"]) and v["verified"]:
            sums_counted[v["arm"]] += v["bitops"]
            n_counted[v["arm"]] += 1
    out["sum_bitops_all_records_by_arm"] = dict(sums_all)
    out["records_all_by_arm"] = dict(n_all)
    out["sum_bitops_measured_verified_by_arm"] = dict(sums_counted)
    out["records_measured_verified_by_arm"] = dict(n_counted)
    for scope, s in (("all_records", sums_all), ("measured_verified", sums_counted)):
        if s.get("dgb"):
            for arm in ("dgbf", "dgbf-edge", "dgb-aa"):
                if arm in s:
                    out["ratio_%s_over_dgb_%s_repr" % (arm, scope)] = repr(s[arm] / s["dgb"])
    # matched identity
    by = collections.defaultdict(dict)
    for rid, v in info.items():
        if v is not None:
            by[v["arm"]].setdefault(v["key"], v)
    ident = {}
    all_ok = True
    for arm in ("dgbf", "dgbf-edge"):
        if arm not in by:
            continue
        n = 0
        bad = []
        for k, v in by[arm].items():
            b = by["dgb"].get(k)
            n += 1
            if b is None or v["tfbs"] is None:
                bad.append([list(k), "unmatched or no counter"])
                continue
            d = v["bitops"] - b["bitops"]
            diff_phases = sorted(name for name in set(v["phases"]) | set(b["phases"]) if v["phases"].get(name) != b["phases"].get(name))
            if d != 384 * v["tfbs"] or diff_phases != ["oracle_setup"]:
                bad.append([list(k), d, 384 * v["tfbs"], diff_phases])
        ident[arm] = {"matched_records": n, "records_violating": len(bad), "first_violations": bad[:3]}
        all_ok = all_ok and n == 336 and not bad
    out["identity_vs_dgb"] = ident
    out["required_arms"] = list(required)
    out["identity_ok_and_336_each"] = all_ok and notok == 0 and sorted(ident) == sorted(required)
    json.dump(out, open(os.path.join(an, "work_checks.json"), "w"), indent=1, sort_keys=True)
    return out


for pth in (fb, os.path.join(eks_w, "session", "records.jsonl"), os.path.join(eks_c, "session", "records.jsonl")):
    if not os.path.isfile(pth):
        stop("absent: " + pth)
w = session(eks_w, an_w, "wall", ("dgbf", "dgbf-edge"))
c = session(eks_c, an_c, "counted", ("dgbf",))
print("WORK_CHECKS_FACTS (mechanical facts; the executor does not interpret them)")
for label, o in (("WALL", w), ("COUNTED", c)):
    print("WORK_" + label + " " + json.dumps(o, sort_keys=True))
print("R_COND8_BITOPS=" + tf(w["identity_ok_and_336_each"]))
print("R_COUNTED_BITOPS_IDENTITY=" + tf(c["identity_ok_and_336_each"]))
print("WORK_CHECKS_OK")
