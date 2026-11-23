#set page(margin: 1.6cm)
#set text(size: 9.5pt)
#set heading(numbering: none)
#show raw: set text(size: 8.5pt)
= Genus-4 non-hyperelliptic images: trigonal-pencil and plane-quintic relations
2026-11-15. Study `research/low-gonality-genus4_20261115/`, plan frozen in `PLAN.md` before the first measured run (amendments dated in the plan). Local development check at small parameters on one shared 4-CPU VM, not a lab benchmark. Every number is `[pcc:unverified]`. No improvement is claimed; gaps to the matched rho are distances. Repo HEAD at run time `3c24520f7cdc1c47bac26cc7778d058e655b79a8`.
== Question
Diem-Kochinke predict exponent 2 - 2/(g-1) = 4/3 for non-hyperelliptic curves of genus g = 4 (hyperelliptic genus 4: 3/2; rho: 2) from the lowest-degree pencils: the g^1\_3 of a ruling of the quadric (3-point fibres) and the degree-5 plane model obtained by projecting the canonical curve from one of its points (lines meet it in 5 points). What do these two relation sources yield at small q, do they close a DLP in `Pic^0` of a smooth genus-4 curve with sparse linear algebra, and where does that sit against a matched rho?
== Answer in brief
- *Arithmetic layer built and tested.* `Pic^0` of a smooth (3,3) curve on P^1 x P^1 (canonical genus-4 model on the split quadric) by Riemann-Roch linear algebra, with canonical keys including the exceptional (coplanar) classes; 8 unit tests pass, among them the zeta-function group order killing random classes at p = 7 and 11 and the relation sums being equal in `Pic^0`.
- *Relation yields match the formulas.* Fibre splitting pooled fraction 0.1644 (P1 confirmed); line yield 12 cells, measured/naive in [0.58, 1.10]; outside +-30%: p=251 phi=0.3 ratio 0.58 (P2 refuted); duplicate factor unique/hits in [0.101, 0.104] over 15 cells (P3 confirmed).
- *DLP solved and verified.* 20 of 20 IC passes in 10 cells (p = 61, 127, 251, 509) gave x with x g = h; rho solved and verified in every run (P6 confirmed). The factor base needed: p=61: A+B insufficient (needed C), A+B insufficient (needed C), A+B insufficient (needed C); m used [62, 60, 58]; p=127: phi\*=1, phi\*=0.9, A+B insufficient (needed C); m used [129, 126, 116]; p=251: phi\*=0.8, phi\*=0.9, phi\*=0.9; m used [231, 211, 217]; p=509: phi\*=0.8; m used [382]; exponent of m\* over the p \>= 127 values where every cell was solved by A+B alone (p = 251, 509; 2 points only): 0.78; all-cell fit over p = 61..509 is in the scaling table.
- *Distance to rho.* See the table: IC is far below rho at zero-priced native work and at the calibrated price, and above it at 0.1 and 1 GAE per native operation, so inside the section 5a band the order flips and the ranking is *unresolved*, not a win (P7 confirmed).
- *Scaling.* Measured log-log slopes against p (all four p; restricted to p \>= 127 in brackets): rho GAE 1.66 +- 0.19 (4 values of p) [1.89 +- 0.31], IC group operations 0.02 +- 0.05 (4 values of p) [0.01 +- 0.11], IC native F\_p work 2.08 +- 0.08 (4 values of p) [2.23 +- 0.04], IC mod-ell linear-algebra work 1.33 +- 0.09 (4 values of p) [1.47 +- 0.11], factor-base size m 0.87 +- 0.03 (4 values of p) [0.81 +- 0.02]. The 4/3 exponent is not visible and cannot be: the cells have a constant-fraction factor base, no large primes, and 3 to 4 values of p. The IC group-operation count is flat (about 750 combines, dominated by the 16-adder table of the individual-log step, not by relations), so at w = 0 IC/rho falls like rho grows; the native work grows like p^2.2, a little faster than rho, which is why the break-even price w\* stays near 1e-3 instead of rising.
=== Distance to the matched rho (S\_IC / S\_rho, plain rho measured, mean over seeds [min, max])
#table(columns: 8, align: right, inset: 3pt, stroke: 0.4pt + gray,
[p],[cells],[w = 0],[w = w\_cal],[w = 10 w\_cal],[w = 0.1],[w = 1],[break-even w],
[61],[3],[0.161 [0.126, 0.191]],[0.165 [0.128, 0.195]],[0.194 [0.149, 0.226]],[40.4 [28.5, 47.1]],[234 [163, 275]],[0.00204 [0.00159, 0.00284]],
[127],[3],[0.0686 [0.0397, 0.0968]],[0.0729 [0.0426, 0.103]],[0.111 [0.0692, 0.157]],[54.4 [33.9, 77.4]],[306 [189, 439]],[0.00177 [0.00107, 0.00259]],
[251],[3],[0.0289 [0.0219, 0.0389]],[0.0349 [0.0277, 0.0469]],[0.0888 [0.0688, 0.118]],[86.3 [64.3, 115]],[479 [356, 640]],[0.00109 [0.000762, 0.00138]],
[509],[1],[0.00435 [0.00435, 0.00435]],[0.00904 [0.00904, 0.00904]],[0.0512 [0.0512, 0.0512]],[66.5 [66.5, 66.5]],[366 [366, 366]],[0.00136 [0.00136, 0.00136]],
)
GAE = one `combine` (about 2.67e+05 counted mulmods, 660 us here). w\_cal(F\_p mulmod) about 6e-06, w\_cal(mod-ell word op) about 1.2e-05 GAE: best-of-5 timings in the same binary on a loaded shared host. Native work is counted in field multiplications (IC) and one hash/DP operation per walk step (rho). The ratios use the final ladder rung (oracle choice of the factor base); `RESULTS.md` also gives the ratio including all failed rungs, a free-negation rho reference (plain/sqrt 2, assumed) and a row pricing a GAE at 2.6e3 mulmods (optimised arithmetic).
== Method
*Curve and group.* Smooth bidegree-(3,3) curves on P^1 x P^1 over F\_p (split quadric; K = O(1,1); smoothness certified by the partial derivatives generating all (8,8)-forms). A class is D - D0 with D0 four rational points; D is an effective degree-4 divisor carried by the 5-dimensional space H^0(2K - D) of biquadratic forms. `combine(Da,Db,Dc) ~ Da+Db-Dc` multiplies the spaces, finds f with f S\_2 in Wa Wb, finds g in S\_2 with g (Wa Wb) in f H^0(4K - Dc) (this makes div g contain R + Dc as a divisor; a first version with g merely in H^0(2K - Dc) failed when R met Dc), divides to get H^0(4K - Dc - D), and recovers H^0(2K - D). The Mumford-type spanning check falls back to level 3 (never needed in the runs). Coplanar D (h^0(D) = 2) gets the key of H^0(2K - E), E = plane section - D, so keys are canonical.
*Order.* N\_1, N\_2 by fibrewise root counting over F\_p, F\_{p^2}; (a3, a4) then bracketed by Weil's bounds and N found by interval baby-step giant-step and filtered by random classes. Unit-tested against exact N\_1..N\_4 at p = 7, 11.
*Relations.* Unknowns lambda(P) = phi(P - Pa) for P in the factor base F (Pa = first point of D0), plus kappa = phi(K - 6Pa), tau\_i = phi(T\_i - 3Pa). B: a fibre of either ruling with 3 distinct rational points all in F gives sum lambda = tau\_i; K = T1 + T2 adds kappa - tau\_1 - tau\_2 = 0. A: the plane through Pa, P1, P2 (P\_i in F) meets C in three more points; if all are rational and in F, sum of 5 lambdas = kappa (this is the line through two factor-base points on the projection of the canonical curve from Pa, a plane quintic with two nodes). C (not part of the lead): planes through three F points, used only when A+B leave the system rank deficient at F = all points.
*Linear algebra and logarithm.* Wiedemann on A^T D A over F\_ell (ell the largest prime factor of N, N/ell \<= 16, ell exactly dividing N), cross-checked against a dense kernel whose dimension must be 1. Individual logs by a random walk a g + b h until two decompositions into four rational points of F; the scale cancels in the ratio. Verified by x g = h.
*Matched rho.* Distinguished-point rho in \<g\>, 16-adding walk with the same `combine`, same g and h, same group. Negation costs one `combine` in this arithmetic, so the measured rho is plain.
== Results against the preregistered predictions
#table(columns: (auto, 1fr, auto, 2fr), align: left, inset: 3pt, stroke: 0.4pt + gray,
[id],[claim],[outcome],[evidence],
[P1],[fibres with 3 rational points = 1/6 +-25% (pooled p \>= 61)],[*confirmed*],[pooled fraction 0.1644; per p: 31:0.153, 61:0.166, 127:0.169, 251:0.167, 509:0.159],
[P2],[A-line yield per pair = phi^3/6 +-30% for |F| \>= 60],[*refuted*],[12 cells, measured/naive in [0.58, 1.10]; outside +-30%: p=251 phi=0.3 ratio 0.58],
[P3],[unique A relations = hits/10 +-15%],[*confirmed*],[unique/hits in [0.101, 0.104] over 15 cells],
[P4],[A+B insufficient at p=61; phi\* ~ 1.0, 0.8, 0.67 at p=127, 251, 509 (+-0.1); exponent of m\* in [0.6, 0.9]],[*refuted*],[p=61: A+B insufficient (needed C), A+B insufficient (needed C), A+B insufficient (needed C); m used [62, 60, 58]; p=127: phi\*=1, phi\*=0.9, A+B insufficient (needed C); m used [129, 126, 116]; p=251: phi\*=0.8, phi\*=0.9, phi\*=0.9; m used [231, 211, 217]; p=509: phi\*=0.8; m used [382]; exponent of m\* over the p \>= 127 values where every cell was solved by A+B alone (p = 251, 509; 2 points only): 0.78; all-cell fit over p = 61..509 is in the scaling table],
[P5],[level-3 fallback in \< 1% of combine operations],[*confirmed*],[0 fallbacks in at least 1517355 combine calls (rho only counted)],
[P6],[IC and rho solve and verify (x g = h) at every cell],[*confirmed*],[10 cells; IC unsolved passes []; rho cells with unverified runs []],
[P7],[S\_IC/S\_rho \< 1 at w=0 and w\_cal, \> 1 at w=0.1 and w=1],[*confirmed*],[p=127 s1: 0.0968/0.103/77.4/439; p=127 s1: 0.0867/0.0924/71.6/406; p=127 s2: 0.0695/0.0733/51.8/291; p=127 s2: 0.0635/0.0685/66.4/373; p=127 s3: 0.0397/0.0426/33.9/189; p=127 s3: 0.0333/0.0359/30.2/168; p=251 s1: 0.0219/0.0277/79.5/440; p=251 s1: 0.0191/0.0275/112/618; p=251 s2: 0.0258/0.0301/64.3/356; p=251 s2: 0.0193/0.0231/57.3/317; p=251 s3: 0.0389/0.0469/115/640; p=251 s3: 0.0335/0.0411/110/611; p=509 s1: 0.00435/0.00904/66.5/366; p=509 s1: 0.00582/0.0108/69.9/385; p=61 s1: 0.167/0.171/45.5/263; p=61 s1: 0.137/0.14/38.2/221; p=61 s2: 0.191/0.195/47.1/275; p=61 s2: 0.18/0.184/46.2/270; p=61 s3: 0.126/0.128/28.5/163; p=61 s3: 0.128/0.13/29.5/169],
[P8],[A/A within +-30% (IC pass 2 vs 1, GAE and native); rho halves within 2 s.e.],[*refuted*],[p=127 s1: GAE x0.90, native x0.93, rho z=0.91; p=127 s2: GAE x0.91, native x1.28, rho z=-0.85; p=127 s3: GAE x0.84, native x0.89, rho z=-0.35; p=251 s1: GAE x0.87, native x1.40, rho z=-0.67; p=251 s2: GAE x0.75, native x0.89, rho z=0.26; p=251 s3: GAE x0.86, native x0.95, rho z=-2.07; p=509 s1: GAE x1.34, native x1.05, rho z=None; p=61 s1: GAE x0.82, native x0.84, rho z=-0.16; p=61 s2: GAE x0.94, native x0.98, rho z=-0.93; p=61 s3: GAE x1.02, native x1.04, rho z=1.71],
[P9],[native F\_p cost of IC ~ p^[2.0,2.6]; IC group-op count roughly flat in p],[*confirmed*],[native\_p slope 2.0849355031522188, group\_ops slope 0.021581215581848192; rho GAE slope 1.6607609295278303],
)
=== Fibre splitting and line yield
#table(columns: 4, align: left, inset: 3pt, stroke: 0.4pt + gray,
[p],[curves],[fibres with 3 points],[A-line yield/pair at phi = 0.5, 0.7, 1.0 (measured/naive)],
[31],[16],[0.1533 \[0.1158, 0.1909\]],[0.0100 (0.46) ; 0.0570 (0.95) ; 0.1502 (0.90)],
[61],[16],[0.1663 \[0.1419, 0.1907\]],[0.0174 (0.81) ; 0.0565 (0.96) ; 0.1734 (1.04)],
[127],[16],[0.1694 \[0.1502, 0.1886\]],[0.0177 (0.84) ; 0.0570 (0.98) ; 0.1665 (1.00)],
[251],[12],[0.1675 \[0.1549, 0.1801\]],[0.0196 (0.94) ; 0.0581 (1.01) ; 0.1686 (1.01)],
[509],[8],[0.1591 \[0.1487, 0.1695\]],[0.0201 (0.96) ; 0.0542 (0.94) ; 0.1587 (0.95)],
)
=== Factor base and cost per cell
#table(columns: 11, align: right, inset: 2.5pt, stroke: 0.4pt + gray, [p],[seed],[N],[ell],[phi\*],[m],[rows/cols],[GAE],[native F\_p],[native mod ell],[rho GAE mean (min-max; runs)],
[127],[1],[263162128],[16447633],[1],[129],[166/131],[734],[5.42e+06],[9.8e+05],[7586 (2807-13751; 8 runs)],
[127],[2],[285868647],[31763183],[0.9],[126],[149/127],[736],[5.14e+06],[8.61e+05],[10590 (3446-21090; 8 runs)],
[127],[3],[237295559],[237295559],[1+C],[116],[358/118],[789],[5.46e+06],[1.93e+06],[19897 (2898-31725; 8 runs)],
[251],[1],[4587199576],[573399947],[0.8],[231],[321/232],[802],[2.86e+07],[3.37e+06],[36551 (17520-63407; 6 runs)],
[251],[2],[3700231295],[740046259],[0.9],[211],[271/213],[967],[2.38e+07],[2.63e+06],[37490 (20922-55775; 6 runs)],
[251],[3],[3806870030],[380687003],[0.9],[217],[307/218],[836],[2.41e+07],[2.98e+06],[21487 (7362-32606; 6 runs)],
[509],[1],[62950418363],[8992916909],[0.8],[382],[558/384],[760],[1.18e+08],[9.62e+06],[174836 (45942-303736; 3 runs)],
[61],[1],[14231047],[14231047],[1+C],[62],[210/64],[801],[1.74e+06],[6.23e+05],[4789 (2051-6878; 8 runs)],
[61],[2],[13441646],[6720823],[1+C],[60],[198/62],[770],[1.48e+06],[5.7e+05],[4030 (2537-6304; 8 runs)],
[61],[3],[13103087],[13103087],[1+C],[58],[178/60],[700],[1.22e+06],[4.97e+05],[5568 (3032-8959; 8 runs)],
)
=== Honest projection of the crossover against rho's q^2
Break-even price w\* (S\_IC = S\_rho) by p: measured implementation 61: 0.00204, 127: 0.00177, 251: 0.00109, 509: 0.00136; root-finding model 61: 0.000828, 127: 0.000693, 251: 0.000577, 509: 0.00111.
Log-log slope of w\* vs p, measured implementation: -0.24 +- 0.13. A negative slope means the native work of this implementation grows faster than rho's GAE count, so the break-even price falls with p and there is no crossover at w \>= 0.1 on this route.
No crossover can be projected for the measured implementation: the slope is negative (or its 2-sigma interval reaches zero), so w\* stays near 1e-3 and never approaches the band prices 0.1 and 1 as p grows.
Root-finding model (each A pair costs 2000 mulmods instead of the scan of F; model, not measured): slope 0.10 +- 0.20.
The root-finding model's slope is not distinguishable from zero (2-sigma interval includes 0), so its crossover with w = 0.1 or 1 cannot be projected from these cells either; extrapolating the point estimate would give p beyond 2^80, which is not a result.
Reading: at w = 0 and at the calibrated price the IC route already beats this arithmetic's rho at every measured p (distance below 1), so a crossover is only meaningful at band prices. The measured relation search scans all of F for each pair (cost m per pair, m^3 in total), which grows like m^3, measured p^2.2 (p \>= 127: 2.23 +- 0.04) against rho's q^2 (measured slope 1.9 +- 0.3 over p \>= 127; the cofactor of ell moves single cells by up to sqrt 16); Diem's analysis replaces the scan by root finding (about log q per pair) and adds two large primes, neither of which the 3 to 4 measured values of p can validate. The 4/3 exponent is therefore a literature figure here, not a measurement.
=== Figure
#image("figure.svg", width: 100%)
== Limitations
- Split quadric only (two g^1\_3 over F\_p); the non-split quadric (no rational pencil) and the cone are not built, so these cells say nothing about curves of those types.
- No large primes, no sieving, no Diem two-large-prime step: the factor base is a constant fraction of all rational points (phi\* between 0.7 and 1), so the p^0.75 factor-base size of the plan is an observed trend over 3-4 values of p, not an asymptotic result.
- p \<= 509 (N up to about 7e10). One to three curves per p; rho varies with ell between curves, and rho runs per cell are 3 to 8, so the S\_rho interval is wide.
- The arithmetic is generic linear algebra (about 2.6e5 mulmods per combine, 0.65 ms); rho in this arithmetic is slower than rho on a Cantor-type Jacobian, which moves w\_cal. The sensitivity row (GAE = 2.6e3 mulmods) shows the effect on the calibrated ratio. Rho is plain (no free negation); the free-negation reference is plain/sqrt 2, an assumption.
- The dense kernel-dimension monitor used to decide when to add C relations is not priced; the C relations themselves (general hyperplane sections) are not part of the Diem-Kochinke lead and were needed only at p = 61 (and where noted in the table).
- Group order N is found by BSGS in the Weil range with random-class filtering (validated by exact zeta counts only at p = 7, 11); at larger p the check is that N kills 3 further random classes and that g, h verify. A wrong N would be exposed by the mandatory x g = h check.
- A first batch of cells was discarded after finding a rank-monitor flaw (see PLAN amendments); the final cells are a full re-run with the final source (cargo reports the release binary up to date). Re-running `dlp 61 1 8` afterwards reproduced `results/dlp_p61_s1.json` exactly except for timings (`results_repro/`).
- Everything here is `[pcc:unverified]`: no independent replay, no validator report, no VALIDATION.json; nothing in this directory is a leaderboard or benchmarks row.
== Exact repro command
```
cd /home/user/crypto && CARGO_TARGET_DIR=/home/user/crypto/target cargo build --release --bin genus4_pencil_ic -j2 && cd research/low-gonality-genus4_20261115 && B=/home/user/crypto/target/release/genus4_pencil_ic && for cfg in '31 16' '61 16' '127 16' '251 12' '509 8'; do set -- $cfg; $B yield $1 $2 20261115 results/yield_p$1.jsonl; done && for s in 1 2 3; do $B dlp 61 $s 8 results/dlp_p61_s$s.json; done && for s in 1 2 3; do $B dlp 127 $s 8 results/dlp_p127_s$s.json; done && for s in 1 2 3; do $B dlp 251 $s 6 results/dlp_p251_s$s.json; done && $B dlp 509 1 3 results/dlp_p509_s1.json && python3 analyze.py && python3 plot.py && python3 make_report.py
```
Smallest cell: `genus4_pencil_ic dlp 61 1 8 OUT` (about 30 s). Unit tests: `cargo test --release --bin genus4_pencil_ic -j2`. PDF: `/tmp/report-venv/bin/python -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"`.
== Sources read
- `research/genus4_n4_census_20261024/REPORT.md` (genus-4 non-hyperelliptic Jacobians in the isogeny class of Res E; Theorem 6 an unreviewed agent proof; no map computed)
- `src/cryptanalysis/plane_quartic_pic.rs` (header and API: the Riemann-Roch linear-algebra pattern generalised here)
- `src/bin/quartic_ic.rs`, `src/cryptanalysis/genus_ladder.rs` (headers only), `benchmarks/20261026-humbert-genus5-cover/` (file list; the cautionary precedent was not re-read in detail)
- `AGENTS.md` section 5a (pricing of unpriced native work), `CLAUDE.md` (session rules)
- `research/quartic-diem-sparse_20261115/make_report.py` and `REPORT.typ` (report-building pattern only)
- Not consulted: research-kb, ecdb, lead atlas (not available in this run); literature figures (Diem 2006, Diem-Kochinke) are quoted from the task text and unverified here.
