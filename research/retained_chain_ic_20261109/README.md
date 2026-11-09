# Retained power and elliptic-addition chains

Implementation and registered experiment; cloud execution is blocked before
submission. There are no new scaling measurements or admitted performance
conclusions here. Producer observations carry `[pcc:unverified]` standing.

The user requested retaining both kinds of intermediate variable, comparing
orders as the summand count increases, and then checking complete relation
collection, independence, linear algebra, and recovery. See
[PROTOCOL.md](PROTOCOL.md) for the frozen scope and acceptance criteria.

## Implementation

The existing Rust PKM/tower solver now accepts opt-in `layout=blocks|levels`,
`charts=affine|all`, and `tree=left|balanced`. All-chart Kummer chains support
three or four summands; balanced trees require four. Legacy defaults remain
affine and use the original layout. Each projective intermediate has a finite
or infinity chart. A target is refuted only if every chart refutes; otherwise
the existing exact grid extraction and curve sign-lift remain separately
counted. A hybrid result must not be described as pure F4 extraction.

The native `ecbench retained-chain-study` command writes flushed JSONL,
equation templates, binary/system hashes, a flow summary, and an HTML table.
Its `probe` phase has 144 registered cases; its single-pass `pipeline` phase
has 11. Both retain failures. The latter is a diagnostic with row replay;
the separate L2 specs below define warmup and five-round measurements.

Over a fixed prime field the coefficients stay reduced modulo p. The growth
question here concerns equation support, degree, matrix dimensions/nonzeros,
and elimination work, rather than unbounded integer coefficient magnitude.
The panel stops at four summands and cannot establish an asymptotic bound.

## Review and execution

- Native command: `ecbench retained-chain-study --phase probe --out NEW_DIR`
  or `--phase pipeline`. Use a fresh directory; no experiment has run here.
- L2 specs: [pipeline-m3.json](pipeline-m3.json) and
  [pipeline-m4.json](pipeline-m4.json), with matched rho and A/A controls.
- [RUNBOOK.md](RUNBOOK.md) records source/image binding, submission and
  artifact obligations. [admission.json](admission.json) records current gaps.
- Focused producer checks exercise signed four-tuples including infinity,
  layout/tree parity, sparse negative cases, Sylvester arithmetic, full toy
  relation recovery, and rejection of an altered relation [pcc:unverified].
- The native ecbench adapter preserves the existing optional useful-rank
  receipt in `SolveReport.detail.useful_rank` for replay.

The earlier standalone Python workspace is historical exploratory material.
It is preserved outside this checkout and is not an execution path or admitted
evidence for this native study.
