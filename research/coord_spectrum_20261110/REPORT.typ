#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#show raw: set text(size: 8pt)
#let pu = text(size: 7pt)[`[pcc:unverified]`]
= F3: Fourier spectrum and additive energy of coordinate functions along $chevron.l G chevron.r$ on prime-order curves over $F_p$

Run F3-RUN-02 of study `coord_spectrum_20261110`. Class: boundary diagnostic; exact counts, no S, no rho row, no leaderboard change. *Status: closed by decision rule 3; no replication is prescribed.* All numbers below are #pu (no pcc claim id exists for this run). No improvement over rho, or over anything, is claimed.

== Purpose and mechanism
For a cheap $g: chevron.l G chevron.r -> F_p$ the study measures $S_g(a,b) = sum_(k in ZZ"/"r) e(a g([k]G)"/"p + b k"/"r)$ and $eta_g = max |S_g(a,b)| "/" r$. A coefficient $eta >> r^(-1"/"4)$ would give the logarithm by a hidden-shift step in about $log r "/" eta^2$ group operations (generator's derivation, derived, not checked). The bound it must escape: for $g$ of algebraic degree $d$ in the coordinates the Weil bound gives $eta lt.tilde 2 d r^(-1"/"2)$ (Kohel--Shparlinski, recalled), which restates rho's barrier; only functions of degree about $r^(1"/"4)$ or more that stay cheap could escape it.

The second statistic is the additive energy $E(A) = \#{(a,b,c,d) in A^4: a + b = c + d "in" F_p}$ of $A_K = {x([k]G): 1 <= k <= K}$, which single character sums do not control: an excess over random growing with $p$ would mean Freiman-type structure in the orbit (4-term $x$-coordinate relations harvestable without summation polynomials). Instrument: native Rust, exact-arithmetic (f64) FFT of length $r$ at 64 values of $a$; exact energies by hashing pair sums for $K in {ceil(r^(1"/"2)), ceil(r^(2"/"3))}$; nulls: 20 per class (spectrum), 50 random $K$-subsets of $F_p$ (energy).

== Rows and functions
*R16, R20:* 8 prime-order curves each, $p$ the largest prime below $2^16$ / $2^20$. *ANOM:* 2 anomalous curves ($\#E = p$), CM with $D = -11$, $p = 74869$ and $p = 1060513$ (the "$2^16$" curve lands above $2^16$); positive control. Functions per curve: $x$, $y$, $chi(x + c)$ for $c = 0, 1, 2$ (Legendre), Dickson $D_n(x)$ of degree $n = ceil(r^(1"/"4))$ (the Weil-escape class). Controls: $lambda$ the Smart p-adic logarithm (ANOM only; must be linear in $k$) and PLANT, $g(k) = floor(k p "/" r)$ (every curve).

== Frozen predictions and rules (PROTOCOL.md sections 3 and 4, verbatim)
- *P1.* $eta_x, eta_y, eta_chi, eta_D$ within $[0.8, 1.25] times "median"(eta_"NULL")$ on every R16/R20 curve. *P2.* $lambda$ on ANOM and PLANT everywhere: $eta >= 0.5$. *P3.* $E(A_K) "/" "mean" E("random")$ within $1 plus.minus 3 "sd"/"mean"$ of the null on every curve and $K$. *P4.* ANOM's $x, y, chi$ rows behave as P1 (anomaly lives in $lambda$, not in coordinates). Prior that P1 or P3 fails on $>= 2$ curves of one size: 0.03.
+ *Instrument failure:* P2 fails, or FFT error bound exceeds $10^(-6) r$. Fix, re-freeze, re-run.
+ *Lead alive:* a non-control $g$ has $eta_g > 1.5 times "median"(eta_"NULL")$ on $>= 2$ curves of one size, or P3 fails in the same direction on $>= 2$ curves of one size. Then replicate on 16 fresh curves and at $2^24$ before any claim.
+ *Closed:* otherwise: no heavy Fourier coefficient and no additive-energy excess in the coordinate orbit for these function classes at $r <= 2^20$. Rule 3 does not bound functions outside the classes listed, higher Gowers norms ($U^3$ and up), or larger $p$.

== Amendments before the run (section 6, summarised)
Predictions and rules were committed (`c8bbc80e1a`) before the tool existed. After the implementer's `--quick` (not committed, not scored), three corrections to the statistic and null, not to predictions or rules: (1) *$b = 0$ is excluded from the gated $eta$*: on a prime-order curve $chevron.l G chevron.r = E(F_p)$, so $S_g(a,0)$ is a character sum over the whole group and carries no logarithm information; `eta_b0` is reported separately. (2) *Legendre rows enter as complex $plus.minus 1$ values* against a null of random $plus.minus 1$ sequences (the map $plus.minus 1 |-> {1, p-1}$ was an encoding artefact). (3) *Nulls carry the negation symmetry*: $x, chi, D_n$ even, $y, lambda$ odd; nulls per class (`even_fp`, `even_pm1`, `odd_fp`, 20 each); P1/P4 ratios are against the null of the function's class; PLANT keeps the uniform null.

*Freeze note (before the run).* The quick run read $x$ 0.74--0.79, $y$ 0.73, $chi$ 0.455--0.464, $D_n$ 0.99--1.04 of class-null medians, so P1's band was "expected to *fail downward* on $x, y, chi$; it is kept as written and will be scored as failed." Reading to be tested, not a rule: a Weil-type bound $|S_g(a,b)| <= C sqrt(p)$ for every $b$ sits below the random extreme $approx sqrt(r ln r)$, so orbit spectra are flatter than random; rule 2 fires only upward.

== Results #pu
Source: `results/run.json`, run F3-RUN-02, 15,500.9 s on 4 threads, shared local host. $eta$ columns are `eta_over_null_median` (coordinate function over the median of its symmetry-class null, 20 nulls); $lambda eta$ and PLANT $eta$ are raw; energy columns are `ratio_to_null_mean` and $z$ for $K_1 = ceil(r^(1"/"2))$, $K_2 = ceil(r^(2"/"3))$. P3: ratio within $1 plus.minus 3 "sd"/"mean"$, equivalent to $|z| <= 3$; true in all 36 pairs.

*R16 (8 curves, $p = 65521$)*

#include "figures/table_R16.typ"

*R20 (8 curves, $p = 1048573$)*

#include "figures/table_R20.typ"

*ANOM (2 anomalous curves)*

#include "figures/table_ANOM.typ"

The three $chi$ columns agree to within 0.001 on every curve, so their markers nearly coincide in the figure.

*Summary block (verbatim, `results/run-02.stdout.txt`, "numbers, not verdicts")*
#raw(read("figures/summary.txt"), block: true)

#figure(image("figures/eta_energy.svg", width: 100%),
  caption: [Left: $eta$ / median $eta$ of the matched symmetry-class null for $x, y, chi(x), chi(x+1), chi(x+2), D_n$ on each curve; green band: P1 $[0.8, 1.25]$; red line: rule 2 at 1.5. Right: energy $z$ per (curve, $K$) with the $plus.minus 3$ band (equivalent to $1 plus.minus 3 "sd"/"mean"$; recorded ratios 0.9948 to 1.0072). Run F3-RUN-02; 20 nulls per class, 64 values of $a$, 50 energy nulls; local shared host. Source: `results/run.json`; `figures/plot.py`.])

== Rule application (verbatim, session log 2026-11-12 00:05Z) #pu
Rules applied verbatim (§4, first applicable): rule 1 no (P2 holds, FFT bound holds); rule 2 no (no non-control g with $eta > 1.5 times$ null median on $>= 2$ curves of one size; P3 fails nowhere); *rule 3: closed* --- "no heavy Fourier coefficient and no additive-energy excess in the coordinate orbit for these function classes at r $<= 2^20$". P4 reads as P1 (ANOM coordinates flat; the anomaly lives in $lambda$, $eta = 1$). P1 as written fails: 40 of 48 pairs outside $[0.8, 1.25]$ on R16 and on R20, all below, as announced in the freeze note; on ANOM 10 of 10 below. P2 holds; P3 holds (0 of 36 outside).

== Reading (offered, not a result, not a rule) #pu
The $x$, $y$, $chi$ rows sit at 0.40--0.78 of their class-null medians: orbit spectra of these functions are flatter than random, consistent with a Weil-type bound $|S| <= C sqrt(p)$ for every $b$, which lies below the random extreme $approx sqrt(r ln r)$. The Dickson rows (degree 16 to 33) sit at 0.96--1.10, at the null: a reading is that the degree-dependent constant exceeds the random maximum, leaving no room to flatten them. Not tested by a separate experiment.

*Rule 3's own caveats.* The closure does not bound functions outside the listed classes, higher Gowers norms ($U^3$ and up), or larger $p$; $r <= 2^20$ only; one Dickson degree per curve.

== Limitations
Local host (4 vCPU, shared with three other running studies); the isolab-eks confirmatory run is open (no aws CLI in the session); counts exact, no timing result. 20 nulls per class, 64 values of $a$, 50 energy subsets. 8 + 8 + 2 curves; $c in {0,1,2}$ for $chi$. R16_1 at $K_2$ has $z = -2.94$, 0.06 inside the band (all others $|z| <= 2.22$); noted, not scored. No research-kb query; no novelty claim.

== Evidence pointers
#table(columns: (auto, 1fr), stroke: 0.4pt, inset: 3pt,
  [Card], [`CARD-coord-spectrum-20261110-F3-RUN-02`, sha256 `d09186447235f7da48e4e1e0d188c90141206dff54661cec1e733dc054c66f3a`],
  [Receipt], [`cards/receipts/F3-RUN-02.receipt.json` (completed, 13 commands, each one attempt, exit 0)],
  [Pinned head], [`9465664dbe6ba5757b9cb011192215a00605e8d8`],
  [Binary], [`tool/target/release/coord_spectrum` sha256 `7e08c12ec3865247299923e9f71398260dfe605b261ce66c5a78226496fe1c4e`],
  [run.json], [sha256 `642bf9cd48c7f11fabc73f29c7563e488c14742148c5aa4f948ea52daa8dc30a` (151,455 bytes)],
  [FREEZE.sha256], [sha256 `4891d5aa563392fe32dd9c3035c2d65c43bd749af8efca329853facb8775bf7a`; PROTOCOL.md `4bfd872a04d0e7b671446c6016a541f2e7384fdecc411e1c4d05a0810be2ec7f`; tool/src/main.rs `ecb877ef619ea88e80fbe096059cd43bfb4022f67ecc1e5b2f3fc73c208289a6`; tool/Cargo.toml `a3f4cf8b04cd89c2b071ba0c27f113cd7f5c54559d064dc5fda807239c76f52a`; tool/Cargo.lock `3cc4284b70d5c7155750195fc11e5a1c8b66d4fcb25cd9886682c0adc3cfd203` (all OK in the receipt)],
  [Tests], [7 passed, 0 failed (`F3-RUN-02.test.stdout`)],
  [Instrument], [Parseval max rel. deviation 8.68e-16; FFT vs naive max $|Delta S|"/"r$ 1.35e-15 (bound $10^(-6)$ holds); $lambda$ linearity violations 0],
  [Report build], [typst Python wheel 0.15.2 (scratch venv); figure by matplotlib 3.11.2],
)
