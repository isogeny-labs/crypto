# Endpoint-corrected word factors

PCC-20261108-87a4e4 is unsigned producer evidence. [pcc:unverified]

Validation01 passes220 paired transcript systems,1140 image-membership checks,
164 phase checks and all29 public outcomes. Thirty-five image values require a
nonzero correction. Four explicit endpoint controls cover a single word, repeated
factors and both initial-sign phases; correction-cap refusal also passes.
No native build/assertion failure. [pcc:unverified]

| Fixed workload | Zero-safe reference | Corrected factors | Additional correction work |
|---|---:|---:|---|
| 220 transcript systems: monomial pairs | 4841072 | 4439003 | 156070 field multiplications, including16031 exponentiations |
| Aggregate circuit nodes across systems | 222420 | 203402 | Public endpoint maps and scalar corrections |
| 29 public cases: monomial pairs | 2600842 | 2388639 | 19844 field multiplications, including1730 exponentiations |
| Large control: circuit nodes | 11379 | 11097 | 15388 field multiplications, including415 exponentiations |

Exponentiations are NOT added again to multiplication counts; their binary-power
multiplications are already included. Counts are deterministic software-operation
diagnostics, not total work or timing measurements. Source setup/calibration,
point shifts, DFT, stabilizer comparison, node construction, maps/sets/copies,
allocation, additions, phase inversion/gcd and final verification remain charged.

Public fixtures: E/F17,a2,b2,P(5,1),r19,m3, every k0..18; and
E/F257,a1,b7,P(1,3),r281,m5,k=[0,1,2,3,7,17,31,63,127,280]. Radii[2,1,3,2].
Both methods return18 extracted scalars and11 verified identity/endpoints, with
332 nominal plus27 guard queries each:718 combined queries excluding setup.
No scalar or image is passed to either recovery method. [pcc:unverified]

Large r998244353,m7,L71303168 control: both methods refuse the64-term cap after
14594 monomial pairs. The corrected circuit's fixture-image membership agrees,
but this does not constitute unknown-image extraction at that size.
[pcc:unverified]

Eleven copied source files and15 compiler dependencies are hash-bound. An
optional corrected implementation lives in corrected.rs; word.rs preserves the
reference byte-for-byte. The original4563-case cohort and older adapters are
unchanged. No cloud work or full Cargo/inherited-cohort rerun. The one-page report
is an inspected derived view of validation01/check.stdout.

The new path stores public endpoint-image corrections sparsely and subtracts two
scalar correction moments. It removes character-section squares. It still expands
circuit nodes in the phase quotient. Next push selected trace functionals through
that product while accounting for intermediate or dual representations.
