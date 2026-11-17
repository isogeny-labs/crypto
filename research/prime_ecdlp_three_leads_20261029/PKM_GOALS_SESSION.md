# 2026-10-29 — PKM goals and novelty audit

## What was done (commands, files, features)

Reviewed PKM 2016 primary sections 1,3–5, PR2650 at adde64fdc, the older tower
rounds, the raw m=4,N=20 external-solver report, fall-degree preregistration and
existing backlog cards. Added PKM_GOALS.md, a six-stage diagram/PDF proposal,
the blocked/unfunded verified-oracle backlog card, and a separate Lean module
for the arithmetic m=4 threshold in the existing cost model. No existing
instrument, frozen input or earlier receipt is changed.

## Decisions made (and why)

Prioritize actual point correctness, complete accounting and ablation before
one targeted optimization or larger reach. New local work is the native
extraction/verification integration; the PKM framework and lab symmetry are
prior work. Global novelty and fast solving remain unproved. The old raw/plain
N20 cell already reached productive degree 8, and the symmetric attempt was
memory-capped; neither is an untouched cell. Maximum degree 8 is an input floor
in the m=4 symmetric instrument. Proposed20% cost and 2x memory objectives are
pending protocol freeze and held-out verification, not measured improvements.

## Open threads (unfinished, pick up next time)

G1–G6 remain pending. The panel/replay harness, full cost ledgers, qualified
external arm, frozen protocol and supported execution/resume route are missing.
No run, funding, queue lease or capacity change is posted. PR2868 remains the
integration destination; review any Copilot head before treating it as verified.
Append this handoff to the daily session log after conflict resolution: that
file is under an existing Copilot merge-conflict request and is deliberately
not edited concurrently with that resolver.

## Errors (what went wrong, how it was handled)

Conductor is absent (exit 127); the documented warn-and-allow setup fallback
applies, with no shared reservation claimed. APS ePrint2017/609 returned 403;
the audit does not claim global novelty from inaccessible literature. Existing
checkpoint/artifact access blockers are not bypassed. Validation receipts for
the new planning deliverable are separate from historical software receipts.

Validation completed: native backlog card check exit0; new Lean module build
and official module replay exit0; project proof gate PASS10 declarations,
zero trusted exceptions. All three rendered PDF pages were inspected.
