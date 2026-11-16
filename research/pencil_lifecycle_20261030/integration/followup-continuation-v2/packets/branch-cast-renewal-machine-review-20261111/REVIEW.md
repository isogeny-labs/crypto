# Branch-helper-cast renewal machine review

## Result

The bounded attempt ended terminally after 192.1372683380032 worker-wall seconds. Pre-custody and post-custody passed. `BranchRootMultiplicity` and `BranchCoefficientMultiplicity` compiled successfully; `IndependentControls` failed, so the separate thirteen-name readback and expected `False` rejection were not attempted.

This is a partial machine result, not a complete five-call validation. The packet outcome is `needs_revision`.

## Machine outcomes

| Module | Status | Evidence |
| --- | --- | --- |
| `BranchRootMultiplicity` | passed | exit 0; `.olean` SHA-256 `e999a7c8e322b2c2fd4a0cb0182a04fed188458b41e1effa72df20cc373c786a`; seven printed declarations used only `propext`, `Classical.choice`, and `Quot.sound` |
| `BranchCoefficientMultiplicity` | passed | exit 0; `.olean` SHA-256 `e770427600c120992331b2ecd77764643d3fb09211f097d960ef604a79e8fa84`; the named theorem used only the same three standard axioms |
| `IndependentControls` | failed | exit 1; lines 22 and 28 each reported ``simp` made no progress`; no `.olean` |
| `Readback` | not verified | not attempted after the controls failure; the thirteen-name readback therefore has no renewal result |
| `ExpectedFailure` | not verified | not attempted after the controls failure; the expected rejection therefore has no renewal result |

The five workspace source files are byte-identical to the published frozen packet. Commit `f3c1b8cc7663ca7d15374ae5e9a16c86839c9d18` contains the published source and independent operational-review commits and exposes the same source, freeze, launcher and replay bytes reviewed here.

## Custody and termination

Both custody phases checked 16,736 dependency files totaling 3,111,892,318 bytes. The pre/post dependency inventories, retained-import inventories and runtime inventories are byte-identical. All 26 retained source/olean bindings match the frozen packet.

The recorded phase wall times were 54.44882508798037 seconds pre, 104.72157733800123 seconds compiler, and 32.84840777597856 seconds post. Each phase retained the declared 1-CPU, 8-GiB, no-swap and 128-task limits. The completion record marks every phase owner terminal. The failed compiler unit retains its failed systemd record but has an empty control group; its manager and all three compiler child PIDs are absent, and the recorded cgroup path is absent.

The run used one attempt, no automatic retry, and completed within the 600-second attempt envelope and before the 2026-11-11T12:00:00Z deadline. Root's campaign accounting charges the full reserved 600 seconds even though observed worker wall was lower.

## Claim boundary

The two successful compiler calls establish acceptance by the pinned Lean compiler and its declared dependency trust boundary. No independent binary kernel rechecker was used. This audit did not independently rederive the theorem semantics or promote a research claim. The controls failure and two unattempted modules prevent a complete machine-validation outcome.

The receipts themselves state the remaining custody limitation: pinned shared-cache pre/post checks do not exclude transient concurrent changes. No discrepancy was observed between the pre/post snapshots.

## Next action

Preserve this attempt as terminal failed evidence. A correction to the two no-progress control steps would be a new source revision. It needs affected-source review, publication and a separately authorized new attempt; this attempt must not be resumed, retried, or treated as if its unattempted controls passed.
