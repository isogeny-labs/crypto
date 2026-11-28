# Cold p+1 Dickson/Semaev m=7 elimination: retained null result

- Date: 2026-11-27
- Draft PR: #5259
- Candidate: target-specialized PKM/Semaev chain over the norm-one Dickson tower
- Baseline: archived cold single-target retained-3/streamed-4 exact-x-first m=7
- Cross-target amortization: forbidden and not used
- Outcome: **do not promote**

## Result

This candidate does not deliver the requested speedup. It is exact on the
depth-one finite control, but that four-point carrier already costs
558,571,973 Fp multiply-adds plus 1,433,122 basis-extraction multiplications.
The planted depth-two hit never returns a labeled solver row within the
bounded protocol. The miss and production target were therefore not admitted.

Under the complete predeclared sensitivity panel, the favorable depth-one
lower bound is 86.47x to 182.63x the *full cold production* 3+4 control, and
345.88x to 730.51x the fourfold-promotion ceiling. This comparison counts
field operations uniformly across the finite rung and the production control;
it is an operation-count warning, not a portable wall-time comparison. The
candidate lower bound omits uninstrumented factor-base construction and
symbolic-product arithmetic, so omitted work cannot reverse the result.

This is a scoped falsifying null for the implemented left-chain layout. It is
not a lower bound for every possible Dickson/Semaev elimination and not an
ECDLP breakthrough.

## What was tested

The system uses seven leaf abscissae constrained by the production p+1 tower

```text
y_(i,j)^2 = y_(i,j+1) + 2,
y_(i,t-1)^2 = 2,
x_i = y_(i,0),
```

and six chained S3 equations joining those leaves to one public target. A
fresh factor base, system, F4 oracle, extraction state, and verifier state are
created for every target. Trace learning, warm bases, shared lifting, and any
state from another target are disabled. No generic factor-base swap is part of
the candidate.

The implementation extends the ordinary F4 monomial key to five free
variables, which is the capacity needed by a seven-leaf left chain. Existing
four-variable wire/radix adapters reject a nonzero fifth exponent rather than
silently truncating it.

## Finite correctness evidence

The independent verifier does not call the production Dickson factor-base or
prime-curve helpers. It scans field roots, uses its own affine group law, and
computes exact seven-layer reachability.

| Fixture | Signed carrier | Exact seven-sum states | Result |
| --- | ---: | ---: | --- |
| `F7: y^2=x^3+3`, depth 1 | 4 | 13 | candidate hit resolved and replayed |
| `F31: y^2=x^3+12`, depth 2 | 2 | 8 | planted solver hit unresolved; generator is an exact miss control |

For depth one, the candidate returned `[0,0,0,0,0,0,0]`; independent affine
replay reaches `(4,2)`. The raw row has SHA-256
`b87a24204b4268ba1a57382bae32b2286b0ae1eef5437d49e049cf0e9c075c3e`.

The resolved depth-one system has 12 variables and 13 equations, reaches
degree six, and peaks at 6,288 rows, 6,710 columns, and 651,374 nonzeros. Its
local cold wall is 466,622,769 ns. Wall time is diagnostic; exact work is the
decision metric.

## Depth-two resource censoring

The planted hit was executed three times as the cooperative deadline defect
was narrowed. None emitted stdout, so none is represented as a hit, miss, or
partial solver certificate.

| Custody stage | Minimum elapsed | Peak RSS | Outcome |
| --- | ---: | ---: | --- |
| original between-step polling | 669 s | 1,854,096 KiB | SIGINT, exit 130, unresolved |
| poll between product waves | 179 s | 1,409,036 KiB | SIGINT, exit 130, unresolved |
| poll inside term expansion | 181 s | 1,472,340 KiB | SIGINT, exit 130, unresolved |

The final overrun occurs when a large carried-term sort begins just before the
deadline; Rust's in-progress sort is not cooperatively cancellable. Continuing
to patch resource controls after repeatedly observing the same target would be
adaptive engineering, not evidence for a speedup. The algebraic ladder stops.

## Exact cost panel

The cold 3+4 baseline combines retained-side setup and query:

- 4,981,763 multiplications;
- 1,399,151 squarings;
- 477 inversions;
- 1,058,505 additions and 68 doublings; and
- 971,635 terminal query candidates.

The depth-one candidate lower bound charges the F4 multiply-adds, all recorded
basis DFS and system-specialization multiplications, and witness replay. The
legacy counter name `pkm_grid_fp_mults_uncharged` denotes basis DFS in this
run; it is charged here and is not an enumerative grid fallback.

| Square | Inversion | Multiply-add | Candidate/control | Candidate/promotion ceiling |
| ---: | ---: | ---: | ---: | ---: |
| 0.8 | 50 | 1 | 91.430445x | 365.721780x |
| 1.0 | 100 | 1 | 87.111425x | 348.445700x |
| 1.0 | 200 | 1 | 86.469916x | 345.879664x |
| 0.8 | 50 | 2 | 182.626855x | 730.507420x |
| 1.0 | 100 | 2 | 173.999823x | 695.999292x |
| 1.0 | 200 | 2 | 172.718355x | 690.873420x |

Full arithmetic is archived in `ANALYSIS.json`.

## PKM and Amadori assessment

Petit--Kosters--Messeng supply the relevant construction: replace a high-degree
factor-base equation with a composition of low-degree maps and couple the map
to Semaev equations. This experiment implements that idea on the p+1 Dickson
tower, but m=7 solver width dominates even on the second finite rung.

Amadori--Pintore--Sala add a factor-base dependency and study prime-field
systems through m in `{3,4,5}`. Their S3/S4 splitting suggests different
elimination trees, but does not provide a measured m=7 extractor or a theorem
that a balanced tree avoids the observed intermediate explosion. Changing
the chain to a balanced S3/S4/S5 layout remains a separate candidate requiring
a new symbolic-width preflight and frozen protocol.

Primary references:

- Petit, Kosters, Messeng, *Algebraic approaches for the Elliptic Curve
  Discrete Logarithm Problem over prime fields*, PKC 2016, sections 3.1--3.4.
- Amadori, Pintore, Sala, *On the discrete logarithm problem for prime-field
  elliptic curves*.
- Semaev, summation polynomials for elliptic curves.

No novelty-priority claim is made.

## Why there is no pair-spectrum “win” in this PR

No admissible exact subquadratic pair-spectrum query was found. The known
leaf-to-pair norm/resultant has degree Theta(B^2); writing its output or a full
pair table is already quadratic. A single-target two-sum scan is linear, but
plugging it behind an explicit five-sum frontier does not reduce the cold m=7
total and is not a pair-spectrum breakthrough. Moreover, the Dickson fibre's
curve-lift mask is not closed under elliptic addition, and torus translation
does not supply an elliptic-group homomorphism or one rational lift cover.

An implicit resultant/GCD oracle would become a genuine next lead only after
it specifies an o(B^2) intermediate representation, exact sign/lift filtering,
miss certificates, and witness recovery. This report does not claim such an
oracle is impossible; it records that no construction meeting those contracts
was obtained here.

## Evidence and proof boundary

Public inputs, source-custody manifests, binary hashes, raw JSONL, censored
process records, and independent affine recomputation are archived. They make
the finite claims replayable.

There is intentionally no zero-knowledge proof. A Schnorr-style proof could
prove knowledge of the completed depth-one witness while hiding it, but would
not prove unbiased fixture selection, native execution, operation counts,
memory, or wall time. The witness is public, so transparent replay is stronger
for this engineering claim. There is no depth-two solver result to prove.

## Validation status

- normal release library and both examples: built successfully;
- release finite-control example test: 1/1 passed;
- independent fixture reconstruction: passed;
- independent depth-one raw-row replay: passed;
- `cargo check --release --lib`: passed after the inner deadline amendment;
- focused private release unit test: not executed because the monolithic test
  rustc was SIGKILLed near 8.45 GiB while Cargo concurrently built another
  library; no pass is claimed.

## Reproduce

From the repository root:

```sh
sha256sum -c \
  research/pplus_dickson_m7_elimination_20261127/artifacts.sha256

jq -r '.files | to_entries[] | "\(.value)  \(.key)"' \
  research/pplus_dickson_m7_elimination_20261127/source-manifest-depth2-product-deadline.json \
  | sha256sum -c -

cargo run --release --example pplus_dickson_m7_elimination -- depth1
cargo run --release --example pplus_dickson_m7_elimination_verify -- \
  research/pplus_dickson_m7_elimination_20261127/depth1.jsonl
cargo run --release --example pplus_dickson_m7_elimination_verify -- \
  --fixture-controls
```

The depth-two process records are censoring evidence and should not be rerun as
if they contained solver verdicts. Replayed walls may differ; hashes, finite
carriers, reachability, and the completed witness must not.
