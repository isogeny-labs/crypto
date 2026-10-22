#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Validation gate evidence for PR \#1978's IC rows

*What this is.* The validation gate (owner rule 2026-10-19, `docs/VALIDATION_GATE.md`) admits a row to `benchmarks/README.md` or the IC leaderboard only with a replay certificate, an independent validator report and a matched rho reference with the §5a band. PR \#1978's rows landed after the gate's freeze. This report covers the evidence written for them: 14 benchmark rows and `research/ic_tool_program/rule/v7/`, which carries the leaderboard's six rule-v7 rows.

#figure(image("validation.svg", width: 100%), caption: [Left: published figures recomputed from raw records, and certificates verified, per evidence directory. Right: index calculus over rho in cold S at each row's largest size, against rho as built (the reference the rows quote) and rho at its canonical step (the gate's reference). Values from each directory's `VALIDATION.json`, `validator-report.json` and `replay-certificate.json`. Replay seed `77767620…` (`REPLAY_SEED.md`, commit `766d2dede`).])

== Result

- *Every published figure recomputes.* 2,162 figures from the raw `*.price.json` and `*.isolation.jsonl` records. The rounds' A/A, suite and holdout intervals and rule v6/v7's S figures agree to 1e-9. The explorations' paired cold times agree exactly, and their summaries agree at the three decimals they print.
- *Every recorded scalar verifies.* 6,809 records and 13,618 certificates (both arms of every process) pass $[s]G = T$ and $[n]G = O$. The negative control $[s+1]G != T$ is rejected each time. The fields are GF($2^19$) to GF($2^61$).
- *56 of 56 native replays are identical.* The committed seed drew 4 records per directory. Each was rerun with its own recorded binary, hash-checked, and reproduced the same certificates, counts and flags.
- *The gate passes.* `scripts/validation_gate.py check` reports every row validated or frozen.

== Finding: the published distance to rho is the smaller of two

Each `ic price --single-target` process records rho as built and rho priced at the canonical batched step that the same process measures. In rule v6 and v7, rho as built pays 1.98 to 2.38 times the canonical step for each walk step, as validation sweep A found for rule v3. The gate asks for the strongest available reference, so every `VALIDATION.json` uses the canonical-step rho. At each row's largest size the distance to rho is then 1.84 to 2.05 times the published one:

#table(columns: 4, align: (left, left, right, right),
  [*row*], [*size*], [*IC / rho as built (published)*], [*IC / rho, canonical step*],
  [rule v7], [$2^47.2$], [11.51], [22.37],
  [rule v6], [$2^47.2$], [12.06], [23.48],
  [R12 (v8)], [$2^47.2$], [9.86], [18.86],
)

The order holds across the §5a band: rho is faster at 0, 0.1 and 1 GAE for each of rho's uncharged canonicalisations. The index calculus is timed by the clock, so it carries no unpriced native work. The speed-ups between IC versions are unaffected, because both arms of each round meet the same rho. The leaderboard still quotes the as-built ratio. Whether to switch its quoted reference is for the owner to decide.

== Rows withdrawn from `benchmarks/README.md`

- `20261017-ic-a20-build-line-streams`: a smoke check transcribed from a terminal, with no raw records. Its own README says "not a measurement", so it cannot be validated.
- `20261017-two-sylow-fp64`: archived on main. An old merge had restored its line.

== How it ran

- *Tool:* `src/bin/ic_validate.rs`, native Rust. It does not use the crate's library or `icprog`, and has its own GF($2^m$) field, Koblitz group law and Student-t statistics.
- *Definitions,* taken from the protocols, not from `icprog`'s code:
  - *cold time:* the median over a process's repetitions of set-up plus the online interval;
  - *a process's figure:* its first attempt not marked contended;
  - *paired comparison:* the geometric mean of the ratios, with a 95% t interval.
- *Host:* Linux 6.18.44-fc-v140 x86_64 (Emerald Rapids, the rounds' class). Replays compare outputs, not times.
- *Disclosure:* a first replay pass used a Python driver, against AGENTS.md's no-Python rule. It stopped at A18, whose parameter files were renamed after the run. `ic_validate` replaced it, reran every step and drew the same 56 records. Its log is kept in `raw/`.

Raw outputs: `research/ic_tool_program/validation/raw/` (`certverify-*.json`, `native-replay.json`).
