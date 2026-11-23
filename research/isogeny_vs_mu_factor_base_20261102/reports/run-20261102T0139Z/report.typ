#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
#show table: set text(size: 8.5pt)

= Isogeny-kernel vs $mu_k$ factor bases for $m = 2$ decomposition

Runs 2026-11-02, from 01:39Z. Branch `claude/isogeny-kernel-factor-base-20261102`.
Protocols are frozen at `de0b5628` (`PROTOCOL.md`, Q1–Q3) and `e2cb7331`
(`PROTOCOL_Q4_SEARCH.md`, Q4). The task cards CARD-1 and CARD-2 were executed by the
mechanical tier and assessed by the director (`cards/`).

*Scope.* This is a component study at $m = 2$ and toy primes ($p <= 40961$). It looks at
how a factor base is *presented* to a Gröbner solver. There is no ECDLP cost claim and no
rho row. The distance remaining is stated against the $mu_k$ reference.

*Instrument.* The engine is a native Rust Buchberger (normal strategy, both Buchberger
criteria) identical for every arm. The primary unit is `termops`, a deterministic count:
one per coefficient multiply-subtract on a reductor term or an S-polynomial term. Wall time
is recorded but unadmitted, because the host was a shared cloud container with no isolation
tier. The EKS route was not reachable from the session (no AWS identity), so both runs were
local. Every primary column is host-independent, and a replay elsewhere must reproduce it
byte for byte. Gates: 6144 + 6828 systems, with *0* brute-force/evaluation gate failures.

== Results at a glance

#table(columns: (auto, 1fr), stroke: 0.4pt,
  [*Q1 yield*], [*Null in 15/15 cells* (frozen verdict `null_H1_0_retained`). The median
  normalised yield $r$ of the isogeny V minus that of a random V lies between $-0.020$ and
  $+0.029$, with $|Delta|/r <= 3.3%$. The positive control is valid in every cell
  ($r_"PC"$ = 0.06–0.49 vs $r_R$ = 0.87–1.02). The user's 7/8 vs 3/8 was noise.],
  [*Q2 cost*], [The user's slowdown reproduces: lex $B\/A_0$ = 6.4–11.7 in 16/16 cells.
  The frozen H2 verdict is *inconclusive*: the rule held in 8/16 grevlex comparisons, with
  0 falsifying. All 8 misses are in the "nodec" stratum, where $B\/A_1$ = 1.58–1.77
  falls under the pre-set 2× threshold. Descriptively, Möbius-presented $mu_k$ (A2)
  costs 0.97–1.00× the Vélu chain (B) in every grevlex and lex cell. The whole gap
  follows the presentation, not the set V.],
  [*S1 density*], [A0 and A1 have exactly 1.000 nonzeros per product. A2 has 3.11 ($k=3$)
  and 5.44 ($k=4$); B has 3.27 and 5.75. Prediction holds.],
  [*Q3 terminal*], [With the order-4 terminal, B is radical, with $|V| = 2^k$ in $k$ steps.
  The user's 2-torsion terminal (B2) is non-radical: $2^k$ distinct points of total
  multiplicity $2^(k+1)$ in $k + 1$ steps. B2 costs 7.3–12.9× B in grevlex and 5.3–44×
  in lex. At $k = 4$ the lex values are lower bounds: all 384 of those systems were censored
  at the budget, and no other Q2 or Q4 row was censored. Prediction holds.],
  [*Q4 lex*], [*Improvement, 16/16* (frozen verdict). Coordinates selected on a disjoint
  split cut termops to *0.21–0.30×* native Vélu on held-out targets. The gap to $mu_k$
  shrinks from 6.4–11.7× to *1.9–2.6×*.],
  [*Q4 grevlex*], [*No improvement, 0/16*: selected representation / native = 0.82–1.17.
  The S1-optimal coordinates are 0.92–1.24×. The inverse-variable form is worse
  (1.04–1.87×). The gap to $mu_k$ stays at 1.5–2.2×.],
  [*Kernels*], [Flat. Native density across 40+ kernels per cell ranges over 3.20–3.27
  ($k=3$) and 5.66–5.75 ($k=4$), and the best-density kernel costs 0.98–1.01× native.
  Four $j = 0$ kernels (found only at $p = 12289$) match the random kernels. No
  $j = 1728$ curve with a point of order $2^(k+2)$ was found.],
  [*Obstruction*], [None of the 11811 valid Vélu representations reached density 1 (minimum
  1.359), as the ramification-alignment argument predicts (§4).],
  [*Dickson*], [The $x$-only $GG_m$ chain $t' = t^2 - 2$ costs 0.98–1.03× $mu_k$ in both
  orders, despite density 1.69/2.33. So working only on the $x$-line is not what costs;
  the cost is elliptic.],
)

== Q1: yield
#figure(image("figures/q1_yield.svg", width: 100%), caption: [Median normalised yield
$r = |D|\/E_0$ per arm and cell (`q1_cells.csv`). The red bar is the 95% bootstrap CI of
$"median"(r_B) - "median"(r_R)$, drawn at $"median"(r_R)$.])

== Q2 and S1: presentation explains the cost
#figure(image("figures/q2_ratios_grevlex.svg", width: 100%), caption: [Grevlex: median
termops relative to A1 (`q2_cells.csv`). The dashed line is the pre-registered 2×
threshold.])
#figure(image("figures/q2_ratios_lex.svg", width: 100%), caption: [Lex, the user's
order. At $k = 4$, every lex B2 system hit the $2 dot 10^7$ termops budget (censored), so
those B2 bars are lower bounds.])
#figure(image("figures/s1_density.svg", width: 70%), caption: [Nonzero structure constants
per product in the chain quotient algebra (`s1_structure.csv`).])

== Q4: the representation search
*Why density 1 is out of reach.* A step is monomial ($u' = lambda u^2$) in suitable
coordinates only when its two ramification points sit at $0$ and $infinity$ and its branch
points are the next step's ramification points. For a 2-isogeny $x$-map with kernel
$chevron.l T chevron.r$:
- the ramification points are $x(1/2 T)$, which are 4-torsion;
- the branch points are 2-torsion abscissas of the codomain.

The next step ramifies at the halves of its own kernel point, again 4-torsion, so the
alignment always fails. For $ell >= 3$ the $x$-map has ramification index $<= 2$, so a
monomial step is impossible. This is a written argument with an empirical check
($0$ of 11811); a Lean proof is follow-up work.

#figure(image("figures/q4_lex.svg", width: 100%), caption: [Lex: selected representation /
native Vélu (dots, 95% CI) and selected / $mu_k$ (diamonds; the remaining gap), with
Dickson / $mu_k$ (squares). Source: `q4_cells.csv`.])
#figure(image("figures/q4_grevlex.svg", width: 100%), caption: [Grevlex, same layout.])

The selected lex coordinates are uniform recipes such as `gen>0;ram>inf` (the image of
$G$ sent to $0$, a ramification point to $infinity$), or a greedy per-level refinement of
one; `q4_candidates.csv` names each one. S1 density is a usable proxy in lex, with
Spearman 0.56–0.80 per cell. It is not one in grevlex (median 0.13).

== Caveats
- Toy primes and $m = 2$. Constant factors in a hand-written Buchberger may not carry over
  to F4/msolve, which this session could not build (external code was blocked).
- The cells at $p = 1153$ were seen in Q4 development before the freeze, and the
  termops-selection split was added because of what they showed. The frozen run recomputes
  them, and the other six cells reproduce the same pattern.
- One confirmation target at (7681, 3) has no V abscissa lifting to $E$ ($|F| = 0$), so its
  yield ratio is NaN, as recorded. With $D$ empty, no "dec" targets can be drawn there,
  so `q4_systems.csv` has 6828 rows rather than 6912. The 84 missing rows are the 7 Vélu
  candidates × 6 × 2 orders at that target, so those cells' "dec" medians rest on 3
  targets. The director found the shortfall and the code explains it (`eval_systems`
  draws "dec" targets only when $D != emptyset$).
- `host.txt` `dirty 2` counts untracked paths, because the `run.sh` filter does not match
  root-relative porcelain paths. All bound files hash to their card values.
- The relay check produced a false negative caused by output order in `tools/task_card.py`
  (`cards/relay/CARD-1.relay-recheck.md`).

== Raw records
`results/run-20261102/` and `results/run-q4-20261102/`: CSVs, `verdicts.json`,
`q4_verdicts.json`, `SHA256SUMS`, `host.txt`, and the receipts beside them.
