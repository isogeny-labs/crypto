#set document(title: "Three levers on the genus-5 cover")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[Three levers on the genus-5 cover:\ the incumbent's IC time drops to 0.17×, same relations, same matrix]\
  #v(2pt)
  Study `research/humbert_genus5_cover_20261021/levers_20261116` · 2026-11-16 · follows `la_solver_20261116` (PR \#4065)
]

#v(6pt)
*Question.* After `la_solver_20261116` the incumbent was `dlpf50_l1`. At $q = 32003$ it spent
38 s collecting relations and 21 s in Lanczos, out of 62 s. Three levers were built in isolated
crate copies, each checked by an independent verifier agent (workflow `wf_ff8cffdd-dad`), and
merged behind flags. `HUMBERT_REL_FAST` changes the arithmetic of the relation search: a
closed-form 16-term norm, 6 interpolation nodes instead of 9, root finding by equal-degree
splitting, and Barrett reduction. It keeps the same random draws and candidates.
`HUMBERT_LA_THREADS=4` runs the Krylov iterations on threads. `HUMBERT_CLIQUE_SAFE` stops dlpx
from removing a clique tree that would empty a base column. [pcc:unverified]

*Scope and standing.* Toy fields only. 3 values of $q$ × 5 seeds × 9 arms = 135 runs under
`PROTOCOL.md`, frozen before data, with one binary (`bin/humbert-v6`, sha256 prefix `85be709b`),
serial runs, the arm order rotated per seed, and no addendum. The host is a 4-vCPU shared VM (L0).
Every wall time is exploratory and an unverified assertion [pcc:unverified]. Counted fields are
host-independent. No leaderboard row: there is no replay certificate or `VALIDATION.json`.

= Correctness and counted identity

- 135/135 runs solved, with the end-to-end check $x P = Q$ passing. [pcc:unverified]
- *C0:* the flag-free `base` arm reproduces the `la_solver` `dlpf50_l1` row on 15/15 instances
  (candidates, relations, LA rows, columns and nnz, mulmods, $x$).
- *R0:* `rel`, `la4`, `comb` and `comb_aa` have counted fields identical to `base` on all 60
  runs, and `lp_comb` matches `la_solver` `lp50_l1` on 15/15. Every saving below is pure speed.
  Nothing about which relations are found or which matrix is solved changes.

= Predictions against the data

#let rt = csv("results/ratios_v6.csv").slice(1)
#figure(
  text(size: 8pt, table(
    columns: 6, align: right,
    [*q*], [*metric*], [*median*], [*min*], [*max*], [*n*],
    ..rt.map(r => (r.at(0), r.at(1), str(calc.round(float(r.at(2)), digits: 3)), str(calc.round(float(r.at(3)), digits: 3)), str(calc.round(float(r.at(4)), digits: 3)), r.at(5))).flatten()
  )),
  caption: [Paired per-instance ratios (`results/ratios_v6.csv`, L0). R1 = rel/base t_rel, L1 = la4/base t_LA, K1 = comb/base t_ic, A0 = comb_aa/comb, G1 = lp_comb/comb, X10/X05/X02 = dlpxsNN/comb (t_ic).],
)

- *R1 (wall): holds.* Relation time with REL_FAST, relative to base: median 0.043, 0.041 and 0.042 at the
  three $q$ (bound 0.10). This is about a 24× speed-up of the relation phase. [pcc:unverified]
- *L1 (wall): holds.* t_LA with 4 threads, relative to base: median 0.29 at every $q$ (bound 0.60). That is 3.4× on 4 vCPUs.
- *K1 (wall): holds.* comb/base t_ic: median 0.184, 0.182 and 0.170, with the 32003 range 0.163–0.192
  (bound 0.35). The A/A ratio comb_aa/comb is 0.85–1.12. [pcc:unverified]
- *X3 (safety): fails as frozen on 2/45 runs.* At $q = 24001$, seed 3, dlpxs05 and dlpxs02 needed 55
  and 91 precheck extensions. This is not the clique loop. On the same instance every flag-free
  dlpf arm (base, rel, la4, comb, comb_aa) needs 115 extensions with no clique removal at all. The
  instance has base columns that few relations hit. dlpxs10 needs 0 there, and safe clique removal
  never needed more extensions than plain dlpf. The `la_solver` pathology (17 087 extensions,
  2368 s of filtering) is gone. The bound in X3 was set without checking dlpf's own precheck
  count, and that was the error. [pcc:unverified]
- *X4 (two-sided): no effect.* dlpxsNN/comb medians are 0.93–1.12 and inside the A/A spread. The counted LA
  saving (0.967–0.992 of comb's mulmods) is paid for by 1–6 % more candidates.
- *G1' (two-sided): the dlpf lead over lp disappears.* lp_comb/comb is 1.07, 0.95 and 0.97. dlpf's advantage
  was cheaper relation collection; with relations 24× cheaper, lp's smaller matrix
  (t_LA 4.0 s against 6.3 s at 32003) offsets it. [pcc:unverified]

#figure(image("figures/levers_tic.svg", width: 74%), caption: [Median IC time per arm, same instances (L0).])

= Where the time is now

#figure(image("figures/levers_phases.svg", width: 100%), caption: [Phase medians (L0). At $q = 32003$, comb spends 6.3 s in LA, 2.5 s in setup, 1.7 s in relations and 0.45 s in descent.])

At $q = 32003$ the combined arm's 10.8 s splits into 58 % linear algebra, 23 % setup
(instance construction: the group order of $E(FF_(q^4))$, the class count of the cover, the factor
base and the nodes) and 16 % relation collection. Setup stays charged to IC: leaving it out would
be an accounting change, not an improvement. [pcc:unverified] Log–log slope of median $t_"ic"$ from 16001 to 32003:
base 1.38, comb 1.29, lp_comb 1.12.

= Distance remaining to rho

#figure(image("figures/levers_gap_to_rho.svg", width: 70%), caption: [Median IC time / expected rho time of this crate's scalar rho, same instance.])

comb's median IC time is 0.150, 0.105 and 0.072 of the expected rho time at $q = 16001$,
24001 and 32003, or 0.21, 0.15 and 0.10 against rho with the negation map. [pcc:unverified] The
reading in `la_solver` §5 still applies. This is the expected crossover of a known attack on toy
fields: about $q^(1.3)$ against $q^2$. It breaks no curve, and the rho reference is this
crate's single-threaded scalar rho. A threaded or vectorised rho would close part of the gap,
so the comb arm's use of 4 threads must be weighed against a 4-thread rho before any ranking.
§5a: IC does no group operations, so the 0 / 0.1 / 1 GAE band does not change the order.

= Leads this opens

+ *Re-balance the factor base.* With relations 24× cheaper, the optimum shifts towards smaller bases and
  matrices. lp's smaller matrix already ties dlpf. Sweep `lp_alpha` and the dlpf fraction under comb.
+ *Setup (23 %).* Point counting and the class count run once per instance. They need a faster [pcc:unverified]
  method, not removal.
+ *Instance-intrinsic precheck* ($q = 24001$, seed 3: 115 extensions, about 2.8 s of filter). Under cheap
  relations, a few percent excess (dlpxs10) avoids it. This is a candidate default.
+ *Matched multi-thread rho.* Needed before the 4-thread comb arm is compared with rho as a ranking.
+ *L2 rerun with a replay certificate* (isolab-eks) before any leaderboard row.

= Reproduce

```
cargo build --release && cargo test --release          # 12 tests
BIN=levers_20261116/bin/humbert-v6 BIN_SHA256_PREFIX=85be709b PRIMES=16001 \
  ./levers_20261116/run_v6.sh levers_20261116/campaign/q16001     # likewise 24001, 32003
python3 levers_20261116/analyze_v6.py
```
