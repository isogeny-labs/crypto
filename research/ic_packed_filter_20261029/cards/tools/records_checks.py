"""Record-level mechanical facts for XP-20261029-packed-filter: PROTOCOL.md section 6.1 condition 8 (records half, with the table_filter_bit_sets
requirements of APPROVAL.md E2), the counted-equals-wall clause of "Counted session", and condition 10 (packed layout, APPROVAL.md E3).
Reads records.jsonl of both sessions; counts and compares only. It computes no statistic and reads no wall figure (no field named *wall*, time.*,
online.* or isolation.* is read; a phase's wall_ns is not read). It decides nothing and exits 0 whatever the values are.

usage: records_checks.py WALL_EKS COUNTED_EKS WALL_AN COUNTED_AN WALL_SPEC COUNTED_SPEC FB_PATH
Writes WALL_AN/records_checks.json and prints the lines R_COND8_RECORDS=, R_TFBS_REQUIREMENTS=, R_COUNTED_EQUALS_WALL=, R_COND10_WALL=, R_COND10_COUNTED=.
Exits 1 with a line beginning STOP: only when a records.jsonl, a frozen spec or the binary is absent or unreadable.

Matching key of a record: (workload.curve.slug, workload.target_index, round); the warm-up flag follows from the round.
Condition 8 fields compared between each dgbf and dgbf-edge record and the dgb record of its key: the recovered logarithm (outcome.recovered, with
outcome.status and outcome.matches_target), cost.total_gae, the list cost.unpriced, every unpriced-operation count as ecbench band's unpriced_counts
finds it (src/cryptanalysis/ecbench/band.rs at 3271a25d02), and per phase in order: name, adds, doubles, scalar_mults, gae, every key and value of
native, and every key and value of work_native except table_filter_bit_sets.
table_filter_bit_sets requirements: absent on every dgb and dgb-aa record; present on every dgbf and dgbf-edge record in the oracle_setup phase only;
one value on all of a curve's dgbf and dgbf-edge records.
Descriptive only (printed as separate lines, not part of any condition): equality of the top-level counters, and dgb-aa against dgb.
Condition 10: p from the ICV1 curve string <= 4294967295; signed points <= 65536 (record factor_base.signed_points, else 2 x factor_base.abscissae,
else 2 x the size in the frozen spec's plan document), for the arms dgb, dgb-aa, dgbf and dgbf-edge (wall) and dgb and dgbf (counted)."""
import collections, json, os, re, subprocess, sys

eks_w, eks_c, an_w, an_c, spec_w, spec_c, fb = sys.argv[1:8]
P_MAX = 4294967295
POINTS_MAX = 65536
ICV1_RE = re.compile(r"^ICV1:fp-([0-9]+):")
TFBS = "table_filter_bit_sets"


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
    """Mirror of band.rs unpriced_counts (3271a25d02)."""
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


def without_tfbs(wn):
    return {k: v for k, v in (wn or {}).items() if k != TFBS}


def tfbs_phases(r):
    """[(phase name, value)] of the phases that carry the counter."""
    return [[p.get("name"), (p.get("work_native") or {})[TFBS]] for p in (r.get("phases") or []) if TFBS in (p.get("work_native") or {})]


def sig(r):
    o = r.get("outcome") or {}
    c = r.get("cost") or {}
    return {
        "recovered": [o.get("recovered"), o.get("status"), o.get("matches_target")],
        "total_gae": c.get("total_gae"),
        "unpriced_list": c.get("unpriced"),
        "unpriced_counts": unpriced_counts(r),
        "phase_ops": [[p.get("name"), p.get("adds"), p.get("doubles"), p.get("scalar_mults"), p.get("gae"), p.get("native")] for p in (r.get("phases") or [])],
        "work_native_except_tfbs": [[p.get("name"), without_tfbs(p.get("work_native"))] for p in (r.get("phases") or [])],
    }


FIELDS = ["recovered", "total_gae", "unpriced_list", "unpriced_counts", "phase_ops", "work_native_except_tfbs"]


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


def compare_arms(ra, rb, with_tfbs=False):
    ka, kb = set(ra), set(rb)
    both = sorted(ka & kb, key=lambda k: (str(k[0]), str(k[1]), k[2] if isinstance(k[2], int) else -1))
    mism = collections.Counter()
    dmism = collections.Counter()
    first = collections.defaultdict(list)
    ident_mismatch = 0
    fields = FIELDS + (["table_filter_bit_sets"] if with_tfbs else [])
    for k in both:
        a, b = ra[k], rb[k]
        if (a.get("workload") or {}).get("workload_id") != (b.get("workload") or {}).get("workload_id") or a.get("warmup") != b.get("warmup"):
            ident_mismatch += 1
        sa, sb = sig(a), sig(b)
        if with_tfbs:
            sa["table_filter_bit_sets"] = tfbs_phases(a)
            sb["table_filter_bit_sets"] = tfbs_phases(b)
        for f in fields:
            if sa[f] != sb[f]:
                mism[f] += 1
                if len(first[f]) < 3:
                    first[f].append(list(k))
        if (a.get("counters") or {}) != (b.get("counters") or {}):
            dmism["counters"] += 1
    return {
        "records_a": len(ka), "records_b": len(kb), "matched": len(both),
        "only_in_a": len(ka - kb), "only_in_b": len(kb - ka),
        "workload_id_or_warmup_mismatch": ident_mismatch,
        "mismatch_by_field": {f: mism.get(f, 0) for f in fields},
        "first_mismatch_keys": {f: first[f] for f in fields if first[f]},
        "descriptive_counters_mismatch": dmism.get("counters", 0),
        "all_equal": (len(both) == len(ka) == len(kb) and ident_mismatch == 0 and sum(mism.values()) == 0),
    }


def tfbs_requirements(recs):
    """The three requirements on table_filter_bit_sets (APPROVAL.md E2)."""
    absent_bad = []
    present_bad = []
    per_curve = collections.defaultdict(set)
    n = collections.Counter()
    for r in recs:
        arm = r.get("arm")
        ph = tfbs_phases(r)
        n[arm] += 1
        if arm in ("dgb", "dgb-aa"):
            if ph:
                absent_bad.append([r.get("seq"), arm, ph])
        elif arm in ("dgbf", "dgbf-edge"):
            if len(ph) != 1 or ph[0][0] != "oracle_setup" or not isinstance(ph[0][1], int):
                present_bad.append([r.get("seq"), arm, ph])
            else:
                per_curve[((r.get("workload") or {}).get("curve") or {}).get("slug")].add(ph[0][1])
    one_value = all(len(v) == 1 for v in per_curve.values()) and len(per_curve) > 0
    ok = (not absent_bad) and (not present_bad) and one_value
    return {
        "records_by_arm": dict(n),
        "records_on_dgb_or_dgb_aa_that_carry_the_counter": len(absent_bad), "first_such": absent_bad[:3],
        "records_on_dgbf_or_dgbf_edge_without_exactly_one_oracle_setup_value": len(present_bad), "first_such_2": present_bad[:3],
        "values_per_curve_over_dgbf_and_dgbf_edge": {c: sorted(v) for c, v in sorted(per_curve.items(), key=lambda kv: str(kv[0]))},
        "one_value_per_curve": one_value,
        "ok": ok,
    }


def plan_points_upper(spec_path):
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


def cond10(recs, spec_path, arms):
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
base, dup_base = by_key(wall, "dgb")
out["wall_dgb_vs"] = {}
c8 = dup_base == 0
for arm in ("dgbf", "dgbf-edge"):
    rb, dup = by_key(wall, arm)
    f = compare_arms(base, rb)
    f["duplicate_keys_in_arm"] = dup
    out["wall_dgb_vs"][arm] = f
    c8 = c8 and f["all_equal"] and f["records_a"] == 336 and f["records_b"] == 336 and dup == 0
raa, dupaa = by_key(wall, "dgb-aa")
f = compare_arms(base, raa)
f["duplicate_keys_in_arm"] = dupaa
out["wall_dgb_vs"]["dgb-aa (descriptive, not in condition 8)"] = f
out["wall_dgb_duplicate_keys"] = dup_base
out["tfbs_wall"] = tfbs_requirements(wall)
c8 = c8 and out["tfbs_wall"]["ok"]
out["cross_session"] = {}
cx = True
for arm in ("dgb", "dgbf"):
    rw, d1 = by_key(wall, arm)
    rc, d2 = by_key(cnt, arm)
    f = compare_arms(rw, rc, with_tfbs=True)
    f["duplicate_keys"] = [d1, d2]
    out["cross_session"][arm] = f
    cx = cx and f["all_equal"] and f["records_a"] == 336 and f["records_b"] == 336 and d1 == 0 and d2 == 0
out["tfbs_counted"] = tfbs_requirements([r for r in cnt if r.get("arm") in ("dgb", "dgbf")])
out["cond10_wall"] = cond10(wall, spec_w, ("dgb", "dgb-aa", "dgbf", "dgbf-edge"))
out["cond10_counted"] = cond10(cnt, spec_c, ("dgb", "dgbf"))
out["lines"] = {
    "R_COND8_RECORDS": c8,
    "R_TFBS_REQUIREMENTS": out["tfbs_wall"]["ok"],
    "R_COUNTED_EQUALS_WALL": cx,
    "R_COND10_WALL": out["cond10_wall"]["ok"],
    "R_COND10_COUNTED": out["cond10_counted"]["ok"],
}
with open(os.path.join(an_w, "records_checks.json"), "w") as fo:
    json.dump(out, fo, indent=1, sort_keys=True)
print("RECORDS_CHECKS_FACTS (mechanical facts; the executor does not interpret them)")
for arm, f in out["wall_dgb_vs"].items():
    print("COND8_RECORDS dgb_vs_" + arm.replace(" ", "_") + " " + json.dumps(f, sort_keys=True))
print("TFBS_WALL " + json.dumps(out["tfbs_wall"], sort_keys=True))
print("TFBS_COUNTED " + json.dumps(out["tfbs_counted"], sort_keys=True))
for arm, f in out["cross_session"].items():
    print("COUNTED_VS_WALL " + arm + " " + json.dumps(f, sort_keys=True))
print("COND10_WALL " + json.dumps(out["cond10_wall"], sort_keys=True))
print("COND10_COUNTED " + json.dumps(out["cond10_counted"], sort_keys=True))
for k in ("R_COND8_RECORDS", "R_TFBS_REQUIREMENTS", "R_COUNTED_EQUALS_WALL", "R_COND10_WALL", "R_COND10_COUNTED"):
    print(k + "=" + tf(out["lines"][k]))
print("RECORDS_CHECKS_OK")
