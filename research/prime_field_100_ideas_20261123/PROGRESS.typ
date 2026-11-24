= Execution follow-up, first bounded pass
<execution-follow-up-first-bounded-pass>
Date: 2026-11-24. Scope: the #link("IDEAS.md")[100 source proposals] and
their #link("README.md")[item-by-item assessment]. Three parallel
workstreams examined items 1--34, 35--67, and 68--100. Each item has a
specific proof obligation or falsifiable next check in the assessment;
this follow-up records what was actually checked beyond that assessment.
No 100-way experiment or full ECDLP pipeline run was performed. The
exact curve, subgroup, target, and hardware for future comparisons
remain to be frozen separately.

#box(image("PROGRESS.svg", alt: "Execution status diagram: mechanism counterexamples, existing baselines, and pending direct decomposition diagnostic."))

== Direct mathematical checks
<direct-mathematical-checks>
These are #strong[derived counterexamples to the literal proposed
mechanism], not impossibility theorems for every variant.

#figure(
  align(center)[#table(
    columns: 3,
    align: (right,auto,auto,),
    table.header([Item], [Check], [Consequence],),
    table.hline(),
    [26], [A subfield of `F_p` has cardinality `p^d` for a divisor `d`
    of `1`\; the only choice is `d=1`.], [A proper-subfield sieve cannot
    run inside `F_p` itself. An extension and norm need a separate
    costed construction.],
    [35], [Let three edge rows on a triangle be
    `A = [[1,1,0],[0,1,1],[1,0,1]]`. The underlying graph has a
    one-cycle, while `det(A)=2`, so `A` has full rank modulo
    `5`.], [Topological cycle detection alone does not identify a
    modular row dependency. A coefficient-preserving cycle-to-row map is
    required.],
    [46], [In `F_23^×`, `2^11 = 1` and `13^11 = 1`, while neither base
    is `1`\; since `11` is prime, each element has order `11`.
    Multiplication by either element permutes the 22 nonzero field
    elements in two 11-cycles, giving the same spectrum. With integer
    smoothness bound `B=3`, canonical representatives `2` and `13` have
    different smoothness.], [Phase information from the multiplication
    operator\'s spectrum cannot alone decide representative
    `B`-smoothness. A separate factorization predicate is needed.],
    [80], [The binary order-two de Bruijn cycle `0011` has length-two
    windows `00, 01, 11, 10`, interpreted as `0,1,3,2`. Reduction modulo
    `3` yields `0,1,0,2`.], [Window uniqueness does not imply distinct
    field residues. A candidate-map injectivity proof is needed before
    claiming nonredundant coverage.],
    [83], [The one-entry matrix `[2]` has rank one over the reals and
    rank zero over `F_2`.], [Real SVD cannot certify finite-field rank
    without an exact modular reconstruction step.],
  )]
  , kind: table
  )

The \#35 check also shows why the repository\'s
#strong[coefficient-aware] large-prime cycle filtering matters: the
graph topology alone omits row coefficients and the subgroup modulus.
These arithmetic checks are short enough to replay directly from the
displayed values. They are not general algebraic identity claims or
performance measurements.

== Historical evidence checked against proposed work
<historical-evidence-checked-against-proposed-work>
#figure(
  align(center)[#table(
    columns: 3,
    align: (auto,auto,auto,),
    table.header([Items], [Existing record and bounded reading], [Next
      discriminating step],),
    table.hline(),
    [24], [The
    #link("../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md")[technique inventory]
    reports dense `S4`/`S5` construction and a 40-bit diagnostic with
    125/125 and 6561/6561 nonzero coefficients;
    #link("https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/knowledge/open-problems/KN-OPEN-004.md")[KN-OPEN-004]
    already poses the sparse-support question. A bounded exact GP
    diagnostic below found a special-coordinate support reduction on a
    prime-order curve.], [Freeze a matched elimination and
    relation-collection protocol before any performance claim. Test
    whether the support reduction survives coordinate changes and lowers
    exact solver cost, then check independent relation rank.],
    [53--60], [The
    #link("../../benchmarks/records/20261112T2313Z-ic-gpu-cards.json")[A100 kernel record]
    reports SpMV 0.008175 → 0.0005398 s, `n=5`, equal CPU digests. The
    #link("../../benchmarks/records/20261115T2002Z-ic-gpu-rootfind-la-r3.json")[complete-solve record]
    reports a 39-bit, factor-base-size-4096 GPU solve 1.541 → 0.305 s,
    `n=6`, with a 0.0866 s matched rho reference. Both records are
    historical and `[pcc:unverified]`\; the former is a kernel result,
    the latter is a complete solver result on its recorded workload. The
    complete candidate time is about 3.52 times that
    reference.], [Profile the actual limiting stage in a current, frozen
    run, then compare exact GPU queue, arithmetic, and memory-layout
    variants with the same input and transfer accounting. No new GPU run
    was made here.],
    [75--76, 84], [The
    #link("../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md")[technique inventory]
    identifies an existing coefficient-aware multi-large-prime
    hypergraph filter, CADO-style filtering, and block Wiedemann. The
    #link("https://github.com/isogeny-labs/workspace/pull/59")[graph-theory study]
    reports that structured Gauss changed little in its tested prime
    cells, where linear algebra was 0.03--0.14% of recorded total GAE
    `[pcc:unverified]`.], [Replay current verified partial-relation
    streams through the existing filter and a proposed separator/rank
    variant, retaining source-row witnesses, modular rank, target
    column, and complete cost.],
    [96--97], [The
    #link("https://github.com/isogeny-labs/workspace/pull/59")[graph-theory study]
    reports a 24-repeat frozen base-size sweep on prime 16-bit and
    18-bit instances. Its recorded `S` for `m=2` is 13.460 at base size
    16 versus 19.620 at size 40, and 12.839 at size 32 versus 20.389 at
    size 63 `[pcc:unverified]`. These are #strong[fixed-base]
    comparisons; they do not test a dynamic controller or target
    descent.], [Compare a predeclared online resizing policy with the
    best frozen base on held-out, time-ordered relation streams,
    including tuning, setup, rank, solve, and target descent. No
    suitable interval trace was found in the selected study for \#97.],
  )]
  , kind: table
  )

The observed GPU ratios and frozen-base values belong to their source
workloads. We made no new performance measurement and did not transfer
those numbers to a different curve or hardware. The
#link("../../docs/perf/20261112-ic-gpu-audit.md")[GPU audit] and the
source records contain the original methods and limits.

== \#24 exact support diagnostic
<24-exact-support-diagnostic>
The committed #link("S4_SUPPORT_GP.gp")[PARI/GP source] and
#link("S4_SUPPORT_GP.txt")[raw output] use GP 2.17.3 on arm64. The
source computes the standard short-Weierstrass `S3`, takes
`Res_t(S3(x,y,t), S3(t,w,z))` modulo 1009, and enumerates every
coefficient in the degree-four box. The fixed `z` values are listed in
the source; no random seed or timing is involved. The wrapper separately
checks group cardinality, primality, and the order of the point with
`x=0`.

Reproduce with `gp -q < S4_SUPPORT_GP.gp`\; compare stdout byte-for-byte
with `S4_SUPPORT_GP.txt` and reject any GP `***` error line. This source
and raw output are a structural diagnostic, not a frozen performance
protocol.

#strong[Native corroboration protocol, frozen before invocation.] Run
the existing `semaev_prime_structure` example from the checked-out
commit with Rust 1.99.1,
`--p 1009 --a 1 --b 14 --m 3 --zs 3 --lines 0 --seed 20261124`. Its
three seeded `z` values are not selected by hand. Accept only parseable
JSON rows with `p=1009`, `m=3`, and `support_max=125`\; independently
recompute their support counts with GP before interpreting agreement.
Record the command, toolchain, exact rows, and any build failure. Do not
use elapsed time from this local correctness check as a benchmark. The
tracked `panel.json` dependency was materialized from `isogeny/main`
after a first sparse-checkout build could not find it; its bytes were
not edited. The final release build succeeded and the committed
#link("S4_SUPPORT_RUST.txt")[native output] has `z=750,620,320`, each
with `125/125` support and nonzero box corners. The GP source was
extended to check those exact returned values independently; its output
agrees on all three counts. `--lines 0` means this run did not test
irreducibility. The example printed a short execution time, which is
excluded from cost comparisons.

#figure(
  align(center)[#table(
    columns: 5,
    align: (auto,right,right,right,auto,),
    table.header([Curve over `F_1009`], [Fixed `z`], [Nonzero
      coefficients / 125], [Nonzero box corners / 8], [`#E`\; order of
      point at `x=0`],),
    table.hline(),
    [`y²=x³+2x+3`], [7, 17, 29], [125 each], [8 each], [1068; 267],
    [`y²=x³+2x+3`], [0], [88], [4], [1068; 267],
    [`y²=x³+3`], [0], [37], [1], [948; 3],
    [`y²=x³+2x`], [0], [35], [4], [980; 2],
    [`y²=x³+x+14`], [0], [88], [4], [#strong[1013 prime; 1013]],
    [`y²=x³+x+14`], [7], [125], [8], [1013 prime; 1013],
    [`y²=x³+x+14`], [320, 620, 750], [125 each], [8 each], [1013 prime;
    1013],
  )]
  , kind: table
  )

#box(image("S4_SUPPORT.svg", alt: "Exact nonzero coefficient counts for the fixed S4 specializations."))

The last pair shows that a special-coordinate support reduction occurs
even when the whole curve group is prime-order. All specialized
resultants in this panel retain degree four in each free variable. This
is a #strong[structural component observation]: fewer nonzero
coefficients need not imply lower elimination degree, lower
decomposition time, more independent rows, or lower full solve cost. The
40-bit inventory result and this 1009-field panel have different curves
and specializations. The original Rust example was attempted first but
could not compile on this merged checkout because
`research/ecbench_phased_panel_20261117/panel.json` was missing; the GP
calculation supplies an independent bounded diagnostic, not a Rust
performance comparison. An initial version of the GP script also had a
multiline `for` parse error after printing support rows; the point-order
block was rewritten as a function and the final raw output was
regenerated without GP error lines.

== Coverage of all 100 items
<coverage-of-all-100-items>
Every numbered item remains in the
#link("README.md")[assessment ledger], with its stated decisive check.
The three parallel workstreams refined those checks by range: 1--34
require a relation-preserving construction or a costed sieve comparison;
35--67 require exact modular output or an exactly checked relation
predicate before hardware or learning cost can be compared; 68--100
require exact rank/coverage or a sound map from proposed representations
to point relations. The five arithmetic counterexamples above close only
their #strong[literal premises]. The source-list, ledger, and this
report together distinguish `screened`, `historical evidence inspected`,
`diagnostic completed`, and `future experiment` statuses. No result is
silently promoted by a screen.

== Next controlled batch
<next-controlled-batch>
+ Freeze a matched \#24 elimination protocol on the prime-order
  `y²=x³+x+14` curve with `z=0` and ordinary `z` controls. Check exact
  decompositions, independent rows, and total cost; preserve the
  committed structural output as a distinct pre-comparison diagnostic.
+ Freeze one immutable set of independently checked full and partial
  relations for \#75/\#76/\#84, then replay existing and candidate
  filters on the same rows. Stop if source-row combinations or
  target-column recovery fail.
+ Freeze the base-size and interval policy panel for \#96/\#97 against
  the best fixed controls in the same harness. Include explicit compute
  and artifact budgets before launching new runs.
+ Profile a current exact GPU end-to-end run before allocating
  \#54/\#55/\#57 engineering, since the historical kernel and
  complete-solve records identify different boundaries.

This first bounded pass is incomplete relative to experimental pursuit
of all 100 proposals. Many descriptions need a defined mechanism before
an executable experiment exists; their exact missing obligations are
retained in the assessment ledger.
