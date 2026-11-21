# RESULT: LEAD-ICB-retained-setup-online

## Outcome
- Status: ran to completion. Exit code 0, wall 0 s by `date` (run.py reports elapsed_s = 0.144).
- Single run, no retries. run.stderr empty. RESULT.json is one JSON object.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): "min R over 9 generic cells x 2 independent seed sets >= 0.5; IC positive-control R <= 0.25; A/A max ratio-of-ratios <= 2.0." Falsified if generic min R < 0.25; otherwise inconclusive.

Measured:
- Generic min R = 4.25 (r=16411, S=50, seed set A). Range of R over the 18 cells: 4.25 to 118.1. Condition >= 0.5 holds; falsification condition (< 0.25) not met.
- IC positive control: R = 0.00296 (p=262147, S=150, T=2.275 online ops). Condition <= 0.25 holds.
- A/A max ratio-of-ratios = 8.50 (r=16411, S=12: 118.1 vs 13.9). Condition <= 2.0 FAILS.
- All 60 trials solved in every generic cell.

Verdict (run.py): inconclusive. Controls: null (generic) behaved as predicted (R far above floor); positive control passed (detector sees the non-generic oracle); A/A failed (noise too wide), which is the stated reason for inconclusive.

## Reading
- Numbers match the verdict field; no discrepancy. The inconclusive verdict comes solely from the A/A noise gate.
- A/A spread is large because T is heavy-tailed at 60 trials (walk restarts, DP collisions), as the protocol warned. Several A/A pairs differ by 2-4x in T, so R varies 4-8x.
- Generic R sits at 4-118, well above 0.5: the toy table-rho is far from the BL floor of order 1 (small constants, uncharged but crude DP choice, S very small, restarts counted). So the toy does not locate the floor; it only shows generic >> IC.
- IC control R is about 1700x below the generic minimum, but that follows by construction (150 retained logs, 1-op smoothness oracle, T~2 trials), so it shows only that the detector separates the two oracle types.
- Caveat: elapsed 0.14 s is far below the protocol's "few minutes" cost estimate. The IC table of ~262k entries plus 18x2x60 walks in 0.14 s is surprising; I did not inspect or alter run.py, so I cannot say whether the setup was truncated or Python was simply fast. Treat with mild suspicion.
- The toy does not touch Koblitz curves, Frobenius/|Aut| reduction or real IC cost (PROTOCOL section 6).

## Bearing on the sub-sqrt(p) question
none. A toy generic table and a multiplicative-group smoothness oracle say nothing about whether an EC online IC can reach S*T^2 <= r/|Aut|; the run is inconclusive on its own gate.

## Standing
[pcc:unverified], single local toy run, no independent replay.
