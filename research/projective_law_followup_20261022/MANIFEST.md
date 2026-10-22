# Reproduction manifest

- Frozen study base: `f879244559b368a69e97e4f86988e5c0bdb7f896`
  on PR #2304's head at study creation; new code and outputs are in this
  directory on the study branch.
- Frozen input archive:
  `research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`,
  SHA-256 `550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
- External-law comparator:
  `research/genus2_external_gate_20261021/results/order-genus-<bits>.jsonl`,
  matched by frozen workload id.
- Prior branch-screen source:
  `research/four_followups_20261021/src/bin/complete_law_gate.rs`,
  SHA-256 `4df53e21caa0f1f08a98ea75bf572462019f825b57cb28fdd3e54750fdeab2e4`.
- Published law: Ozbay Gurler and Hisil,
  <https://link.springer.com/article/10.1007/s13389-024-00351-7>.
  Transcribed code: <https://github.com/ozbayelif/jac-on-jac>, inspected at
  `ec98e115ed359c8b7facf559eb0d9f240703f6f5`; the relevant Magma file's
  inspected SHA-256 was
  `7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a`.
- Host: Linux `6.18.54-121.166.amzn2023.x86_64` x86_64, Rust/Cargo 1.99.1.
  The representative arithmetic prices are defined in `PROTOCOL.md`; no
  isolated wall-time or fastest-external-tool claim is made.

The study crate is built from this repository root with
`cargo build --release --locked --manifest-path research/projective_law_followup_20261022/Cargo.toml --bins`.
The arithmetic gate runs as `projective-law-gate` and writes
`results/arithmetic-branches.jsonl`. For each bit size in
`16 20 24 28 32 36 40`, the two cold collectors run with arguments
`<bits> 1 16` and `FOUR_FOLLOWUPS_ARCHIVE_INPUT=1`, with output redirected to
`results/projective-<bits>.jsonl` and `results/state-<bits>.jsonl`.
`1` selects the full factor base; `16` selects the frozen jump count. The
archive-input setting requires reading the committed archive even if an
extracted directory exists. Each collector process executes 180 candidate
targets and 180 same-process rho controls. The earlier wrapper first pass is
in `results/firstpass/` and is replay-matched by the analyzer.

From the repository root, run `analyze-projective projective`, then
`analyze-projective state`, and redirect `validate-projective` to
`results/independent-validation.json`. The validator also writes both the
direct arm grid and cold-phase breakdown. The plotting/layout-only command
`python render.py`, followed by `tectonic REPORT.tex`, writes the SVG/PNG
graphs and three-page PDF. A packaged PDF and graphs appear under
`reports/run-20261022T0547Z/` for independent review.

`SHA256SUMS` gives hashes for all tracked study inputs, code, records,
summaries, graphs, PDF, and run package. Verify it from the study directory
with `sha256sum -c SHA256SUMS`.
