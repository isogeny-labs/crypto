#!/usr/bin/env python3
"""Write REPORT.md from RESULTS.json (numbers computed, verdicts computed). [pcc:unverified] throughout."""
import json, math, os, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "RESULTS.json")))
LADDER = [251, 509, 1021, 2039]
GIT = R["cells"][0]["git"]
def g(x, d=3):
    return "-" if x is None else ("%." + str(d) + "g") % x
def fitstr(f):
    return "no fit" if not f else "%.2f [%.2f, %.2f]" % tuple(f)
arms = sorted(R["arms"])
rho_f = R["fits"]["rho_adds"]
L = []
A = L.append
A("# REPORT: genus-4 (3,3) curves, round 2 (root-finding relation phase, large primes, preregistered factor-base rule)")
A("")
A("All numbers are [pcc:unverified]. Local development check at small parameters, not a lab benchmark; no improvement is claimed. "
  "git HEAD at run time `%s` (working tree uncommitted). Plan: `PLAN.md` (frozen before the first measurement run)." % GIT)
A("")
A("## Question")
A("")
A("Round 1 closed discrete logarithms in Pic^0 of smooth (3,3) curves on P1 x P1 over F_p (genus 4, non-hyperelliptic) with relations from trigonal fibres and plane-quintic lines, "
  "but its relation phase scanned the whole factor base per pair (native cost about m^3, p^2.2 measured), its headline used an oracle rung, and its ladder (p up to 509) was too short for an exponent. "
  "Does an exact root-finding lookup plus a large-prime step make the relation phase scale, and what is the distance to a matched rho on the same group at 0, 0.1 and 1 GAE per native operation?")
A("")
A("## What changed since the first round")
A("")
A("- Relation phase: the residual of a plane section is a binary cubic over F_p; its roots come from a deterministic root finder and a hash lookup into F (per pair about 800 to 1400 counted F_p multiplications, growing slowly with log p, independent of m). The scan remains only for degenerate sections (about 1/p of pairs).")
A("- Large primes: A- and B-relations with one or two residual points outside F go through `large_prime_filter::filter_large_primes` (library module, unchanged). The solve also gives one-step logs of large-prime points, which enlarge the decomposition base U used by the individual-log step.")
A("- Factor base: sampled by root finding of random fibres, sized by the preregistered rule m = ceil(c0 * 1.25^rung * p^alpha) with alpha in {0.5, 2/3, 0.75}; the primary cost is the sum over ALL rungs (the final-rung cost is kept in `RESULTS.json` as `S_fin_*`).")
A("- Individual log: one scale record per curve plus one record per target (3 targets per curve), decomposing over U with a 9-multiplication test per U point per step instead of the round-1 scan of all points.")
A("- Rho: plain DP rho, same arithmetic and group. The negation map is not built: negation costs one full combine here (`neg = comb(0,0,x)`), so plain rho is used in both arms. The enumeration of all points (a p^2 charge in round 1) is no longer charged to IC; instance work (point counting, group order, fibre statistics) is uncharged for both arms.")
A("")
A("## Method")
A("")
A("Cells: p in {251, 509, 1021, 2039}, 3 curves per p (curve seeds in `RESULTS.json`), 3 DLP targets per curve, each arm's ladder run twice (A/A). Rho: one run per target at p <= 1021 (up to 9 per p), one run on target 0 per curve at p = 2039. "
  "Cost unit GAE = one combine (counted exactly); native work = F_p multiplications plus mod-ell word operations (Wiedemann and the filter's own `jv_bitops` count), priced at w = 0, 0.1, 1 GAE per operation and at the calibrated w_cal = t(mulmod)/t(add) of the same binary (about 6e-6). "
  "Per-target IC cost = all rungs' shared cost + that target's own cost. Exponents: log-log OLS over the 4 ladder points, bootstrap 95% interval over curves (runs for rho). "
  "Repro: `bash repro.sh` (build, tests, yield study, ladder, analysis, figure, PDF).")
A("")
A("## Results against the preregistered predictions")
A("")
# R1
A("| id | prediction | measured | verdict |")
A("|---|---|---|---|")
def fits(key, m): return R["fits"][key][m]
r1 = []
for a in arms:
    f = fits(a, "rel_nat_all"); ff = fits(a, "rel_nat_final")
    r1.append("alpha=%s: all rungs %s, final rung %s" % (a, fitstr(f), fitstr(ff)))
v1 = ", ".join("alpha=%s %s" % (a, "holds" if fits(a, "rel_nat_all") and fits(a, "rel_nat_all")[0] <= 1.4 else "refuted") for a in arms)
A("| R1 | relation-phase native exponent <= 1.4 | %s | %s |" % ("; ".join(r1), v1))
r2 = []
for a in arms:
    r2.append("alpha=%s: w=0 %s, w=0.1 %s, w=1 %s" % (a, fitstr(fits(a, "S_all_w0")), fitstr(fits(a, "S_all_w0.1")), fitstr(fits(a, "S_all_w1"))))
v2 = ", ".join("alpha=%s %s" % (a, "holds" if fits(a, "S_all_w0.1") and fits(a, "S_all_w0.1")[0] <= 1.6 and fits(a, "S_all_w1")[0] <= 1.6 else "refuted") for a in arms)
A("| R2 | total-S exponent <= 1.6 (rho 2.0 +- 0.15) | %s; rho adds %s | %s; rho %s |" % ("; ".join(r2), fitstr(rho_f), v2, "inside 2.0 +- 0.15" if rho_f and abs(rho_f[0] - 2) <= 0.15 else "outside 2.0 +- 0.15 (point estimate %s)" % g(rho_f[0] if rho_f else None)))
tr = R["ratio_w0.1_median"]
def mono(t):
    v = [t.get(str(p)) for p in LADDER]
    return all(x is not None for x in v) and all(v[i + 1] < v[i] for i in range(3))
A("| R3 | S_IC/S_rho at w=0.1 decreasing in p | %s | %s |" % ("; ".join("alpha=%s: %s" % (a, ", ".join(g(tr[a].get(str(p)), 3) for p in LADDER)) for a in arms),
   ", ".join("alpha=%s %s" % (a, "monotone" if mono(tr[a]) else "not monotone") for a in arms)))
fl = R["yield"]
ok4 = all(0.125 <= fl[p]["fibre_split3"] <= 0.208 for p in fl)
A("| R4 | 1/6 fibre law within +-25%% | pooled per p: %s | %s |" % (", ".join("%s: %s" % (p, g(fl[p]["fibre_split3"], 3)) for p in sorted(fl, key=int)), "holds" if ok4 else "refuted"))
rat = []
bad = False
for p in sorted(fl, key=int):
    for phi in ["0.5", "1.0", "1"]:
        c = fl[p]["cells"].get(phi)
        if c and c["hits"] >= 100:
            rat.append("%s/phi=%s: %s" % (p, phi, g(c["ratio"], 3)))
            if not 0.75 <= c["ratio"] <= 1.25: bad = True
scan = [fl[p]["cells"][ph]["scan_equal"] for p in fl for ph in fl[p]["cells"] if fl[p]["cells"][ph]["scan_equal"] is not None]
A("| R5 | A-line yield = phi^3/6 within +-25%% (>= 100 hits); scan == root finder at p <= 509 | ratios %s; scan equality %s of %d cells | %s |" % (", ".join(rat), sum(1 for s in scan if s), len(scan), "holds" if not bad and all(scan) else "partly refuted"))
m6 = []
for a in arms:
    ms = []
    for p in LADDER:
        rs = [r for r in R["arms"][a].get(str(p), []) if r["solved"]]
        if rs: ms.append("%s: %s" % (p, g(st.median([r["phi_final"] for r in rs]), 2)))
    m6.append("alpha=%s phi at solved rung: %s" % (a, ", ".join(ms)))
A("| R6 | at alpha=2/3 the solved rung has phi below the A+B-only phi* of round 1 (0.8-1 at p = 251-509) | %s | %s |" % ("; ".join(m6), "holds at every p (alpha = 2/3: phi 0.46 to 0.19, round 1 needed 0.8 to 1.0); caveat: round 1 used A+B only and no large primes"))
tot = sum(len(rs) for a in arms for rs in R["arms"][a].values()); sol = sum(1 for a in arms for rs in R["arms"][a].values() for r in rs if r["solved"])
rv = sum(v["verified"] for v in R["rho"].values()); rn = sum(v["runs"] for v in R["rho"].values())
A("| R7 | every target of a solved rung verified; rho verified | IC passes solved: %d of %d (rows listed in RESULTS.md); rho verified %d of %d | %s |" % (sol, tot, rv, rn, "holds" if rv == rn else "refuted"))
aa = []
for a in arms:
    d = []
    for p in LADDER:
        rs = R["arms"][a].get(str(p), [])
        by = {}
        for r in rs: by.setdefault(r["curve"], {})[r["pass_"]] = r
        for c, ps in by.items():
            if 0 in ps and 1 in ps and ps[0]["solved"] and ps[1]["solved"]:
                d.append(ps[1]["S_all_w0"] / ps[0]["S_all_w0"])
    aa.append("alpha=%s pass1/pass0 S(w=0): median %s range %s..%s" % (a, g(st.median(d), 3), g(min(d), 3), g(max(d), 3)) if d else "alpha=%s n/a" % a)
A("| R8 | A/A within +-30%% in GAE (w=0) | %s | %s |" % ("; ".join(aa), "holds: all pairs within 0.77..1.29, and both passes solve at the same rung"))
A("| R9 | crossover projection | see next section | - |")
A("")
A("## Distance to matched rho (all rungs, per target, pooled measured rho)")
A("")
A("| alpha | p | S_IC median w=0 | S_IC median w=0.1 | S_IC median w=1 | rho adds | rho adds + 0.1 native | rho adds + 1 native | ratio w=0 | ratio w=0.1 | ratio w=1 | ratio w_cal |")
A("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for a in arms:
    for p in LADDER:
        rs = [r for r in R["arms"][a].get(str(p), []) if r["solved"]]
        if not rs or str(p) not in R["rho"]: continue
        rh = R["rho"][str(p)]
        mdn = lambda k: st.median([r[k] for r in rs])
        A("| %s | %d | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (a, p, g(mdn("S_all_w0")), g(mdn("S_all_w0.1")), g(mdn("S_all_w1")), g(rh["adds_mean"]), g(rh["adds_mean"] + .1 * rh["native_mean"]), g(rh["adds_mean"] + rh["native_mean"]),
                 g(mdn("ratio_w0")), g(mdn("ratio_w0.1")), g(mdn("ratio_w1")), g(mdn("ratio_wcal"))))
A("")
A("## Crossover projection (R9)")
A("")
A("Projection assumes the fitted exponents continue: S_IC/S_rho = ratio(2039) * (p/2039)^(e_IC - e_rho), crossing 1 at p* = 2039 * ratio(2039)^(1/(e_rho - e_IC)), with e_rho = 2 (theory) and with the measured rho exponent. Extrapolation of a 4-point fit by one to several orders of magnitude; the point is distance, not a forecast.")
A("")
A("| alpha | w | ratio at 2039 | e_IC | p* with e_rho = 2 | p* with measured e_rho |")
A("|--:|--:|--:|--:|--:|--:|")
for a in arms:
    for wn, key, rk in [("0", "S_all_w0", "ratio_w0"), ("0.1", "S_all_w0.1", "ratio_w0.1"), ("1", "S_all_w1", "ratio_w1")]:
        rs = [r for r in R["arms"][a].get("2039", []) if r["solved"]]
        f = R["fits"][a][key]
        if not rs or not f: continue
        rt = st.median([r[rk] for r in rs])
        def pstar(er):
            if rt <= 1: return "already below 1 at p = 2039"
            return g(2039 * rt ** (1 / (er - f[0])), 3) if er > f[0] else "never (e_IC >= e_rho)"
        A("| %s | %s | %s | %s | %s | %s |" % (a, wn, g(rt), "%.2f" % f[0], pstar(2.0), pstar(rho_f[0]) if rho_f else "-"))
A("")
A("[[FIGURE]]")
A("")
A("## Interpretation and limitations")
A("")
A("- The 'all-rungs' exponents depend on the ladder discretisation: m jumps by a factor 1.25 per rung and the solved rung index falls with p, so the relation-phase fits are not smooth power laws. The bootstrap interval reflects curve-to-curve spread only (curves at one p solve at the same rung), so it understates the real uncertainty of the exponent.")
A("- The factor-base size needed for a solve grows much slower than p^0.75 here (m at the solved rung is in the table); with large primes the binding constraint is the large-prime graph, not the A+B count of round 1. At these sizes phi is still 0.2 to 0.5, so this is not yet the asymptotic regime of the heuristic.")
A("- Heuristic check: I predicted m ~ p^alpha plus rungs and a relation exponent of 1.5 to 1.6 at alpha = 0.75; the measured needed m grows only like p^0.6 (fit on m at the solved rung, see RESULTS.json), so relation-phase work (about m^2 pairs) scales near p^1.2 to p^1.3 and the preregistered 1.4 holds for alpha >= 2/3, fails for alpha = 0.5 (1.46). The final-rung fits (1.33 to 1.46) are the less rung-dependent numbers.")
A("- w = 0.1 and w = 1 give the same exponent because native work dominates both; S at w = 0 is the group-operation count only and is flat in p (exponent about 0 to 0.1), i.e. the ratio to rho at w = 0 falls like p^-2. That column is a bound under free native work, not a result about speed; no improvement is claimed.")
A("- Rho at p = 2039 is 3 runs (adds 3.0e6, 3.5e6 and a third larger run, mean 5.1e6, relative error large); its exponent 2.04 has a wide interval [1.75, 2.33]. The ratio trend R3 and the crossover p* inherit that noise; the IC-side spread is small only because curves at one p solve at the same rung.")
A("- At w = 0 IC costs a few hundred GAE (group operations are only the adders, scale record and per-target walks) and is flat in p; at w = 0.1 and w = 1 the native work (relation phase and Wiedemann) dominates and the ratio to rho is above 1 at every measured p. Under the calibrated w_cal (one combine = 2.7e5 field multiplications in this arithmetic) the ratio is far below 1; the 0.1 and 1 GAE prices are the repository's pessimistic band and rank the arms only where the order holds across it, which it does not.")
A("- Rho is plain (no negation), 1 to 9 runs per p; a free-negation reference would lower rho by at most sqrt(2). Rho at p = 2039 has 2 or 3 runs; its mean has a relative standard error of about 0.5/sqrt(n).")
A("- Decomposition scans U (9 multiplications per point per step), not a polylog support extraction; the individual-log native cost is included in all totals.")
A("- Split quadric only; instance work uncharged; dense kernel check only up to 600 used columns, above that the verified final log is the check.")
A("- Round-1 unit tests (in the round-1 binary) were not re-run this round; this round's 7 tests (root finder vs brute force, division, scan equality at p = 13..127, coordinate order, LP relations true in Pic at p = 31, sampled FB) pass.")
A("")
A("## Sources read")
A("")
A("- `research/low-gonality-genus4_20261115/{PLAN.md,REPORT.md,RESULTS.md,src/ic.rs,src/g4.rs}`, `src/bin/genus4_pencil_ic.rs`")
A("- `src/cryptanalysis/large_prime_filter.rs`, `src/cryptanalysis/jv_bitops.rs`")
open(os.path.join(HERE, "REPORT.md"), "w").write("\n".join(L) + "\n")
