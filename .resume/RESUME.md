# Resume: crypto @ claude/tooling-build-alq44y

## Goal
EXP-KOBLITZ-REF-002: adopt min(strong, v5) as the matched Koblitz rho
reference below 2^38 (owner decision 2026-11-20), with v5's jump-index
selection tallied as priced work_native, and re-measure both references on the
audit's curves (n17, 23, 29, 31, 37) and targets.

## Lead / hypothesis
v5 was cheaper than strong at small n in the 2026-11-15 audit but left its
jump selection uncharged. Charged at 2 WordOp per selection, M2 (w=0 S equal
to the audit's exactly) and M3 (v5/strong < 1, CI excluding 1, n17-n31 at all
weights) are the frozen predictions; M4 restates the IC distance.

## Spec
research/koblitz_matched_reference_20261120/PROTOCOL.md (frozen e141a2ca27),
specs/n{17,23,29,31,37}.json, run.sh, analyze.py.

## State
Tally code pushed (3783ff5346, test fix 06b18504df); the pricing test passes
locally. PR #4742 is open with auto-merge; its CI is blocked by main-side
failures. ecbench release build, then run.sh.

## Next steps
1. bash research/koblitz_matched_reference_20261120/run.sh (resumes: skips sessions with session.json).
2. python3 research/koblitz_matched_reference_20261120/analyze.py; write REPORT, figures, session log.
3. Close the bench record, regenerate benchmarks/progress, tick docs/ACTIVE.md, update PR #4742.

## Runs in flight
run.sh in the session container (local, L0), sessions/ under the study dir.

## Stop and success conditions
Stop when all five sessions are verified (replay-all) and M1-M4 are evaluated
against the frozen protocol; success is a complete, verified measurement with
verdicts reported as written, whatever they are. Abort on a failed replay audit
or a w=0 mismatch with the audit (M2) and report it.
