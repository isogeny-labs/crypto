# Resume: crypto @ claude/peaceful-babbage-wddn3o (genus 2-16 IC speed-up)

## Goal
Measured index-calculus speed-ups for genus 2-16 curves (hyperelliptic first,
then non-hyperelliptic small-e models). Plan: research/ic_genus2_16_speedup_20261117/PLAN.md.

## Lead / hypothesis
The 1/g! split probability of a degree-1 base dominates from genus ~8. A base
with degree-2 places (arm D) or degree-2 large primes (arm E) should cut trials
by orders of magnitude at g >= 8, at an LA price (D: base ~p^2/4).

## Spec
Instrument: genus_ladder from fffa1b1243 + P1/instrument.patch (MAX_GENUS 16,
REL_FB 16, F_{p^k} point counts, arms D/E). Binary /home/user/bin/genus_ladder_g16
(build in sparse worktree /home/user/crypto-c0b657b; ~8 min). Cell runner P1/run_cell.sh.

## State
P0 passed (exact count identity at g=4 p=2053). P1 gates: relation verification at
g=3 p=101 (0 bad, all certified, arms A,c,D,E); order cross-check K=2 vs K=3 at g=3
identical. Pilot grid g=6..12 running into P1/pilot/.

## Next steps
Finish pilot, freeze P1 protocol, run the grid, write RESULTS.md; then P2 levers
and P3 non-hyperelliptic. Container restarts ~30 min: keep cells short, commit often.
