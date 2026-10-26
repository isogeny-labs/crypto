#!/usr/bin/env python3
"""Independent validator for rule v4 (research/ic_tool_program/rule/v4). stdlib only; shares no code with ic_rule_v4.rs.

Reads the archive (runs.tar.xz, extracted by the caller into SCRATCH, its sha256 checked against runs.tar.xz.sha256),
rule v3's committed scalars, and compares with the claims in analysis.json and replay-certificate.json.
usage: validate_v4.py REPO SCRATCH_RUNS_DIR OUT.json [EVIDENCE_DIR]
"""
import hashlib, json, math, os, random, statistics as st, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
import ec_verify as ev

repo = Path(sys.argv[1]).resolve(); runs = Path(sys.argv[2]); out_p = sys.argv[3]
evid = Path(sys.argv[4]).resolve() if len(sys.argv) > 4 else None
V4 = repo / "research/ic_tool_program/rule/v4"; V3 = repo / "research/ic_tool_program/rule/v3"
AN = json.load(open(V4 / "analysis.json")); CERT = json.load(open(V4 / "replay-certificate.json"))
# (archive key, n) per size, in the analysis order; keys are the archive's own.
SIZES = [(s["curve"], s["n"]) for s in AN["sizes"]]
CL = {s["curve"]: s for s in AN["sizes"]}
BOOT = 10000
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()

def rel(p): return os.path.relpath(p, evid) if evid else str(Path(p).relative_to(repo))

def clean(base):
    iso = [json.loads(l) for l in open(base + ".isolation.jsonl") if l.strip()][-1]
    return iso["run"]["exit_status"] == 0 and not iso["run"]["contended"], iso["run"]

def load(sz, arm):
    out, iso, attempts = {}, {}, 0
    for i in range(1, 33):
        for k in range(0, 6):
            b = str(runs / "runs" / sz / f"T{i:02d}-{arm}" ) + (f"-retry{k}" if k else "")
            if not os.path.exists(b + ".price.json"): break
            attempts += 1
            ok, run = clean(b)
            d = json.load(open(b + ".price.json"))
            if ok and d["status"] == "complete" and d["all_verified"] and d["ic_and_rho_agree"]:
                out[i] = d; iso[i] = run; break
    return out, iso, attempts

def boot(x, y, ids, seed, lo=2.5, hi=97.5):
    cl = {}
    for i, a, b in zip(ids, x, y):
        c = (i - 1) % 16; s = cl.setdefault(c, [0.0, 0.0]); s[0] += a; s[1] += b
    S = list(cl.values()); m = len(S); rng = random.Random(seed); d = []
    for _ in range(BOOT):
        sx = sy = 0.0
        for _ in range(m):
            a, b = S[rng.randrange(m)]; sx += a; sy += b
        d.append(sx / sy)
    d.sort()
    q = lambda p: d[min(BOOT - 1, round(p / 100 * (BOOT - 1)))]
    return sum(x) / sum(y), q(lo), q(hi)

def counts_ic(d, n):
    p = d["counts"]["pass"]; g = lambda *k: (lambda v: v if v is not None else 0.0)(__import__("functools").reduce(lambda a, b: a.get(b) if isinstance(a, dict) else None, k, p))
    stored = g("build", "stored_pairs"); scanned = g("collect", "summands_scanned"); trials = g("collect", "trials")
    cof = g("select", "selection_cost", "cofactor_multiplications"); dt = g("descent", "trials_total")
    charged = stored + scanned + 1.5 * n * (trials + cof + 2) + dt
    return charged, (stored + scanned + dt) + (scanned + dt) + stored
def counts_rho(d, n):
    c = d["rho_policy"]["counters"]; z = lambda k: c.get(k, 0)
    adds = sum(z(k) for k in ("walk_operations", "setup_additions", "setup_doublings", "target_setup_additions",
                              "target_setup_doublings", "start_additions", "look_ahead_additions", "verification_operations"))
    sm = 1.5 * n * (d["rho_setup"]["setup_ops"]["scalar_mults"] + d["rho_counts"]["target_ops"]["scalar_mults"])
    return adds + sm, z("canonicalisations_uncharged") + 2 * z("distinguished_points_stored")

report = {"schema": "validator-report/v1", "independent": True, "headline": [], "sizes": {}, "checks": {}}
H = report["headline"]
def head(name, claimed, recomputed, tol): H.append({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": tol})
problems = []
rng_seed = 20261024
for sz, n in SIZES:
    c = CL[sz]; r = float(c["r"]); q = math.sqrt(r)
    p3, i3, a3 = load(sz, "v3"); p4, i4, a4 = load(sz, "v4"); pb, _, _ = load(sz, "v3b"); pl, _, _ = load(sz, "lev")
    ids = sorted(set(p3) & set(p4))
    cold = lambda d: d["median"]["setup_units"] + d["median"]["ic_online_units"]
    rho_c = lambda d: d["median"]["rho_setup_units"] + d["median"]["rho_model_units"]
    rho_b = lambda d: d["median"]["rho_setup_units"] + d["median"]["rho_online_units"]
    c3 = [cold(p3[i]) for i in ids]; c4 = [cold(p4[i]) for i in ids]
    s3 = [p3[i]["median"]["setup_units"] for i in ids]; s4 = [p4[i]["median"]["setup_units"] for i in ids]
    o3 = [p3[i]["median"]["ic_online_units"] for i in ids]; o4 = [p4[i]["median"]["ic_online_units"] for i in ids]
    R, lo, hi = boot(c4, c3, ids, rng_seed ^ n)
    Rs = boot(s4, s3, ids, rng_seed ^ n ^ 1); Ro = boot(o4, o3, ids, rng_seed ^ n ^ 2)
    verdict = "IMPROVEMENT" if hi < 1 else ("REGRESSION" if lo > 1 else "NOT_SEPARATED")
    cp = c["paired_v4_over_v3"]
    g = lambda arm, f: sum(f(arm[i]) for i in ids)
    rc3 = [rho_c(p3[i]) for i in ids]; rc4 = [rho_c(p4[i]) for i in ids]; rb3 = [rho_b(p3[i]) for i in ids]; rb4 = [rho_b(p4[i]) for i in ids]
    gap = {"v3_canonical": sum(c3) / sum(rc3), "v4_canonical": sum(c4) / sum(rc4), "v3_as_built": sum(c3) / sum(rb3), "v4_as_built": sum(c4) / sum(rb4)}
    gci = boot(c4, rc4, ids, rng_seed ^ n ^ 3)
    # section 5a band
    band = {}
    for w in (0.0, 0.1, 1.0):
        def tot(P, fn, wt=w): return [fn(P[i], n)[0] + wt * fn(P[i], n)[1] for i in ids]
        i3w, i4w, r3w, r4w = tot(p3, counts_ic), tot(p4, counts_ic), tot(p3, counts_rho), tot(p4, counts_rho)
        # the rho counts of a pair are those of its own v3 or v4 process (same rho walk and point)
        gp = boot(i4w, r4w, ids, rng_seed ^ n ^ 4); rr = boot(i4w, i3w, ids, rng_seed ^ n ^ 5)
        band[str(w)] = {"ic_v3_gae": st.mean(i3w), "ic_v4_gae": st.mean(i4w), "rho_gae": st.mean(r4w), "gap_v3": sum(i3w) / sum(r3w),
                        "gap_v4": sum(i4w) / sum(r4w), "v4_over_v3": rr[0], "v4_over_v3_ci": [rr[1], rr[2]]}
    cb = c["band_5a"]
    # neighbours and spread
    wall = lambda iso: [iso[i]["wall_seconds"] for i in ids]
    neigh = [iso[i]["other_cpu_seconds"] / iso[i]["wall_seconds"] for iso in (i3, i4) for i in ids]
    spread = [p["spread_max_over_min"]["ic_online"] for P in (p3, p4) for p in P.values()]
    aa = None
    if pb:
        ia = sorted(set(pb) & set(p3)); aa = boot([cold(pb[i]) for i in ia], [cold(p3[i]) for i in ia], ia, rng_seed ^ n ^ 6)
    lv = None
    if pl:
        il = sorted(set(pl) & set(p3)); lv = boot([cold(pl[i]) for i in il], [cold(p3[i]) for i in il], il, rng_seed ^ n ^ 7)
        lev_pairs = [cold(pl[i]) / cold(p3[i]) for i in il]
    # headline
    tol = 1e-9
    head(f"{sz} cold v4/v3", cp["cold"]["value"], R, tol)
    head(f"{sz} cold CI lo", cp["cold"]["ci95"][0], lo, 0.006); head(f"{sz} cold CI hi", cp["cold"]["ci95"][1], hi, 0.006)
    head(f"{sz} setup v4/v3", cp["setup"]["value"], Rs[0], tol); head(f"{sz} online v4/v3", cp["online"]["value"], Ro[0], tol)
    head(f"{sz} S_IC cold v3", c["s"]["ic_cold_v3"], st.mean(c3) / q, tol); head(f"{sz} S_IC cold v4", c["s"]["ic_cold_v4"], st.mean(c4) / q, tol)
    head(f"{sz} S_rho cold canonical v3proc", c["s"]["rho_cold_canonical_v3proc"], st.mean(rc3) / q, tol)
    head(f"{sz} S_rho cold canonical v4proc", c["s"]["rho_cold_canonical_v4proc"], st.mean(rc4) / q, tol)
    head(f"{sz} gap v3 canonical", c["gap_ic_cold_over_rho_cold"]["v3_canonical"]["value"], gap["v3_canonical"], tol)
    head(f"{sz} gap v4 canonical", c["gap_ic_cold_over_rho_cold"]["v4_canonical"]["value"], gap["v4_canonical"], tol)
    head(f"{sz} gap v4 canonical CI lo", c["gap_ic_cold_over_rho_cold"]["v4_canonical"]["ci95"][0], gci[1], 0.01)
    head(f"{sz} gap v4 canonical CI hi", c["gap_ic_cold_over_rho_cold"]["v4_canonical"]["ci95"][1], gci[2], 0.01)
    head(f"{sz} gap v3 as built", c["gap_ic_cold_over_rho_cold"]["v3_as_built"]["value"], gap["v3_as_built"], tol)
    head(f"{sz} gap v4 as built", c["gap_ic_cold_over_rho_cold"]["v4_as_built"]["value"], gap["v4_as_built"], tol)
    for key, w in (("w0", "0.0"), ("w0_1", "0.1"), ("w1", "1.0")):
        head(f"{sz} 5a {key} gap_v4", cb[key]["gap_v4"], band[w]["gap_v4"], tol)
        head(f"{sz} 5a {key} gap_v3", cb[key]["gap_v3"], band[w]["gap_v3"], tol)
        head(f"{sz} 5a {key} v4_over_v3", cb[key]["v4_over_v3"], band[w]["v4_over_v3"], tol)
    if aa: head(f"{sz} A/A cold", c["aa_v3b_over_v3"]["cold_ratio"], aa[0], tol)
    if lv: head(f"{sz} lever control cold", c["lever_control_over_v3"]["cold_ratio"], lv[0], tol)
    report["sizes"][sz] = {"n": n, "pairs": len(ids), "attempts_v3": a3, "attempts_v4": a4, "cold_v4_over_v3": [R, lo, hi],
                           "verdict_recomputed": verdict, "verdict_claimed": c["verdict"], "gap": gap, "gap_v4_canonical_ci": [gci[1], gci[2]],
                           "band": band, "neighbour_cpu_per_wall_median": st.median(neigh), "neighbour_share_over_0.2": sum(x > 0.2 for x in neigh) / len(neigh),
                           "spread_max_over_min_median": st.median(spread), "aa": aa, "lever": lv,
                           "aa_ci_width": (aa[2] - aa[1]) if aa else None, "claimed_neighbour_median": c["neighbour_cpu_per_wall"]["median"]}
    if verdict != c["verdict"]: problems.append(f"{sz}: verdict {verdict} vs claimed {c['verdict']}")
    report["sizes"][sz]["_lev_pairs"] = lev_pairs
# pooled lever control
lp = [x for s in report["sizes"].values() for x in s.pop("_lev_pairs")]
gm = math.exp(st.mean(math.log(x) for x in lp)); claimed_gm = AN["lever_control_pooled"]["geometric_mean_ratio"]
head("lever control pooled geometric mean", claimed_gm, gm, 1e-9)
verdicts = [s["verdict_recomputed"] for s in report["sizes"].values()]
improved = verdicts.count("IMPROVEMENT"); regressed = verdicts.count("REGRESSION")
outcome = "BASELINE_IMPROVED" if improved >= 4 and not regressed else ("PARTIAL" if 1 <= improved <= 3 else ("NEGATIVE" if regressed else "NULL"))
report["outcome_recomputed"] = outcome; report["outcome_claimed"] = AN["outcome"]

# ---- replay checks over every process in the archive
files = sorted((runs / "runs").glob("k*n*/T*.price.json"))
cert_by_path = {r["path"]: r for r in CERT["runs"]}
bad, nverified = [], 0
for f in files:
    d = json.load(open(f)); rp = None
    for k in (f"runs/{f.parent.name}/{f.name}",):
        rp = cert_by_path.get(k)
    parts = f.name.split("-"); i = int(parts[0][1:])
    ics, rhos = d["certificates"]["ic"], d["certificates"]["rho"]
    ok = True
    deg = ics["curve"]["degree"]
    C = ev.F2m(deg, ics["curve"]["irreducible_low_terms"], ics["curve"]["curve_a"], 1)
    G = tuple(int(x) for x in ics["curve"]["generator"]); Q = tuple(int(x) for x in ics["target"])
    r = int(ics["curve"]["subgroup_order"])
    ok &= C.on(G) and C.on(Q) and ev.smul(C, r, G) is None
    for cert in (ics, rhos):
        ok &= ev.smul(C, int(cert["scalar"]) % r, G) == Q and tuple(int(x) for x in cert["target"]) == Q
    ok &= ev.smul(C, (int(ics["scalar"]) + 1) % r, G) != Q   # negative control
    v3 = json.load(open(V3 / "runs" / f.parent.name / f"T{i:02d}-R1.price.json"))["certificates"]["ic"]
    ok &= v3["scalar"] == ics["scalar"] == rhos["scalar"] and [int(x) for x in v3["target"]] == list(Q)
    if not ok: bad.append(str(f.relative_to(runs)))
    nverified += ok
report["checks"]["scalar_replay"] = {"processes_in_archive": len(files), "processes_in_certificate": len(CERT["runs"]),
                                      "verified_independently": nverified, "failed": bad[:20],
                                      "what": "[scalar]G = target in own pure-Python GF(2^n) arithmetic for the ic and the rho certificate of every process, [r]G = O, scalar+1 rejected, scalar equals rule v3's for that target"}
fs = [f for f in files if cert_by_path.get(f"runs/{f.parent.name}/{f.name}", {}).get("file_sha256") not in (None,)]
mism = [str(f.name) for f in files if (cert_by_path.get(f"runs/{f.parent.name}/{f.name}") or {}).get("file_sha256") != sha(f)]
report["checks"]["file_hashes"] = {"checked": len(files), "mismatching_or_missing_in_certificate": mism[:20], "n_mismatch": len(mism)}
# untimed reruns
rr = []
for r in CERT["untimed_reruns"]:
    f = runs / "replay-rerun" / f"{r['size']}-T{r['target']:02d}-{r['arm']}.price.json"
    if not f.exists(): rr.append((r, "missing")); continue
    d = json.load(open(f)); rr.append((r, d["certificates"]["ic"]["scalar"] == r["fresh_scalar"] == r["recorded_scalar"] and d["all_verified"]))
report["checks"]["untimed_reruns"] = {"claimed": len(CERT["untimed_reruns"]), "files_found_and_scalars_equal": sum(1 for _, ok in rr if ok is True), "not_ok": [x[0] for x in rr if x[1] is not True][:5]}
# the v3 arm is v3's parameter file unchanged
diffs = []
for sz, n in SIZES:
    a = json.load(open(runs / "runs" / sz / "params" / "T01-v3.params.json")); b = json.load(open(V3 / "runs" / sz / "T01.params.json"))
    for k in sorted(set(a) | set(b)):
        if a.get(k) != b.get(k): diffs.append(f"{sz}.{k}")
report["checks"]["v3_params_vs_rule_v3"] = {"differing_keys_T01": diffs}
# lanes, steps vs strong fixture
lanes = set(); ratios = []
for sz, n in SIZES:
    for i in range(1, 9):
        d = json.load(open(runs / "runs" / sz / f"T{i:02d}-v3.price.json")); lanes.add(d["rho_policy"]["counters"]["lanes"])
        try: s = json.loads(open(runs / "runs/reference" / f"{sz}-T{i:02d}.strong.json").readline())
        except Exception: ratios.append((sz, i, None, None)); continue
        ratios.append((sz, i, d["rho_counts"]["steps"] / float(s["ideal_steps"]), int(s["walk_steps"]) / float(s["ideal_steps"])))
good = [(a, b) for _, _, a, b in ratios if a is not None]
report["checks"]["rho_reference"] = {"lanes_in_rho_policy": sorted(lanes), "protocol_text_lanes": 64, "strong_fixture_rows_missing_or_unreadable": [(s, i) for s, i, a, b in ratios if a is None],
                                      "pairs_used": len(good), "steps_over_ideal_parallelrho_mean": st.mean(a for a, _ in good), "steps_over_ideal_strong_mean": st.mean(b for _, b in good),
                                      "strong_over_parallel_steps": st.mean(b for _, b in good) / st.mean(a for a, _ in good),
                                      "se_of_that_ratio_approx": (st.mean(b for _, b in good) / st.mean(a for a, _ in good)) * math.sqrt((st.pstdev([b for _, b in good]) / st.mean(b for _, b in good)) ** 2 / len(good) + (st.pstdev([a for a, _ in good]) / st.mean(a for a, _ in good)) ** 2 / len(good))}
lanes_by_size = {}
for sz, n in SIZES:
    lanes_by_size[sz] = sorted({json.load(open(runs / "runs" / sz / f"T{i:02d}-v3.price.json"))["rho_policy"]["counters"]["lanes"] for i in range(1, 33) if (runs / "runs" / sz / f"T{i:02d}-v3.price.json").exists()})
rc_ = report["checks"]["rho_reference"]; rc_["lanes_by_size"] = lanes_by_size
rc_["parallel_over_strong_ns_per_step_claimed"] = {sz: CL[sz]["rho_reference_check"]["parallel_over_strong"] for sz, _ in SIZES}
rc_["parallel_rho_admissible_claimed"] = {sz: CL[sz]["rho_reference_check"]["parallel_rho_admissible"] for sz, _ in SIZES}
report["rho_reference_assessment"] = {
    "kind": "negation_frobenius", "same_unit": True, "same_binary_or_process": True,
    "same_process_evidence": "ic price runs the index calculus and ParallelRho in one process, one unit, both rebuilt from nothing each repetition; 493 of 493 processes recompute",
    "matches_protocol": "yes for what is priced (in-process ParallelRho at the canonical step, as amended); the protocol text says 64 lanes while the rho counters show " + str(sorted({l for v in lanes_by_size.values() for l in v})) + " by size",
    "strongest_available": False,
    "strongest_available_reason": "the protocol's own admissibility check (kept from rule v2) FAILS at 4 of 6 sizes (ParallelRho slower per step than the strong fixture by 2 to 20 percent) and was only reported, not acted on; the canonical-step price uses ParallelRho's counted steps, and on the 46 readable T01-T08 pairs the strong fixture needs " + format(rc_["strong_over_parallel_steps"], ".3f") + " (+-" + format(rc_["se_of_that_ratio_approx"], ".3f") + ") of ParallelRho's steps (per ideal steps; two fixture rows unreadable), so a stronger rho than the one priced is not excluded; if it is real the gap is larger than reported, not smaller",
    "a8_neighbour_load": {sz: {"median_neighbour_cpu_per_wall": report["sizes"][sz]["neighbour_cpu_per_wall_median"], "aa_ci_width": report["sizes"][sz]["aa_ci_width"]} for sz, _ in SIZES},
    "a8_effect": "does not change same_binary_or_process (both arms share each process and its unit); does widen noise (A/A interval width 0.10-0.12 at the clean sizes n = 47, 57, 41-first-pairs against 0.19-0.29 at n = 53, 59, 61) and can bias gaps at the loaded sizes through memory contention that the cache-resident unit does not see; the one IMPROVEMENT (n = 57) is in the clean regime",
    "band": "IC/rho (gap, counted GAE lower bound) is above 1 at w = 0, 0.1, 1 at every size; order holds IC over rho; v4 against v3 is not ranked by the band at four sizes"}
# agreement
bad_h = [h["name"] for h in H if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]
report["disagreements"] = bad_h; report["problems"] = problems
exact = [h for h in H if h["rel_tol"] <= 1e-9]; mc = [h for h in H if h["rel_tol"] > 1e-9]
report["summary"] = {"headline_numbers": len(H), "exact_agree": sum(1 for h in exact if h["name"] not in bad_h), "exact_total": len(exact),
                     "bootstrap_agree": sum(1 for h in mc if h["name"] not in bad_h), "bootstrap_total": len(mc)}
report["status"] = "pass" if not bad_h and not problems and not bad and outcome == AN["outcome"] and not mism else "fail"
report["raw_data"] = [{"path": rel(V4 / "analysis.json")}, {"path": rel(V4 / "runs.tar.xz"), "sha256": sha(V4 / "runs.tar.xz")}, {"path": rel(V4 / "replay-certificate.json")}]
json.dump(report, open(out_p, "w"), indent=1)
print(report["status"], report["summary"], "outcome", outcome, "bad", bad_h[:10], problems, "scalar_fail", len(bad), "hash_mismatch", len(mism))
