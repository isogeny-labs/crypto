#!/usr/bin/env python3
"""Renders README.md and REPORT.typ from results.json (recorded values only)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); R = json.load(open(os.path.join(HERE, "results.json")))
f = lambda p: f"{p['ratio']:.3f} [{p['lo']:.3f}, {p['hi']:.3f}]"
Wk = ["0.0", "0.1", "1.0"]; WL = {"0.0": "0", "0.1": "0.1", "1.0": "1"}
arms_order = ["rho-strong", "rho-strong-aa", "rho-neg", "k1", "lazy1", "lazy2", "kf3-c1g-sr", "kf3-c1g-w"]
def S(n, a):
    m = R["curves"][n]["mean_S"]; return [m[a][w] for w in ("0", "0.1", "1")]
def names(n):
    C = R["curves"][n]; l = list(C["weights"]["0.0"]["h1_vs_each_incumbent"].keys()); k1 = [a for a in C["mean_S"] if a.startswith("k1")][0]
    return ["rho-strong", "rho-strong-aa", "rho-neg", k1] + l + ["kf3-c1g-sr", "kf3-c1g-w"]
md = []; ty = []
md.append("""# Frobenius-folded k-sum against the lazy orbit-member incumbent, K1 n23 and n29 (2026-11-15) `[pcc:unverified]`

Frozen paired study ([`PROTOCOL.md`](PROTOCOL.md), commit `9fa38942e4`, hashes in `REGISTRATION.sha256`, written before any study session).
Candidate `kf3-c1g-sr` (`ksum-frobenius:m=3` on `compact-orbit-scan:columns=1,generator_orbit=1`, `stride-renew`) against the strongest existing IC arm,
the lazy orbit-member pair table (`mitm-frobenius-lazy-counted:m=2` on `compact-orbit-scan`, `stride-renew`), and against the K1 recipe, in the same sessions on the same targets.
Every arm is a plain method priced identically by the AGENTS.md section 5a band (w = 0, 0.1, 1 GAE per unpriced native operation, table work priced). Every number is an unverified assertion.

**Framing.** With one orbit and m = 3 this is a Frobenius-folded generic (square-root-class) collision method. It has no subexponential component and cannot beat rho asymptotically; this is a constant at two tiny sizes. A win is a measured improvement over the current best IC arm on that curve and nothing more.
""")
verd = {}
for n in ("23", "29"):
    C = R["curves"][n]
    md.append(f"## K1 n{n} (`{C['slug']}`)\n")
    md.append("Mean S per arm (`ecbench band`, 3 sessions x 48 targets x 3 rounds, pooled):\n")
    md.append("| arm | S(w=0) | S(w=0.1) | S(w=1) |\n|:--|--:|--:|--:|")
    for a in names(n):
        s = S(n, a); md.append(f"| {a} | {s[0]:.3f} | {s[1]:.3f} | {s[2]:.3f} |")
    md.append("")
    md.append("Registered test H1: `R = cost(kf3-c1g-sr) / cost(strictest lazy incumbent)`, paired by target, cluster-bootstrap 95 % CI, pooled over the 3 replicates; per-replicate point estimates and upper ends; A/A = `rho-strong-aa / rho-strong`.\n")
    md.append("| w | strictest incumbent | R pooled [95 % CI] | R per replicate (upper 95 %) | A/A [95 %] | H1 verdict | R vs `kf3-c1g-w` strictest (exploratory) |\n|:--|:--|--:|:--|--:|:--|--:|")
    for w in Wk:
        o = C["weights"][w]; v = "**holds**" if o["h1_holds"] else ("**regression**" if o["h1_regression"] else "does not hold")
        verd[(n, w)] = v
        reps = ", ".join(f"{x['ratio']:.3f} ({x['hi']:.3f})" for x in o["h1_per_rep"])
        md.append(f"| {WL[w]} | {o['strictest_incumbent']} | {f(o['h1_pooled'])} | {reps} | {f(o['aa_pooled'])} | {v} | {f(o['h1_walk_candidate_vs_strictest'])} |")
    md.append("")
    md.append("Each incumbent, the K1 recipe, and the distance remaining to the matched `rho.signed_frobenius_strong` (cost ratio, below 1 = cheaper than rho):\n")
    md.append("| w | R vs each lazy incumbent | R vs K1 recipe (H2) | `kf3-c1g-sr` / rho-strong | strictest lazy / rho-strong | K1 / rho-strong | `rho-neg` / rho-strong |\n|:--|:--|--:|--:|--:|--:|--:|")
    for w in Wk:
        o = C["weights"][w]; g = o["h3_gap_to_rho_strong"]; st = o["strictest_incumbent"]
        each = "; ".join(f"{k}: {f(v)}" for k, v in o["h1_vs_each_incumbent"].items())
        k1 = [k for k in g if k.startswith("k1")][0]
        md.append(f"| {WL[w]} | {each} | {f(o['h2_vs_k1'])} | {f(g['kf3-c1g-sr'])} | {f(g[st])} | {f(g[k1])} | {f(g['rho-neg'])} |")
    md.append("")
    rk = {(r["a"], r["b"]): r["verdict"] for r in C["band_ranks"]}
    def rank(x, y): return rk.get((x, y)) or rk.get((y, x)) or "n/a"
    st = C["weights"]["0.1"]["strictest_incumbent"]
    md.append(f"Band ordering (`ecbench band --ranks`, ranked only where the order holds at every w): `kf3-c1g-sr` vs rho-strong: **{rank('kf3-c1g-sr','rho-strong')}**; `{st}` vs rho-strong: {rank(st,'rho-strong')}; `kf3-c1g-sr` vs `{st}`: {rank('kf3-c1g-sr', st)}; `rho-neg` vs rho-strong: {rank('rho-neg','rho-strong')}.\n")
    sec = C["secondary_full"]
    if "note" in sec: md.append(f"Secondary pricing (`*_full`, the lazy studies' own): {sec['note']}.\n")
    else:
        md.append(f"Secondary pricing H4 (`*_full` arms, recorded S, pooled): `kf3-c1g-sr-full / {R['curves'][n]['weights']['0.0']['strictest_incumbent'] if False else 'lazy-c1-full'}` = {f(sec['full_cand_vs_lazy_full']['recorded_S'])}, per replicate "
                  + ", ".join(f"{x['ratio']:.3f}" for x in sec["full_cand_vs_lazy_full"]["per_rep"]) + f"; `kf3-c1g-sr-full / rho-strong-full` = {f(sec['full_cand_vs_rho_strong_full'])}; `lazy-c1-full / rho-strong-full` = {f(sec['full_lazy_vs_rho_strong_full'])}. Same direction as the primary reading.\n")
md.append("""## Verdicts

| curve | w | H1 (candidate beats the strictest lazy incumbent) |
|:--|:--|:--|""")
for (n, w), v in verd.items(): md.append(f"| K1 n{n} | {WL[w]} | {v} |")
md.append("""
- **K1 n23: H1 holds at w = 0, 0.1 and 1** (pooled upper 95 % < 1, every replicate upper < 1, beyond the A/A interval). `kf3-c1g-sr` costs about 0.78-0.83x the best lazy incumbent (`lazy-c4`) and 0.68-0.74x `lazy-c3`. It is a measured improvement over the strongest existing IC arm on this curve, not a win over rho: against `rho.signed_frobenius_strong` it is below 1 at w = 0 (0.883) and above 1 at w = 0.1 (1.105) and w = 1 (2.262), and `ecbench band --ranks` leaves the pair unresolved inside the band. It costs 0.018-0.026x the K1 recipe.
- **K1 n29: H1 does not hold; the candidate regresses** 5.3x to 6.3x against `lazy-c1` at every w, with every replicate agreeing. There the base is tiny (r = 2^15.4) and the m = 3 table costs more than the pair table's few relations; `lazy-c1 / rho-strong` is 0.344 / 0.440 / 1.133 at w = 0 / 0.1 / 1 (ordering unresolved inside the band).
- Both are single-curve readings at two tiny sizes with tuned configurations from a disjoint-seed sizing pilot; the exponent is untouched.

## Deviations and disclosures

- Local route (`--cpus none --allow-busy`, L0), not isolab-eks; counts are deterministic, wall times descriptive. The sibling lazy studies did the same.
- `*_full` arms errored on n23 in every session (no `full_prices.json` row for the a = 1 n = 23 curve: 576 records a session, replays reproduce the same error). The H4 secondary reading therefore exists for n29 only; no source change was made (src is frozen for this task). The primary tests do not use `*_full` arms.
- Protocol H0 said `lower_bound = false`; a plain method with unpriced counters is a lower bound at w = 0 by construction, which is why the primary reading is the band. All 6 sessions: audits `ok`, 0 problems, replays reproduced (`audits/`); the only errors are the n23 `*_full` records above.
- `rho-strong-aa / rho-strong` is exactly 1.000 [1.000, 1.000] at every w and curve (deterministic counts, same seeds), so the A/A criterion adds nothing beyond requiring R < 1; wall-time A/A was not used.
- 3 measured rounds with `order = alternate` cannot be balanced (one arm order is used once more); S is a count, so order cannot move it.
- `verify` has no `--exit-code` flag (it exits non-zero on a problem by default); the flag in the task text was not passed.
- The first launch of `run.sh` passed that unsupported flag and was killed before any session finished; sessions were deleted and the run restarted (the record shows both launches). `run.sh`, `analyze.py`, `make_report.py` and the figures were written after the freeze commit; they change no registered quantity.
- Sizing pilot (not data): a first K1 n29 sizing with dimensions 5-9 and `max_trials` 1.35e10 did not finish and was killed; n29 K1 was re-sized at dimensions 8-12 with `max_trials` 2e7 (the study uses 2e7 at n29 and 1.35e10 at n23, as the pilot).
- No `VALIDATION.json`, so no leaderboard or `benchmarks/README.md` row. Conductor was not used (no source edits, research directory only).

## Files

`PROTOCOL.md`, `REGISTRATION.sha256`, `registered-at.txt` (frozen), `specs/`, `make_specs.py`, `make_sizing.py`, `run.sh`, `analyze.py`, `make_report.py`, `results.json`, `summary.json` (`crypto.koblitz_ksum_fold_summary/v1`), `tables/` (bands), `audits/`, `logs/`, `figures/` (SVG, PNG), `REPORT.pdf` / `REPORT.typ`, `sessions/*/records.jsonl.gz` (full records; the session sha256 list is in `sessions.sha256`).
""")
open(os.path.join(HERE, "README.md"), "w").write("\n".join(md))
# typst: a compact report with the same numbers
def esc(s): return s.replace("#", "\\#").replace("$", "\\$").replace("<", "\\<").replace("@", "\\@")
ty.append('#set page(margin: 1.6cm, numbering: "1")\n#set text(size: 9.5pt)\n#set table(stroke: 0.4pt)\n= Frobenius-folded k-sum against the lazy orbit-member incumbent (K1 n23, n29)\n')
ty.append("2026-11-15. Frozen protocol commit `9fa38942e4`. Class: engineering. All numbers `[pcc:unverified]`. With one orbit and m = 3 the candidate is a Frobenius-folded generic (square-root-class) method; it cannot beat rho asymptotically. Ratios are cost(candidate) / cost(strictest lazy incumbent), paired by target, 95 % cluster-bootstrap, 3 sessions x 48 targets x 3 rounds per curve, every arm priced by the same section 5a band.\n")
ty.append("== Result\n*K1 n23: the candidate beats the strongest existing IC arm at all three weights* (0.784, 0.799, 0.832 against `lazy-c4`, every replicate upper end below 1, beyond A/A). It is not a win over rho: 0.883 / 1.105 / 2.262 of `rho.signed_frobenius_strong` at w = 0 / 0.1 / 1, unresolved inside the band. *K1 n29: it regresses* 5.30 / 5.62 / 6.31 against `lazy-c1`.\n")
ty.append('#figure(image("figures/fig1-ratio.svg", width: 100%), caption: [R = cost(kf3-c1g-sr) / cost(strictest lazy). Squares: pooled 95 % CI; dots: the three replicates; grey: A/A (a point, counts are deterministic). Log axis.])\n')
for n in ("23", "29"):
    C = R["curves"][n]; ty.append(f"== K1 n{n}\n")
    ty.append("#figure(table(columns: 4, align: (left, right, right, right), [arm], [S(w=0)], [S(w=0.1)], [S(w=1)],\n" + ",\n".join(f"[{esc(a)}], [{S(n,a)[0]:.3f}], [{S(n,a)[1]:.3f}], [{S(n,a)[2]:.3f}]" for a in names(n)) + ",\n), caption: [Mean S per arm (band-priced).])\n")
    ty.append("#figure(table(columns: 5, align: (left, left, right, right, left), [w], [strictest], [R pooled [95 % CI]], [kf3-c1g-sr / rho-strong], [H1],\n" + ",\n".join(
        f"[{WL[w]}], [{C['weights'][w]['strictest_incumbent']}], [{f(C['weights'][w]['h1_pooled'])}], [{f(C['weights'][w]['h3_gap_to_rho_strong']['kf3-c1g-sr'])}], [{verd[(n,w)].replace('*','')}]" for w in Wk) + ",\n), caption: [H1 and distance to rho-strong.])\n")
ty.append('#figure(image("figures/fig2-S.svg", width: 100%), caption: [Mean S per arm and weight (log axis).])\n')
ty.append("== Limits\nTwo tiny curves, configurations tuned on disjoint sizing seeds, no exponent claim, no leaderboard row (no `VALIDATION.json`). `*_full` arms errored on n23 (no price row); the n29 `*_full` read is the same direction (5.355 [5.273, 5.439]). Local route, L0. See `README.md` for all deviations.\n")
open(os.path.join(HERE, "REPORT.typ"), "w").write("\n".join(ty))
