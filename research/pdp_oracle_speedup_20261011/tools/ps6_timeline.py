#!/usr/bin/env python3
"""Write PS-6's rows into the scoreboard's source data and its rendered copies (page assembly only).

  ps6_timeline.py REPO_ROOT

Reads research/pdp_oracle_speedup_20261011/ps6_charge/raw/summary.json (the study's own analysis output) and, from
it, quotes (computes nothing new beyond formatting):
  1. four points in docs/ic/progress-timeline.json (the canonical source data of the progress chart);
  2. the embedded copy of that file in docs/index-calculus-scoreboard.html (`progress-data`);
  3. the same points in the page's "every plotted point" table;
  4. the panel `lab-ps6-charge` (between the markers `ps6-charge:begin` / `ps6-charge:end`).
Idempotent: points and rows anchored at #lab-ps6-charge and the marked panel are replaced, not duplicated.
Afterwards run `python3 scripts/site/panel_index.py` (the current-state page's generated panel index) and
`python3 scripts/site/test_build.py`.
"""
import html
import json
import re
import sys

root = sys.argv[1].rstrip("/") + "/"
S = json.load(open(root + "research/pdp_oracle_speedup_20261011/ps6_charge/raw/summary.json"))
TIMELINE = root + "docs/ic/progress-timeline.json"
PAGE = root + "docs/index-calculus-scoreboard.html"
ANCHOR = "#lab-ps6-charge"
DATE = "2026-10-31"
STUDY = "https://github.com/aburan28/crypto/blob/main/research/pdp_oracle_speedup_20261011/ps6_charge/README.md"
W0, W01, W1 = "w = 0 (recorded)", "w = 0.1", "w = 1"


def rung(n):
    return next(r for r in S["rungs"] if r["n"] == n)


def arm(n, name, conv=W0):
    return next(a for a in rung(n)["arms"] if a["arm"] == name and a["convention"] == conv)


def order(n, a, b):
    return next(o for o in rung(n)["order_across_band"] if o["a"] == a and o["b"] == b)["ratio_a_over_b_at_w0_w01_w1"]


def g(x, d=4):
    return f"{x:.{d}g}"


def comma(x):
    return f"{x:,.0f}" if abs(x) >= 100 else g(x, 4)


def slope_diff(name):
    return next(d for d in S["slope_differences"] if d["difference"] == name)["median"]


def memory_entries(n):
    return next(m for m in S["memory"] if m["n"] == n and m["fold"] == "full")


def wall_ratio(n, a, b):
    t = next(t for t in S["timed"] if t["n"] == n)
    return next(p for p in t["paired_wall"] if p["a"] == a and p["b"] == b)["median_ratio_b_over_a"]


ns = [r["n"] for r in S["rungs"]]
top = ns[-1]
slug = {r["n"]: r["slug"] for r in S["rungs"]}
log2r = {r["n"]: r["log2_r"] for r in S["rungs"]}

# --- the four points -------------------------------------------------------------------------------------
best_cold = min(ns, key=lambda n: arm(n, "mitm-negfold")["S_over_rho"])
mit = arm(best_cold, "mitm-negfold")
ci = mit["S_over_rho_ci95"]
p_cold = {
    "date": DATE,
    "value": round(mit["S_over_rho"], 3),
    "reference": "matched",
    "unit": "S",
    "label": f"PS-6: the ledger's S4-family oracles in ecbench, best cold cell: negation-folded MITM on {slug[best_cold]} (r = 2^{log2r[best_cold]:.1f})",
    "detail": f"One-target cold S of the negation-folded meet-in-the-middle table over rho.negation on the same {mit['verified']} verified runs: {mit['S_over_rho']:.2f} [{ci[0]:.2f}, {ci[1]:.2f}] at w = 0; {arm(best_cold, 'mitm-negfold', W1)['S_over_rho']:.2f} when rho's unpriced canonicalisations cost 1 GAE each (section 5a band). The index-calculus arms are above rho at every rung (n = {ns[0]}..{top}) and weight; the ratio grows with n (MITM {arm(top, 'mitm')['S_over_rho']:.0f}x at n = {top}). Class: engineering; no advance, no leaderboard row.",
    "class": "engineering",
    "anchor": ANCHOR,
    "source": STUDY,
}
best_lb = min(ns, key=lambda n: arm(n, "s4-grouplaw")["S_over_rho"])
k = arm(best_lb, "s4-grouplaw")
kb = order(best_lb, "s4-grouplaw", "mitm")
p_lb = {
    "date": DATE,
    "value": round(k["S_over_rho"], 2),
    "reference": "matched",
    "unit": "counted GAE lower-bound diagnostic",
    "label": f"PS-6: PS-2 group-law kernel (S4 pairs-and-solve), recorded S over rho.negation, best cell {slug[best_lb]} (r = 2^{log2r[best_lb]:.1f})",
    "detail": f"The kernel's pairs have no pinned conversion, so they are counted and left out of S: the recorded S is a lower bound and a ratio of two lower bounds does not bound the full-cost ratio. Over the same {k['verified']} verified runs per arm the recorded ratio is {k['S_over_rho']:.2f}; with each kernel pair priced at 0.1 and 1 GAE it is {arm(best_lb, 's4-grouplaw', W01)['S_over_rho']:.0f} and {arm(best_lb, 's4-grouplaw', W1)['S_over_rho']:.0f}. Against the baseline pairs-and-solve the kernel is below at every weight; against the library's MITM table the order flips inside the band (kernel/MITM {kb[0]:.2f}, {kb[1]:.2f}, {kb[2]:.1f} at w = 0, 0.1, 1): unresolved. Class: engineering (stage diagnostic).",
    "class": "engineering",
    "anchor": ANCHOR,
    "source": STUDY,
}
kb1 = order(top, "s4-grouplaw", "s4-baseline")
p_stage_k = {
    "date": DATE,
    "value": round(kb1[2], 4),
    "reference": "stage",
    "label": f"PS-6: group-law kernel vs pairs-and-solve in the counted unit at n = {top} (l = {S['rungs'][-1]['l']}), each kernel pair priced at 1 GAE",
    "detail": f"Sum S of the kernel over sum S of the baseline pairs-and-solve on the same {S['rungs'][-1]['workloads']} one-target runs (identical trial streams; 0 witness mismatches in {sum(r['parity']['audit_checks'] for r in S['rungs'] if r.get('parity')):,} audited targets over the five rungs): {kb1[0]:.5f} with the kernel's pairs unpriced, {kb1[1]:.4f} at 0.1 GAE per pair, {kb1[2]:.4f} at 1 GAE (a baseline pair is priced {rung(top)['conversions']['c_s4_pinned']:.2f} GAE by the pinned calibration). Value plotted: the 1 GAE end of the band, the conservative one. Stage diagnostic; the kernel has no pinned price; no end-to-end speedup is claimed.",
    "class": "engineering",
    "anchor": ANCHOR,
    "source": STUDY,
}
mb = order(top, "mitm", "s4-baseline")
p_stage_m = {
    "date": DATE,
    "value": round(mb[0], 5),
    "reference": "stage",
    "label": f"PS-6: the library's MITM pair table vs pairs-and-solve in the counted unit at n = {top} (l = {S['rungs'][-1]['l']}), table build inside S",
    "detail": f"Sum S of mitm:m=3 over sum S of the baseline on the same {S['rungs'][-1]['workloads']} one-target runs, every phase priced (no unpriced counter): {mb[0]:.5f} (the baseline costs {1 / mb[0]:.0f} times as much); the table has {memory_entries(top)['table_entries']:,} entries and grows resident memory by about {memory_entries(top)['rss_delta_kib'] / 1024:.1f} MiB, which is not in S. Fitted total-cost exponents over n = {ns[0]}..{top}: baseline {next(f for f in S['fits'] if f['arm'] == 's4-baseline' and f['convention'] == W0)['exponent_e']:.3f}, MITM {next(f for f in S['fits'] if f['arm'] == 'mitm' and f['convention'] == W0)['exponent_e']:.3f}. Class: engineering.",
    "class": "engineering",
    "anchor": ANCHOR,
    "source": STUDY,
}
points = {"ec-cold": p_cold, "ec-bounded-counts": p_lb, "binary-decomp-oracle": [p_stage_k, p_stage_m]}

# --- the canonical data file -----------------------------------------------------------------------------
raw = open(TIMELINE).read()
tl = json.loads(raw)
assert json.dumps(tl, indent=1, ensure_ascii=False) + "\n" == raw or json.dumps(tl, indent=1, ensure_ascii=False) == raw.rstrip("\n"), "the file's formatting is not json.dumps(indent=1); edit by hand"
for series in tl["series"]:
    series["points"] = [p for p in series["points"] if p.get("anchor") != ANCHOR]
    add = points.get(series["id"])
    if add is None:
        continue
    for p in (add if isinstance(add, list) else [add]):
        series["points"].append(p)
tl["updated"] = max(tl["updated"], DATE)
text = json.dumps(tl, indent=1, ensure_ascii=False) + ("\n" if raw.endswith("\n") else "")
open(TIMELINE, "w").write(text)

# --- the page ----------------------------------------------------------------------------------------------
page = open(PAGE).read()
m = re.search(r'(<script type="application/json" id="progress-data">)(.*?)(</script>)', page, re.S)
page = page[: m.start(2)] + text + page[m.end(2):]

series_label = {s["id"]: s["label"] for s in tl["series"]}
chip = '<span class="chip">engineering</span>'


def row(series_id, p):
    unit = f" <small>{html.escape(p.get('unit'))}</small>" if p.get("unit") and p["unit"] != "S" else (" <small>S</small>" if p.get("unit") == "S" else "")
    val = f"{p['value']:g}&times;{unit}" if p["reference"] != "stage" else f"{p['value']:g}&times; <small>stage</small>"
    return (
        f'<tr><td>{p["date"]}</td><td>{html.escape(series_label[series_id])}</td><td>{html.escape(p["label"])}</td>'
        f'<td class="n">{val}</td><td>{p["reference"]}</td><td>{chip}</td>'
        f'<td><a href="{ANCHOR}">panel</a> &middot; <a href="{p["source"]}">source</a></td></tr>'
    )


t = re.search(r"(<summary>Every plotted point as a table</summary>)(.*?)(</details>)", page, re.S)
body = t.group(2)
body = "\n".join(l for l in body.split("\n") if f'href="{ANCHOR}"' not in l)
new_rows = "\n".join(row(sid, p) for sid, add in points.items() for p in (add if isinstance(add, list) else [add]))
i = body.index("</tbody>")
body = body[:i] + new_rows + "\n" + body[i:]
page = page[: t.start(2)] + body + page[t.end(2):]


# --- the panel ---------------------------------------------------------------------------------------------
def fmt_row(n):
    r = rung(n)
    a = lambda name, conv=W0: arm(n, name, conv)
    ko = order(n, "s4-grouplaw", "mitm")
    return (
        f'<tr><td class="n">{n}</td><td class="n">{comma(a("rho-negation")["mean_S"])}</td>'
        f'<td class="n">{comma(a("s4-baseline")["mean_S"])}</td>'
        f'<td class="n">{comma(a("s4-grouplaw")["mean_S"])}</td><td class="n">{comma(a("s4-grouplaw", W01)["mean_S"])}</td><td class="n">{comma(a("s4-grouplaw", W1)["mean_S"])}</td>'
        f'<td class="n">{comma(a("s4-grouplaw", "anchored estimate")["mean_S"])}</td>'
        f'<td class="n">{comma(a("mitm")["mean_S"])}</td>'
        f'<td class="n">{ko[0]:.2f}, {ko[1]:.2f}, {ko[2]:.1f}</td></tr>'
    )


fit = lambda arm_name, conv=W0: next(f for f in S["fits"] if f["arm"] == arm_name and f["convention"] == conv)
panel = f"""<!-- ps6-charge:begin -->
<details class="dash-details"><summary>PS-6 &middot; the pairs-and-solve kernels charged in the counted unit</summary><div class="dash-detail-body"><section class="dash-section" id="lab-ps6-charge" aria-labelledby="ps6-charge-title" data-collapse="open"><div class="dash-section-head"><div><p class="dash-kicker">Binary decomposition oracle, one harness / 31 October 2026</p><h2 id="ps6-charge-title">The group-law kernel lowers the S4 oracle&rsquo;s counted cost at every rung; against the pair table its order is unresolved.</h2>
<p class="panel-summary">Charges the S4 pairs-and-solve baseline, the PS-2 group-law kernel and the library&rsquo;s MITM table in ecbench on the ledger&rsquo;s five char2 rungs (n = {ns[0]}..{top}); the kernel&rsquo;s pairs have no pinned price, so it is a lower bound with the section 5a band, a stage diagnostic; all three arms sit above matched rho at every rung and weight.</p></div><span class="status neutral">Engineering &middot; stage diagnostic</span></div>
<p class="dash-section-intro">Same curves, subgroups, factor bases (<code>binary-subspace</code>, l = n/3), targets and seeds for every arm, one target per run, cold, every phase inside <code>S</code>; {sum(r['workloads'] for r in S['rungs'])} workloads, all runs verified, exact replays reproduce, every recovered scalar re-verified by separate arithmetic. The baseline row reproduces the ledger&rsquo;s S4 row (n = {top}: {comma(arm(top, 's4-baseline')['mean_S'])} against the ledger&rsquo;s 115,459) and its exponent ({fit('s4-baseline')['exponent_e']:.3f} against 1.06). <strong>The kernel&rsquo;s pairs are counted but not priced</strong>: its recorded <code>S</code> is a lower bound; the columns at 0.1 and 1 GAE per pair are the AGENTS.md section 5a band and the anchored estimate (the pinned baseline-pair price times the measured kernel/baseline time ratio) is a further point that is not a pinned conversion. Whether the kernel is above or below the table changes inside the band at every rung.</p>
<div class="dash-detail-body" style="padding:0"><table>
<caption>Mean <code>S</code> = group-addition equivalents / &radic;r per arm (16 runs per cell, 8 at n = {top}); the kernel at 0, 0.1 and 1 GAE per unpriced pair, and the order of kernel against the MITM table at those weights. Quoted from <code>raw/summary.json</code>.</caption>
<thead><tr><th>n</th><th>rho (negation)</th><th>S4 baseline</th><th>kernel, recorded (lower bound)</th><th>kernel, 0.1 GAE/pair</th><th>kernel, 1 GAE/pair</th><th>kernel, anchored estimate (not pinned)</th><th>MITM</th><th>kernel / MITM at w = 0, 0.1, 1</th></tr></thead><tbody>
{chr(10).join(fmt_row(n) for n in ns)}
</tbody></table></div>
<p class="dash-footnote">Fitted total-cost exponents over the five rungs (total operations ~ r<sup>e</sup>, 95% bootstrap over targets): rho {fit('rho-negation')['exponent_e']:.3f}, baseline {fit('s4-baseline')['exponent_e']:.3f}, kernel {fit('s4-grouplaw')['exponent_e']:.3f} (lower bound), {fit('s4-grouplaw', W01)['exponent_e']:.3f} (0.1 GAE/pair), {fit('s4-grouplaw', W1)['exponent_e']:.3f} (1 GAE/pair), {fit('s4-grouplaw', 'anchored estimate')['exponent_e']:.3f} (anchored estimate), MITM {fit('mitm')['exponent_e']:.3f}. The finding rule asks whether the ratio to the floor changes slope: over these rungs the kernel&rsquo;s slope is below the baseline&rsquo;s by {-slope_diff('s4-grouplaw [w = 1] minus baseline'):.2f} to {-slope_diff('s4-grouplaw [anchored estimate] minus baseline'):.2f} depending on the pair price (none if a pair is priced as a baseline pair), tracking the per-pair time ratio that falls with l; five rungs ending at l = 9 do not show whether it persists. Timed sessions (descriptive; long runs reach L1 only): kernel over baseline wall {wall_ratio(ns[0], 's4-baseline', 's4-grouplaw'):.3f} at n = {ns[0]} to {wall_ratio(top, 's4-baseline', 's4-grouplaw'):.3f} at n = {top}. Memory is not in <code>S</code>: the table has {memory_entries(top)['table_entries']:,} entries ({memory_entries(top)['rss_delta_kib'] / 1024:.1f} MiB) at n = {top}. Nothing here is a claim about ECC2K-130 or the m = 83 gate; no end-to-end speedup of the kernel. Class: engineering. No progress-chart point crosses rho, and the best-of table, the IC leaderboard and the other panels are unchanged. <a href="https://github.com/aburan28/crypto/blob/main/research/pdp_oracle_speedup_20261011/ps6_charge/README.md">Study, requirement table and deviations</a>, <a href="https://github.com/aburan28/crypto/blob/main/research/pdp_oracle_speedup_20261011/ps6_charge/report/report.pdf">results report (PDF)</a>, <a href="https://github.com/aburan28/crypto/tree/main/benchmarks/20261031-pdp-ps6-charge">benchmarks entry</a>.</p>
</section></div></details>
<!-- ps6-charge:end -->"""
page = re.sub(r"<!-- ps6-charge:begin -->.*?<!-- ps6-charge:end -->\n?", "", page, flags=re.S)
# after the pair-claw panel
j = page.index('id="lab-pair-claw"')
k2 = page.index("</section></div></details>", j) + len("</section></div></details>")
page = page[:k2] + "\n" + panel + page[k2:]
open(PAGE, "w").write(page)
print("ps6 rows written:", sum(len(a) if isinstance(a, list) else 1 for a in points.values()), "points")
