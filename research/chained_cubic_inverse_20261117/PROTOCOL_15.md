# Frozen cubic-subfield inverse unit15

This is a new bounded correctness and source-level operation-count unit for the
existing operational Richelot evaluator. It does not replace the original full
12,317-class or LP/EKS workloads and schedules no paid job. Source14 and its
completed native run remain unchanged. No timing or ECDLP-solver advantage is
claimed from this protocol. [pcc:unverified]

## Question and sources checked

Can the existing compositum inverse use direct cubic inversion when its second
component is zero, preserving the zero rejection and the exact general fallback?
Does that replacement preserve the completed closed-fibre divisor outputs while
eliminating the redundant cubic multiplications surrounding the inner inverse?
[pcc:unverified]

The exact source is the successful14F snapshot f35867831245560527428e3562ab55d74eefb5a0,
also retained in full265b97cabcc41ad60ba86d2d769b4cc24836c4ea. Read sources:
`src/nonsplit.rs` CubicField inverse/multiplication, `src/nonsplit_map.rs` Field
inverse and polynomial/Jacobian code, `src/closed_fibres.rs`; ConsumerE079
`CLOSED_FIBRES.md` resultant/unit/fallback argument; Lean4.35.0's actual
`Init/Grind/Ring/Field.lean` definition and inverse-product/cancellation lemmas.
The prior Kunzweiler correspondence and chart references remain those of14.
KB tool discovery finds none; prior wrapper127/documented fallback persists.
No novelty or literature-absence inference is made. Subfield inversion is
standard; this unit qualifies its concrete integration. [pcc:unverified]

## Fixed implementation boundary

Preserve the entire original compositum inverse body as an explicitly callable
baseline. Default mode invokes it. Candidate mode checks the second component:
if b=0, preserve the original nonzero assertion and return (inverse_K(a),0);
otherwise invoke the unchanged original inverse. No sqrt, target-scale, pairing,
dual-sign, curve, polynomial, group, descent or closed-boundary formula changes.
The source14 old tests remain compiled and filtered. [pcc:unverified]

Add observable counters at the CubicField multiplication/inversion boundary,
and counts for guarded/general Field inverse routes. Counter arithmetic is Rust.
The underlying CubicField multiplication and Euclidean inverse expressions remain
unchanged. The inner Euclidean inverse can receive a versus a², so eliminating
wrapper multiplications does not establish a complete base-field cost or runtime
gain. Counters are not CPU-instruction or timing measurements. [pcc:unverified]

## Exact prospective population and acceptance

Use p=101 and irreducible cubic [3,26,3,1], the same checked nonresidue and
literal Cpi1_j1 source/target/pairing/scale13 as14. No outcome-based selection.

1. Cubic inputs: exactly64 encodings n_i=1+(65537*i mod1030300), i=0..63.
   Require distinct nonzero encodings and exact original/candidate inverse
   equality, with both products equal1. For each inverse-only interval require
   baseline5 CubicField.mul calls,2 with both operands nonzero,1 CubicField.inv;
   candidate0 multiplication calls,1 CubicField.inv and1 guarded shortcut.
2. General compositum inputs: exactly8 elements with a-encoding i+1 and
   b-encoding3*i+1, i=0..7. Require b nonzero, exact inverse equality, both
   products1,5 wrapper multiplication calls on each arm and general fallback
   taken by the candidate. Record every full input/output encoding and count.
3. Zero: both original/default and candidate paths must reject via the retained
   nonzero assertion. Catch only these2 declared expected panics and preserve
   their complete raw diagnostics. Do not suppress the panic hook.
4. Wrong-sign controls: for the first8 cubic inputs, negating the correct
   inverse must fail the product-one check. Three constructor controls must
   reject: (101,[0,0,0]), (101,[100,0,0]), (15,[1,1,1]).
5. Construct two identical actual14 maps in default mode, then enable only the
   candidate inverse policy in one. Reuse the exact17 fixed14 witnesses once.
   Compare canonical output classes and route codes for17 forward closed calls
   and17 dual closed calls on those forward images. Require the same forward
   route histogram9 LowerDegree/8 Quadratic, including both declared closed
   quadratic-orbit labels; no boundary is silently removed.
6. For all34 paired map-call intervals require equal inner cubic-inverse counts,
   exact class/route equality, and baseline-minus-candidate multiplication counts
   equal5 times the recorded shortcut count (nonzero-operand count difference2
   times shortcuts). Candidate general inverse calls must be0 on these closed
   observations, with positive total shortcut coverage. General/noncubic fallback
   is covered separately by the fixed8 compositum inputs. Unobserved closed
   boundary routes remain explicitly untested, with their source unchanged.
7. Emit a complete compact native stream with every primitive encoding, class
   key, route and arm count, final exact totals, and no DLP solve or time ratio.
   New filter is `nonsplit::map::tests_14::frozen_cubic_inverse_15`; require
   one new test passed and14 inherited tests compiled/filtered. No old full-class
   test is copied into the extracted original prefixes or executed.

All criteria above are prospective. A failure is preserved before any separately
frozen source revision; do not narrow the population, change a sign, disable a
warning, modify counters to fit observations or raise a resource limit.
[pcc:unverified]

## Formal and written obligations

Derive the field identity and subfield closure in PROOF_15.md. Implement a new
Lean source using the installed standard `Lean.Grind.Field`, with norm-on-(a,0),
a*(a*a)^(-1)=a^(-1) for nonzero a, and equality of the two inverse pairs.
Compile the separately copied immutable proof source with Lean4.35.0, warnings
as errors and default proof limits, then inspect `#print axioms` for every named
theorem. No sorry/admit/custom axiom or native_decide. Standard logical axioms,
if printed by imported library lemmas, must be listed; no Field instance for the
Rust cubic implementation or complete Picard formalization is claimed.
The inverse statement is conditional field algebra, not an unconditional
polynomial AST identity over the identity-certificate format's fixed prime.
[pcc:unverified]

## Execution, custody and continuing obligations

Freeze the protocol publicly before new code; freeze all exact source/caller/
proof/configuration inputs publicly before scientific compilation or execution.
Rust1.99.1, one worker,120s per compiler/lint/Lean/native call, same8MiB RAM and
3MiB scratch preflight floors; no cold dependency build. Reuse the full three
exact rlibs from the retained unit12 archive. Strict direct Clippy requires
the existing positive0/negative1 sentinel and canonical diagnostic. Format and
full original/copied/dependency/tool/qualifier guards remain. Run the copied555
native qualifier with the exact declared filter; pin its full hash before calls.
Lean source is independently copied read-only before its call. [pcc:unverified]

Retain the entire declared source, complete3rlibs, both native ELF copies, Lean
source/output and all failed/successful raw streams, commands, environments,
exit statuses and metadata in a whole byte/mode-checked archive. Only compiler
scratch is prospectively excluded and fully inventoried. Keep full raw packets
while S3 publication is blocked; source, full textual results, proof sources and
exact custody/status records are checkpointed in GitHub under existing artifact
rules. Do not split/rename data or add a policy exception to evade a gate.
An unavailable upload is an explicit remaining dependency, not completed custody.
[pcc:unverified]

After exact correctness/count/proof outcomes, create inspected reports and an
admitted native timing/base-field cost comparison when its access/budget permits.
The original full EKS populations, paidSTOPe42949, locked vault8edec422, signed
independent replay, generic field/Picard bridge and full solver comparisons remain
open. Do not access the vault or use another identity/provider to bypass a stop.
[pcc:unverified]
