"""Record-level mechanical facts for XP-20261029-dg-blocked: PROTOCOL.md section 6.1 condition 8 (records half),
the counted-equals-wall clause of section 6.1 "Counted session", and RULING-P2.md condition A1 (packed layout).
Reads records.jsonl of both sessions; counts and compares only. It computes no statistic and no wall figure
(no field named *wall*, time.*, online.* or isolation.* is read). It decides nothing and exits 0 whatever the values are.

usage: records_checks.py WALL_EKS COUNTED_EKS WALL_AN COUNTED_AN WALL_SPEC COUNTED_SPEC FB_PATH
Writes WALL_AN/records_checks.json and prints the lines R_COND8_RECORDS=, R_COUNTED_EQUALS_WALL=, R_A1_WALL=, R_A1_COUNTED=.
Exits 1 with a line beginning STOP: only when a records.jsonl, a frozen spec or the binary is absent or unreadable.

Matching key of a record: (workload.curve.slug, workload.target_index, round). The warm-up flag follows from the round.
Fields compared (condition 8): the recovered logarithm (outcome.recovered, with outcome.status and outcome.matches_target),
cost.total_gae, the list cost.unpriced, every unpriced-operation count as ecbench band's unpriced_counts finds it
(src/cryptanalysis/ecbench/band.rs lines 39-64 at be2969ce: the top-level counter of that name, else the sum over the phases of
the native counter of that name or of its name without the suffix _uncharged), and, per phase in order, the phase name,
adds, doubles, scalar_mults and gae.
Descriptive only (printed as separate lines, not part of any condition): equality of every phase's native map, of every phase's
work_native map, and of the top-level counters.
"""
import collections, json, os, re, subprocess, sys

eks_w, eks_c, an_w, an_c, spec_w, spec_c, fb = sys.argv[1:8]
P_MAX = 4294967295
POINTS_MAX = 65536
ICV1_RE = re.compile(r"^ICV1:fp-([0-9]+):")


def stop(msg):
    print("STOP: " + msg)
    sys.exit(1)


def load_records(eks):
    p = os.path.join(eks, "session", "records.jsonl")
    try:
        return [json.loads(l) for l in open(p) if l.strip()]
    except Exception as e:
        stop("cannot read %s: %s" % (p, type(e).__name__))


def tf(x):
    return "true" if x else "false"


def key(r):
    w = r.get("workload") or {}
    return ((w.get("curve") or {}).get("slug"), w.get("target_index"), r.get("round"))


def unpriced_counts(r):
    """Mirror of band.rs unpriced_counts (be2969ce lines 39-64)."""
    out = {}
    counters = r.get("counters") or {}
    phases = r.get("phases") or []
    for name in (r.get("cost") or {}).get("unpriced") or []:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in counters:
            out[name] = counters[name]
            continue
        total = 0
        found = False
        for p in phases:
            nat = p.get("native") or {}
            if name in nat:
                total += nat[name]
                found = True
            elif base in nat:
                total += nat[base]
                found = True
        if (not found) and base.startswith("solver_") and r.get("solver") is not None:
            out[name] = (r.get("solver") or {}).get("ops")
            continue
        out[name] = total if found else None
    return out


def sig(r):
    o = r.get("outcome") or {}
    c = r.get("cost") or {}
    return {
        "recovered": [o.get("recovered"), o.get("status"), o.get("matches_target")],
        "total_gae": c.get("total_gae"),
        "unpriced_list": c.get("unpriced"),
        "unpriced_counts": unpriced_counts(r),
        "phase_ops": [[p.get("name"), p.get("adds"), p.get("doubles"), p.get("scalar_mults"), p.get("gae")] for p in (r.get("phases") or [])],
    }


def desc_sig(r):
    return {
        "native_maps": [[p.get("name"), p.get("native")] for p in (r.get("phases") or [])],
        "work_native_maps": [[p.get("name"), p.get("work_native")] for p in (r.get("phases") or [])],
        "counters": r.get("counters"),
    }


def by_key(recs, arm):
    d = {}
    dup = 0
    for r in recs:
        if r.get("arm") == arm:
            k = key(r)
            if k in d:
                dup += 1
            d[k] = r
    return d, dup


FIELDS = ["recovered", "total_gae", "unpriced_list", "unpriced_counts", "phase_ops"]


def compare_arms(ra, rb):
    """ra, rb: dict key -> record. Returns a fact dict."""
    ka, kb = set(ra), set(rb)
    both = sorted(ka & kb, key=lambda k: (str(k[0]), str(k[1]), k[2] if isinstance(k[2], int) else -1))
    mism = collections.Counter()
    dmism = collections.Counter()
    first = collections.defaultdict(list)
    ident_mismatch = 0
    for k in both:
        a, b = ra[k], rb[k]
        if (a.get("workload") or {}).get("workload_id") != (b.get("workload") or {}).get("workload_id") or a.get("warmup") != b.get("warmup"):
            ident_mismatch += 1
        sa, sb = sig(a), sig(b)
        for f in FIELDS:
            if sa[f] != sb[f]:
                mism[f] += 1
                if len(first[f]) < 3:
                    first[f].append(list(k))
        da, db = desc_sig(a), desc_sig(b)
        for f in da:
            if da[f] != db[f]:
                dmism[f] += 1
    return {
        "records_a": len(ka), "records_b": len(kb), "matched": len(both),
        "only_in_a": len(ka - kb), "only_in_b": len(kb - ka),
        "workload_id_or_warmup_mismatch": ident_mismatch,
        "mismatch_by_field": {f: mism.get(f, 0) for f in FIELDS},
        "first_mismatch_keys": {f: first[f] for f in FIELDS if first[f]},
        "descriptive_mismatch_by_field": {f: dmism.get(f, 0) for f in ("native_maps", "work_native_maps", "counters")},
        "all_equal": (len(both) == len(ka) == len(kb) and ident_mismatch == 0 and sum(mism.values()) == 0),
    }


def plan_points_upper(spec_path):
    """Third source of A1: ecbench plan --doc of the frozen spec; 2 x the size of the prime-abscissa base of each arm."""
    r = subprocess.run([fb, "plan", "--spec", spec_path, "--doc"], capture_output=True, text=True)
    if r.returncode != 0:
        stop("ecbench plan --doc failed for " + spec_path)
    doc = json.loads(r.stdout)
    out = {}
    for a in doc.get("arms") or []:
        fbp = ((a.get("method") or {}).get("params") or {}).get("factor_base") or ""
        m = re.match(r"^prime-abscissa:size=([0-9]+)$", fbp)
        if m:
            out[a.get("name")] = 2 * int(m.group(1))
    return out


def a1(recs, spec_path, arms):
    n = 0
    bad_p = []
    bad_pts = []
    p_max = 0
    pts_max = 0
    src = collections.Counter()
    unparsed = 0
    plan_cache = None
    for r in recs:
        if r.get("arm") not in arms:
            continue
        n += 1
        icv1 = ((r.get("workload") or {}).get("curve") or {}).get("icv1")
        m = ICV1_RE.match(icv1 or "")
        if not m:
            unparsed += 1
            continue
        p = int(m.group(1))
        p_max = max(p_max, p)
        if p > P_MAX:
            bad_p.append([r.get("seq"), r.get("arm"), p])
        fbk = r.get("factor_base") or {}
        if isinstance(fbk.get("signed_points"), int):
            pts, s = fbk["signed_points"], "record.signed_points"
        elif isinstance(fbk.get("abscissae"), int):
            pts, s = 2 * fbk["abscissae"], "record.2x_abscissae"
        else:
            if plan_cache is None:
                plan_cache = plan_points_upper(spec_path)
            pts, s = plan_cache.get(r.get("arm")), "plan_doc.2x_size"
        src[s] += 1
        if pts is None:
            unparsed += 1
            continue
        pts_max = max(pts_max, pts)
        if pts > POINTS_MAX:
            bad_pts.append([r.get("seq"), r.get("arm"), pts])
    ok = n > 0 and unparsed == 0 and not bad_p and not bad_pts
    return {
        "arms": sorted(arms), "records_checked": n, "records_unparsed": unparsed,
        "p_max": p_max, "p_limit": P_MAX, "records_with_p_over_limit": len(bad_p), "first_bad_p": bad_p[:3],
        "signed_points_max": pts_max, "signed_points_limit": POINTS_MAX, "records_with_points_over_limit": len(bad_pts), "first_bad_points": bad_pts[:3],
        "signed_points_source_counts": dict(src), "ok": ok,
    }


for pth in (spec_w, spec_c, fb):
    if not os.path.isfile(pth):
        stop("absent: " + pth)
wall = load_records(eks_w)
cnt = load_records(eks_c)
out = {}
dg_w, dup_dg = by_key(wall, "dg")
tot_dup = dup_dg
out["wall_dg_vs"] = {}
c8 = True
for arm in ("dgb", "dgb-edge"):
    rb, dup = by_key(wall, arm)
    tot_dup += dup
    f = compare_arms(dg_w, rb)
    f["duplicate_keys_in_arm"] = dup
    out["wall_dg_vs"][arm] = f
    c8 = c8 and f["all_equal"] and f["records_a"] == 336 and f["records_b"] == 336
out["wall_dg_duplicate_keys"] = dup_dg
# dg-aa against dg: reported, not a condition 8 requirement
raa, dupaa = by_key(wall, "dg-aa")
f = compare_arms(dg_w, raa)
f["duplicate_keys_in_arm"] = dupaa
out["wall_dg_vs"]["dg-aa (descriptive, not in condition 8)"] = f
c8 = c8 and dup_dg == 0 and all(out["wall_dg_vs"][a]["duplicate_keys_in_arm"] == 0 for a in ("dgb", "dgb-edge"))
out["cross_session"] = {}
cx = True
for arm in ("dg", "dgb"):
    rw, d1 = by_key(wall, arm)
    rc, d2 = by_key(cnt, arm)
    f = compare_arms(rw, rc)
    f["duplicate_keys"] = [d1, d2]
    out["cross_session"][arm] = f
    cx = cx and f["all_equal"] and f["records_a"] == 336 and f["records_b"] == 336 and d1 == 0 and d2 == 0
out["a1_wall"] = a1(wall, spec_w, ("dgb", "dgb-edge"))
out["a1_counted"] = a1(cnt, spec_c, ("dgb",))
out["lines"] = {
    "R_COND8_RECORDS": c8,
    "R_COUNTED_EQUALS_WALL": cx,
    "R_A1_WALL": out["a1_wall"]["ok"],
    "R_A1_COUNTED": out["a1_counted"]["ok"],
}
with open(os.path.join(an_w, "records_checks.json"), "w") as fo:
    json.dump(out, fo, indent=1, sort_keys=True)
print("RECORDS_CHECKS_FACTS (mechanical facts; the executor does not interpret them)")
for arm, f in out["wall_dg_vs"].items():
    print("COND8_RECORDS dg_vs_" + arm.replace(" ", "_") + " " + json.dumps(f, sort_keys=True))
for arm, f in out["cross_session"].items():
    print("COUNTED_VS_WALL " + arm + " " + json.dumps(f, sort_keys=True))
print("A1_WALL " + json.dumps(out["a1_wall"], sort_keys=True))
print("A1_COUNTED " + json.dumps(out["a1_counted"], sort_keys=True))
for k in ("R_COND8_RECORDS", "R_COUNTED_EQUALS_WALL", "R_A1_WALL", "R_A1_COUNTED"):
    print(k + "=" + tf(out["lines"][k]))
print("RECORDS_CHECKS_OK")
