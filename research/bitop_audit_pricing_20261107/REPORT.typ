#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Two instruction audits, the unpriced `bitop/v1` counters decided, and the open `bitop/v2` choices measured

2026-11-07 · branch `claude/bitop-audit-and-pricing` · class: *accounting*
(no algorithm changed and no cost fell; not an improvement under the
success criterion). Rules: `docs/units/BITOPS.md`, "Extensions → 2026-11-07"
and "bitop/v2 → Open definition choices". Companion instruction audits:
`research/ic_acct1_cover_bitop_20261021/AUDIT_IR.md`,
`research/ic_acct3_hec_ecbench_20261021/AUDIT_IR.md`.

== Task A: instruction audits of two accounting studies

*`ic_acct3` (hyperelliptic ecbench arms).*

- *Method.* `ecbench audit-ir` on all 45 records of `sessions/g2-p41`, at the
  study commit `8c37d9a6e7`. The `claude/ic-acct-3` head does not compile: a
  union merge left an unclosed delimiter in `ecbench/workload.rs`.
- *Reproduction.* All 45 reproduce their `bitop/v1` total, every phase's
  primitive counts and the recovered scalar.
- *Instructions per gate.* IC 1,259 Ir per kilo-gate, rho 1,313 (ratio
  0.96). *Pass.*

#figure(image("../ic_acct3_hec_ecbench_20261021/audit/figures/ir_per_gate.svg", width: 80%),
  caption: [`ic_acct3`: Ir per 1000 counted gates, every committed run re-executed under Callgrind.])

*`ic_acct1` (cover IC against strong rho, custom examples).*

- *Hook.* The example gains Callgrind brackets: arm start and end, and every
  phase change.
- *Reproduction.*
  - All 16 committed IC rows reproduce exactly at the study commit
    `b4eb521af1`.
  - On `main` the descent's field counts fall 9–20%: the code changed after
    the study.
  - 3 of 48 rho rows were re-executed on `main`; all reproduce.
- *Instructions per gate.* The committed rows are out of Callgrind's reach
  here (5·10#super[12] to 2·10#super[14] gates), so the protocol's pilot size
  `p = 503` was audited.
  - IC: 175.8 Ir per kilo-gate. Rho: 9.77.
  - A counting-off control shows that the per-operation hooks cause 55% of
    the IC instructions.
  - Without them, IC is still *8.1×* rho, and the sieve 9.3×.

  *Fail:* the IC counts are not admitted as a `bitop/v1` bound until a
  symbolised profile settles whether the gap is uncounted sieve work or
  convention-excluded control.

#figure(image("../ic_acct1_cover_bitop_20261021/audit/figures/ir_per_gate.svg", width: 90%),
  caption: [`ic_acct1` at `p = 503`: Ir per 1000 counted gates by phase, counting on and off, against rho R3 at `p = 503` and `251` and twice those levels.])

== Task B: each counter left unpriced on purpose

Each decision reads what the code executes per increment.

- *Priced (2):*
  - `abscissae_enumerated_uncharged` is now
    $C("atoms", w) dot (w dot "field_add" + "hash_probe")$, read from the
    factor-base parameter: every $w$-subset is summed and inserted, and the
    counter is only the deduplicated set's size.
  - A binary-field `sqrt_solves` is now the Artin–Schreier lift attempt
    `field_sqr + field_inv + field_mul + 2·field_add + as_solve`: no square
    root is taken on $y^2 + x y = x^3 + a x^2 + b$.

  Each adds a `known_uncounted` item for the work beside it that no counter
  sees.
- *Kept unpriced, with reasons:*
  - $F_(p^4)$ square roots: a randomised Tonelli–Shanks.
  - SAT conflicts: no constant per conflict; see the SAT figure below.
  - The d4 and PKM "multiplication" units: modelled or mixed counts, not
    products executed.
  - The CM-folded backend's gap: `power` calls, string signatures and pivots
    with no counter.

No primitive weight and no vector changed.

#figure(image("figures/archive_reasons.svg", width: 100%),
  caption: [Archive before/after (`examples/ic_acct6_archive.rs`; 84,687 verified records of the 709-session archive checkout of 2026-10-26). Complete, incomplete and error totals are unchanged (58,277 / 24,682 / 1,728): every record a decision touched is incomplete for another reason too.])

#figure(image("figures/sat_per_conflict.svg", width: 90%),
  caption: [The 8 smallest committed SAT records re-executed on the counted engine reproduce their conflicts and propagations exactly. Word operations per conflict are 3,092–3,185 on one arm and 4,595–4,712 on the other, so no per-conflict rule exists. A re-run prices such a record exactly.])

== Task C: the eight open `bitop/v2` choices

The evidence is `examples/bitop_v2_choices.rs` on the 12,080 hold-out
records of `research/bitop_ladder_20261019` and the archive's 477 CM-folded
records. Every recommendation is *pending owner sign-off*, and none is
implemented.

#table(columns: (auto, 1fr, 1.2fr),
  table.header([*\#*], [*evidence*], [*recommendation*]),
  [1 sum/max], [IC sum/max 1.37 median, [1.03, 1.61]; no ranking changes], [keep the sum],
  [2 dense/sparse], [matrix 23.7× smaller sparse; IC total 1.29× (max 1.48×); IC is 3.9–193× the generic arms in gates], [keep dense (the measured `incremental-gauss` builds dense rows); report sparse beside it],
  [3 attempts], [attempts ≤ 3.1% of the $2^kappa$ keys; collisions ≲ 2%], [keep attempts],
  [4 kangaroo width], [$2 kappa + w + 1$ moves 2 of 25 BSGS/kangaroo pairs to "trade"], [adopt $2 kappa + w + 1$],
  [5 `at` and ranking], [204/204 IC-versus-generic pairs rank; true trades stay unranked], [keep],
  [6 bound fit], [memory slopes differ from gate slopes (rho 0.30 against 0.57; IC 0.65 against 0.71)], [fit memory and `at` exponents too],
  [7 walk state], [rho walk state 13% median, up to 3.4×; kangaroo up to 12×; no ranking changes], [include the jump table],
  [8 native backends], [477/477 CM-folded records are memory-incomplete], [backends declare per-structure bytes],
)

#figure(image("figures/v2_choices.svg", width: 100%),
  caption: [The choices on the hold-out: how much the declared memory moves (C1, C2), the walk state left out (C7), which pairs the rule ranks under the frozen rule and two alternatives (C5, C4, C7), and the gate, memory and `at` exponents (C6).])

== Limits

- *The archive.* It is the host's newest full checkout (2026-10-26); the
  artifact-store records on `main`'s tip were unreachable without store
  credentials.
- *The hold-out's scope.* It covers prime-field generic arms and
  `ic.pipeline` only.
- *No checkpoint.* No S3 checkpoint could be pushed (no upload token). The
  runs were local re-executions of committed seeds.
