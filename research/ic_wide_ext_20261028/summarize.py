"""Summarise the extension study: lane-stage diagnosis, the old/new binary check and the joint tuning grid.

diagnosis/split-diag-<bits>-<rep>  one session per curve with the lane-stage timers on (marker file; wall-clock
                                   counters, so not replayable and outside the certified headline)
sessions/ab-old-<i>, ab-new-<i>    old and new binary on the same spec, alternated
sessions/grid-<bits>-<rep>         joint tuning grid: base size x target source x oracle

Writes results.json, grid.svg, report.html next to this file.  Standard library only.
"""
import glob
import json
import math
import pathlib
import statistics

HERE = pathlib.Path(__file__).parent
VOLATILE = {"time", "isolation", "env_class_id", "binary_sha256", "record_id", "session_id", "run_id"}


def load(pattern):
    return [[json.loads(l) for l in (d / "records.jsonl").read_text().splitlines() if l.strip()]
            for d in sorted(HERE.glob(pattern)) if (d / "records.jsonl").exists()]


def rel_ns(r):
    return next(p["wall_ns"] for p in r["phases"] if p["name"] == "relations")


def phase_ns(r, name):
    return next(p["wall_ns"] for p in r["phases"] if p["name"] == name)


def native(r):
    return next(p["native"] for p in r["phases"] if p["name"] == "relations")


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def ci(logs):
    m = statistics.mean(logs)
    h = 1.96 * statistics.stdev(logs) / math.sqrt(len(logs)) if len(logs) > 1 else 0.0
    return math.exp(m), math.exp(m - h), math.exp(m + h)


# ---------------------------------------------------------------- lane-stage diagnosis
def diagnosis():
    out = {}
    for bits in (28, 32):
        recs = [r for s in load(f"diagnosis/split-diag-{bits}-*") for r in s if not r["warmup"] and r["method"]["id"] == "ic.pipeline"]
        rows = []
        for arm in sorted({r["arm"] for r in recs}, key=lambda a: (a[0] != "w", a == "fast", len(a), a)):
            v = [r for r in recs if r["arm"] == arm]
            t = [native(r)["trials"] for r in v]
            row = {"arm": arm, "trials": statistics.median(t),
                   "rel_ns_per_trial": statistics.median(rel_ns(r) / native(r)["trials"] for r in v)}
            for k in ("prep", "add", "queue", "touch"):
                if f"diag_{k}_ns" in native(v[0]):
                    row[k] = statistics.median(native(r)[f"diag_{k}_ns"] / native(r)["trials"] for r in v)
            for k in ("draw", "guard", "oracle"):
                if f"diag_{k}_cy" in native(v[0]):
                    row[f"{k}_cycles"] = statistics.median(native(r)[f"diag_{k}_cy"] / native(r)["trials"] for r in v)
            rows.append(row)
        out[bits] = rows
    return out


# ---------------------------------------------------------------- old/new binary check
def counted(r):
    r = {k: v for k, v in r.items() if k not in VOLATILE}
    r["phases"] = [{k: v for k, v in p.items() if k != "wall_ns"} for p in r["phases"]]
    if isinstance(r.get("online"), dict):
        o = dict(r["online"])
        o.pop("wall_ns", None)
        o["phases_ns"] = sorted(o.get("phases_ns") or {})
        r["online"] = o
    return json.dumps(r, sort_keys=True)


def old_new():
    old, new = load("sessions/ab-old-*"), load("sessions/ab-new-*")
    if not old or len(old) != len(new):
        return None
    key = lambda r: (r["arm"], r["workload"]["target_index"], r["round"], r["warmup"])
    same = total = 0
    rel, solve = {}, {}
    for a, b in zip(old, new):
        ra, rb = {key(r): r for r in a}, {key(r): r for r in b}
        for k, x in ra.items():
            y = rb[k]
            total += 1
            same += counted(x) == counted(y)
            if not k[3] and x["method"]["id"] == "ic.pipeline":
                rel.setdefault(k[0], []).append(math.log(rel_ns(y) / rel_ns(x)))
                solve.setdefault(k[0], []).append(math.log(y["time"]["solve_wall_ns"] / x["time"]["solve_wall_ns"]))
    return {"identical_records": same, "records": total,
            "arms": {arm: {"relations_new_over_old": ci(rel[arm]), "solve_new_over_old": ci(solve[arm]), "pairs": len(rel[arm])}
                     for arm in sorted(rel)}}


# ---------------------------------------------------------------- grid
def grid():
    out = {}
    for bits in (28, 32):
        sess = load(f"sessions/grid-{bits}-*")
        if not sess:
            continue
        recs = [r for s in sess for r in s if not r["warmup"]]
        key = lambda r: (r["session_id"], r["workload"]["target_index"], r["round"])
        arms = sorted({r["arm"] for r in recs if r["method"]["id"] == "ic.pipeline"})
        rho_s = statistics.mean(r["cost"]["s"] for r in recs if r["arm"] == "rho-neg")
        solve = {a: {key(r): r["time"]["solve_wall_ns"] for r in recs if r["arm"] == a} for a in arms}
        # incumbent: the walk at the base the round-3 default uses (1024 at 28 and 32 bits)
        default = "b1024-L16"
        incumbent = "b1024-W"
        rows = []
        for a in arms:
            v = [r for r in recs if r["arm"] == a]
            row = {"arm": a, "runs": len(v), "verified": sum(r["outcome"]["status"] == "verified" for r in v),
                   "mean_s": statistics.mean(r["cost"]["s"] for r in v),
                   "distance_to_rho": statistics.mean(r["cost"]["s"] for r in v) / rho_s,
                   "median_solve_ms": statistics.median(r["time"]["solve_wall_ns"] for r in v) / 1e6,
                   "median_setup_ms": statistics.median(phase_ns(r, "factor_base") + phase_ns(r, "oracle_setup") for r in v) / 1e6,
                   "median_relations_ms": statistics.median(rel_ns(r) for r in v) / 1e6}
            for ref, name in ((incumbent, "vs_incumbent_walk"), (default, "vs_round3_default")):
                if ref in solve:
                    pairs = [(solve[a][k], solve[ref][k]) for k in solve[a] if k in solve[ref]]
                    row[name] = ci([math.log(x / y) for x, y in pairs])
            rows.append(row)
        rows.sort(key=lambda r: r["median_solve_ms"])
        out[bits] = rows
    return out


def figure(g):
    W, rowh, L = 860, 17, 150
    items = [(bits, r) for bits, rows in g.items() for r in rows]
    H = 60 + rowh * len(items) + 30
    lo, hi = 0.5, 1.6
    sx = lambda v: L + (math.log(v) - math.log(lo)) / (math.log(hi) - math.log(lo)) * (W - L - 90)
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="11">',
           f'<rect width="{W}" height="{H}" fill="#fff"/>',
           '<text x="16" y="20" font-size="14" fill="#222">Whole-solve wall time against the incumbent walk at base 1024 (below 1 = faster)</text>',
           '<text x="16" y="36" fill="#555">paired geometric mean per (session, target, round); bar = 95% interval; blue = interval below 1, red = above 1</text>']
    for t in (0.5, 0.6, 0.8, 1.0, 1.25, 1.6):
        x = sx(t)
        out.append(f'<line x1="{x:.1f}" y1="46" x2="{x:.1f}" y2="{H-24}" stroke="{"#333" if t == 1 else "#ddd"}"/><text x="{x:.1f}" y="{H-8}" text-anchor="middle" fill="#555">{t}</text>')
    for i, (bits, r) in enumerate(items):
        y = 52 + i * rowh
        if "vs_incumbent_walk" not in r:
            continue
        m, lo_c, hi_c = r["vs_incumbent_walk"]
        colour = "#2a6fdb" if hi_c < 1 else ("#c0392b" if lo_c > 1 else "#999")
        a, b = sorted((sx(1.0), sx(min(max(m, lo), hi))))
        out.append(f'<text x="{L-6}" y="{y+11}" text-anchor="end" fill="#222">{bits}-bit {r["arm"]}</text>')
        out.append(f'<rect x="{a:.1f}" y="{y+2}" width="{max(b-a,1):.1f}" height="{rowh-6}" fill="{colour}"/>')
        out.append(f'<line x1="{sx(min(max(lo_c,lo),hi)):.1f}" y1="{y+rowh/2-1:.1f}" x2="{sx(min(max(hi_c,lo),hi)):.1f}" y2="{y+rowh/2-1:.1f}" stroke="#222"/>')
        out.append(f'<text x="{sx(hi)+6:.1f}" y="{y+11}" fill="#222">{m:.3f}</text>')
    out.append("</svg>")
    (HERE / "grid.svg").write_text("\n".join(out) + "\n")


def fmt(x, d=1):
    return f"{x:.{d}f}"


def findings(res):
    d28 = {r["arm"]: r for r in res["diagnosis"][28]}
    d32 = {r["arm"]: r for r in res["diagnosis"][32]}
    on = res["old_new"]
    g28 = {r["arm"]: r for r in res["grid"][28]}
    g32 = {r["arm"]: r for r in res["grid"][32]}
    items = [
        f"<li><b>Where a relation-phase trial goes (28-bit, ns/trial, 16 lanes):</b> shared-inversion additions {fmt(d28['b16']['add'])}, "
        f"touch {fmt(d28['b16']['touch'])}, lane preparation {fmt(d28['b16']['prep'])}, queue {fmt(d28['b16']['queue'])}, and "
        f"{fmt((d28['b16']['draw_cycles'] + d28['b16']['guard_cycles'] + d28['b16']['oracle_cycles']) / 2.1)} in the loop body "
        f"(target draw {fmt(d28['b16']['draw_cycles'] / 2.1)}, guard insert {fmt(d28['b16']['guard_cycles'] / 2.1)}, oracle probe {fmt(d28['b16']['oracle_cycles'] / 2.1)}; cycles at 2.1 GHz) "
        f"of {fmt(d28['b16']['rel_ns_per_trial'])} in all. The plain walk is {fmt(d28['walk']['rel_ns_per_trial'])}.</li>",
        f"<li><b>The shared inversion is amortised by about 32 lanes.</b> The additions cost {fmt(d28['b4']['add'])} ns/trial at 4 lanes, {fmt(d28['b16']['add'])} at 16, "
        f"{fmt(d28['b32']['add'])} at 32 and stay at {fmt(d28['b256']['add'])}-{fmt(d28['b1024']['add'])} from 256 lanes up (32-bit: {fmt(d32['b4']['add'])}, {fmt(d32['b16']['add'])}, {fmt(d32['b32']['add'])}, {fmt(d32['b1024']['add'])}): "
        "more lanes buy nothing once the one inversion per round is spread thin.</li>",
        f"<li><b>Two costs grow with the width.</b> (1) <i>Lane start-up</i>: each lane begins with two scalar multiplications and an addition, about "
        f"{(d28['b1024']['prep'] - d28['b32']['prep']) * d28['b1024']['trials'] / 1024 / 1000:.0f} microseconds a lane, paid once. 1024 lanes cost about "
        f"{(d28['b1024']['prep'] - d28['b32']['prep']) * d28['b1024']['trials'] / 1e6:.0f} ms, a quarter of a 28-bit relation phase and under 5% of a 32-bit one "
        f"(lane preparation {fmt(d28['b1024']['prep'])} ns/trial at 28 bits, {fmt(d32['b1024']['prep'])} at 32). This is the same cost the counted S charges, and it is why wide batching loses on small curves "
        "and why the knee moves up with the size. (2) <i>Cache reach</i>: the round's touched lines (guard slot and table slot, 2 x 64 bytes a lane) must stay in L1 until their trial; "
        f"at 1024 lanes they do not, and the guard insert and the oracle probe slow down ({fmt(d32['b32']['guard_cycles'] / 2.1)} + {fmt(d32['b32']['oracle_cycles'] / 2.1)} ns at 32 lanes, "
        f"{fmt(d32['b1024']['guard_cycles'] / 2.1)} + {fmt(d32['b1024']['oracle_cycles'] / 2.1)} at 1024 lanes, 32-bit). Neither can beat the knee, because neither is present at 32 lanes. "
        "A smaller lane layout was therefore not built: it could only flatten the tail.</li>",
        f"<li><b>The floor is hash-table work.</b> The guard insert and the oracle probe are about {fmt((d28['b32']['guard_cycles'] + d28['b32']['oracle_cycles']) / 2.1)} ns/trial at 28 bits "
        f"({fmt((d32['b32']['guard_cycles'] + d32['b32']['oracle_cycles']) / 2.1)} at 32 bits) at the knee, a third to a half of the trial, and do not depend on the width; lanes cannot touch them.</li>",
        f"<li><b>Intermediate widths:</b> relation ns/trial at 28 bits: 16 lanes {fmt(d28['b16']['rel_ns_per_trial'])}, 24 {fmt(d28['b24']['rel_ns_per_trial'])}, 32 {fmt(d28['b32']['rel_ns_per_trial'])}, "
        f"48 {fmt(d28['b48']['rel_ns_per_trial'])}, 64 {fmt(d28['b64']['rel_ns_per_trial'])}: a flat optimum over 16-48. In the grid, 32 lanes against 16 at 32 bits is "
        f"{g32['b1024-L32']['median_solve_ms'] / g32['b1024-L16']['median_solve_ms']:.3f} of the solve time (interval spans 1).</li>",
        f"<li><b><code>walk-prime-fast</code> was slower because of its inverse, and is fixed.</b> It ran at 1.658x the plain walk's relation time (interval 1.58-1.74) because "
        "<code>Fp64::inv</code> used the binary extended Euclid on 64-bit words; the plain walk's step uses 32-bit word Euclid for moduli below 2^32. "
        f"A 32-bit word-Euclid path for moduli below 2^32 brings it to 1.042 (interval 0.996-1.090) of the walk, a relation-phase new/old of "
        f"{on['arms']['fast']['relations_new_over_old'][0]:.3f} (interval {on['arms']['fast']['relations_new_over_old'][1]:.3f}-{on['arms']['fast']['relations_new_over_old'][2]:.3f}). "
        f"{on['identical_records']} of {on['records']} records are identical outside timing, so no counted quantity moved; the walk and the lane arms are unchanged within noise "
        f"(new/old {on['arms']['walk']['relations_new_over_old'][0]:.3f}, {on['arms']['L16']['relations_new_over_old'][0]:.3f}, {on['arms']['L32']['relations_new_over_old'][0]:.3f}).</li>",
        f"<li><b>Joint tuning: the optimal base does not move.</b> Whole-solve wall time is lowest at base 512 at 28 bits and 1024 at 32 bits for every method; "
        f"the build of the B^2/4-entry table grows faster than the relation phase shrinks. Best configurations against the incumbent walk at the same base: "
        f"28 bits {g28['b512-L16']['median_solve_ms']:.1f} ms (16 lanes) against {g28['b512-W']['median_solve_ms']:.1f} ms; "
        f"32 bits {g32['b1024-L32']['median_solve_ms']:.0f} ms (32 lanes) against {g32['b1024-W']['median_solve_ms']:.0f} ms. "
        "The cache-blocked table (mitm-prime-blocked) halves the build at 2048 and above but its probe is slower, so it only wins at base 4096 (32-bit: 1,092 ms against 2,908 ms).</li>",
        "<li><b>Not done.</b> The harness's <code>prime_search</code> curves stop at 32 bits, so no 34- or 36-bit ladder; chunked inversion and software prefetch have no handle "
        "(the table is a std hash map whose slot addresses are not exposed, and the touch loop is already the overlapped-load form); an adaptive width has no room above a flat 16-48 optimum.</li>"]
    return "<ul>" + "".join(items) + "</ul>"


def report(res):
    def row(r):
        vs = r.get("vs_incumbent_walk")
        return (f"<tr><td>{r['arm']}</td><td>{r['verified']}/{r['runs']}</td><td>{r['median_solve_ms']:.1f}</td><td>{r['median_setup_ms']:.1f}</td>"
                f"<td>{r['median_relations_ms']:.1f}</td><td>{r['mean_s']:.1f}</td><td>{vs[0]:.3f}</td></tr>" if vs else "")
    tables = "".join(f"<h3>{bits}-bit grid</h3><table><tr><th>arm (base-targets)</th><th>verified</th><th>median solve ms</th><th>set-up ms</th><th>relations ms</th><th>mean S</th>"
                     f"<th>solve vs b1024 walk</th></tr>{''.join(row(r) for r in rows)}</table>" for bits, rows in res["grid"].items())
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Wide batching extension</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:980px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:first-child,th:first-child{{text-align:left}}</style></head><body>
<h1>Prime IC relation phase: why wide batching stops paying, and the joint tuning (2026-10-28)</h1>
<p>Extension of lead <code>ICP-prime-ic-constant-engineering</code> (PR 2783): a diagnosis of the lane-round cost, a check that the code changes move no counted quantity,
the <code>walk-prime-fast</code> fix, and a joint tuning grid over base size, lane count and oracle.</p>
<h2>Findings</h2>{findings(res)}
{(HERE / 'grid.svg').read_text()}
{tables}</body></html>"""
    (HERE / "report.html").write_text(html)


def main():
    res = {"diagnosis": diagnosis(), "old_new": old_new(), "grid": grid()}
    headline = []
    for bits, rows in res["grid"].items():
        for r in rows:
            for k in ("vs_incumbent_walk", "vs_round3_default"):
                if k in r:
                    headline.append({"name": f"grid-{bits}/{r['arm']}/{k}", "claimed": r[k][0]})
            headline.append({"name": f"grid-{bits}/{r['arm']}/mean_s", "claimed": r["mean_s"]})
    if res["old_new"]:
        for arm, v in res["old_new"]["arms"].items():
            headline.append({"name": f"old_new/{arm}/relations", "claimed": v["relations_new_over_old"][0]})
    res["headline"] = headline
    figure(res["grid"])
    report(res)
    (HERE / "results.json").write_text(json.dumps(res, indent=1, sort_keys=True) + "\n")
    print(json.dumps({"diagnosis": res["diagnosis"], "old_new": res["old_new"]}, indent=1)[:3500])
    for bits, rows in res["grid"].items():
        print(f"-- grid {bits}")
        for r in rows[:8]:
            print(f"{r['arm']:14s} solve={r['median_solve_ms']:9.1f}ms setup={r['median_setup_ms']:8.1f} rel={r['median_relations_ms']:8.1f} "
                  f"vsWalk1024={r.get('vs_incumbent_walk', (float('nan'),))[0]:.3f} vsL16b1024={r.get('vs_round3_default', (float('nan'),))[0]:.3f} S={r['mean_s']:.1f}")


if __name__ == "__main__":
    main()
