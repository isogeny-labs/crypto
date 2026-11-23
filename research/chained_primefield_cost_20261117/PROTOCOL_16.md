# Frozen prime-field inverse cost unit16

Task tracking precedes research: ACTIVE-only014387238b9aee0afeafd37b2ce34fe475013160
is published on the task branch and open lane crypto#4544. This is a bounded
development unit for the existing guarded cubic inverse; no timed benchmark,
paid job, original full-class or LP population is requested. [pcc:unverified]

## Question, prior work and scope

Unit15 qualifies the replacement and records5 surrounding cubic multiplications
removed per shortcut,2 with nonzero operands. That does not account for the
changed Euclidean input a versus a² or underlying prime-field inversion calls.
Measure those actual calls on the complete unchanged15 population. Preserve
every loss, gain and incomparable vector; no improvement is an acceptance test.
[pcc:unverified]

Sources checked before this protocol: successful15B sourcefe5d0da and actual
26,368B stdout394c8e2a; exact FpPoly BigUint arithmetic, CubicField::inv/mul/c,
Field::inv/inv_original15, completed closed fibres; Cargo.lock num-bigint0.4.8;
that registry's actual BigUint::modpow and power.rs odd-modulus Montgomery route;
AGENTS implementation/cost/admission rules. KB tools are absent; status wrapper
and Conductor return127. Use the documented source fallback, without novelty
or absence inference. Reuse conditional field proof69b890 unchanged.
[pcc:unverified]

## Prospective arithmetic accounting model

The ordered vector is [M,A,S,R,I,R64]: explicit BigUint multiplication, addition,
subtraction and remainder operators outside modpow; one atomic BigUint::modpow
call used for prime inversion; and the u64 remainder in CubicField::c. Include
the p−2 exponent subtraction in S. Do not silently expand modpow into a guessed
binary exponentiation: the actual dependency selects Montgomery arithmetic for
odd p. Record a complete sparse histogram of every normalized inversion input
a at each arm/interval; p is fixed at101 for the main population,7 for the
eleven polynomial calibrations. I counts only nonzero inputs reaching modpow.
This model charges the entire hidden inversion atomically, without claiming
internal limb, allocation, instruction, CPU-time or GAE accounting.
[pcc:unverified]

Instrument17 explicit sites: constant reduction; coefficient reduction; poly
add/sub/neg; poly multiply product/accumulate; scalar reduction/product; division
coefficient/accumulate/subproduct; Horner evaluation; inverse reduction/exponent/
modpow; and CubicField::c's u64 remainder. Preserve each original arithmetic
expression. Counters are Rust, default disabled, with a declared test-only
corrupt-multiply counter mode. Do not edit num-bigint or rebuild dependencies.

Keep two simultaneous budgets: all operations in the instrumented backend, and
operations occurring inside Field::inv, delimited by an exception-safe scope.
The latter is the complete prime-field arithmetic model for that inverse call,
including the original norm wrappers and inner Euclidean algorithm. The former
on a whole map excludes direct Fp2/u64 work outside the instrumented backend;
do not call it a complete map/solver cost. Exclude setup, encoding, assertions,
product checks, counter bookkeeping and formatting from measured intervals.
Require the two budgets equal for each primitive Field::inv interval.
[pcc:unverified]

## Twelve independent hand-counted calibration cases

For the first eleven, p=7, x=2+3X, y=4+X; construct setup with counters off.
All expected R64=0. The twelfth uses the already-qualified p101 cubic field.

| Case | Expected output | [M,A,S,R,I,R64] |
| --- | --- | --- |
| constant(9) | 2 | [0,0,0,1,0,0] |
| from_coeffs([8,0,9]) | [1,0,2] | [0,0,0,3,0,0] |
| x+y | [6,4] | [0,2,0,2,0,0] |
| x−y | [5,2] | [0,2,2,2,0,0] |
| −x | [5,4] | [0,0,2,2,0,0] |
| x*y | [1,0,3] | [4,4,0,8,0,0] |
| scalar_mul(x,2) | [4,6] | [2,0,0,3,0,0] |
| x.eval(2) | 1 | [2,2,0,2,0,0] |
| fp_inv(2) | Some(4) | [0,0,1,1,1,0], histogram{2:1} |
| fp_inv(0) | None | [0,0,0,1,0,0], empty histogram |
| x.divrem(1) | ([2,3],0) | [4,5,4,10,1,0], histogram{1:1} |
| cubic101.c(203) | 1 | [0,0,0,1,0,1] |

Require all twelve positive outputs/vectors/histograms and all17 site coverage.
Under the declared corrupt mode, omit multiplication increments at poly product
sites: x*y must retain its mathematical output but fail the frozen cost oracle.
Detect that one negative, restore the original mode, and require the positive
multiply oracle once again. CALIBRATION_DERIVATION_16.md gives the manual traces.
[pcc:unverified]

## Exact main population, outputs and acceptance

1. Reuse all64 n_i=1+(65537*i mod1030300), all8 compositum inputs a=i+1,
   b=3i+1, both zero assertion rejections,8 sign controls and3 invalid constructors
   from15. Use the same p101,cubic[3,26,3,1],nonresidue2 and literal14 maps.
2. Construct both maps before enabling the candidate policy; reuse all17 fixed
   witnesses,17 forward and17 positive-dual calls. Preserve the forward9/8 route
   histogram and both declared closed quadratic orbits, all wrapper5/2 count
   relations, equal inner inverse counts,1054 shortcuts and0 general calls.
3. Compare every one of72 primitive and34 map rows with the complete frozen15B
   scientific snapshot: exact inverse encodings/canonical classes/routes, never
   its harness elapsed literal. Keep every snapshot byte and hash. No scientific
   output is selected, modified or discarded after measurements.
4. For all106 pairs emit complete both-arm [M,A,S,R,I,R64] budgets, inversion
   input histograms and site totals for the whole backend and inverse-only scope.
   Require histograms sum to I. All8 general primitive arms must have identical
   vectors, histograms and site counts. Complete arithmetic must agree with15
   and between arms even if a cost comparison is unfavorable.
5. Rust records aggregate and per-pair component comparisons: candidate smaller,
   baseline smaller, equal or incomparable. Also compare expanded vectors that
   price each inversion input separately, avoiding an equal-price assumption.
   Assign no timing/GAE/empirical prices and require no cost win.
6. New exact filter: nonsplit::map::tests_14::frozen_prime_cost_16. Require one
   new test passed and15 inherited tests compiled/filtered. All arithmetic,
   calibrations, comparisons, sums and output generation are Rust. Any failure
   is preserved whole before a separately frozen source/criterion revision.

These are prospective criteria. No limit, population, expression, price or
oracle may be fitted to an observed outcome. [pcc:unverified]

## Execution and custody

Publish protocol/report before new code, then the complete actual source/caller/
configuration closure before scientific compilation or calls. Use Rust1.99.1,
same3 exact prebuilt rlibs, one worker,120s per compiler/Clippy/native phase,
strict Clippy with original positive0/negative1 controls, format, and unchanged
8MiB RAM/3MiB scratch preflight floors. No cold dependency build.

Use shell for thin build/run orchestration; Python may handle only file/hash/
JSON/custody I/O. Preserve distinct immutable build, pre-call and post-call
manifests; copy a separate555 qualifier and hash it before native calls. Full
original/copied/dependency/compiler/tool guards remain. The scientific driver
does not delegate to Python. Reuse15's already-compiled conditional proof; new
cost observations are not a new field/Picard theorem or signed replay.
[pcc:unverified]

Retain a whole byte/mode-checked packet containing every declared source, all
3rlibs,bothELFs,all full streams/commands/environments/exits and all negatives.
Only compiler scratch is prospectively excluded and fully inventoried. Follow
current artifact policy: no new inline compressed dataset, file splitting,
suffix change, keep exception or hidden raw-output pruning. Retain raw whole
packets privately while S3 migration is unresolved; publish source/reports and
the allowed complete text/derived custody metadata, with the exact unresolved
raw publication status. Use the existing GitHub Git-data connection when CLI
write credentials fail, checking exact trees and expected-SHA fast-forwards.
[pcc:unverified]

After this unit continue actual nontrivial Jacobian-chain/prime-field descent
work and admitted matched costs when unblocked. Original full EKS workloads,
paidSTOPe42949,locked vault8edec422,signed independent replay and generic formal
bridges remain unchanged. Do not start paid work or change provider/identity.
[pcc:unverified]
