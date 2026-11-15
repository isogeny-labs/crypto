#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#show raw: set text(size: 8pt)
#let pu = text(size: 7pt)[`[pcc:unverified]`]
= F2: linear complexity and DFT support of sequences from the $p^2$-lift of a prime-field curve

Run F2-RUN-02 of study `p2_lift_linear_complexity_20261110`. Class: boundary diagnostic; exact integer computation, no S, no rho row, no leaderboard change. *Status: closed by decision rule 3.* Evidence standing: local evidence; §9a confirmatory EKS run open. All numbers below are #pu (no pcc claim id exists for this run). No improvement over rho, or over anything, is claimed.

== Question and mechanism
Follow-up to H110 (`p2_lift_defect_20261101`), which closed only the coordinate-section scalar-polynomial bridge. For $G$ of prime order $r$ and $a_k = f([k]G)$, where $f$ is the formal-layer defect of the Hensel section ($tau_0 = r f$), the sequence is $r$-periodic over $F_p$. With $m = op("ord")_r (p)$, its linear complexity is $L = [op("mean") != 0] + m c$, where $c$ is the number of nonzero degree-$m$ components of the DFT over $F_(p^m)$ and $N = (r-1)/m$ (Blahut; standard, not formalized). A sparse DFT ($c << N$) would make $f([k]G)$ a sparse exponential polynomial in $k$ and transfer the ECDLP to a DLP in $mu_r subset F_(p^m)^*$. Question: is $c(f_0)$ or $c(op("torsion")_u)$ below the null (uniform random periodic sequence) on any curve?

== What was run
21 curves: H (the 7 H110 curves), R12, R14, R16 (4 fresh prime-order curves each) and S (2 supersingular curves, $m = 2$); multipliers ${1,2,3,5}$; observables O1 $f_0$, O6 $op("torsion")_u$ (gated), O2 $f_1$, O3 $f_1 - f_0$; 200 null sequences per curve; plant O5 on S. 336 rows. Card `CARD-p2-lift-linear-complexity-20261110-F2-RUN-02` (receipt completed, 13 commands, rc 0), pinned head `9465664d`. Local host, 4 CPUs, started 2026-11-11T19:39Z, finished 2026-11-12T01:25Z (20,761.7 s). Tier deviation: the main session ran the card itself (`executor-mechanical` rate-limited). F2-RUN-01 was lost to a container restart (`benchmarks/losses/…f2-run-01…`) and relaunched as RUN-02.

== Frozen predictions and rules (PROTOCOL.md §6, §7)
*P1* (H, R12, R14, R16): $c("O1") = c("O6") = N$, $L in {r-1, r}$. *P2* (S): $c = (r-1)/2$. *P3* O3 behaves as P1/P2. *P4* O5 gives $L = 2$; O7 drops $L$ by 1 iff mean $!= 0$. *P5* A/A: identical $(L, c)$ across multipliers. Prior that a gated observable breaks P1 or P2: $<= 0.02$.
+ *Instrument failure:* P4 or P5 fails, or any H110 self-check fails.
+ *Lead alive:* a gated observable has $c < N$ on a curve with $N p^(-m) < 10^(-3)$, or on S $c <= (r-1)/2 - 3$.
+ *Closed:* otherwise.

== Results #pu
Source `results/run.json` (21 curves x 4 multipliers x 4 observables). All 336 rows pass every self-check (c integer, two $tau_0$ splits agree, $tau(op("torsion")_u) = 0$, O1 $L_7 = L - [op("mean") != 0]$). *P1, P2, P3 hold:* every observable has $c = N$ exactly and $L in {r-1, r}$ (O1, O2, O3: $L = r-1$; O6: $L = r$). *P4 holds:* O5 $L = 2$, $c = 1$ on S_0 and S_1. *P5 holds:* identical $(L, c)$ across the four multipliers on 21 of 21 curves. *Empirical null:* 0 of 4,200 random sequences has $c < N$; $c_min = c_max = N$ on every curve. Cells are $L slash c$ at multiplier 1; $c slash N = 1.0$ for all four observables.

*H (7 curves)*
#include "figures/table_H.typ"
*R12*
#include "figures/table_R12.typ"
*R14*
#include "figures/table_R14.typ"
*R16*
#include "figures/table_R16.typ"
*S ($m = 2$)*
#include "figures/table_S.typ"

On S, $c = (r-1)/2 = 2724$ and $2715$; the rule-2 threshold $(r-1)/2 - 3$ is $2721$ and $2712$.

#figure(image("figures/support_L.svg", width: 100%),
  caption: [Left: $c slash N$ per curve for O1, O6, O2, O3 (all 1.0), the null band $c_min .. c_max$ over $N$ (a point at 1) and on S the rule-2 threshold. Right: $L - (r-1)$: 0 for O1, O2, O3 and 1 for O6. Multiplier 1 drawn; the other multipliers coincide. Source: `results/run.json`; `figures/plot.py`.])

#figure(image("figures/pvanish.svg", width: 78%),
  caption: [$log_10 P_"vanish"$ (union bound $N p^(-m)$) per curve against the rule-2 gate $10^(-3)$ (symlog axis). All 21 curves lie below it; closest: S_0 ($-5.59$) and S_1 ($-6.19$).])

== Cost counts #pu
Generation is shared per (curve, multiplier), so it is summed over the 84 pairs, not the 336 rows (row sums are exactly 4.0x): `hensel_lifts` 5,726,624; `inversions_mod_p2` 236,445,680; `inversions_mod_p` 8,589,852; `lifted_adds` 71,525,800; `lifted_doublings` 159,193,256. Berlekamp--Massey over 336 rows: `bm_multiplications` 739,548,609,053; `bm_inversions` 14,315,528 (plant: 43,522 multiplications). Wall time is a shared-host figure, not a result.

== Decision (PROTOCOL.md §7, verbatim) #pu
Rule 1: no (P4, P5 and all self-checks pass). Rule 2: no (no curve has $c < N$; on S, $c = 2724$ and $2715$ are not $<= (r-1)/2 - 3$). *Rule 3 applies:*
#quote(block: true)[3. *Closed:* otherwise. Reading: the first formal layer's defect has a full DFT support on every curve measured; a sparse-exponential-polynomial transfer through the $p^2$-lift is closed for the fixed-$x$ section, the $u = 1$ section and the canonical torsion lift's $x$-digit, at these sizes.]
Caveat paragraph (verbatim): #quote(block: true)[Rule 3 does not establish a generic lower bound, says nothing about higher formal layers ($p^3$, H112) or about sections that fix other coordinates, and is a measurement at toy sizes ($r <= 2^16$).]

== Limitations
For most curves $N$ is 1 to 8, so the test has resolving power mainly on R14_2 ($N = 21$) and S ($N approx 2700$, where the null vanishing probability is only $2.6 times 10^(-6)$ and $6.4 times 10^(-7)$). The four multipliers are an A/A check, not four samples. Local shared host; the isolab-eks confirmatory run (byte diff of `run.json`) is open; tier deviation recorded. Pre-freeze `--quick` runs were disclosed (§11) and not scored. No matched-rho comparison applies; no research-kb query; no novelty claim.

== Evidence pointers
#table(columns: (auto, 1fr), stroke: 0.4pt, inset: 3pt,
  [Card], [sha256 `8dbad5d02c190f8196c17684e8ed59bbd5c8fd009f61c1b863afc1d4ae05bd1a`; receipt `cards/receipts/F2-RUN-02.receipt.json`],
  [Protocol], [sha256 `a9e727d29fca88dded3f9f5a2939e32dd23b8e1b10c6b0d93f35a27b6e185ebb`],
  [Tool source], [`tool/src/main.rs` sha256 `72dadcb9416a7facb4e6cdb6eb9aa0794ebe646179481d370c4f8d5eaf1d1fe6`],
  [Binary], [sha256 `172f1f4dc2db4e0a0d263ad0de4f4439d6d750fa2dce08e0dfe82ebad2ea19d2`],
  [run.json], [sha256 `21591ebbafaea0477b4aeb7cd2fdb9caae00c6c7f77902373e90e5332340bf1e` (523,764 bytes)],
  [Tests], [7 passed, 0 failed],
)
