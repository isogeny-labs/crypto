# Complete-or-capped degree-four closure from retained n83 parents

The preceding propagation dataset made progress: all twelve retained systems
have249 independent cubic rows outside complete V3, with exact native/fresh/S3
replay. That procedure never multiplied cubics. Its exhausted queue therefore
does not test complete V4. The full objective remains all n83 families,m2–9,
actual fixed2^27-candidate point bases, FFD/LFD and elite bases lowering PDP
regularity. No earlier dataset, equation, target or point base changes here.

## Sources checked before implementation

- Caminata–Gorla, https://arxiv.org/html/2112.05579, Definition1.5 and section2:
  V4 must contain every allowed degree-bounded product, and finite stabilization
  is not itself a global LFD upper certificate.
- Semaev, https://arxiv.org/html/1504.01175, section4.5: the field-multiplier
  identities explain the retained degree-four seeds; the F4 bound is an
  assumption tested in the cited paper, not a theorem inherited by this run.
- Native donor src/cryptanalysis/sparse_macaulay.rs, SHA256
  9f794a1e73774dcce0fa24d4468d73a2500d939858857700cfc9fcfe30ab4efa:
  reuse the sorted-index symmetric-difference primitive. Its descending-column
  bulk bucket engine is not directly used because these parents have ascending
  graded-colex columns and the experiment needs streaming replay provenance.
- The local lead-atlas query returned no vocabulary match; its expected atlas
  Markdown is absent in that checkout. This is not evidence of novelty.

## Algorithm and mathematical scope

Replay complete V2, the immutable V3 prefix and completed suffix, every saved
degree-four witness, and the previous propagation trace. Keep its verified
echelon rows as the initial basis. Each belongs to V4; the complete V3 part is
identified separately and retains its earlier completeness proof.

Represent each row by a dense degree-at-most-three block and a sorted, unique
list of quartic column indices. Columns retain graded colex ordering and the
existing lower-block indices. Multiplication is allowed only for a source of
degree at most3, so its ordinary product with a variable has degree at most4.
Reduce repeated variables using the original Boolean field equations. Record
every reduction and inserted row. Quartic rows are never multiplied.

Queue variable products of all added parent rows of degree at most3 first,
then all degree3 rows of the complete V3 parent. Within each initial group,
process variables in increasing index, then row IDs in increasing index.
Products of complete-V3 rows of degree at most2 are already in complete V3.
Every newly inserted row of degree at most3 receives an urgent FIFO row task
covering all variables; urgent tasks precede the remaining initial groups.
Each task records its next variable. No unprocessed source is discarded.

Graded echelon form makes the intersection with degree-at-most-three the span
of the basis rows of those degrees. Consequently, when every task is exhausted,
the Boolean normal-form space contains all original equations and is closed
under every permitted variable product. Iterated products give all permitted
monomial products. This certifies the complete normal-form image of V4.
The original Boolean equations justify lifting every normal-form operation to
ordinary degree at most4. Queue exhaustion is a completeness claim only for
this new algorithm with its full initial coverage, not for the earlier method.

Even complete V4 does not by itself give global LFD<=4. Reuse only the verified
unit or exhaustive affine ideal-equality upper gate from
DEGREE4_PROPAGATION_DESIGN.md. At most12 free variables may be enumerated for
that gate. Upper4 combined with the inherited strict lower4 gives exact4.
Otherwise exact/global upper stay null. No cap or incomplete frontier implies
LFD>=5, and ranks do not rank factor-base quality.

## Bounds, retention and controls

All twelve unchanged m3 identities are attempted, without automatic retries.
Proposed production ceilings, to be frozen with the tested implementation:
16,384 new independent rows;2GiB serialized new-basis payload;131,072 new
product attempts;1,048,576 quartic nonzeros per active reduction;8GiB virtual
memory;one native process;1800s per producer and fresh replay plus5s grace;
48,000s outer. Caps are explicit finite-prefix observations, not substitutes
for the full objective. Preserve every admitted row, reducer ID, queue cursor,
blocked candidate and partial operation. Flush deterministic progress at least
every32 completed attempts and at every clean stop.

A weight cap during reduction saves the partially reduced candidate and the
exact reducer prefix, marks reduction incomplete and leaves that source pending.
A storage cap after reduction saves the unreached insertion separately and
leaves its source pending. Neither row is counted as an independent basis row.
An external timeout preserves partial files but has no successful replay claim.

Independent tiny dense V4 calculations must match the hybrid engine when it
completes. Test quartic indexing, Boolean multiplication against symbolic sets,
sparse XOR against dense XOR, a genuine quartic-to-lower cancellation, positive
unit/affine and negative controls, every cap, all queue coverage and priorities,
and rejection of altered/truncated rows, reducer traces, queues and verdicts.
Retain all predecessor tests. Native Rust performs every polynomial computation.

Freeze/push source and protocol before production, round-trip the source bundle
through S3, and checkpoint. Preserve all raw/partial files in a documented
archive. Replay from restored S3 data with all four immutable parent archives,
checking manifests before/after. Publish data before the inspected report.
Producer replay remains distinct from independent scientific review.

Fresh EKS inventory has no duplicate n83 execution. The retained storage probe
18249a5f-9893-4cb6-97e6-5af2f63c327c remains failed, application cause unconfirmed.
Other owners have active jobs. Preserve the existing-host fallback with no
IAM/capacity changes and no timing claim. This phase does not claim CRIU recovery.
