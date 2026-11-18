# Historical genus-3 cold-target audit

This study preserves the completed pre-repository Python experiment and adds
a native Rust check of its saved arithmetic and summaries. The Python sources
are an immutable provenance archive, **not a supported research execution path**.
There is no new performance experiment in this import.

## Recorded observation and standing

The historical producer reports the following wall times, with a fresh solver
and target-independent DLP setup for each of 64 separately timed targets.
All values in this paragraph and table are **[pcc:unverified]**; claim
`PCC-20261115-7c5845` is not independently signed or admitted by the validation gate.

| Historical implementation | Mean cold wall time (s) | Median cold wall time (s) |
| --- | ---: | ---: |
| Cover / prime-field divisor arithmetic | 0.0566323933 | 0.0560952370 |
| Negation-aware, single-target-sized BSGS | 0.0787853895 | 0.0778257455 |

The producer's ratio of means is 1.3911718161, its paired-bootstrap interval
is [1.3019612161, 1.4815007697], and its cover time is smaller on 52/64
recorded targets. **[pcc:unverified]** The interval is preserved from the
producer; the native checker does not reproduce NumPy's bootstrap sample stream.

This is a bounded comparison between two Python implementations at one
small, deliberately selected fixture. It is not a native performance result,
a comparison with the fastest external implementation, a scaling result, or
an admitted comparison with Pollard rho. The historical record has no counted
operation ledger, matched-rho arm, qualified host/isolation/A/A manifest, or
independent-host replay certificate. These omissions cannot be repaired by
renaming the recorded BSGS arm or synthesizing a `VALIDATION.json`.

See the [admission problem](../../problems/genus3-cold-target-native-counted-replay.md).

## Exact fixture and method

The unregistered historical fixture is identified by the exact field, model,
subgroup and generator in the frozen JSON, not by field degree alone:

```text
k = F_1061
K = k[alpha]/(alpha^3 + alpha + 65)
E/K: Y^2 = X(X-alpha)(X-alpha^1061)
C/k: y^2 = (t^3+t+65)(t^4-2t^2-520t+1)
reported #E(K) = 1194429244 = 4 * 298607311
G = ((167,623,69), (1004,699,413))
```

Triples are coefficients of `1, alpha, alpha^2`. Identity registration as
ICV1/EC1 remains pending; this import does not invent an alias or modify the
registry. Parameters and geometric claims retain **[pcc:unverified]** standing
until the repository's independent certification steps are complete.

The frozen implementation constructs a degree-four cover through a
bielliptic quotient and a two-isogeny. Its transfer is pullback followed by
field trace. It corrects the pole divisor by the two-torsion class `(N,0)`;
subtracting four infinities alone gives the wrong pullback. Relations come
from `y-h(t)` for a cubic interpolated through four rational cover points.
Seven distinct rational roots of `f-h^2` give a principal relation. The cold
version uses exact finite-field conjugate gradients on randomized symmetric
normal equations, checks the original equations, then shifts the descended
target until its reduced divisor splits. These are descriptions of the
archived code, not new theorem or novelty claims.

The generator was selected as a cofactor multiple of a factor-base image.
Both historical arms receive that same generator. The observation does not
demonstrate the same cold normalization cost for an arbitrary supplied generator.

## What “cold” included

Each recorded cover interval includes a new solver, construction of the cover,
factor-base enumeration, fresh relation collection with a distinct seed,
linear solving, generator and target descent, scalar recovery and its answer
check. Each BSGS interval includes its fresh negation-aware table, search and
answer check. The order of the two methods is randomized per target.

The supplied curve/order/generator, process/library startup, and common
known-answer target construction are excluded for both arms. A “cold target”
here is not a fresh operating-system process. No relation-log table is shared
across targets. The archived source and raw records retain the exact seeds,
excluded interval, per-target timings and producer checks.

## Preserved evidence and local native checks

- [Frozen audit](historical/cold_target_audit_q1061_v65.json): unchanged bytes.
- [Source archive](historical/source.tar): five measured Python files with
  their original names/bytes; the native checker compares every member with
  the hashes recorded before the audit. Extract for inspection, not to extend
  the Python execution path.
- [Native checker](native-check/src/main.rs): fresh schoolbook field reduction
  and exponentiation inversion, distinct from the producer's adjugate inverse.
  It replays the recorded scalars, checks the subgroup/Hasse witness, source
  hashes, means, medians, ratio and win count. **[pcc:unverified]**
- [Local checker report](reports/native-check.json): a producer-local check,
  not a signed independent validator or an execution proof.
- [Descriptive report](reports/REPORT.md), [PDF](reports/REPORT.pdf), and
  [recorded-values graph](reports/recorded-wall.svg).

Run only the native checks:

```sh
cargo test --release --locked --manifest-path research/genus3_cover_cold_target_20261115/native-check/Cargo.toml
cargo run --release --locked --manifest-path research/genus3_cover_cold_target_20261115/native-check/Cargo.toml
```

The checker also tests rejection of changed scalars, source pins, missing
trials and edited summaries. Its pass means those local checks passed; it
does not authenticate the historical clock readings or physical execution.

## Sources checked

- Joux–Vitse, *Cover and Decomposition Index Calculus on Elliptic Curves
  Made Practical*, [author PDF](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CoverDecomp.pdf),
  Appendix A.1, equations (6)–(8): the restricted odd-characteristic cubic
  extension family, the genus-three cover, quotient and two-isogeny.
  The paper was read at import time; the construction is established prior art.
- [Residual-walk note](../notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md),
  sections 9.6, 11.1 and 11.7: frozen references, cubic-extension accounting,
  and the need to charge linear algebra and large-prime elimination.
- [Earlier cubic-extension historical import](../cubic_extension_transfer_20261025/README.md):
  preserve Python provenance separately from a native execution path.
- Sources checked (KB): no KB MCP tools are loaded; `scripts/research-kb status`
  returned 127 because the executable/catalog are absent. The files above are
  the fallback, not evidence that no related result exists. ecdb was not
  accessible without its configured AWS/token route.

## Native execution continuation

The [native solver](native-check/src/solver.rs) implements factor-base
construction, the degree-four map, pole-corrected pullback and Frobenius
trace, Cantor arithmetic, interpolation relations, exact finite-field CG,
individual descent and checked scalar recovery for this exact fixture.
It provides a public-target interface accepting only a point and relation
seed; no known scalar is accepted. The RNG is explicitly specified and
differs from the archived NumPy stream.

See the [bounded development contract](native-check/DEVELOPMENT.md) and
its commands. These checks exercise the execution path; they do not add a
performance comparison. Their claim publication is pending the divergent
PCC-ledger conflict in PR #4284. The existing signed standing is unchanged;
there is no new admitted result or timing series in this continuation.

The historical PDF and its graph remain the report of the original Python
record. They have not been relabelled as native solver measurements.
The [native implementation companion](reports/NATIVE-IMPLEMENTATION.pdf)
diagrams the new path and its bounded development contract.

## Admission, dashboards, and continuation

| Requirement | Status |
| --- | --- |
| Preserve original source and raw record | Imported unchanged; local hashes checked |
| Native scalar/summary checks | Local checks added; signed independence pending |
| Native cover solver | Implementation and bounded development interface added; admission pending |
| Matched strongest native rho reference | Missing |
| Counted cold cost, five online phases, calibration band | Missing |
| Qualified isolation, A/A controls and external comparison | Missing |
| Native formal curve/transfer identity and Lean correspondence | Pending |
| S3 benchmark-record publication and independent validation | Pending |

This PR changes no admitted leaderboard, progress-timeline, curve-registry or
benchmark-index row. The browsing-only copy under `benchmarks/` has no admitted
row. `S`, GAE, rho ratios and scaling exponents remain unknown, not zero.
Generated dashboards and progress exporters remain unchanged because no
qualified numeric series or registered identity is added. No ecdb source row
is promoted by this archive.

Next is native operation accounting and reference integration, followed by
a separately preregistered, fully counted, matched-rho comparison under
the repository's execution/validation gates. No such run is scheduled here.
