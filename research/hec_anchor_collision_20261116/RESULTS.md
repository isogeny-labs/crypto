# Joint anchor-collision descent and signed LP-core solve

Source/target partial relations now enter the SAME weighted large-prime forest as ordinary principal rows. The bounded-shift implementation recovers all six supplied-target scalars in the fixed genus2/3/4panel, with original-source verification and independent sparse scalar replay. This integrates descent with relation collection instead of requiring a completed anchor-only target decomposition first. The algebra and LP variation are classical; this is an engineering integration result, not a mathematical novelty or tenfold performance claim. [pcc:unverified]

## Construction and actual checks

At a full-degree public divisor D=(u,v), bucket only signed anchors by phi=(y-v)/u. Two distinct anchor abscissae in one bucket force two norm factors. The remaining Q has degreeg−1: linear/quadratic/cubic for genus2/3/4. At most2Hsheets are considered, with H catalogue-abscissa scans. One pair per parameter t avoids repeated counting of the same split fibre. Repeated target support is allowed. Direct rational-split input rows are also admitted.

The forest core includes anchor logs, source variable s and target variable t. Seed s=1; t is the requested scalar. Source/target norm rows can retain up to two unresolved catalogue endpoints; forest potentials carry both core variables until principal rows resolve them. Every admitted raw relation is checked in the original Jacobian; every emitted pure equation is checked under a certified catalogue multiplier. Candidate successes are independently recomputed by solve_fast and verified as [k]source=the ORIGINAL public target. No expected scalar enters the solver API. Extra certificate/replay work is gate-only and is not called free production setup.

The panel reuses g2/F251/c11/ell61667, g3/F211/c12/ell4620611 and g4/F61/c2/ell16790591, targets ell/7+19 and ell/11+29. Full model/source/target data are in the preceding [input receipt](../hec_fibre_pencil_20261116/DESCENT-INPUTS.txt); imported instance-generator bytes are frozen. This is not a fresh holdout. Source inputs in this panel are degree-one generators; arbitrary full-degree source coverage is not established by these controls. Prefix H13/36/22 differs from Compact's target-seeded anchor membership.

## First run: one pencil, failures retained

Frozen sourcef121c5264de2, pre-run checkpoint20261116T162027Z. Once-only watchdog180seconds, exit0/empty stderr, no mismatch. Candidate recovers g2target0 and bothg3targets. It retains missing_input_row for g2target1 and bothg4targets. On those outcomes no usable target equation enters the forest, so principal rows alone leave t free. Complete Compact recovers all six controls. Original raw CONTROL.tsv and source main.rs remain unchanged.

The second arm was pre-registered AFTER these observations, not presented as confirmation of the first arm. Original Cargo.toml bytes are preserved as ORIGINAL-Cargo.toml because adding the second binary changes the current Cargo file; the original freeze identifies source versions atf121c5264de2. No original outcomes are overwritten or rerun.

## Second run: bounded input shifts

Frozen source6caad1753ce8, pre-run checkpoint20261116T162341Z. At most8canonical anchor additions/input, separate deterministic RNG, retry only while that input has no admitted row. Track exact integer offsets: direct-split equations subtract walk offsets; collision equations add them. Original-group checks use ORIGINAL source/target, preventing shifted-target substitution. All scans and additions count, no extension past the frozen bound. Once-only watchdog run exits0/empty stderr.

| Genus / p | Target | One-pencil outcome | Shifted k | Shift adds | Anchor scans | Candidate prepared outputs | Compact prepared outputs (derived) |
|---|---:|---|---:|---:|---:|---:|---:|
|2 / 251|0|verified|8828|0|13|416|200|
|2 / 251|1|missing input row|5635|1|26|192|248|
|3 / 211|0|verified|660106|0|36|776|1048|
|3 / 211|1|verified|420084|0|36|768|920|
|4 / 61|0|missing input row|2398674|2|66|912|848|
|4 / 61|1|missing input row|1526446|1|44|1264|768|

All six shifted scalars pass original verification and independent sparse replay. Four shift additions total;22unused final batch outputs remain charged. Compact output count is derived natively as unique_tuples*2^g, including its prepared tail. Recorded Compact count vectors are identical between the two separate control runs; this is deterministic-work reproduction, not a wall-time A/A claim.

Source/target rows precede at most1024unique principal tuples or4096draws (or exhaustion). Duplicate unsigned tuples and unused prepared tails remain real work. The candidate reuses the exact compact signed norm kernel and table root oracle. Its principal sampler and prefix anchors differ from the control. More principal outputs are prepared on three inputs and fewer on three; counts are not seconds or speed ratios. Additional catalogue/table/inverse setup, source/target scans, polynomial checks, root queries, graph/potential work, LA, allocations and final verification remain costs. Counters are selected categories, not complete operation accounting. All benchmark claims remain outside this gate.

## Decision and remaining dependency

The missing-input-row failure is resolved on this finite panel by a separately frozen bounded-shift arm. This is a working joint source/target/LP solve rather than another equivalent residual presentation. It does not establish a complete tenfold cost premise: mixed generation counts and new scans require an actual complete cost argument against the incumbent; the gate includes extra validation unavailable as a production timing comparator. No timing campaign or speedup is inferred from correctness alone.

The mechanism still needs broader source/field coverage and a cost case identifying which charged stage can fall enough to support the mathematical objective. Merely increasing shift/tuple bounds or comparing to Gaussian interpolation would not supply that case. Frozen original failures remain. The full mathematical10x objective stays active; no exponent or novelty result is claimed.

## Artifacts and scope

[Theory/source receipt](THEORY.md), [original protocol](PROTOCOL.md), [shift amendment](SHIFT-PROTOCOL.md), [one-pencil raw](CONTROL.tsv), [shifted raw](SHIFT-CONTROL.tsv), [native comparison](SUMMARY.tsv), [one-pencil graph](ONE-PENCIL.svg), [prepared-output graph](COUNTS.svg), [combined report PDF](reports/run-20261116T1623Z/REPORT.pdf). Identical combined three-page reports are stored under BOTH run ids and explicitly describe both freezes/outcomes. Native Rust summarizes exact counters and renders figures. All pages visually inspected.

No independent scientific validation, machine-checked full group certificate, elapsed benchmark, benchmark-index/leaderboard entry or performance promotion. Canonical performance graphs and previous sealed reports unchanged. Today's shared daily log remains reserved by T-6720; SESSION.md holds this handoff. Primary literature PDF access gap remains explicit, and classical source pointers do not support a priority claim. PR remains unmerged. Unrelated .resume/RESUME.md is preserved.
