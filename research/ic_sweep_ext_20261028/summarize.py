"""Summarise the extended residual-sweep study (lead ICP-residual-partial-relations).

sessions/ext-<bits>-<rep>/   three sessions per curve size (20..44 bits), every arm alternated in each

Per (size, arm): runs verified, mean S with its standard error, memory in table entries as the
arm held them (residual table, baby-step table, distinguished-point table, pair table; rho keeps
none), and the paired geometric mean of S against the planner-tuned pair-table incumbent over
the same (session, target, round).  Then:

  * exponent fits of log2 S against curve bits, bootstrap 95% intervals (runs resampled within
    each size), for the sweep at fixed B, the incumbent, BSGS, kangaroo and rho;
  * the slope of log2 S against log2 B at each size and the constant c in S = c * sqrt(2 B);
  * the birthday model check: collision rows found against probed^2 / r;
  * the time-memory trade of the distinguished-point variants;
  * the base-shape controls (smallest abscissae, strided null, known-log progression, known-log random).

Writes results.json, three SVG figures and report.html next to this file.  Standard library only.
"""
import json
import math
import pathlib
import random
import statistics

HERE = pathlib.Path(__file__).parent
SIZES = list(range(20, 46, 2))
SWEEP_B = [2, 4, 8, 16, 32, 64, 128]
# Compact encodings assumed for the byte column (the implementation uses a wider HashMap slot):
# residual (key 8, a 8, b 8, index+sign 4 -> 32), BSGS baby step (key 8, index 8 -> 16),
# kangaroo distinguished point (x 8, distance 8, kind 8 -> 24), pair-table slot (key 8, pair 8 -> 16).
ENTRY_BYTES = {"sweep": 32, "variant": 32, "bsgs": 16, "kangaroo": 24, "pair": 16, "rho": 0}


def load(bits):
    return [[json.loads(l) for l in (d / "records.jsonl").read_text().splitlines() if l.strip()]
            for d in sorted(HERE.glob(f"sessions/ext-{bits}-*")) if (d / "records.jsonl").exists()]


def phase_native(r, key):
    for p in r["phases"]:
        v = (p.get("native") or {}).get(key)
        if v is not None:
            return v
    return None


def family(arm):
    if arm.startswith("sweep-b"):
        return "sweep"
    if arm.startswith("pair-"):
        return "pair"
    return {"bsgs-neg": "bsgs", "kangaroo": "kangaroo"}.get(arm, "rho" if arm.startswith("rho") else "variant")


def entries(r, arm):
    fam = family(arm)
    if fam == "rho":
        return 0
    if fam == "bsgs":
        return r["counters"].get("baby_steps", 0)
    if fam == "kangaroo":
        return r["counters"].get("inserts_uncharged", 0)
    if fam == "pair":
        for p in r["phases"]:
            v = (p.get("work_native") or {}).get("table_insert_attempts")
            if v:
                return v
        return 0
    return phase_native(r, "residuals_stored") or 0


def base_size(r):
    params = r["method"].get("params") or {}
    fb = params.get("factor_base", "")
    return int(fb.split("size=")[1].split(",")[0]) if "size=" in fb else None


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def slope(xs, ys):
    mx, my = statistics.mean(xs), statistics.mean(ys)
    d = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / d if d else float("nan")


def boot_slope(points, reps=1500, seed=7):
    """points: {x: [S values]}; slope of log2 mean(S) on x with a 95% interval, runs resampled within x."""
    xs = sorted(points)
    if len(xs) < 3:
        return None
    rng = random.Random(seed)
    est = slope(xs, [math.log2(statistics.mean(points[x])) for x in xs])
    draws = sorted(slope(xs, [math.log2(statistics.mean(rng.choices(points[x], k=len(points[x])))) for x in xs])
                   for _ in range(reps))
    return {"slope": est, "lo": draws[int(0.025 * reps)], "hi": draws[int(0.975 * reps)], "sizes": len(xs)}


def collect():
    rows, runs = [], {}
    for bits in SIZES:
        sess = load(bits)
        allrecs = [r for s in sess for r in s if not r["warmup"]]
        recs = [r for r in allrecs if r["outcome"]["status"] == "verified"]
        if not recs:
            continue
        key = lambda r: (r["session_id"], r["workload"]["target_index"], r["round"])
        arms = list(dict.fromkeys(r["arm"] for r in allrecs))
        inc_arm = next((a for a in arms if a.startswith("pair-walk-b")), None)
        inc = {key(r): r for r in recs if r["arm"] == inc_arm} if inc_arm else {}
        rho_s = statistics.mean(r["cost"]["s"] for r in recs if r["arm"] == "rho-neg")
        log2_r = math.log2(recs[0]["workload"]["curve"]["r"])
        for arm in arms:
            v = [r for r in recs if r["arm"] == arm]
            if not v:
                continue
            ss = [r["cost"]["s"] for r in v]
            ent = statistics.mean(entries(r, arm) for r in v)
            fam = family(arm)
            row = {"bits": bits, "log2_r": log2_r, "arm": arm, "family": fam, "method": v[0]["method"]["id"],
                   "runs": sum(1 for r in allrecs if r["arm"] == arm), "verified": len(v),
                   "mean_s": statistics.mean(ss),
                   "se_s": statistics.stdev(ss) / math.sqrt(len(ss)) if len(ss) > 1 else 0.0,
                   "distance_to_rho": statistics.mean(ss) / rho_s,
                   "entries": ent, "bytes": ent * ENTRY_BYTES[fam],
                   "base": base_size(v[0]) if v[0]["method"]["id"] == "ic.pipeline" else None}
            # time-memory: group additions times table bytes, per r (dimensionless)
            row["tm_per_r"] = row["mean_s"] * math.sqrt(2 ** log2_r) * row["bytes"] / 2 ** log2_r if row["bytes"] else 0.0
            if v[0]["method"]["id"] == "ic.pipeline":
                row["rows_used"] = statistics.mean(r["counters"].get("matrix_rows", 0) for r in v)
                row["rank_at_stop"] = statistics.mean(r["counters"].get("matrix_rank", 0) for r in v)
                row["dependent_rows"] = statistics.mean(r["counters"].get("matrix_dependent", 0) for r in v)
                row["s_over_sqrt_rows"] = row["mean_s"] / math.sqrt(row["rows_used"]) if row["rows_used"] else None
            if inc and arm != inc_arm:
                pairs = [(r, inc[key(r)]) for r in v if key(r) in inc]
                if pairs:
                    row["paired_s_vs_incumbent"] = geomean([r["cost"]["s"] / b["cost"]["s"] for r, b in pairs])
            if v[0]["method"]["id"] == "ic.pipeline" and phase_native(v[0], "sweeps") is not None and "oracle" in (v[0]["method"].get("params") or {}) \
                    and "residual-sweep" in v[0]["method"]["params"]["oracle"] and "pairs=1" not in v[0]["method"]["params"]["oracle"]:
                b = row["base"]
                chk = []
                for r in v:
                    params = r["method"]["params"]["oracle"]
                    dp = int(params.split("dp=")[1].split(",")[0]) if "dp=" in params else 0
                    fold = 1.0 if "fold=0" in params else 2.0       # classes: r (no fold) or r/2 (signed fold)
                    probed = phase_native(r, "sweeps") * 2 * b       # one residual per base point and sign
                    r_exact = r["workload"]["curve"]["r"]
                    # equal abscissae hash alike, so a pair collides only if it passes the filter: /2^dp
                    predicted = probed ** 2 * fold / (2 * r_exact) / 2 ** dp
                    found = (phase_native(r, "collisions") or 0) + (phase_native(r, "identity_hits") or 0)
                    chk.append((found, predicted))
                if chk:
                    row["collisions_observed"] = statistics.mean(c[0] for c in chk)
                    row["collisions_predicted"] = statistics.mean(c[1] for c in chk)
            rows.append(row)
            runs.setdefault((bits, arm), ss)
    return rows, runs


def fits(rows, runs):
    out = {"s_vs_bits": {}, "s_vs_base": {}, "c_const": {}}
    families = {"sweep B=2": "sweep-b2", "sweep B=8": "sweep-b8", "sweep B=32": "sweep-b32", "sweep B=128": "sweep-b128",
                "rho.negation": "rho-neg", "rho.negation_lanes": "rho-lanes16", "bsgs.negation": "bsgs-neg",
                "kangaroo": "kangaroo"}
    for name, arm in families.items():
        pts = {r["log2_r"]: runs[(r["bits"], arm)] for r in rows if r["arm"] == arm}
        f = boot_slope(pts)
        if f:
            out["s_vs_bits"][name] = f
    inc = {r["log2_r"]: runs[(r["bits"], r["arm"])] for r in rows if r["arm"].startswith("pair-walk-b")}
    f = boot_slope(inc)
    if f:
        out["s_vs_bits"]["pair-table incumbent (walk, base ~ r^(1/3))"] = f
    for bits in SIZES:
        pts = {math.log2(b): runs[(bits, f"sweep-b{b}")] for b in SWEEP_B if (bits, f"sweep-b{b}") in runs}
        f = boot_slope(pts)
        if f:
            out["s_vs_base"][bits] = f
        cs = [statistics.mean(runs[(bits, f"sweep-b{b}")]) / math.sqrt(2 * b) for b in SWEEP_B if (bits, f"sweep-b{b}") in runs]
        if cs:
            out["c_const"][bits] = {"min": min(cs), "max": max(cs), "mean": statistics.mean(cs)}
    return out


VARIANTS = [("b8-null-strided", "strided pseudo-random abscissae (null control)"), ("b8-nofold", "no sign fold"),
            ("b8-hybrid", "pair table added (hybrid)"), ("b8-klprog", "known-log base, progression"),
            ("b8-klrand", "known-log base, random scalars"), ("b8-dp2", "distinguished points, k = 2"),
            ("b8-dp4", "distinguished points, k = 4"), ("b8-dp6", "distinguished points, k = 6")]


def variant_comparisons(bits_list=range(20, 38, 2)):
    """Paired S ratio of each B = 8 variant to the plain B = 8 sweep, per size and pooled over sizes."""
    out = []
    for arm, label in VARIANTS:
        per, pooled = {}, []
        for bits in bits_list:
            sess = load(bits)
            recs = [r for s in sess for r in s if not r["warmup"] and r["outcome"]["status"] == "verified"]
            key = lambda r: (r["session_id"], r["workload"]["target_index"], r["round"])
            base = {key(r): r["cost"]["s"] for r in recs if r["arm"] == "sweep-b8"}
            logs = [math.log(r["cost"]["s"] / base[key(r)]) for r in recs if r["arm"] == arm and key(r) in base]
            if len(logs) < 3:
                continue
            m = statistics.mean(logs)
            se = statistics.stdev(logs) / math.sqrt(len(logs))
            per[bits] = {"ratio": math.exp(m), "lo": math.exp(m - 1.96 * se), "hi": math.exp(m + 1.96 * se), "n": len(logs)}
            pooled += logs
        if pooled:
            m = statistics.mean(pooled)
            se = statistics.stdev(pooled) / math.sqrt(len(pooled))
            out.append({"arm": arm, "label": label, "pooled_ratio": math.exp(m), "lo": math.exp(m - 1.96 * se),
                        "hi": math.exp(m + 1.96 * se), "n": len(pooled), "per_size": per})
    return out


def svg_lines(path, title, subtitle, series, xlabel, ylabel, xlog=True, W=820, H=460):
    """series: [(label, colour, [(x, y)], dashed)] -> log-log SVG."""
    L, R, T, B = 70, 190, 54, 56
    xs = [x for _, _, pts, _ in series for x, _ in pts]
    ys = [y for _, _, pts, _ in series for _, y in pts if y > 0]
    x0, x1 = (math.log2(min(xs)), math.log2(max(xs))) if xlog else (min(xs), max(xs))
    y0, y1 = math.floor(math.log2(min(ys))), math.ceil(math.log2(max(ys)))
    sx = lambda v: L + ((math.log2(v) if xlog else v) - x0) / ((x1 - x0) or 1) * (W - L - R)
    sy = lambda v: H - B - (math.log2(v) - y0) / ((y1 - y0) or 1) * (H - B - T)
    o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="11">',
         f'<rect width="{W}" height="{H}" fill="#fff"/>',
         f'<text x="16" y="20" font-size="14" fill="#222">{title}</text><text x="16" y="38" fill="#555">{subtitle}</text>']
    for k in range(y0, y1 + 1):
        y = sy(2.0 ** k)
        o.append(f'<line x1="{L}" y1="{y:.1f}" x2="{W-R}" y2="{y:.1f}" stroke="#eee"/><text x="{L-6}" y="{y+4:.1f}" text-anchor="end" fill="#555">{2.0**k:g}</text>')
    ticks = sorted(set(xs))
    step = max(1, math.ceil(len(ticks) / 8))
    for v in ticks[::step]:
        o.append(f'<text x="{sx(v):.1f}" y="{H-B+16}" text-anchor="middle" fill="#555">{v:.4g}</text>')
    o.append(f'<text x="{(L+W-R)/2:.0f}" y="{H-12}" text-anchor="middle" fill="#222">{xlabel}</text>')
    o.append(f'<text x="14" y="{(T+H-B)/2:.0f}" transform="rotate(-90 14 {(T+H-B)/2:.0f})" text-anchor="middle" fill="#222">{ylabel}</text>')
    for label, col, pts, dashed in series:
        pts = sorted(p for p in pts if p[1] > 0)
        if not pts:
            continue
        d = ' stroke-dasharray="5 4"' if dashed else ""
        o.append(f'<polyline fill="none" stroke="{col}" stroke-width="2"{d} points="' + " ".join(f"{sx(x):.1f},{sy(y):.1f}" for x, y in pts) + '"/>')
        for x, y in pts:
            o.append(f'<circle cx="{sx(x):.1f}" cy="{sy(y):.1f}" r="2.5" fill="{col}"/>')
        slot = [x[0] for x in series].index(label)
        o.append(f'<line x1="{W-R+6}" y1="{T+10+slot*16}" x2="{W-R+22}" y2="{T+10+slot*16}" stroke="{col}" stroke-width="2"{d}/>')
        o.append(f'<text x="{W-R+26}" y="{T+14+slot*16}" fill="{col}">{label}</text>')
    o.append("</svg>")
    (HERE / path).write_text("\n".join(o) + "\n")


def figures(rows):
    by = lambda arm: [(r["log2_r"], r["mean_s"]) for r in rows if r["arm"] == arm]
    inc = [(r["log2_r"], r["mean_s"]) for r in rows if r["arm"].startswith("pair-walk-b")]
    svg_lines("fig-scaling.svg", "Counted S against curve size (log2 r)",
              "sweep at fixed base B, the planner-tuned pair-table incumbent, and the generic searches",
              [("pair-table incumbent", "#c0392b", inc, False), ("sweep B=128", "#d98c00", by("sweep-b128"), False),
               ("sweep B=8", "#1a9850", by("sweep-b8"), False), ("sweep B=2", "#2a6fdb", by("sweep-b2"), False),
               ("kangaroo", "#8e44ad", by("kangaroo"), True), ("bsgs.negation", "#555", by("bsgs-neg"), True),
               ("rho.negation", "#000", by("rho-neg"), True)], "log2 r", "S")
    cols = {20: "#2a6fdb", 24: "#1a9850", 28: "#d98c00", 32: "#c0392b", 36: "#8e44ad"}
    series = []
    for bits, c in cols.items():
        pts = [(r["base"], r["mean_s"]) for r in rows if r["bits"] == bits and r["arm"].startswith("sweep-b")]
        series.append((f"{bits}-bit", c, pts, False))
    svg_lines("fig-base.svg", "Counted S of the sweep against its base size B",
              "model: S = c * sqrt(2 B), slope 1/2 in log-log", series, "base abscissae B", "S")
    dp = []
    for bits, c in cols.items():
        pts = []
        for k, arm in ((0, "sweep-b8"), (2, "b8-dp2"), (4, "b8-dp4"), (6, "b8-dp6")):
            r = next((r for r in rows if r["bits"] == bits and r["arm"] == arm), None)
            if r and r["bytes"]:
                pts.append((r["bytes"] / 2 ** 20, r["mean_s"]))
        dp.append((f"{bits}-bit B=8 dp 0/2/4/6", c, pts, False))
    svg_lines("fig-timememory.svg", "Time-memory trade of the distinguished-point sweep (B = 8)",
              "each point: dp = 0, 2, 4, 6 (a stored residual needs k trailing zero bits); memory in MB at the compact entry size",
              dp, "table size, MB (compact entries)", "S")


def row_law(rows):
    """Least-squares K = alpha*B + beta for the collision rows used by the abscissa-base sweep, and the dependent share."""
    pts = [(r["base"], r["rows_used"]) for r in rows if r["arm"].startswith("sweep-b") and r["bits"] <= 36]
    xs = [b for b, _ in pts]
    ys = [k for _, k in pts]
    mx, my = statistics.mean(xs), statistics.mean(ys)
    alpha = sum((x - mx) * (y - my) for x, y in pts and zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    dep = [r["dependent_rows"] / r["rows_used"] for r in rows if r["arm"].startswith("sweep-b") and r["bits"] <= 36 and r["base"] >= 8]
    return alpha, my - alpha * mx, statistics.mean(dep)


def dp_trade(rows, k):
    """Mean over sizes 20..36 of (S ratio, stored-entry ratio) of the B = 8 dp=k sweep to the plain B = 8 sweep."""
    sr, er = [], []
    for bits in range(20, 38, 2):
        p = next((r for r in rows if r["bits"] == bits and r["arm"] == "sweep-b8"), None)
        q = next((r for r in rows if r["bits"] == bits and r["arm"] == f"b8-dp{k}"), None)
        if p and q and q["entries"]:
            sr.append(q["mean_s"] / p["mean_s"])
            er.append(p["entries"] / q["entries"])
    return statistics.mean(sr), statistics.mean(er)


def findings(rows, fit, runs, var):
    g = lambda bits, arm, k: next((r[k] for r in rows if r["bits"] == bits and r["arm"] == arm), float("nan"))
    sb = fit["s_vs_bits"]
    fx = lambda n: f"{sb[n]['slope']:+.3f} [{sb[n]['lo']:+.3f}, {sb[n]['hi']:+.3f}]" if n in sb else "n/a"
    vr = {v["arm"]: v for v in var}
    vf = lambda a: f"{vr[a]['pooled_ratio']:.2f} [{vr[a]['lo']:.2f}, {vr[a]['hi']:.2f}]" if a in vr else "n/a"
    top = max(r["bits"] for r in rows if r["arm"] == "sweep-b2" or r["arm"] == "b2-dp4")
    big = max(r["bits"] for r in rows if r["arm"] == "pair-walk-b" + str(next((x["base"] for x in rows if x["arm"].startswith("pair-walk-b") and x["bits"] == 36), 0)) and r["bits"] == 36) if False else 36
    alpha, beta, dep = row_law(rows)
    e2, e4, e6 = dp_trade(rows, 2)[1], dp_trade(rows, 4)[1], dp_trade(rows, 6)[1]
    s2, s4, s6 = dp_trade(rows, 2)[0], dp_trade(rows, 4)[0], dp_trade(rows, 6)[0]
    items = [
        "<li><b>What it is.</b> A birthday collision search that runs through the index-calculus pipeline. Evidence: (1) at a fixed base the counted S does not grow with the curve "
        f"(slope of log2 S against log2 r, 95% bootstrap interval: B=2 {fx('sweep B=2')}, B=8 {fx('sweep B=8')}, B=32 {fx('sweep B=32')}, B=128 {fx('sweep B=128')}), exactly like rho.negation "
        f"{fx('rho.negation')}, bsgs.negation {fx('bsgs.negation')} and kangaroo {fx('kangaroo')}; the tuned pair-table incumbent grows: {fx('pair-table incumbent (walk, base ~ r^(1/3))')}. "
        "(2) The collision rows found equal the birthday count N&sup2;/(r 2<sup>k</sup>) in every variant (ratios 0.7 to 1.2, table below). "
        f"(3) Replacing the smallest abscissae by pseudo-random strided ones changes S by {vf('b8-null-strided')}: the base has no index-calculus structure to contribute. "
        f"(4) Adding the pair table changes S by {vf('b8-hybrid')}. (5) With a known-log base a single collision row pins the logarithm (rows used = 1): that is a baby-step/giant-step-style search "
        f"(S ratio to the abscissa base: progression {vf('b8-klprog')}, random scalars {vf('b8-klrand')}).</li>",
        "<li><b>Cost model.</b> A target contributes 2B signed residuals, one group addition each. After N such additions the expected number of colliding pairs is N&sup2;/r "
        "(classes {Z, -Z}: N&sup2;/(2 r/2)), without the sign fold N&sup2;/(2r), and N&sup2;/(r 2<sup>k</sup>) when only residuals whose hashed abscissa has k trailing zero bits are kept "
        "(equal abscissae hash alike, so a pair collides only if it passes the filter). The solve stops when the column of d is pinned, which takes K collision rows: "
        f"K = {alpha:.2f} B + {beta:.1f} for the abscissa base (least squares over sizes 20 to 36), {dep:.0%} of the rows linearly dependent, and K = 1 for a known-log base. "
        "So N = sqrt(K r) and S = N/sqrt(r) is about c sqrt(K): flat in r, growing like sqrt(B) once B exceeds a few, smallest at the smallest base. The table holds N entries (N/2<sup>k</sup> with the filter).</li>",
        f"<li><b>Placement at 36 bits.</b> rho.negation {g(36,'rho-neg','mean_s'):.2f}, rho.negation_lanes {g(36,'rho-lanes16','mean_s'):.2f}, bsgs.negation {g(36,'bsgs-neg','mean_s'):.2f}, kangaroo {g(36,'kangaroo','mean_s'):.2f}; "
        f"the sweep is {g(36,'sweep-b2','mean_s'):.2f} at B=2, {g(36,'sweep-b8','mean_s'):.2f} at B=8 and {g(36,'sweep-b128','mean_s'):.2f} at B=128; the tuned pair-table incumbent is {g(36,'pair-walk-b2048','mean_s'):.1f}. "
        "The generic searches stay flat to 44 bits and the plain B = 2 sweep to 40 bits (past that only the limited-memory variants run: memory). The sweep sits between the generic searches and the incumbent, at the generic searches' exponent.</li>",
        f"<li><b>Distinguished points (B = 8, paired S against the plain sweep, pooled over sizes).</b> k=2 {vf('b8-dp2')}, k=4 {vf('b8-dp4')}, k=6 {vf('b8-dp6')}. "
        f"Stored residuals fall by only {e2:.1f}, {e4:.1f} and {e6:.1f} times (not 4, 16, 64: the run lasts longer), while S grows {s2:.2f}, {s4:.2f} and {s6:.2f} times, so S x memory falls to "
        f"{s2/e2:.2f}, {s4/e4:.2f} and {s6/e6:.2f} of the plain sweep's and then flattens. The trade never reaches rho's constant memory at a rho-like S.</li>",
        f"<li><b>Memory (table entries, 36 bits).</b> sweep B=2 {g(36,'sweep-b2','entries'):.0f}, B=8 {g(36,'sweep-b8','entries'):.0f}; bsgs.negation {g(36,'bsgs-neg','entries'):.0f} baby steps; "
        f"kangaroo {g(36,'kangaroo','entries'):.0f} distinguished points; rho.negation none; the pair-table incumbent {g(36,'pair-walk-b2048','entries'):.0f}. The counted unit does not see any of it. "
        "The small-base sweep needs several times BSGS's memory for a larger S, which is the sense in which BSGS dominates it.</li>",
        "<li><b>Verdict.</b> Not a genuine index calculus: its exponent is the generic one and the algebraic structure of the base contributes nothing measurable. It is a real, verified and cheap arm "
        "of the pipeline (the lowest S of any abscissa-base arm we have measured; the known-log bases are lower still and are collision searches by construction), but under the owner's rule that relabelling is not an improvement it does not go on the leaderboard; the owner decides.</li>",
    ]
    return "<ul>" + "".join(items) + "</ul>"


def model_rows(rows):
    out = []
    for r in rows:
        if r.get("collisions_predicted") and r["bits"] in (24, 32, 36):
            out.append(f"<tr><td>{r['bits']}</td><td>{r['arm']}</td><td>{r['collisions_observed']:.1f}</td><td>{r['collisions_predicted']:.1f}</td>"
                       f"<td>{r['collisions_observed']/r['collisions_predicted']:.2f}</td></tr>")
    return "\n".join(out)


def variant_rows(var):
    return "\n".join(f"<tr><td>{v['label']}</td><td>{v['pooled_ratio']:.3f}</td><td>[{v['lo']:.3f}, {v['hi']:.3f}]</td><td>{v['n']}</td></tr>" for v in var)


def report(rows, fit, runs, var):
    def fmt(v, spec):
        return "" if v is None or (isinstance(v, float) and math.isnan(v)) else format(v, spec)
    tr = "\n".join(
        f"<tr><td>{r['bits']}</td><td>{r['arm']}</td><td>{r['verified']}/{r['runs']}</td><td>{r['mean_s']:.2f} &plusmn; {r['se_s']:.2f}</td>"
        f"<td>{r['distance_to_rho']:.1f}</td><td>{fmt(r.get('paired_s_vs_incumbent'), '.3f')}</td>"
        f"<td>{r['entries']:.0f}</td><td>{r['bytes']/2**20:.2f}</td></tr>" for r in rows)
    figs = "".join((HERE / f).read_text() for f in ("fig-scaling.svg", "fig-base.svg", "fig-timememory.svg"))
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Extended residual sweep</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:1000px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:first-child,th:first-child{{text-align:left}}</style></head><body>
<h1>The residual sweep, exhaustively (2026-10-28)</h1>
<p>Lead <code>ICP-residual-partial-relations</code>. Sizes 20 to 44 bits, three sessions each, the sweep against the generic searches and the
pair-table incumbent, with variants (no sign fold, distinguished points, pair-table hybrid) and base-shape controls.</p>
{figs}
<h2>Findings</h2>
{findings(rows, fit, runs, var)}
<h2>Birthday model check (collision rows found against N&sup2; fold / (2 r 2<sup>k</sup>))</h2>
<table><tr><th>bits</th><th>arm</th><th>observed</th><th>predicted</th><th>ratio</th></tr>{model_rows(rows)}</table>
<h2>Variants at B = 8, paired S against the plain sweep</h2>
<table><tr><th>variant</th><th>pooled ratio</th><th>95% interval</th><th>n</th></tr>{variant_rows(var)}</table>
<h2>All arms</h2>
<table><tr><th>bits</th><th>arm</th><th>verified</th><th>mean S</th><th>S / rho.negation</th><th>paired S vs incumbent</th><th>table entries</th><th>table MB (compact)</th></tr>
{tr}</table></body></html>"""
    (HERE / "report.html").write_text(html)


def main():
    rows, runs = collect()
    fit = fits(rows, runs)
    var = variant_comparisons()
    figures(rows)
    report(rows, fit, runs, var)
    headline = []
    for r in rows:
        headline.append({"name": f"{r['bits']}/{r['arm']}/mean_s", "claimed": r["mean_s"]})
        if "paired_s_vs_incumbent" in r:
            headline.append({"name": f"{r['bits']}/{r['arm']}/paired_s_vs_incumbent", "claimed": r["paired_s_vs_incumbent"]})
    (HERE / "results.json").write_text(json.dumps({"rows": rows, "fits": fit, "variants": var, "headline": headline}, indent=1, sort_keys=True) + "\n")
    print(len(rows), "rows;", len(headline), "headline numbers")
    print(json.dumps(fit, indent=1)[:3000])


if __name__ == "__main__":
    main()
