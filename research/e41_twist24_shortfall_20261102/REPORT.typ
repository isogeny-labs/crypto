#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E41: the order-24 walk's step shortfall on E18's $j = 0$ curves — sampling noise, not a walk defect; the corrected reference

*Run report and results, 2026-11-02.* Note §9.52 (pre-registration, pushed `f99a56359e` before any fix code or registered run) and §9.53 (results). Stage 0 code `089f7f7823` / `6115e0ea0a`; fix and harness `b0e307d9b8` (`src/cryptanalysis/twist24_shortfall.rs`, option `Jumps`, default `Jumps::E34`; `examples/e41_run.rs`), tables `examples/e41_tables.rs` at `5d15482969`. Binaries built from clean `git archive` exports (blob-checked, 0 mismatches), sha256 in `experiments/48_twist24_shortfall/binary_sha256.txt`. Records `experiments/48_twist24_shortfall/` (the three records over 1 MiB are kept by sha256; the store push needs the `aws` CLI, absent here). Host: the shared 4-vCPU container, one core per process, isolab-eks unreachable; *operation counts only*, no wall-time claim. Python only lays out recorded values (`figures/plot.py`); every number comes from the Rust examples.

== Question
E34 (§9.39) measured the cheap order-24 $chevron.l plus.minus psi chevron.r$ walk at $1.701 plus.minus 0.111$ times fewer steps than the order-6 walk (16 walks per curve), against the $sqrt(24 slash 6) = 2$ of ideal walks; its $1$ GAE lead over the cheap order-6 walk ($1.026$) was within noise. Where does the shortfall come from, and does a fix make the walk measurably cheaper across the §5a band? The walk is the matched rho reference: a cheaper walk moves the reference, never an index-calculus cost.

== Stage 0: the shortfall by cause (counts only, before the registration)
E34's stored walks put the order-6 walk at $0.924 plus.minus 0.046$ of its ideal $sqrt(pi (n-1) slash 2m)$ and the order-24 walk at $1.086 plus.minus 0.045$ (128 walks each): the $1.701$ is the two arms' sampling errors in opposite directions. On 512 fresh walks each (seeds 41000–41063) the instrument finds order 6 at $1.027 plus.minus 0.023$ and order 24 at $1.038 plus.minus 0.023$. By cause: distinguished-point losses $0$ (exhaustive storage); fruitless cycles $0.84 dot 10^(-4)$ revisits a step for order 24 ($98 %$ 2-cycles), costing $< 10^(-4)$ of the ideal; look-ahead redirects $1 slash r$ in both orders; representative ties ($x_0 = 0$ inputs) cost no step. No order-24-specific cause above $0.1 %$. The only non-noise component is a small excess common to both orders, consistent with the r-adding factor $1 slash sqrt(1 - 1 slash r)$.

#figure(image("figures/stage0.svg", width: 92%), caption: [Stage 0: steps/ideal − 1 and the share of each cause, 512 fresh walks per order (bars 95 %), with E34's stored order-24 walks for comparison. Source: `stage0_run.json`, `stage0_records.json`.])

== Fix and arms
Fix F (chosen from Stage 0, null prediction): E34's 32 jumps plus $R_(32+j) = R_j + R_((j+1) mod 32)$, $r = 64$; added work $32$ group additions and $64$ mod-$n$ operations a walk, charged. Arms on E18's eight curves: O6B (E29's order 6), O6C (cheap order 6), O24 (E34's cheap order 24), O24L (its mul-lean twin), O24F (O24 with F), and the A/A arm O24′ (O24 on a disjoint seed stream). 128 walks per curve and arm, schoolbook (seeds 48000+k; A/A 49000+k), 32 per curve in Karatsuba. Band: $F_p$ multiplications (set-up included, $[d]G$ excluded) $+ w dot 97 dot$ native operations, $w in {0, 0.1, 1}$, pooled $sum "cost"/sqrt(n)$. Resolved = 95 % interval excludes 1 at all three weights and $|"ratio" - 1|$ exceeds the A/A deviation.

== Checks
- *Identity (fix off)*: 1,280 of 1,280 E34 Stage R records recomputed byte-identical (serialized); E27's walk code (`quartic_folds.rs`) is untouched by every E41 commit.
- *Correctness*: 0 wrong logs in 7,424 walks (6,144 schoolbook, 1,280 Karatsuba).
- *Positive control* (F with $b_(j+1)$ dropped): wrong log flagged on 32 of 32 walks.
- *Added cost of F*: exactly 32 additions and 64 mod-$n$ operations on all 1,280 F walks; zero on every other arm. Redirects fall to $1 slash 64$, fruitless revisits $3.6 times$.
- rustfmt (`--edition 2021`) and clippy (`-D warnings`) clean on E41's files.

== Steps against the ideal (schoolbook, 1,024 walks per arm)
O6B $0.969 plus.minus 0.016$; O24 $1.006 plus.minus 0.016$; O24F $0.999 plus.minus 0.016$; O24′ $0.987 plus.minus 0.016$. Steps ratio order 6 / order 24 pooled per $sqrt(n)$: $1.927$ (main stream), $1.965$ (against the A/A stream); E34 $1.701$.

#figure(image("figures/shortfall.svg", width: 100%), caption: [Steps over the ideal per curve: E34's 16 stored walks (orange) and E41's 128 fresh walks (blue), bars 95 %. Source: `stage0_records.json`, `summary.json`.])

== The band (pooled; > 1: the second arm cheaper; 95 % intervals)
#table(columns: 6, align: (left, left, right, right, right, left),
  [ratio], [arith.], [0 GAE], [0.1 GAE], [1 GAE], [resolved],
  [O6B / O24], [school], [1.761 [1.686, 1.836]], [1.418 [1.356, 1.481]], [1.275 [1.218, 1.332]], [yes (8/8 curves)],
  [O6C / O24], [school], [1.696 [1.623, 1.768]], [1.320 [1.262, 1.378]], [1.162 [1.110, 1.214]], [yes (7/8; 1 unresolved)],
  [O24 / O24F], [school], [1.021 [0.978, 1.064]], [1.019 [0.975, 1.064]], [1.018 [0.973, 1.064]], [no],
  [O24 / O24′ (A/A)], [school], [1.019], [1.019], [1.020], [floor],
  [O6B / O24], [Karatsuba], [1.711], [1.369], [1.249], [yes],
  [O6C / O24], [Karatsuba], [1.635], [1.267], [1.137], [yes],
  [O24 / O24F], [Karatsuba], [0.984], [0.981], [0.979], [no],
)
E34 (16 walks): O6B / cheap $1.563, 1.254, 1.126$; O6C / cheap $1.505, 1.167, 1.026$.

#figure(image("figures/band.svg", width: 92%), caption: [Pooled cost ratios across the §5a band with 95 % intervals, schoolbook, 8 curves × 128 walks; hollow diamonds are E34's 16-walk values. Source: `summary.json`.])

== Verdicts (§9.52)
G2.216, G2.217, G2.218 (kill gates) hold. *G2.219 falsified*: O6B ran at $0.969$ of its ideal, under the registered $[0.98, 1.08]$; O24 $1.006$ and the steps ratio $1.927$ are inside. G2.220 holds (A/A $1.019$–$1.020$). G2.221 holds (F $0.75 %$ fewer steps; theory $0.8 %$). G2.222 holds: F is not measurably cheaper (null). G2.223 and G2.224 hold: the corrected reference is resolved at every weight against both order-6 walks, including $1.162$ at $1$ GAE against the cheap order-6 walk (E34: $1.026$, within noise). G2.225 (7/8 curves) and G2.226 (Karatsuba, same orders) hold.

== Consequences
*E18's $psi$ rows (reference correction).* Against E29's order-6 walk, the band-ranked order-24 walk is $1.761 times$, $1.418 times$ and $1.275 times$ cheaper at 0, 0.1 and 1 GAE (E34: $1.563$, $1.254$, $1.126$). E18's $psi$-arm distances to rho lengthen by these factors, and the modelled handover moves later by $3.27$, $2.02$ and $1.40$ bits (E34: $2.58$, $1.31$, $0.68$). The unranked point estimates for the cheapest arm at each weight are $1.838$ (O24L), $1.446$ and $1.298$ (O24F).
*IC rows (E31 solver A/B with E27 products, 20 rows on the GLS curves).* Order 24 does not apply there ($"Aut" = {plus.minus 1}$), and E41 changes no walk on those curves, so their reference stays walk (iii) and every distance is unchanged (factor 1). Both columns are listed in `tables.md`: for example, $p = 101$ seed 1 arm A gives $7.664 dot 10^4$, $2.672 dot 10^5$, $1.371 dot 10^6$ before and after. Rho is ahead of every row at every weight.

== What it says
E34's step shortfall was sampling noise in a 16-seed design. Both walks sit within a few per cent of their own $sqrt(|G|)$ ceiling, and no order-24-specific defect exists to fix. F, which targets the r-adding excess common to both walks, is not measurably cheaper. The matched rho on E18's $j = 0$ curves is stronger than E34 reported, and resolved at every weight. *Class: a reference correction*, not an index-calculus improvement: E18's rows move further from rho, and the GLS IC rows do not move. No leaderboard, `benchmarks/` or validation-gate change is made here.

== Limitations
One host, no isolation tier, operation counts only. Steps are random: the per-arm SE is $1.6 %$ at 1,024 walks, so effects below about $3 %$ (F's) are not resolvable. Curves run from 32 to 40 bits, and the set-up share shrinks with $n$. The records over 1 MiB are not in the store (no `aws` CLI); they are identified by sha256. Unit tests were not run in this session (release unit-test builds are excluded here); the identity run covers the instrument's and the option's equality with E34.
