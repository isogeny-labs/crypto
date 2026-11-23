#!/usr/bin/env python3
"""Desk model: static XOR / ribbon retrieval structure for the N83 root index.

Reads only committed N83 result files (research/n83_*/measurements.json.gz and
two RESULT.md files for rho) and recomputes every figure. No timing is run.
All outputs are [pcc:unverified] model results; modelled rows are labelled.

Usage: python3 model.py [--json out.json]
"""
import glob, gzip, json, math, os, re, statistics as st, sys

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.abspath(os.path.join(HERE, "..", "..", ".."))  # research/
GiB = 2 ** 30

def S(K):                      # symmetric states, N83 root index (Capacity.lean)
    return 83 * K * (K - 1) // 2 + 42 * K

# ---------------------------------------------------------------- load rows
rows = []
for f in sorted(glob.glob(os.path.join(RES, "n83_*", "measurements.json.gz"))):
    try:
        m = json.load(gzip.open(f))
    except Exception:
        continue
    if not isinstance(m, list):
        continue
    for r in m:
        if isinstance(r, dict) and r.get("root_table_entries") and r.get("peak_rss_bytes") \
           and r.get("root_state_allocated_bytes") and r.get("root_table_allocated_bytes"):
            r = dict(r); r["_src"] = os.path.basename(os.path.dirname(f)); rows.append(r)

def layout(r):
    st_b = r["root_state_allocated_bytes"]; K = r["effective_columns"]
    per_state = st_b / S(K)
    if abs(per_state - 11) < 1e-6:
        return "root4" if r["root_table_allocated_bytes"] == 2 ** 33 else "root11"
    return "root%d" % round(per_state)          # root21/root22: 21/22 B per state

out = {}
P = lambda *a: print(*a)

P("# N83 XOR/ribbon desk model  [pcc:unverified]  rows read: %d" % len(rows))

# ------------------------------------------------ 1. how close are the layouts
P("\n## 1. Measured index layouts versus the retrieval-structure bound\n")
P("bits/entry = table_bytes*8/root_table_entries (measured allocation). payload = bits")
P("a slot carries (root4: 32 = 31-bit state id + selector; root11: 40 = 5-byte slot).")
P("floor(f)=f bits/entry (any retrieval structure); xor=1.23 f; ribbon=1.005f..1.10f.\n")
P("| layout | K | entries | table GiB | bits/entry | payload | load-overhead x | xor 1.23 | ribbon 1.005 | ribbon 1.10 |")
P("|---|---|---|---|---|---|---|---|---|---|")
seen = set(); lay_tab = []
for r in sorted(rows, key=lambda r: (layout(r), r["effective_columns"])):
    lay = layout(r); K = r["effective_columns"]
    if lay not in ("root4", "root11") or (lay, K) in seen: continue
    seen.add((lay, K))
    n = r["root_table_entries"]; tb = r["root_table_allocated_bytes"]
    bpe = tb * 8 / n; pay = 32 if lay == "root4" else 40
    lay_tab.append((lay, K, n, tb, bpe, pay))
for lay, K, n, tb, bpe, pay in lay_tab:
    if K in (3300, 3350, 3425, 3450, 3500, 3550, 3596):
        P("| %s | %d | %d | %.3f | %.2f | %d | %.3f | %.1f | %.1f | %.1f |" % (
            lay, K, n, tb / GiB, bpe, pay, bpe / pay, 1.23 * pay, 1.005 * pay, 1.10 * pay))
out["layout_table"] = lay_tab

# ------------------------------------------------ 2. memory model calibration
P("\n## 2. Memory model: peak_rss = 11*S + table + bloom + residual\n")
res = {}
for r in rows:
    lay = layout(r)
    if lay not in ("root4", "root11"): continue
    bl = r.get("root_bloom_bytes") or 0
    pred = r["root_state_allocated_bytes"] + r["root_table_allocated_bytes"] + bl
    res.setdefault(lay, []).append(r["peak_rss_bytes"] - pred)
for lay, v in res.items():
    P("- %s: residual (peak - states - table - bloom) over %d rows: min %.3f, median %.3f, max %.3f GB" % (
        lay, len(v), min(v) / 1e9, st.median(v) / 1e9, max(v) / 1e9))
RESID = max(res["root4"])
P("- constant used for modelled layouts: max root4 residual %.3f GB (root11 max %.3f GB comes from rows with no Bloom allocation field, not used)" % (RESID / 1e9, max(res["root11"]) / 1e9))
out["resid_gb"] = RESID / 1e9

# entries per state (two roots per state, less duplicates)
ratio = st.median(r["root_table_entries"] / S(r["effective_columns"]) for r in rows if layout(r) in ("root4", "root11"))
P("- entries per state (median over rows): %.6f" % ratio)
E = lambda K: ratio * S(K)

# guards
guards = sorted({r["rss_guard_bytes"] for r in rows if r.get("rss_guard_bytes")})
P("- rss_guard_bytes values in rows: %s" % guards)
G_hi = max(guards); G_lo = 14 * GiB
P("- guards used: %.2f GiB (frontier-run guard) and 14.00 GiB (root11 guard)" % (G_hi / GiB))

# ------------------------------------------------ 3. K-vs-cost fits
P("\n## 3. K versus cost, from the repository's own rows\n")
pts = []
for r in rows:
    tp = r.get("target_probes")
    if tp and r["effective_columns"] >= 2600:
        pts.append((math.log(r["effective_columns"]), math.log(sum(tp) / len(tp))))
n = len(pts); mx = sum(p[0] for p in pts) / n; my = sum(p[1] for p in pts) / n
b = sum((x - mx) * (y - my) for x, y in pts) / sum((x - mx) ** 2 for x, _ in pts)
a = my - b * mx
ss_res = sum((y - (a + b * x)) ** 2 for x, y in pts); ss_tot = sum((y - my) ** 2 for _, y in pts)
R2 = 1 - ss_res / ss_tot; sd = math.sqrt(ss_res / (n - 2))
P("- probes/target ~ K^b: n=%d rows, b=%.3f, R^2=%.3f, residual sd=%.3f (ln units)" % (n, b, R2, sd))
probes = lambda K: math.exp(a + b * math.log(K))
P("- fit probes at K3300/K3596: %.0f / %.0f" % (probes(3300), probes(3596)))
out["probe_fit"] = {"b": b, "a": a, "R2": R2, "sd": sd, "n": n}

fr = [r for r in rows if r["_src"] == "n83_root4_memory_frontier_results_20261015"]
anch = {}
for r in fr:
    anch.setdefault(r["effective_columns"], []).append(r)
A = anch[3596]
on0 = st.mean(r["ic_online_mean_ms_exploratory"] for r in A)
setup0 = st.mean(r["setup_ms"] for r in A) / 1e3
rank0 = st.mean(r["rank_ms"] for r in A) / 1e3
P("- anchor (same-host six-arm frontier run, K3596 root4): online %.3f ms, setup %.1f s, rank %.1f s, peak RSS %.3f GiB" % (
    on0, setup0, rank0, st.mean(r["peak_rss_bytes"] for r in A) / GiB))
rk = [(math.log(k), math.log(st.mean(x["rank_ms"] for x in v))) for k, v in anch.items()]
bx = sum(p[0] for p in rk) / 3; by = sum(p[1] for p in rk) / 3
rb = sum((x - bx) * (y - by) for x, y in rk) / sum((x - bx) ** 2 for x, _ in rk)
P("- rank time ~ K^%.2f (3 K points, frontier run); setup scaled linearly in S(K) (K1107 row is sub-linear: model is pessimistic for setup)" % rb)
setup = lambda K: setup0 * S(K) / S(3596)
rank = lambda K: rank0 * (K / 3596) ** rb
online = lambda K, kap=1.0: on0 * probes(K) / probes(3596) * kap
ms_per_probe = on0 / (st.mean(sum(r["target_probes"]) / len(r["target_probes"]) for r in A))
P("- ms per probe at anchor: %.4f us" % (ms_per_probe * 1e3))

# rho references (cited files)
def grab(path, pat):
    t = open(os.path.join(RES, path)).read()
    mm = re.search(pat, t); assert mm, (path, pat)
    return float(mm.group(1).replace(",", ""))
rho_geo = grab("n83_cold_base_confirm_20261021/RESULT.md", r"rho online geometric mean across the 16 points was ([\d,\.]+) ms")
rho_mean = grab("n83_tier1_combined_results_20261015/RESULT.md", r"mean rho online\s+interval is ([\d\.]+) ms")
P("- matched same-point rho (wall clock): geometric mean %.3f ms (16 pts, cold_base_confirm); mean %.3f ms (5 pts, tier1_combined)" % (rho_geo, rho_mean))
out["rho_ms"] = {"geo16": rho_geo, "mean5": rho_mean}

# ------------------------------------------------ 4. scenarios
def fbits(K, extra=0):
    return math.ceil(math.log2(2 * S(K) + 2)) + extra   # state id + selector (+fingerprint)
def bloom_std(K):                                                 # baseline: pow2 >= 1.25*S bytes (8 bits/request)
    return 2 ** math.ceil(math.log2(1.25 * S(K)))
def table_pow2(K):                                                # baseline root4: 2^31 slots (<=2^30 entries) else 2^32 slots
    return 4 * (2 ** 31 if E(K) <= 2 ** 30 else 2 ** 32)

SC = {}
SC["base root4 (measured layout)"] = lambda K: 11 * S(K) + table_pow2(K) + bloom_std(K) + RESID
for nm, over in (("xor 1.23", 1.23), ("ribbon 1.10", 1.10), ("ribbon 1.05", 1.05), ("ribbon 1.005", 1.005)):
    SC["%s, f=id+sel, Bloom kept" % nm] = (lambda o: lambda K: 11 * S(K) + E(K) * fbits(K) * o / 8 + bloom_std(K) + RESID)(over)
    SC["%s, f=id+sel+8 fp, no Bloom" % nm] = (lambda o: lambda K: 11 * S(K) + E(K) * fbits(K, 8) * o / 8 + RESID)(over)

def kmax(fn, G):
    lo, hi = 1000, 20000
    assert fn(lo) <= G
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if fn(mid) <= G: lo = mid
        else: hi = mid - 1
    return lo

P("\n## 4. Largest K inside the same RSS guard (modelled; memory only)\n")
P("check: model on measured K3596 root4: pred %.3f GiB vs measured %.3f GiB" % (
    SC["base root4 (measured layout)"](3596) / GiB, st.mean(r["peak_rss_bytes"] for r in A) / GiB))
P("\n| scenario | Kmax @%.2f GiB | S(Kmax) | Kmax @14 GiB | vs base K@%.2f |" % (G_hi / GiB, G_hi / GiB))
P("|---|---|---|---|---|")
kb = kmax(SC["base root4 (measured layout)"], G_hi)
res4 = {}
for nm, fn in SC.items():
    k1 = kmax(fn, G_hi); k2 = kmax(fn, G_lo); res4[nm] = (k1, k2)
    P("| %s | %d | %d | %d | %+d |" % (nm, k1, S(k1), k2, k1 - kb))
out["kmax"] = res4

# per-K memory pieces at K3596 for a table of where bytes go
P("\nbytes at K3596 (GB): states %.2f, root4 table %.2f, Bloom %.2f, resid %.2f; ribbon 1.05 f=32 table would be %.2f; xor 1.23: %.2f" % (
    11 * S(3596) / 1e9, table_pow2(3596) / 1e9, bloom_std(3596) / 1e9, RESID / 1e9,
    E(3596) * 32 * 1.05 / 8 / 1e9, E(3596) * 32 * 1.23 / 8 / 1e9))

# ------------------------------------------------ 5. cost prediction
P("\n## 5. Predicted online / cold cost at Kmax (model: repo K-law, same-host anchor)\n")
P("online(K) = %.3f ms * probes_fit(K)/probes_fit(3596) * kappa; kappa = per-probe cost multiplier" % on0)
P("(kappa=1 optimistic: a ribbon/xor probe costs what a Bloom+table probe cost; kappa 1.5/2 pessimistic).")
P("cold = setup + rank + online; rho = %.3f ms (16-pt geometric mean). N* = setup+rank over (rho-online)\n" % rho_geo)
hdr = "| scenario | K | probes | online ms k=1 | k=1.5 | k=2 | setup s | cold s | cold/rho | N* targets to beat rho |"
P(hdr); P("|" + "---|" * 10)
cost_rows = []
def cost_line(nm, K):
    p = probes(K)
    o = [online(K, kk) for kk in (1, 1.5, 2)]
    su, rkk = setup(K), rank(K)
    cold = su + rkk + o[0] / 1e3
    nstar = (su + rkk) / ((rho_geo - o[0]) / 1e3)
    cost_rows.append(dict(scenario=nm, K=K, probes=p, online_ms=o, setup_s=su, rank_s=rkk, cold_s=cold,
                          cold_over_rho=cold / (rho_geo / 1e3), nstar=nstar))
    P("| %s | %d | %.0f | %.2f | %.2f | %.2f | %.0f | %.0f | %.0fx | %.0f |" % (
        nm, K, p, o[0], o[1], o[2], su, cold, cold / (rho_geo / 1e3), nstar))
cost_line("MEASURED K3596 root4 (anchor, fit-normalised)", 3596)
for nm in ("xor 1.23, f=id+sel, Bloom kept", "ribbon 1.05, f=id+sel, Bloom kept",
           "ribbon 1.005, f=id+sel+8 fp, no Bloom", "xor 1.23, f=id+sel+8 fp, no Bloom"):
    cost_line(nm, res4[nm][0])
# measured anchor with real numbers
meas_cold = setup0 + rank0 + on0 / 1e3
P("\nmeasured anchor cold = %.1f + %.1f + %.4f = %.1f s; cold/rho(geo) = %.0fx; cold/rho(mean5) = %.0fx" % (
    setup0, rank0, on0 / 1e3, meas_cold, meas_cold / (rho_geo / 1e3), meas_cold / (rho_mean / 1e3)))
P("measured online gap: rho/IC = %.1fx (geo) at K3596" % (rho_geo / on0))
out["cost_rows"] = cost_rows
out["anchor"] = dict(online_ms=on0, setup_s=setup0, rank_s=rank0, cold_s=meas_cold)

# sensitivity: setup exponent 0.8
P("\nsetup-exponent sensitivity (setup ~ S^0.8 instead of S^1) for the ribbon 1.005 + 8 fp row:")
Kr = res4["ribbon 1.005, f=id+sel+8 fp, no Bloom"][0]
su8 = setup0 * (S(Kr) / S(3596)) ** 0.8
P("  K=%d setup %.0f s, cold %.0f s, cold/rho %.0fx" % (Kr, su8, su8 + rank(Kr) + online(Kr) / 1e3,
    (su8 + rank(Kr) + online(Kr) / 1e3) / (rho_geo / 1e3)))

# ------------------------------------------------ 6. false-positive verification
P("\n## 6. False-positive verification cost\n")
mbits = 8 * bloom_std(3596)
def bloomfpr(m, n, k=3): return (1 - math.exp(-k * n / m)) ** k
n_e = E(3596); n_s = S(3596)
P("baseline Bloom K3596: %.0f bits; k=3; FPR inserting all roots (n=entries): %.4f; one key per state (n=states): %.4f" % (
    mbits, bloomfpr(mbits, n_e), bloomfpr(mbits, n_s)))
pr = probes(3596)
c_probe_us = ms_per_probe * 1e3
for nm, fpr in (("Bloom k=3 (n=entries)", bloomfpr(mbits, n_e)), ("Bloom k=3 (n=states)", bloomfpr(mbits, n_s)),
                ("xor8 (2^-8)", 2 ** -8), ("fp r=8 in ribbon (2^-8)", 2 ** -8), ("fp r=12", 2 ** -12)):
    nfp = pr * fpr
    P("- %-24s FPR %.5f -> %.0f false positives per target; upper bound if each costs a full probe (%.3f us): %.3f ms = %.2f%% of %.2f ms online" % (
        nm, fpr, nfp, c_probe_us, nfp * c_probe_us / 1e3, 100 * nfp * c_probe_us / 1e3 / on0, on0))
P("- with NO prefilter and no fingerprint (retrieval value only): every probe verifies: %.0f state reads/target" % pr)
P("  at 0.1 us per DRAM-missing state read (assumed) = %.2f ms = %.1f%% of anchor online; at the anchor probe cost it is the whole online cost" % (pr * 0.1e-3, 100 * pr * 0.1e-3 / on0))
out["fpr_bloom_entries"] = bloomfpr(mbits, n_e); out["fpr_bloom_states"] = bloomfpr(mbits, n_s)

# ------------------------------------------------ 7. build-memory caveat
P("\n## 7. Construction memory (not in the RSS model above)\n")
for K in (3596, res4["ribbon 1.05, f=id+sel, Bloom kept"][0]):
    P("K=%d: key-hash buffer 8 B x entries = %.2f GB; ribbon banded rows 12 B x rows(1.05 n) = %.2f GB; xor peel (8 B key + 1 B count)x1.23n = %.2f GB" % (
        K, 8 * E(K) / 1e9, 12 * 1.05 * E(K) / 1e9, 9 * 1.23 * E(K) / 1e9))
P("If built unsharded these exceed the final table; sharded/streamed builds are unmeasured here.")

# ------------------------------------------------ 8. counted GAE
P("\n## 8. Which lever is this\n")
P("Counted S (group operations): a larger K needs 166K usable base points (more group operations to generate),")
P("so this lever does not reduce counted GAE at w=0; it moves only memory and native lookups (w>0). [pcc:unverified]")

if len(sys.argv) > 2 and sys.argv[1] == "--json":
    json.dump(out, open(sys.argv[2], "w"), indent=1, default=str)
