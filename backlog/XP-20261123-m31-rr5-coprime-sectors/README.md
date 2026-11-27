# Deferred complete five-sector M31 RR5 comparison

**Status: BLOCKED. No admitted run or execution authorization.**

This card records the next discriminating experiment after
[PR #5119](https://github.com/isogeny-labs/crypto/pull/5119). That work provides
`selector_sectors::construct_hybrid677_sector(fixture, target, k, limits)`
for `k=0,1,2,3,4`. It supplies equations and construction metadata. An
integrated sector solver, complete extraction driver and paired comparison
are not implemented by that API.

The held payload work in
[PR #5099](https://github.com/isogeny-labs/crypto/pull/5099) remains separate.
This card changes neither that PR's hold nor its frozen extraction protocol.
It creates no cloud job, task lease, image publication, funding action or
permission to run either formulation.

## Question and decision

Does eliminating all five coprime-selector sectors, then performing complete
root recovery and signed relation verification, cost less in total than the
coupled RR5 selector on the same public inputs and under matched total limits?

The existing per-system construction counts motivate this comparison; they
do not predict solving degree, matrix fill, runtime or useful relation yield.
Adopt the candidate only if complete, independently authenticated output and
accounting support the stated extraction advantage. Preserve an unchanged
coupled reference when evidence is incomplete or does not support adoption.

## Fixed mathematical scope and input identity

Use the original ordinary curve over `p=2147483647=2^31-1`, the exact B677
factor base, and the same supported RR5 chart as the parent protocol. Keep
all original norm, denominator, distinct-leaf and target-chart conditions.
Do not silently extend the claim to repeated leaves, infinity targets or
target-in-base cases that the existing chart does not cover.

The candidate comparison must cover the **union of all five sectors**. A
single sector or a favorable subset is not complete extraction for that
chart. Both arms receive identical public targets and canonical factor-base
data; no arm receives target scalars, factor-base logs or hidden witnesses.
Retain the frozen positive control and public H0/W0 identities and their
applicable gating order. A successor paired schedule must be separately
frozen and reviewed rather than inserted into the original run by changing
its name or command.

Independently bind the complete factor-base table, its ordering and point
orientations, and the fixed positive target and expected signed control.
Checking only table length, a producer-reported fixture hash or valid group
equations on supplied points is insufficient to establish that these are
the registered inputs. Preserve the archived Y7 CSV and all prior evidence.

## One shared budget across the candidate's sectors

The experiment must allocate **one total work, elapsed-time and memory budget
to the candidate across all five sectors**, matched to the coupled arm.
Starting a new sector must not renew a deadline, replenish critical-pair or
matrix-work allowances, or erase already incurred setup and verification
cost. Specify cumulative counters and peak-memory accounting explicitly.
Charge shared fixture setup once per cold arm and charge all sector-specific
construction, elimination, roots, signs, merging and replay work.

Before readiness, the reviewed protocol must specify the deterministic sector
schedule, budget-sharing implementation, treatment of intermediate data, and
all stopping conditions. It must retain the original native limits and the
one-worker, 180-second scientific outer ceiling and 2-GiB address-space
ceiling unless a separately authorized successor changes those ceilings.
An outer job deadline does not enlarge the scientific deadline. No automatic
retry, fresh per-sector budget, target substitution or enumeration fallback
is permitted by this card.

If any required sector stops, exceeds a bound or lacks complete root/sign
processing, the union is incomplete. Report that outcome and the retained
partial evidence; do not label it Empty or a complete improvement. [pcc:unverified]

## Required observations and checks

- Solving degree; Macaulay dimensions, nonzeros and fill; admitted pairs;
  aggregate arithmetic or work counts; peak memory and elapsed time under the
  same measurement boundary for both arms.
- Complete root recovery, original-equation checks, sign reconciliation,
  identity/exception handling within the supported chart, duplicate removal
  and independently verified public group relations. These are requirements,
  not completed observations. [pcc:unverified]
- Useful independent relation rank with authentic public right-hand sides,
  target recoverability where supported, and independent scalar replay for
  any recovered scalar. Raw relation counts alone do not establish yield.
- Separate fixture/setup, construction, elimination, roots, replay, relation
  processing and rank costs, including failed or censored work. Missing
  costs remain unknown.
- Comparison of the full candidate union with the coupled reference on the
  same targets. An extraction result alone does not establish complete cold
  ECDLP superiority; any later claim requires the full maintained solver
  and its prescribed comparisons.

An independent comparison verifier and result schema must be implemented
and named before the card becomes ready. The generic relation checker can
authenticate supplied rows but does not independently prove extraction
completeness. [pcc:unverified]

## Implementation, admission and cost blockers

No runnable comparison exists, so `code.build` and `outputs.verify` are
empty, `outputs.result_schema` is `unassigned`, and `run.argv` is the
intentional failure sentinel `/bin/false`. These fields must be replaced
through review once an actual implementation exists. They do not describe
an executable research plan.

Readiness additionally requires reviewed immutable source and image identity,
a frozen protocol, a successful checkpoint of the exact inputs, authenticated
submission, complete raw-results readback, approved finite remaining budget,
and live all-tenant capacity. An S3 listing probe does not by itself establish
raw-object read access. No credential, IAM, quota or spending change is
authorized here. Check existing ownership and requests before any later
submission to avoid duplicating concurrent work.

Runtime and core-hour estimates are **unknown**, not extrapolated from the
constructor's variable counts. The numerical resource fields retain ceiling
references from the existing route: two reserved CPUs with one scientific
worker, 2 GiB and a 180-second scientific outer cap. Disk zero means no
allocation has been made. These entries neither reserve capacity nor approve
spending. A defensible estimate and admitted resource plan remain readiness
requirements. Funding is zero while blocked, below the existing funded seed
cards; no cairn funding action has been taken.

The current root-library development gate is also incomplete: attempt 03
exited `124` at its 300-second limit before tests, while its source check
exited `0`. A timeout before tests is not a pass. This card authorizes no
additional build or test attempt and does not replace the preserved failed
or incomplete receipts.

Checkpoint fields remain null because a successor comparison's checkpoint
and resume contract has not been implemented or reviewed. Backlog readiness
requires that contract; it must not silently introduce a resume or retry into
the parent's fixed zero-retry protocol.

## Validation and sources

The card follows the checked-in `backlog.card/v1` schema and the field and
lifecycle rules in `backlog/README.md` and `src/bin/backlog/card.rs`. An existing
blocked card, `XP-20261015-conductor-prime-field-pdp`, supplied a structural
example. The native `backlog` executable was absent from PATH and inspected
workspace build directories. Consequently **native `backlog validate` was
not run**; no new build was attempted and no validation pass is claimed.

Primary references:

- [Construction report](../../research/prime31_rr5_coprime_sectors_20261123/README.md)
- [Construction-only protocol](../../research/prime31_rr5_coprime_sectors_20261123/CONSTRUCTION_PROTOCOL.md)
- [Root qualification record](../../research/prime31_rr5_coprime_sectors_20261123/ROOT_VALIDATION.json)
- [Original frozen extraction protocol](../../research/prime31_four_summand_20261122/scaling-b677/PROTOCOL.md)
- [Held payload integration and admission record](../../research/prime31_four_summand_20261122/scaling-b677/payload-integration/README.md)

Completion requires the admitted, fully accounted comparison and its retained
independent checks to land through a results PR. Until then this card remains
blocked and records pending work, without a scientific result or speed claim.
