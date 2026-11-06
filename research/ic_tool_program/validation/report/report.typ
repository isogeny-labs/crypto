#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Validation gate evidence for PR \#1978's IC rows

*What this is.* The validation gate (owner rule 2026-10-19, `docs/VALIDATION_GATE.md`) admits a row to `benchmarks/README.md` or the IC leaderboard only with a replay certificate, an independent validator report and a matched rho reference with the §5a band. This report covers 17 evidence directories:
- 14 benchmark rows, and the re-admitted rule v8 row (`20261022-ic-matched-rho-v8-validated`);
- `research/ic_tool_program/rule/v8/`, which carries the leaderboard's six Table A rows;
- `rule/v7/`, whose rows Table A cited before.

#figure(image("validation.svg", width: 100%), caption: [Left: published figures recomputed from raw records, and certificates verified, per evidence directory. Right: index calculus over rho in cold S at each row's largest size. Each pair compares the measured rho of the row's own process (the gate's reference) with rho at the canonical step (a model). Values from each directory's `VALIDATION.json`, `validator-report.json` and `replay-certificate.json`. Replay seed `77767620…` (`REPLAY_SEED.md`, commit `766d2dede`; amendment 1, `e9cdf82c0`).])

== Result

- *Every published figure recomputes.* 2,252 figures from the raw records.
  - The rounds' intervals, rule v6/v7/v8's S figures the 30 figures Table A shows for rule v8 and the 12 cold ratios of its `matched.json` agree to 1e-9.
  - The explorations' paired cold times agree exactly, and their summaries agree at the decimals they print.
- *Every recorded scalar verifies.* 7,650 records and 15,300 certificates pass $[s]G = T$ and $[n]G = O$, and the negative control $[s+1]G != T$ is rejected each time.
- *64 of 64 native replays are identical,* 4 drawn per directory by the committed seed.
  - Each round and exploration ran with its own hash-checked binary.
  - Rule v8 and v8c ran with `ic` rebuilt from their recorded commit `370e1919`.
- *The gate passes.* `scripts/validation_gate.py check` reports every row validated or frozen.

== The reference: measured rho, and Table A on rule v8

Each process records rho as measured and a model: the walk's counted operations priced at the canonical batched step (`rho_model`, "a model beside the measured interval, not a measurement of it").

Every `VALIDATION.json` names the measured rho of the row's own process as its reference. That is the reference the protocols name, and its walk uses negation and Frobenius, the strongest fold for these curves. The model is recorded beside it, not as a reference. The order holds across the §5a band.

The model overstates rho. Rule v8 measured a matched rho, its canonicalisation batched 16 lanes at a time, 1.30–1.58× faster than rho as at v7 on the same binary. The model assumes about twice.

#table(columns: 2, align: (left, right),
  [*reference*], [*IC / rho at $2^47.2$*],
  [rule v7, rho as built], [11.51],
  [rule v8, the matched rho (measured)], [*18.07*],
  [rule v8, the canonical-step model], [23.32],
  [rule v8c, rho as at v7 (same binary)], [11.56],
)

*Table A now cites rule v8* (owner's decision, 2026-10-22). It shows the measured matched rho, 4.67–18.07×, with its published interval. The same binary's control and §23's ratio against rho as built sit beside it. An earlier commit in this PR had used the model; it was replaced on finding that the model is not a measurement.

The hand-written pages that quoted rule v6 or v7 now say those ratios are against rho as built, not matched, and point to rule v8.

== Rows withdrawn from `benchmarks/README.md`

- `20261017-ic-a20-build-line-streams`: a transcribed smoke check with no raw records. Its own README says "not a measurement", so it cannot be validated.
- `20261017-two-sylow-fp64`: archived on main. An old merge had restored its line.

== How it ran

- *Tool:* `src/bin/ic_validate.rs`, native Rust. It does not use the crate's library or `icprog`, and has its own GF($2^m$) field, Koblitz group law and Student-t statistics.
- *Definitions,* taken from the protocols:
  - *cold time:* the median over a process's repetitions of set-up plus the online interval;
  - *a process's figure:* its first attempt not marked contended;
  - *paired comparison:* the geometric mean of the ratios, with a 95% t interval.
- *Host:* Linux 6.18.44-fc-v140 x86_64 (Emerald Rapids). Replays compare outputs, not times.
- *Rule v8c:* it set `KIC_RHO_CANON=scalar`, and its replays set it too. Its records differ from v8's only in timings, so a replay confirms its counts and scalars, not which canonicalisation ran.
- *Disclosure:* a first replay pass used a Python driver, against AGENTS.md's no-Python rule. `ic_validate` replaced it and drew the same records. The earlier log is kept in `raw/`.
