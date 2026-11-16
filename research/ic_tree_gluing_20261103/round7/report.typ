#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= Round 7: new directions — Edwards nodes and the 2-torsion quotient,
Riemann–Roch, covers, list decoding, memory-lean F4
<round-7-new-directions--edwards-nodes-and-the-2-torsion-quotient-riemannroch-covers-list-decoding-memory-lean-f4>
2026-11-04 · every result here is an unverified assertion
\[pcc:unverified\]. Claim records are listed at the end. Each sub-study
keeps its own pre-registration, timestamped before its runs. The
decisive runs were also registered in `.resume/RESUME.md` (rounds 7d and
7e).

#align(center)[#table(
  columns: 2,
  align: (col, row) => (auto,auto,).at(col),
  inset: 6pt,
  [sub-study], [directory],
  [Edwards node relations],
  [#link("../round7_edwards/README.md")[`../round7_edwards/README.md`]],
  [Riemann–Roch],
  [#link("../round7_rr/README.md")[`../round7_rr/README.md`]],
  [covers and list decoding],
  [#link("../round7_tests23/README.md")[`../round7_tests23/README.md`]],
  [memory-lean F4],
  [#link("../round7_memory/README.md")[`../round7_memory/README.md`]],
  [survey of directions beyond the summation framework],
  [#link("../round7_beyond/DIRECTIONS.md")[`../round7_beyond/DIRECTIONS.md`]],
)
]

Graph: #link("round7.svg")[`round7.svg`]. PDF:
#link("report.pdf")[`report.pdf`].

#strong[Class: stage diagnostics.] No pipeline is priced and no
leaderboard row changes. The engine is msolve 0.6.5 with p \= 786433,
apart from the covers test, which uses gp at p ≤ 10⁴.

== Result
<result>
+ #strong[The 2-torsion quotient in the Edwards v-coordinate lowers the
  coupling excess, the first change in this line to do so]
  \[pcc:unverified\].
  - #strong[Setup.] The curve is u² + v² \= 1 + 11u²v², with \#E \= 4 ·
    196,873. Translating by the 2-torsion point (0, −1) acts as v ↦ −v.

  - #strong[Encoding (E1Q).] Squares replace the v\'s, and each node
    gets one signed product. Every input has degree ≤ 3, and each tower
    is one level shorter.

  - #strong[Faithfulness (README Round 7b).] Each E1Q solution lifts to
    exactly 2^{m−1} E1 solutions, and the planted checks pass.

  - #strong[Coupling excess D − 4] (the tree T for comparison):

    #align(center)[#table(
      columns: 4,
      align: (col, row) => (right,right,right,right,).at(col),
      inset: 6pt,
      [m], [t], [E1Q], [T],
      [3],
      [4],
      [1],
      [2],
      [3],
      [5],
      [1],
      [2],
      [4],
      [4],
      [1],
      [3],
      [4],
      [5],
      [2],
      [≥ 3],
    )
    ]

    The m4 t5 cell has D \= 6 and finished in 319 s, while T and E1 are
    cut off at ≥ 7. That confirms the round-7e reading.

  - #strong[Controls (Round 7c).]

    - The Edwards model alone, without the quotient (E1), gives D \= T.
    - The same quotient on the Montgomery model of the same group, where
      the translation is x ↦ 1/x and the quotient coordinate is x + 1/x,
      gives D \= T.
    - E1 and E1Q use the same point set, so the base size is not the
      cause.
    - A second target replicates D and width in all 10 comparable cells.

  - #strong[Scope.] The gain needs the 2-torsion translation to act as a
    sign flip in the node coordinate, so it requires an Edwards model:
    rational 2-torsion and a point of order 4. That covers cofactor-4
    curves such as the Curve25519 family, and excludes prime-order
    curves such as P-256 and secp256k1.

  - #strong[Novelty.] This is the Faugère–Gaudry–Huot–Renault 2-torsion
    mechanism, applied here to Kummer-towered prime-field trees. It is
    new in this setting, not new as a mechanism.
+ #strong[The Riemann–Roch norm-divisibility system has lower D than the
  tree at m \= 3. At m \= 5 it is undecided] \[pcc:unverified\].
  - #strong[m \= 3:] D \= 4 against 6 at t \= 4, where it is also
    narrower (2,801 against 7,077 columns). D \= 5 against 6 at t \= 5,
    where it is 1.34× wider.
  - #strong[Depth slope:] about +1 per level, steeper than the tree\'s.
  - #strong[m5 t4:] run alone at 13 GiB, it was cut off at 10,800 s
    still inside degree 7, on a 467,529 × 399,003 matrix. That gives D ≥
    7, against the tree\'s ≥ 9, so no ranking is possible. An earlier
    attempt was killed by the kernel OOM killer at 590 s because it
    overlapped another job; its log is kept as
    `logs/m5t4_13g/m5_t4_k0_RR.oomkilled.log`.
+ #strong[Covers (test 2): no new relation family] \[pcc:unverified\].
  - Every cover relation maps into the set that elliptic functions
    already give.
  - The cover finds relations about 2^k times more often per sample than
    the elliptic sampler restricted to half the base. I read that as the
    2:1 fibre of the cover map, an accounting effect, not an
    improvement, until an independent check of the cost per useful
    relation says otherwise.
+ #strong[List decoding (test 3): closed] \[pcc:unverified\]. The Kummer
  base\'s relation counts fall inside the band of 20 random bases at t
  \= 6–9, while a planted progression is detected at 10³–10⁴×.
+ #strong[Memory-lean F4 does not finish the tree\'s m5 t4 cell on 15
  GB] \[pcc:unverified\]. msolve with `-m 10000`, and also with `-l 44`,
  ran out of memory (exit 139). Memory grows with the basis and hash
  table, not with matrix size. The lab\'s Rust engine would need about
  320 GB, because its rows are dense.

#strong[Reading.]

- The 2-torsion quotient in a sign-flip coordinate is the first change
  in this line that lowers the coupling excess: by 1–2 at m \= 3, 4, and
  it holds at t \= 5. Whether it lowers the #emph[slope] in m, and not
  just the intercept, needs E1Q at m ≥ 5. That needs more than 2 GiB and
  a planted m \= 5 Edwards instance.
- The Riemann–Roch system is the strongest candidate at m \= 3, and it
  is undecided at m \= 5.

#image("round7.svg")

== Claims
<claims>
- Item 1: PCC-20261104-78dcbd. Item 2: PCC-20261104-f53ff4. Both are
  `statement` claims, so they render as unverified.

== Next
<next>
+ E1Q at m \= 5 (t \= 4, 5), with about 10 GiB and a new planted Edwards
  \[pcc:unverified\] instance. This decides whether the quotient changes
  the m-slope.
+ The Riemann–Roch system on the Edwards quotient, which combines both
  gains.
+ The tree and RR at m5 t4 on a host with ≥ 32 GiB, or after eliminating
  the linear relations by hand (round7\_memory).
+ An independent check of the covers per-sample gain, counting the cost
  per useful relation including rank.
+ Uncontended timing with two targets for the E1 and E1Q speedups,
  recorded as a `bench.record/v1` benchmark record before any speedup
  claim.
