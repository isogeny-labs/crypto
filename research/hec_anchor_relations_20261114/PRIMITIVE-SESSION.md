# Primitive relation working log, 2026-11-15

## Protocol freeze

### What was done (commands, files, features)
Read the dated shared log and rational study log, checked dirty state (only
the pre-existing shared resume edit), and reviewed the rejection code.
Primary paper normalization conditions and KB source receipts are recorded
in PRIMITIVE-PROTOCOL.md. Claimed scoped taskT-6032before source edits.

### Decisions made (and why)
Previous turn was progress: established algebra/rank checks, published regular
PR4274and reached green CI at a728d562fe. Test whether common-factor rejection
throws away useful primitive functions. Remove canceled anchor conditions
rather than assuming rational interpolation survives at denominator zeros.

### Open threads (unfinished, pick up next time)
Implement normalization and branch/repeated-factor controls; then compare the
same frozen36arm rank screen with default rejection byte-identical replay.
Keep the full mathematical10x objective active; runtime remains unmeasured.

### Errors (what went wrong, how it was handled)
No new error. Shared resume file remains untouched. The lexical KB predates
this study and did not supply directly relevant normalization prior art; the
primary rational interpolation paper was read directly instead.

## First normalization gate

### What was done (commands, files, features)
Added primitive normalization to rational.rs. All9408generated cases pass,
including684/397/289complete split group checks at genus2/3/4;262of these
were previously rejected common-factor cases. The synthetic branch control
failed exact division. Full output retained as PRIMITIVE-GATE-INITIAL.txt.

### Decisions made (and why)
The control coordinates belong to x^5-x, not the written x^5+x. Amend that
single synthetic-input coefficient before rerun, without changing coordinates,
assertions, production algebra or generated curve fixtures.

### Open threads (unfinished, pick up next time)
Run the corrected branch/repeated-factor controls; only after they pass,
extend the frozen rank screen with explicit --primitive mode and compare yield.

### Errors (what went wrong, how it was handled)
I chose branch coordinates for a different curve equation. Exact division
caught the invalid control. Preserve the failure and protocol amendment;
no generated-curve failure was hidden or removed.

## Corrected algebra gate and rank implementation

### What was done (commands, files, features)
All7tests now pass, including corrected branch/repeated-factor controls. The
production normalization code was unchanged by the synthetic-input correction.
Generated-panel split group checks total1370, including262recovered common-factor
cases. Implemented explicit --primitive rank arm with removed-anchor accounting,
full principal checks and independent Gaussian rank validation. Default replay
is running before the candidate arm.

### Decisions made (and why)
Require the rejection-mode stdout to be byte-identical to the committed prior
rank output before interpreting primitive mode. Count normalization admissions
and projected rows separately; added relations need not add independent rank.

### Open threads (unfinished, pick up next time)
Complete default replay, run primitive mode with the same frozen inputs/caps,
and inspect every arm's rank, admission, duplicate and terminal status.

### Errors (what went wrong, how it was handled)
The synthetic sign error is fixed and the initial failed output remains in Git.
No new algebra error appeared in the corrected gate. Runtime is still unmeasured.

## Paired rank/yield result

### What was done (commands, files, features)
Default rejection output reproduces RATIONAL-RANK-VERIFIED.jsonl byte-for-byte.
Primitive mode completes all36arms, visits133730functions and admits5344rows
before its arm-specific stops. It admits181normalized rows,37projected directly.
Independent Gaussian ranks agree on all36arms. Compared with rejection,11arms
need fewer functions and25are unchanged; maximum count ratio1.4054878049.
All23rank attainments,12exhaustions and one tuple cap remain the same statuses.

### Decisions made (and why)
Normalization is useful but does not by itself establish the10x cost case.
Do not compare aggregate admissions as yield: rank stopping changes stream
prefixes. Preserve both modes and their complete arm outputs. Next integrate
an explicit single-target solver with descent-seeded anchors and scalar check;
the untargeted rank screen cannot price those dependencies.

### Open threads (unfinished, pick up next time)
Freeze and implement an integration correctness gate before timing. It must
report requested/effective anchor size, constructed/consumed functions,
normalization, all cold phases and final verification, with no target scalar
available to the solver. A timing protocol is still separate and unregistered.

### Errors (what went wrong, how it was handled)
No new error. Primitive mode retains all formerly capped/exhausted cases.
Counts are not runtime ratios, and the earlier synthetic-input failure remains.
