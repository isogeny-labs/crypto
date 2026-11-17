// Results and run report for research/scalar_mul_ct_timing_20261022
// (research-visuals skill).  Every number is read from summary.json (run 1),
// summary-run2.json (run 2, PROTOCOL.md amendment 1), both written by
// examples/scalar_mul_timing_analyze.rs, and the runs' manifests.
#let s1 = json("summary.json")
#let s2 = json("summary-run2.json")
#let m1 = json("raw/manifest.json")
#let m2 = json("raw-run2/manifest.json")
#set page(paper: "a4", margin: 1.7cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#let f3(x) = str(calc.round(x, digits: 3))
#let f2(x) = str(calc.round(x, digits: 2))
#let yn(b) = if b [yes] else [*no*]

#let headline(s) = table(
  columns: (3.6cm, 1fr, 1fr, 0.8fr, 1.3fr, 1fr, 1fr), stroke: 0.3pt, inset: 3pt,
  [curve], [`R` = t(224 zeros)/t(full), before, worst round], [flatness `F` after, worst round], [A/A band `δ`], [full-length ms, before → after (round 1)], [cost after/before, median], [range over rounds],
  ..s.curves.map(c => (c.curve, f3(c.R_A_max), f3(c.F_B_max), f3(s.delta),
    f2(c.per_round.at(0).full_length_ms.A) + " → " + f2(c.per_round.at(0).full_length_ms.B),
    f3(c.cost_median), f3(c.cost_min) + "–" + f3(c.cost_max))).flatten()
)

#let verdicts(s) = [
  - Leak present before (`R < 0.9` in every round, on both curves):
    #yn(s.verdicts.leak_present_on_A).
  - Coarse leak closed after (`F ≤ δ` in every round, on both curves):
    #yn(s.verdicts.closed_on_B).
  - Pinned result digests identical across all runs: #yn(s.pinned_identical).
  - Rounds: #s.rounds_complete. Contended runs left out:
    #if s.left_out_contended.len() == 0 [none] else [#s.left_out_contended.join(", ")].
]

#let per_round(s) = for c in s.curves [
  #table(
    columns: 6, stroke: 0.3pt, inset: 3pt,
    [#c.curve, round], [`R`: before, A′, after], [`F`: before, A′, after], [A/A band], [cost], [full-length ms before, after],
    ..c.per_round.map(r => (str(r.round),
      f3(r.R.A) + ", " + f3(r.R.A2) + ", " + f3(r.R.B),
      f3(r.F.A) + ", " + f3(r.F.A2) + ", " + f3(r.F.B),
      f3(r.aa_band), f3(r.cost), f2(r.full_length_ms.A) + ", " + f2(r.full_length_ms.B))).flatten()
  )
]

#align(center)[
  #text(15pt, weight: "bold")[The `scalar_mul_secret` bit-length timing leak, before and after]

  Study `research/scalar_mul_ct_timing_20261022` · 2026-10-22 · audit finding 7
]

*The leak.*
- `ecc::ct::scalar_mul_secret` sends every curve other than secp256k1 and
  P-256 to `Point::scalar_mul_ct`, the affine `BigUint` Montgomery ladder.
  SM2, GOST, EC-KCDSA and EC-ElGamal also call that ladder directly.
- The ladder started at `r0 = O`. Through every leading zero bit of the
  secret scalar it took the `O + P` and `2·O` shortcuts, so wall time
  revealed the scalar's bit length. Nonce bit lengths are what lattice
  attacks on signatures use.

*The fix.*
- The ladder runs on `k + 2^L` over `L + 1` bits, starting from `(P, 2P)`,
  and subtracts `[2^L]P`. The result is exactly `[k mod 2^L]P` for every
  point, including points off the prime-order subgroup.
- `[2^L]P` depends only on the public `P` and `L`. In the shipped version
  (run 2, amendment 1) it comes from the Jacobian `Point::scalar_mul`. In
  the first version (run 1) it was `L` affine doublings.
- The unit test `ladder_work_does_not_depend_on_scalar_length` counts ladder
  steps that hold the identity. It fails on the old ladder and finds none on
  the new one.

*Arms and procedure.*
- Before: `main` plus the driver, #raw(m1.before.commit). A′: the same
  binary, the A/A control.
- After: #raw(m2.after.commit) in run 2, #raw(m1.after.commit) in run 1.
- Each run: rotated rounds, 3 repetitions per process, one process per arm
  per round, single-threaded under `tools/isolated_bench.py`.
- Point: the generator. Scalars: 8 classes of leading zero bits, 8 scalars
  each.
- Build: #m2.build; #m2.rustc. Host: #m2.host.

= Result (run 2: the shipped fix)

#figure(image("fig_timing_run2.svg", width: 100%), caption: [Run 2. Time of `scalar_mul_secret` per leading-zero class, as a fraction of the full-length class: median over rounds, with bars for the range over rounds. Dashed line: no dependence on length.])

#headline(s2)

*Verdicts.* These are the frozen rules of `PROTOCOL.md`.
#verdicts(s2)

= Run 1: `[2^L]P` by affine doublings

#figure(image("fig_timing.svg", width: 88%), caption: [Run 1, the first version of the fix, before amendment 1.])

#headline(s1)

*Verdicts.*
#verdicts(s1)

= Per round

*Run 2.*
#per_round(s2)

*Run 1.*
#per_round(s1)

= Scope

- *Measured.* Coarse wall time on this host, for two curves and their
  generators.
- *Not addressed.* The `num-bigint` field arithmetic is not constant-time at
  the limb level (`SECURITY.md`). Its per-operation timing is untouched here,
  and so are cache, power and EM channels.
- *Small-order points.* On such points, for example attacker-chosen
  off-curve points, the registers can still meet the identity mid-ladder.
  Rejecting those points is the job of point validation.
- *Public point.* `[2^L]P` is computed in variable time from `P`, so `P` must
  be public. It is for every caller: generators, public keys and ciphertext
  points.
- *Cost.* Full-length scalars after/before:
  - run 2: #s2.curves.map(c => c.curve + " " + f2(c.cost_min) + "–" + f2(c.cost_max) + "×").join("; ");
  - run 1: #s1.curves.map(c => c.curve + " " + f2(c.cost_min) + "–" + f2(c.cost_max) + "×").join("; ").

  secp256k1 and P-256 take projective ladders and are unaffected.
