#set page(paper: "a4", margin: (x: 18mm, y: 16mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9.3pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#show link: underline
= Finite residuals in the mixed zero fiber
Native audit · 42 fixed phi/doubling case-arms · 2026-10-25

Both full native processes pass all 42 arms each. Complete-tree parity,
final source/execution seals and the terminal receipt pass, exit zero.
No filter is deployed. Seven primes, three presentations per prime. Public
scalar 271 and its presentations are not independent secret samples.

#table(
  columns: (2.8fr, 1fr, 1fr), inset: 4pt,
  [Native first: case-local totals], [Phi], [Doubling],
  [Canonical S4 systems inspected], [33,696], [33,696],
  [Eligible finite prime-field residuals], [300], [150],
  [Retained non-prime-field residuals], [33,396], [33,546],
  [Infinity residuals], [0], [0],
  [Saved roots in eligible systems], [65,532], [32,766],
  [Accepted representatives in eligible systems], [0], [0],
  [Unique signed physical tuples before / after], [504 / 504], [504 / 504],
  [Total historical F4 row multiplications], [4,112,120,748], [4,110,616,998],
  [Historical F4 row multiplications in eligible systems], [18,166,512], [9,083,256],
)

The eligible count is twice as large for phi. This is a counterfactual skip
count, not a twofold speedup. The algebraic roots are saved symmetric
triples, not physical relations. All eligible roots pass equation,
component, zero-defect and fewer-than-three-distinct-cubic-root checks.
No accepted tuple is lost. Each arm's exact physical set, augmented rows,
zero-row count, independent rank and full-rank solution are preserved.

#figure(image("figures/eligibility-and-rank.svg", width: 100%), caption: [
  Literal first-execution counts and ranks for all 21 presentations in
  both arms. Rank preservation is within each arm; phi and doubling have
  different ranks on some curves. No time or full-cost gain is measured.
])

#pagebreak()
= Why the predicate applies

Reject only a finite residual with x in Fp; retain infinity. The physical
family has three distinct remaining ordinary abscissae, a fourth distinct
peeled ordinary abscissa and one disjoint image. This does not authorize
pruning an expanded signed-multiset relation family.

In y² = x³ + B, the checked independence of 1,B,B²,B³ makes all four
component coefficients vanish when an S4 equation has four abscissae in
Fp. Write a,b,c,d for the quartic elementary coordinates and
S = 64d − (4b−a²)². The existing identity 64S³ = ∑ AᵢCᵢ forces S = 0
in the tested odd characteristics. The checked square bridge
512f(T) − 8Q(T)² = C₃T + 8S, where
Q(T) = 8T² − 4aT + 4b − a², then puts every quartic root on a quadratic
with nonzero leading coefficient. Three distinct ordinary roots are
impossible, including when the residual abscissa repeats one of them.

This reuses IDC1h65c11f01d0b1e67a and the prior ten-theorem Lean 4.34.1
gate: SquareIdeal.component_square_certificate,
coefficients_force_defect_zero, components_exclude_three_distinct_roots
and seven SquareRootBridge declarations. No new universal Lean theorem is
claimed. Full field-embedding and group bridges remain unformalized.
The fresh native program checks the actual field, points, residuals and
saved roots. Infinity has a separate CM-triangle branch and stays retained.

The S4 necessity references are
#link("https://arxiv.org/html/1405.1059")[Gorla–Massierer, Definition 2.4 / Theorem 2.5]
and #link("https://eprint.iacr.org/2004/031")[Semaev, 2004/031].
The prior proof report is under joint_image_elimination/reports/
run-20261024T0445Z-square/ in the parent study.

#figure(image("figures/eligible-row-work.svg", width: 100%), caption: [
  Partial historical row work in eligible systems. Root enumeration,
  construction, other F4 work, memory, hashing, allocation, I/O and the
  proposed predicate are unpriced. Many rejected roots do not price their
  enumeration. No actual filter or complete-cost advantage is established.
])

#pagebreak()
= Verification and requirements retained

The independent Rust implementation rebuilds public orbit columns and
24 scalar actions with scalar multiplication, checks every physical
five-point zero sum, transports retained representatives, compares exact
sets and solves its own RREF. Full-rank factor logs are point checked.
It does not call the old collector, equation deriver or RREF. The old
complete field-root enumeration is reused; this is not a fresh exhaustive
solver or an independent human review. Hash-bound missing or malformed
input refuses execution; any mathematical counterexample is retained.

Eight synthetic controls pass with zero failures. Format, separate build
and source seals pass. Source freeze before production: be9af2af1610.
Binary SHA-256:
#text(size: 7pt)[d0f56b52bddfe3ef9a0731c4fdaeb65daa9e9d311e71a60e851406840005137f]
Workload SHA-256:
#text(size: 7pt)[17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10]
Owner accepted 06:31:41 UTC, PID 3809049; first science admission
06:36:05 UTC. Each of the two processes keeps one 1,200-second/8 GiB
phase under the common reservation. Queue wait is separate. Shared-host
durations are diagnostic and unadmitted.

#table(
  columns: (1.3fr, 2.7fr), inset: 5pt,
  [Requirement], [Evidence / remaining gap],
  [Original 189 + 21 cases], [Complete; not repeated. No consistent twist-specific full cold advantage.],
  [Unique relations / rank in 42-arm audit], [Both full processes, complete-tree parity and terminal seals pass.],
  [Original case-local ranks], [p37 phi 2/2 vs doubling 1/2; p157 2/8 vs 1/8; p337 1/14 vs 3/14. Others zero. Each rank preserved by this rejection.],
  [Public-only filtered collector], [Not implemented here. Current results are counterfactual.],
  [Complete cost from the twist], [Open: complete cold collector/target work, calibration, strongest IC/rho and external controls.],
  [85 target / 42 certificate / 12 degree configurations], [Their original owners, inputs, budgets and full workloads are preserved.],
  [Concrete q² instance proof], [Two bounded memory failures; full failed evidence retained separately.],
)

Next execute a public-only collector fork with the predicate charged
before F4/root enumeration, infinity retained, and the same rule applied
to doubling. Preserve all 42 exact physical sets and ranks and charge the
whole cold pipeline and target recovery. A structurally rejected system
may establish physical completeness; it must not report all algebraic
roots enumerated or absent. Rank, complete price and reference gates remain.

Reproduction: zero_fiber_collector/fp_residual_audit/ contains REPORT.md,
PROTOCOL.md, native/src/main.rs, build/run scripts and this source.
Raw parent runs/: fp-residual-audit-native-gate-v1 and
fp-residual-audit-v1. Per-case files retain every classification, root
witness and both physical sets. The final archive is described in README.md.

Migration reconciliation at 06:35 UTC finds 28 terminal EKS jobs
(18 complete, 10 failed, zero nonterminal), with no resubmission or role
change. Diem stays expired at 50/52 and the legacy waiter stays quiesced.
T-91 remains failed, attempt 4/4; no lease reset. Original receipt unchanged.
No credentials enter the research artifacts. Canonical IC/rho graphs,
leaderboard and benchmark index are unchanged because full-price and
VALIDATION gates remain open. Report copies are unlisted.
