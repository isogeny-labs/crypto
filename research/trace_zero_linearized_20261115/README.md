# Symmetric residual linearization — 2026-11-15

The native solver now finds planted point decompositions on ECC2K-130's
curve with **1,994-point bases**. It decided all **130/130** query executions
in its frozen scale-panel arm, with zero A/A discrepancies and no invalid
answer. Its largest coefficient system has **35 binary columns**; it uses no
SAT variables. These are controlled finite-base point-decomposition results,
not recovery of the public challenge logarithm [pcc:unverified].

[Three-page illustrated report](report.pdf) ·
[saved concrete relation](example-relation.json) ·
[scale-panel summary](summary.json) · [accepted export](export.json) ·
[native truth/relation audit](integrity-audit.json) ·
[preceding terminal panel](../trace_zero_terminal_20261115/README.md).

## Observed results

Each arm runs the same 65 inputs twice, for 130 executions. The repetitions
are paired reuse of the same targets, not independent population samples.

| Method | Found | Refuted | UNKNOWN | Valid decided |
|---|---:|---:|---:|---:|
| Original sliced SAT |78|32|20|110|
| Prepared sliced SAT |80|34|16|114|
| Residual linearization |92|38|0|130|
| Linearized A/A |92|38|0|130|
| Fresh pair-table reference |74|32|24|106|

Flow: **650 planned, 650 attempted, 650 completed**, zero exclusions,
interruptions, retries or invalid decisions. Linear A/A outcomes, conflicts
and native work agree. The exact new-scale-only subset is:

| Method, 32 executions per arm | Found | Refuted | UNKNOWN |
|---|---:|---:|---:|
| Original sliced SAT |10|2|20|
| Prepared sliced SAT |12|4|16|
| Residual linearization |24|8|0|
| Fresh pair-table |6|2|24|

The anchor set contains all 49 previous cases. Sixteen new heldouts cover
n83/131, d10/12, with two distinct-x planted targets, a full-width uniform
scalar target and a cancellation control per cell. New bases have 512/2,008
points at n83 and 442/1,994 at n131. Planted targets are nonidentity, outside
the base, and constructed from three distinct x values before candidate data.
The public challenge Q is not a primary target [pcc:unverified].

**Dated correction to the earlier scoped result:** the first encodings left
every n131 eight-point-base case UNKNOWN. Terminal slicing subsequently
solved those controls and the 26/112-point holdouts. This linearized backend
also solves the new 442/1,994-point planted controls. Preserve the earlier
source-bound reports; their observations are not rewritten [pcc:unverified].

## Mechanism and completeness boundary

For each actual final point R, form the original group residual S=T-R.
Handle S=O separately. Otherwise let z=x(S), and set

    u = x(P)+x(Q),       v = x(P)*x(Q).

The existing binary S3 formula is

    v^2 + z*v + z^2*u^2 + b = 0.

Let V be the binary span of the actual admitted x codes, and W the span of
all products of basis elements of V. Every actual pair has u in V and v in W.
For fixed z, Frobenius squaring is binary linear, so the displayed equation
is linear in the coefficient bits of u and v. **u is a field-coordinate sum;
it is not the x-coordinate of the group sum.** No group partial sum is forced
into V.

The solver builds a particular coefficient solution and its complete kernel.
It factors `X^2+uX+v` with the existing native field solver, checks actual
finite-base x membership, lifts only supplied point signs, and replays the
original group sum. Exhaustion is required for Refuted; a deficient affine
space exceeding the shared model cap returns Inconclusive. No uniqueness
assumption or conversion of UNKNOWN to UNSAT is used.

For the saved n131,d12 example, dim(V)=12 and dim(W)=23, giving 35 columns.
The successful consistent system has rank 35/nullity zero; 400 residual
systems are examined before the returned relation. The three returned x
coordinates are `bda`, `160c`, `6d6`, and the full y/target coordinates are
in [example-relation.json](example-relation.json) [pcc:unverified].

Prepared sliced SAT separately validates immutable base/target inputs once
per outer query. Residuals computed from admitted kernel points use a private
admitted path; no public validation bypass is exposed. The original sliced
API retains repeated admission, so the comparison separates that cost from
the new elimination strategy. Every model's original-group replay remains.

Evaluated integral certificate **IDC1h420f65ecd4500a54** records

    (xy+xz+yz)^2 = (xy)^2 + z^2*(x+y)^2 + 2xyz*(x+y).

Reducing the explicit even correction gives the existing characteristic-two
S3 form. It has degree four, coefficients of magnitude at most two, 32 issued
plus 32 local replay points over 2^61-1. This is an evaluated integral
expansion, not a formal solver-completeness proof or mathematical novelty.

## Accounting, controls and verification

- One CPU, 2-GiB address-space ceiling and 600-second combined pilot/freeze/run
  cap, under the user's continued local EKS/S3 exception. No paid compute
  launch or trial retry. Native input/truth preparation and post-run audit
  are separate capped setup/verification work, not free query costs.
- Query safety fuse 750ms, 2,000 total SAT conflicts where applicable,
  500,000 allocation cap, 1,024 shared models. Linear columns are distinct
  from SAT variables; no fictitious SAT allocation/conflicts are reported.
  Existing admission/deadline semantics remain disclosed in PROTOCOL.md.
- The 45-call pilot includes the largest n/d classes. Its fixed stratified
  planning map gives 150.689 seconds estimated total, a resource estimate
  rather than timing evidence. Every pilot is valid; all n5 positive arms
  Found/replay. Inputs and deterministic native analysis froze before primary
  data, with labels explicitly unmasked for this descriptive tabulation.
- Across 130 rows, native additions+doublings are 17,892 linear, 43,086
  prepared SAT, 43,070 original SAT and 3,776,671 fresh pair-table. They include
  setup/lifting/replay and capped attempts, but exclude Gaussian, field/root
  and SAT work. They are **not runtime or full-cost ratios**. Batch-amortized
  pair-table costs are not measured [pcc:unverified].
- The native post-run auditor rebuilds complete pair tables per distinct
  base, independently of candidate decisions. It rechecks all 65 truths,
  asserts a complete unique query grid and replays all **416 saved original
  point tuples** without mismatch. The audit shares tables only within
  verification; candidates never receive them [pcc:unverified].
- Canonical export validation checks schema, frozen protocol, source/binary,
  analysis and file hashes. Three focused tests cover all small targets and
  sign restrictions, zero budgets and an actually exercised deficient-space
  cap without false refutation. The prior 20-test suite, terminal two-test
  suite and production library type-check pass [pcc:unverified].
- No isolated clock, population, scaling, full-DLP or independently signed
  verification claim. No `experiment.review/v1` from another runner is attached.
  Ordinary whole-root optimized-suite, benchrec and ledger merge limits
  remain separate from these completed component checks.

## Source identity and reproduction

Curve E0 is `y^2+xy=x^3+1`. ECC2K-130 uses F2^131 with modulus
`z^131+z^13+z^2+z+1`, cofactor four and the pinned original Certicom subgroup/G.
The n83 fixture uses `z^83+z^45+z^2+z+1`; n5/7/9 controls use the exact
earlier native fixtures. Full coordinates, seeds and known answers are in
[workload.json](workload.json), with new-case generator rule
`202611151000+100*n+10*d+kind_index`, pinned StdRng/Cargo.lock. Point generation
and candidate algorithms are Rust.

Instrument source: `10c60753c9f96c5576bcbf1883915f149991ea3a`.
Protocol: `24f91347723baadc352584244b33361443acf47718c3900f0d93612fcb10348e`.
Binary: `1f4d989f22da1f4ba5e3cbd4fcb1477d8021e856eb625354f6cf3a05db9ad589`.
[Source pins](SOURCES.sha256) describe this instrument; the terminal study's
earlier pins retain their historical source identity and are checked there.

~~~sh
cargo test --locked --manifest-path research/trace_zero_linearized_20261115/native/Cargo.toml -- --nocapture --test-threads=1
cargo check --lib --no-default-features
# Recompute summaries from retained rows, not trial reruns:
research/trace_zero_linearized_20261115/native/target/release/trace-zero-linearized analyze research/trace_zero_linearized_20261115/rows.jsonl
python3 ../crypto-autoresearcher/tools/experiment_protocol.py check-export research/trace_zero_linearized_20261115/export.json --protocol research/trace_zero_linearized_20261115/protocol.json --verify-files
~~~

The [native post-run auditor](native/src/bin/audit_scale.rs), source/command
receipts in checks/, accepted export and dated run report retain failures,
caps and actual point witnesses. Large raw files may be represented by
content-addressed `.artifacts.json` manifests; frozen Git history also retains
the exact committed bytes. Python only serializes administrative metadata and
renders recorded values/PDF layout; it performs no curve/solver research.

Sources checked (KB): server unavailable and wrapper exit127. Fallback:
`binary_semaev.rs` existing Vieta/S3 derivation and quadratic solver, previous
trace-zero product-span profiles, Kosters–Yeo source copy and the references
to Semaev2004/031 and Diem2011 listed by that native module (not newly fetched
in this round). No global originality assertion. Existing scoreboard,
timeline and performance-gains graphs retain their values: no admitted
runtime/full-cost gain is supplied. Every report PDF page was inspected.

This addresses finite-base relation finding. An amortized multi-target
pipeline, factor-base logarithm/rank collection and a public challenge-Q
recovery remain untested. The earlier tau-transport results retain their
separate scope; this component uses the low product span of the fixed basis.
