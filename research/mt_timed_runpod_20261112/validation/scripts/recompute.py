#!/usr/bin/env python3
"""VALIDATOR recompute for research/mt_timed_runpod_20261112 (independent of the producer's analyze.py / mk_runs.py).

Reads the committed raw data (results/holdout/runs.json and runs.jsonl, opcost.jsonl, host.json) and recomputes, per
build (std, fast): the design and ordering checks, per-configuration icbest/rhobest/bsgsbest from the median of each
key's 3 repeats (ms_arm = solve phase; ms = solve + shared verification), per-cell median ratios with the protocol's
bootstrap and the exact bootstrap over all 4^4 resamples, decisions, per-size configuration counts and ranges, the
fast/std tables, the fixed-key control, the A/A control and the round drift, the opcost ranges and the winners.
Also recomputes the earlier timed study's cell medians (../mt_timed_20261112/results/holdout/runs.json) for the
host-replication statements, and compares the std-build counters with that study's rows.

usage: recompute.py <study dir> <earlier study dir> <out.json>"""
import collections
import hashlib
import itertools
import json
import math
import os
import random
import statistics
import sys

STUDY, EARLIER, OUT = sys.argv[1:4]
H = os.path.join(STUDY, "results/holdout")
IC = ("ic5", "ic5ks", "ic5w", "ic5rw")
RHO = ("rho2xks", "rhodpu")
BS = ("bsgs2",)
CNT = ("targets", "ok", "I", "M", "S", "H", "L", "N")


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def med(xs):
    return statistics.median(xs)


def exact_boot(xs):
    """2.5% / 97.5% quantiles of the bootstrap distribution of the median, enumerated over all n^n resamples."""
    n = len(xs)
    vals = sorted(med(c) for c in itertools.product(xs, repeat=n))
    N = len(vals)
    return vals[int(math.floor(0.025 * N))], vals[int(math.ceil(0.975 * N)) - 1]


def protocol_boot(xs):
    """the protocol's bootstrap: random.Random(1), B = 4000, order statistics 100 and 3899 (as PROTOCOL / analyze rule)."""
    r = random.Random(1)
    B = 4000
    m = sorted(med([xs[r.randrange(len(xs))] for _ in range(len(xs))]) for _ in range(B))
    return m[100], m[3899]


def decide(lo, hi):
    return "faster" if hi < 1 else ("slower" if lo > 1 else "unranked")


# ---------------------------------------------------------------- load
J = json.load(open(os.path.join(H, "runs.json")))
F = J["fields"]
rows = [dict(zip(F, r)) for r in J["rows"]]
raw = [json.loads(l) for l in open(os.path.join(H, "runs.jsonl"))]
host = json.load(open(os.path.join(H, "host.json")))
payload = json.load(open(os.path.join(STUDY, "launch/payload.json")))
checks = collections.OrderedDict()
checks["runs_json_sha256"] = sha(os.path.join(H, "runs.json"))
checks["runs_jsonl_sha256"] = sha(os.path.join(H, "runs.jsonl"))
checks["runs_json_source_sha256_matches"] = J["source_sha256"] == checks["runs_jsonl_sha256"]
checks["rows_json"] = len(rows)
checks["rows_jsonl"] = len(raw)
mm = 0
for a, b in zip(rows, raw):
    for f in F:
        if a[f] != b.get(f):
            mm += 1
checks["runs_json_vs_jsonl_field_mismatches"] = mm
checks["runs_json_commit_field"] = J["commit"]

# ---------------------------------------------------------------- design and order (re-derived from PROTOCOL.md)
design = []
for build in ("std", "fast"):
    for pb in (24, 28, 32):
        for T in (1024, 4096, 16384):
            for idx in range(4):
                for a in IC:
                    design.append((build, pb, T, idx, a, 1.0, None))
                for k in (0.05, 0.1, 0.25, 0.5):
                    design.append((build, pb, T, idx, "rho2xks", k, None))
                for k in (0.0, 0.25):
                    for off in (-1, 0, 1):
                        design.append((build, pb, T, idx, "rhodpu", k, off))
                for mf in (0.7, 1.0, 1.4):
                    design.append((build, pb, T, idx, "bsgs2", mf, None))
checks["design_keys"] = len(design)
rnd = random.Random(20261112)
order = []
for rep in range(3):
    o = list(range(len(design)))
    rnd.shuffle(o)
    order += [(rep, i) for i in o]
bad_order = 0
for s, r in enumerate(rows):
    rep, i = order[s]
    k = design[i]
    got = (r["build"], r["pb"], r["T"], r["idx"], r["arm"], r["param"], r["doff"])
    if r["seq"] != s or r["rep"] != rep or r["key"] != i or got != k or r["seed"] != 43:
        bad_order += 1
checks["rows_out_of_preregistered_order"] = bad_order
checks["seq_order_matches_seeded_shuffle"] = bad_order == 0 and len(rows) == 3 * len(design)
keyrows = collections.defaultdict(list)
for r in rows:
    keyrows[(r["build"], r["pb"], r["T"], r["idx"], r["arm"], r["param"], r["doff"])].append(r)
checks["keys_with_reps_0_1_2"] = sum(sorted(x["rep"] for x in v) == [0, 1, 2] for v in keyrows.values())
errs = [r for r in raw if "error" in r]
checks["errors"] = len(errs)
checks["ok_ne_targets"] = sum(r["ok"] != r["targets"] for r in rows)
checks["targets_ne_T"] = sum(r["targets"] != r["T"] for r in rows)
checks["targets_total"] = sum(r["targets"] for r in rows)
checks["targets_verified_total"] = sum(r["ok"] for r in rows)
checks["bin_sha256_by_build"] = {b: sorted({r["bin_sha256"] for r in rows if r["build"] == b}) for b in ("std", "fast")}
checks["bin_sha256_match_payload"] = all(
    checks["bin_sha256_by_build"][b] == [payload["files_sha256"]["bin/mt_fieldops_" + b]] for b in ("std", "fast"))
checks["host_bin_sha256_match_payload"] = all(
    host["bin_sha256"][b] == payload["files_sha256"]["bin/mt_fieldops_" + b] for b in ("std", "fast"))
# counters identical between builds for every (configuration, arm, param), and across repeats of a key
cdiff_build = 0
cdiff_rep = 0
for k, v in keyrows.items():
    c0 = [tuple(x[c] for c in CNT) for x in v]
    if len(set(c0)) != 1:
        cdiff_rep += 1
    if k[0] == "std":
        w = keyrows[("fast",) + k[1:]]
        if tuple(w[0][c] for c in CNT) != c0[0]:
            cdiff_build += 1
checks["keys_with_counters_differing_across_repeats"] = cdiff_rep
checks["configuration_keys_with_counters_differing_between_builds"] = cdiff_build
# phases (raw only) identical between builds too
ph = collections.defaultdict(set)
for r in raw:
    ph[(r["pb"], r["T"], r["idx"], r["arm"], r["param"], r.get("doff"))].add(json.dumps(r["phases"], sort_keys=True))
checks["configuration_keys_with_phases_differing_between_builds_or_repeats"] = sum(len(v) != 1 for v in ph.values())
la = [r["loadavg"] for r in rows]
checks["loadavg_min_max"] = [min(la), max(la)]
checks["loadavg_median_by_round"] = [med([r["loadavg"] for r in rows if r["rep"] == k]) for k in range(3)]
checks["loadavg_by_sixth_of_run"] = [round(med([r["loadavg"] for r in rows[i * len(rows) // 6:(i + 1) * len(rows) // 6]]), 1) for i in range(6)]
st = [r["steal_ticks"] for r in rows]
checks["steal_ticks_sum_max"] = [sum(st), max(st)]
checks["wall_s_total"] = sum(r["wall_s"] for r in rows)
checks["host"] = {k: host[k] for k in ("cpu", "nproc", "allowed_cpus", "pin", "cpuset", "cpu_max", "kernel", "started")}

# ---------------------------------------------------------------- per build analysis
def build_tables(build):
    key = {}
    for k, v in keyrows.items():
        if k[0] != build:
            continue
        v = sorted(v, key=lambda x: x["rep"])
        key[k[1:]] = dict(arm=k[4], param=k[5], doff=k[6], arm_reps=[x["ms_arm"] for x in v], all_reps=[x["ms"] for x in v],
                          t=med([x["ms_arm"] for x in v]), tall=med([x["ms"] for x in v]))
    cfgs = sorted({k[:3] for k in key})
    per = {}
    for c in cfgs:
        ks = {k[3:]: v for k, v in key.items() if k[:3] == c}
        side = lambda arms, f: min((v for v in ks.values() if v["arm"] in arms), key=lambda v: v[f])
        ib, rb, bb = side(IC, "t"), side(RHO, "t"), side(BS, "t")
        per[c] = dict(ic=ib, rho=rb, bsgs=bb, ic_all=side(IC, "tall"), rho_all=side(RHO, "tall"), keys=ks,
                      r=ib["t"] / rb["t"], r_all=side(IC, "tall")["tall"] / side(RHO, "tall")["tall"], r_bs=ib["t"] / bb["t"],
                      aa=rb["arm_reps"][0] / rb["arm_reps"][1])
    cells = []
    for pb in (24, 28, 32):
        for T in (1024, 4096, 16384):
            cs = [c for c in cfgs if c[:2] == (pb, T)]
            row = dict(bits=pb, T=T, n=len(cs))
            for name, f in (("ic_rho_solve", "r"), ("ic_rho_all", "r_all"), ("ic_bsgs_solve", "r_bs"), ("aa_rho", "aa")):
                xs = [per[c][f] for c in cs]
                plo, phi = protocol_boot(xs)
                elo, ehi = exact_boot(xs)
                row[name] = dict(values=xs, median=med(xs), protocol_ci=[plo, phi], exact_ci=[elo, ehi], range=[min(xs), max(xs)],
                                 decision=decide(plo, phi), below_1=sum(x < 1 for x in xs))
            # fixed-key control: each side's key chosen on repeat 0 (median over the cell's 4 configurations)
            def pick(arms):
                cand = collections.defaultdict(list)
                for c in cs:
                    for kk, v in per[c]["keys"].items():
                        if v["arm"] in arms:
                            cand[kk].append(v["arm_reps"][0])
                return min(cand, key=lambda kk: med(cand[kk]))
            ki, kr = pick(IC), pick(RHO)
            xs = [med(per[c]["keys"][ki]["arm_reps"][1:]) / med(per[c]["keys"][kr]["arm_reps"][1:]) for c in cs]
            plo, phi = protocol_boot(xs)
            row["fixed_ic_rho_solve"] = dict(values=xs, median=med(xs), protocol_ci=[plo, phi], range=[min(xs), max(xs)],
                                             decision=decide(plo, phi), keys=dict(ic=list(map(str, ki)), rho=list(map(str, kr))))
            row["winners_ic"] = dict(collections.Counter(per[c]["ic"]["arm"] for c in cs))
            row["winners_rho"] = dict(collections.Counter(per[c]["rho"]["arm"] for c in cs))
            row["winners_rho_param"] = dict(collections.Counter("%s %s %s" % (per[c]["rho"]["arm"], per[c]["rho"]["param"], per[c]["rho"]["doff"]) for c in cs))
            row["aa_ci_contains_1"] = row["aa_rho"]["protocol_ci"][0] <= 1 <= row["aa_rho"]["protocol_ci"][1]
            cells.append(row)
    size = {}
    for pb in (24, 28, 32):
        xs = [per[c]["r"] for c in cfgs if c[0] == pb]
        size[pb] = dict(n=len(xs), below_1=sum(x < 1 for x in xs), above_1=sum(x > 1 for x in xs), config_range=[min(xs), max(xs)],
                        cell_median_range=[min(r["ic_rho_solve"]["median"] for r in cells if r["bits"] == pb),
                                           max(r["ic_rho_solve"]["median"] for r in cells if r["bits"] == pb)],
                        decisions=[r["ic_rho_solve"]["decision"] for r in cells if r["bits"] == pb])
    # round drift: each repeat relative to its key's median (all keys of this build)
    rel = collections.defaultdict(list)
    devs = []
    for v in key.values():
        m = med(v["arm_reps"])
        for i, x in enumerate(v["arm_reps"]):
            rel[i].append(x / m)
            devs.append(abs(math.log(x / m)))
    devs.sort()
    drift = dict(median_rel_by_round=[med(rel[i]) for i in range(3)], mean_rel_by_round=[statistics.fmean(rel[i]) for i in range(3)],
                 median_abs_log_dev=med(devs), p95_abs_log_dev=devs[int(0.95 * len(devs))])
    return per, cells, size, drift, key


B = {}
for b in ("std", "fast"):
    per, cells, size, drift, key = build_tables(b)
    B[b] = dict(per=per, cells=cells, size=size, drift=drift, key=key)

# pooled round drift over both builds (the RESULTS sentence)
rel = collections.defaultdict(list)
devs = []
for b in ("std", "fast"):
    for v in B[b]["key"].values():
        m = med(v["arm_reps"])
        for i, x in enumerate(v["arm_reps"]):
            rel[i].append(x / m)
            devs.append(abs(math.log(x / m)))
devs.sort()
pooled_drift = dict(median_rel_by_round=[med(rel[i]) for i in range(3)], median_abs_log_dev=med(devs), p95_abs_log_dev=devs[int(0.95 * len(devs))])

# ---------------------------------------------------------------- fast / std
fs = []
for pb in (24, 28, 32):
    for T in (1024, 4096, 16384):
        cs = [c for c in B["std"]["per"] if c[:2] == (pb, T)]
        row = dict(bits=pb, T=T)
        for side in ("ic", "rho", "bsgs"):
            xs = [B["fast"]["per"][c][side]["t"] / B["std"]["per"][c][side]["t"] for c in cs]
            plo, phi = protocol_boot(xs)
            row[side] = dict(values=xs, median=med(xs), protocol_ci=[plo, phi], range=[min(xs), max(xs)])
        fs.append(row)

# ---------------------------------------------------------------- opcost
opc = []
for l in open(os.path.join(H, "opcost.jsonl")):
    o = json.loads(l)
    g = o["padd_ns"]
    opc.append(dict(pb=o["pb"], when=o["when"], gae_ns=g, I_over_M=o["inv_ns"] / o["mul_ns"], native_gae=o["canon_hash_ns"] / g,
                    std_lookup_gae={t["log2_size"]: t["std_lookup_ns"] / g for t in o["tables"]},
                    fast_lookup_gae={t["log2_size"]: t["fast_lookup_ns"] / g for t in o["tables"]}))
e_opc = [json.loads(l) for l in open(os.path.join(EARLIER, "results/holdout/opcost.jsonl"))]
opsum = dict(gae_ns=[min(o["gae_ns"] for o in opc), max(o["gae_ns"] for o in opc)],
             I_over_M=[min(o["I_over_M"] for o in opc), max(o["I_over_M"] for o in opc)],
             native_gae=[min(o["native_gae"] for o in opc), max(o["native_gae"] for o in opc)],
             std_lookup_gae_2_10_to_2_16=[min(o["std_lookup_gae"][s] for o in opc for s in (10, 12, 14, 16)),
                                          max(o["std_lookup_gae"][s] for o in opc for s in (10, 12, 14, 16))],
             std_lookup_gae_2_22_to_2_24=[min(o["std_lookup_gae"][s] for o in opc for s in (22, 24)),
                                          max(o["std_lookup_gae"][s] for o in opc for s in (22, 24))],
             earlier_vm_gae_ns=[min(o["padd_ns"] for o in e_opc if "padd_ns" in o), max(o["padd_ns"] for o in e_opc if "padd_ns" in o)],
             earlier_vm_std_lookup_gae_2_22_to_2_24=[min(t["std_lookup_ns"] / o["padd_ns"] for o in e_opc if "padd_ns" in o for t in o["tables"] if t["log2_size"] in (22, 24)),
                                                     max(t["std_lookup_ns"] / o["padd_ns"] for o in e_opc if "padd_ns" in o for t in o["tables"] if t["log2_size"] in (22, 24))])
opsum["vm_over_runpod_gae_ns_median"] = med([o["padd_ns"] for o in e_opc if "padd_ns" in o]) / med([o["gae_ns"] for o in opc])

# ---------------------------------------------------------------- earlier timed study (session VM): cell medians and counters
EJ = json.load(open(os.path.join(EARLIER, "results/holdout/runs.json")))
ER = [dict(zip(EJ["fields"], r)) for r in EJ["rows"]]
ekey = collections.defaultdict(list)
for r in ER:
    ekey[(r["pb"], r["T"], r["idx"], r["arm"], r["param"], r["doff"])].append(r)
ecells = []
for pb in (24, 28, 32):
    for T in (1024, 4096, 16384):
        xs = []
        for idx in range(4):
            ks = {k: v for k, v in ekey.items() if k[:3] == (pb, T, idx)}
            t = lambda arms: min(med([x["ms_arm"] for x in v]) for k, v in ks.items() if k[3] in arms)
            xs.append(t(IC) / t(RHO))
        plo, phi = protocol_boot(xs)
        ecells.append(dict(bits=pb, T=T, median=med(xs), protocol_ci=[plo, phi], decision=decide(plo, phi), range=[min(xs), max(xs)]))
cmp_counters = dict(keys_compared=0, rows_compared=0, mismatching_keys=0, mismatch_examples=[])
for k, v in keyrows.items():
    if k[0] != "std":
        continue
    ek = ekey.get(k[1:])
    if not ek:
        continue
    cmp_counters["keys_compared"] += 1
    mine = tuple(v[0][c] for c in CNT)
    bad = False
    for x in ek:
        cmp_counters["rows_compared"] += 1
        if tuple(x[c] for c in CNT) != mine:
            bad = True
    if bad:
        cmp_counters["mismatching_keys"] += 1
        if len(cmp_counters["mismatch_examples"]) < 5:
            cmp_counters["mismatch_examples"].append([list(k), mine, tuple(ek[0][c] for c in CNT)])
cmp_counters["earlier_runs_json_sha256"] = sha(os.path.join(EARLIER, "results/holdout/runs.json"))
cmp_counters["earlier_bin_sha256"] = EJ["bin_sha256"]

# ---------------------------------------------------------------- output
def strip(b):
    return dict(cells=b["cells"], size={str(k): v for k, v in b["size"].items()}, drift=b["drift"],
                per_config=[dict(cfg=list(c), ic_key=[v["ic"]["arm"], v["ic"]["param"]], ic_ms=v["ic"]["t"],
                                 rho_key=[v["rho"]["arm"], v["rho"]["param"], v["rho"]["doff"]], rho_ms=v["rho"]["t"],
                                 bsgs_ms=v["bsgs"]["t"], ic_rho=v["r"], ic_rho_all=v["r_all"], ic_bsgs=v["r_bs"], aa=v["aa"])
                            for c, v in sorted(b["per"].items())])


out = dict(schema="validator.recompute/v1", checks=checks, std=strip(B["std"]), fast=strip(B["fast"]), pooled_drift=pooled_drift,
           fast_over_std=fs, opcost=opc, opcost_summary=opsum, earlier_vm_cells=ecells, std_counters_vs_earlier_study=cmp_counters)
json.dump(out, open(OUT, "w"), indent=1, default=str)

# ---------------------------------------------------------------- print
print(json.dumps(checks, indent=None, default=str))
for b in ("std", "fast"):
    print("\n#", b)
    for r in B[b]["cells"]:
        s, a, bs, aa, fx = r["ic_rho_solve"], r["ic_rho_all"], r["ic_bsgs_solve"], r["aa_rho"], r["fixed_ic_rho_solve"]
        print(f"{r['bits']} {r['T']:5d} solve {s['median']:.3f} [{s['protocol_ci'][0]:.3f},{s['protocol_ci'][1]:.3f}] exact [{s['exact_ci'][0]:.3f},{s['exact_ci'][1]:.3f}] {s['decision']:8s}"
              f" | all {a['median']:.3f} {a['decision']:8s} | bsgs {bs['median']:.3f} [{bs['range'][0]:.3f},{bs['range'][1]:.3f}] | A/A {aa['median']:.3f} [{aa['protocol_ci'][0]:.3f},{aa['protocol_ci'][1]:.3f}] c1={r['aa_ci_contains_1']}"
              f" | fixed {fx['median']:.3f} [{fx['protocol_ci'][0]:.3f},{fx['protocol_ci'][1]:.3f}] {fx['decision']} | win {r['winners_ic']} {r['winners_rho']}")
    for pb, v in B[b]["size"].items():
        print("  size", pb, v)
    print("  drift", B[b]["drift"])
print("\npooled drift", pooled_drift)
print("\n# fast/std")
for r in fs:
    print(r["bits"], r["T"], " ".join(f"{s} {r[s]['median']:.3f} [{r[s]['range'][0]:.3f},{r[s]['range'][1]:.3f}]" for s in ("ic", "rho", "bsgs")))
print("\nopcost", json.dumps(opsum))
print("\n# earlier VM cells")
for r in ecells:
    print(r)
print("\nstd counters vs earlier study", json.dumps({k: v for k, v in cmp_counters.items() if k != "mismatch_examples"}), cmp_counters["mismatch_examples"][:2])
