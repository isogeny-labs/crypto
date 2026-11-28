# Classical pencil target-rank gate: pass

The once-only execution at pushed `c11cfde61ba1d27ee2fcc9af919978c8513316d4` exited 0. All eighteen public source/target cases completed and every recovered scalar verified against the original public source and target. This is correctness qualification of a classical sieve, not a timing or novelty result.

| Genus | Field | Cases | Parameters checked | Accepted rows | Final coefficient rank | First determining anchor |
|---|---|---|---|---|---|---|
| 2 | 101 | 6 | 9,696 | 662 | 8 | 2 in every case |
| 3 | 113 | 6 | 10,848 | 77 | 6 | 7–8 |
| 4 | 109 | 6 | 10,464 | 153 | 8 | 4–7 |

Each case used sixteen fixed anchors aD+T with a=1..16. All anchors had degree g, so no degree skips occurred. The gate examined all 31,008 parameters even after recovery. Independent repeated linear division agreed with point buckets in all 31,008 checks. It rejected 239 parameters with shared support; among parameters not already rejected for shared support, 241 had repeated roots and were rejected. These categories follow the ordered rejection logic and are not claimed to enumerate all repeated-root cases including shared support.

All 892 accepted rows passed exact residual-polynomial reconstruction and the original Jacobian identity anchor+residual=0. Folded row coefficients came from qualified signed orbit weights; the target column is 1 and RHS is −a. No expected scalar enters assembly or linear solving. Independent dense elimination records coefficient rank at every accepted prefix; every native sparse-solver answer is checked against the original source/target. Full coefficient ranks reach r+1, namely 8/6/8, resolving the useful-rank concern for this fixed panel. This does not establish rank behavior for arbitrary curves or growing fields.

Raw per-case totals are in CONTROL.tsv; all accepted prefixes, ranks and recovered values are in PREFIX.tsv. FIXTURES.tsv retains original public model qualification. RUN.txt and EXIT.txt record execution identity/status. OUTPUT.sha256 verifies every raw artifact. Git custody is authorized while S3 is unavailable; no upload is claimed. The source, protocol, imports, native implementation and binary were hashed and pushed before execution, and no rerun was made.

The release build produced an executable but reported a debug-info stripping warning caused by a missing LLVM dynamic library in rust-objcopy. Runtime completed without error. The warning is retained in BUILD.stderr. This run makes no timing claim; the next build should explicitly disable stripping as in the prior production comparisons.

## Production extraction boundary

A next runtime implementation can scan rational-point buckets only, accepting a bucket with exactly g+1 distinct residual abscissas and verifying the corresponding polynomial. Since H has degree g+1, those distinct roots exhaust it; shared-support and repeated-root parameters cannot simultaneously have that many distinct disjoint roots. This justifies avoiding exhaustive reference division at every parameter, while keeping final source verification. It does not justify dropping point multiplicity/sign checks or omitting setup costs.

Stop at the first source-verified determining prefix under a prospectively fixed bucket order; charge complete anchor scans before processing their buckets. Construct successive anchors by adding D, retaining public coefficients, and charge those additions. Build fresh factor base, action certificate and orbit maps per solve, with no cofactor/known-log/expected-scalar inputs to the runtime API. Compare with the current Folded and equal-information Expanded methods on identical public inputs. Include all setup, relation generation, linear algebra and final verification. A separate frozen correctness-equivalence and cold timing protocol is required before performance claims.

Known prior art is explicitly resolved in ../PENCIL-NEXT.md. The requested tenfold genus-2+ mathematical breakthrough remains unestablished; these results qualify a completing classical baseline for the next comparison.
