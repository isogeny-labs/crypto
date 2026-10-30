# Sign-safe Kummer differential walk: reference invariant gate

Run ID: `KUMMER-REF-20261022`. Input: the frozen 21-model, 1,260-target
Scholten cohort in `research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`.
Execution was a deterministic single-process algebraic correctness gate; no
wall-time or field-operation performance claim is made. The [graph](kummer-reference-gate.svg)
and [PDF](REPORT.pdf) show the recorded counts. The [protocol](PROTOCOL.md) was
written before the candidate run.

| Observation | Recorded value |
| --- | ---: |
| Source-verified public targets | 1,260 / 1,260 |
| Target coefficient-pair cases | 8,820 / 8,820 passed |
| Exceptional/identity/two-torsion cases | 630 / 630 passed |
| Compatible sign lifts, one | 1,260 cases |
| Compatible sign lifts, two | 7,560 cases |
| Cases with multiple differential quotient outputs | 0 |
| Cases with multiple signed-anchor recoveries | 0 |
| Fast coordinate formula validated | no |
| Full Kummer collector DLP targets solved | 0 / 1,260 |
| Candidate/rho cost ratio | null in each of 36 price cells |

Each target checked the planted scalar afresh on the source elliptic curve,
then checked seven independent coefficient pairs against transfer to the
Jacobian. Each model checked 30 additional cases across all five rational
Weierstrass roots, including equal and inverse inputs. The [raw target
records](results/) retain workload identities, seeds, coefficients, source
checks, compatible-lift counts, and correctness booleans. The native
[summary](results/summary.json) recomputes the totals from those records; the
[price grid](results/price-grid.csv) records every requested cost cell as
missing, rather than estimating an unmeasured collector cost.

## Proved local quotient lemma and its scope

Write `K(D) = {D,-D}`. Given `K(A)`, `K(B)`, and `K(A-B)`, enumerate the four
possible signed input pairs. Any pair compatible with the known difference
has the same `K(A+B)`: relative sign reversal changes `A-B` to `A+B`, and
those classes coincide only when `2A=0` or `2B=0`, in which case the output
classes coincide as well. The reference program uses canonical Mumford
representatives and Jacobian additions to execute that enumeration.

For a signed anchor `G` of order greater than two, `K(T)` and `K(T+G)`
determine signed `T`. If both `T` and `-T` were compatible, then
`K(T+G)=K(-T+G)`, implying `2T=0` or `2G=0`; in the first case the lifts
already coincide, and the second is excluded. The program checked this
recovery against exact Jacobian and source labels, including torsion cases.

The two uniqueness statements are proved from the group identities above;
the finite cohort independently checks their implementation on these models.
They are local input/output lemmas, not a closed recurrence for the sidecar
`K(T+G)` under an arbitrary next jump. This gate does not provide the
required coordinate-only xADD/xDBL,
exception handling for those formulas, or a priced collector. It does not
repair the proved ambiguity of ordinary unsigned arbitrary-jump addition in
the [earlier 21-model gate](../four_followups_20261021/results/kummer-ambiguity.jsonl).
The original Kummer full-collector experiment therefore remains open in the
[problem record](../../problems/genus2-kummer-relation-labels.md).

## Reproduction and verification

From the repository root:

```sh
TMPDIR=/var/tmp cargo build --release --locked --manifest-path research/kummer_walk_20261022/Cargo.toml
for bits in 16 20 24 28 32 36 40; do
  research/kummer_walk_20261022/target/release/kummer-walk-gate "$bits" research/kummer_walk_20261022/results
done
research/kummer_walk_20261022/target/release/summarize
```

A second execution into a separate temporary directory produced byte-identical
target and model JSONL files for all seven size classes (`cmp` exit 0 on all
14 files). `cargo fmt --all -- --check` and `cargo check --release --locked`
passed in the study crate. [`SHA256SUMS`](SHA256SUMS) pins the committed
artifacts; [`VALIDATION_NOTES.md`](VALIDATION_NOTES.md) records the replay and
the limits of its independence. The input archive SHA-256 and exact source
revision are in [`manifest.json`](results/manifest.json).

The published project-pseudomultiply-recover method is described by
[Chung, Costello and Smith](https://eprint.iacr.org/2015/983.pdf). The fast
Kummer model and its rational two-torsion assumptions are described by
[Gaudry](https://eprint.iacr.org/2005/314.pdf). Their coordinate formulas
have not been transplanted into this experiment.

For the existing arbitrary-jump collector, the local lemma does not close a
fixed-offset sidecar. Suppose a state stores `K(T+s)` for each offset in a
finite set `S`, and updates every entry after a jump `J` using one
differential xADD with `K(J)` and a stored difference. Updating the entry at
`s` requires the old `K(T+s-J)`, so `s-J` must also lie in `S`. If `J` has
order `m`, closure forces every nonempty `S` to contain an entire orbit of
`m` offsets. This proves a **restricted** obstruction to that direct
fixed-offset update design. It says nothing about more elaborate
project-pseudomultiply-recover chains or a different walk; no such recurrence
was implemented here.

The next checkable implementation gate is to normalize the first source
quintic to Rosenhain form, derive explicit theta constants and projection
coordinates, implement Gaudry's xDBL/xADD with every denominator or
exception branch checked against the Jacobian on the 60 first-model targets,
and implement the signed recovery in
[Chung, Costello and Smith's later paper](https://eprint.iacr.org/2016/777/).
Only after that and a closed arbitrary-jump sidecar rule should the frozen
1,260-target cost run be attempted. The earlier five-root gate establishes
rational two-torsion on these models; it has not checked theta-constant field
of definition or projection maps.
