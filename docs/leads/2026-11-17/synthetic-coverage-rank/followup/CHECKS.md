# Fixed follow-up checks

This is a deterministic enumeration of synthetic inputs using the two modulo-19 bases and auxiliary matrices from the parent study. The comparisons are exact for these finite populations. The work charges are part of the definitions. There are no timing, workload sampling, or elliptic-curve measurements.

## Coverage and probe costs

Use ordered pairs with repeats. Enumerate all 19 targets and all 36 equally likely planted pairs per base. A probe is membership of the complement after fixing one summand. Use the same ascending list of six candidates in both arms.

- Enumerate raw hit probability and single-draw variance for uniform membership and the exact inverse-multiplicity score. Uniform can exit at its first match. The planted score charges all six probes.
- Define a residual as accepted if one of the first three candidates hits. Uniform always pays three probes. The planted arm reads all six candidates and constructs its inverse-weighted gate score from the same calls. At a budget of six probes per block, compare the mean of two independent uniform draws with one planted draw using the exact variance of each. Report the variance coefficient for a budget of any positive integer number of blocks.
- Check sensitivity by averaging over all 20 three-position subsets of the fixed six positions. Use a fresh subset per observation in the mathematical variance calculation. Report this gate's mean and variance at the same probe budget. This accounting charges membership probes only.
- Confirm the weighted and uniform means agree for each gate and base. Independently check each count by tallying all pairs. Any mismatch fails the replay.

## Rank and charged prefixes

Replay the two streams below. The first two columns are primary, the last two auxiliary. A has successive distinct rows `(1,1,-1,-1), (-1,1,1,1), (1,-1,1,1), (1,1,-1,1)`. B has `(-1,1,-1,1), (1,-1,1,1), (1,1,1,-1), (1,1,-1,1)`.

In both streams the attempt pattern is reject, 1, duplicate 1, 2, reject, 3, duplicate 3, 4. The respective costs are 2, 3, 1, 4, 2, 3, 1, 5. Charge every attempt and retain every accepted row. Use exact span enumeration over fields of orders 2, 3 and 5. At every prefix report accepted count, column coverage, total rank, and rank of the subspace with zero auxiliary coordinates. The primary threshold is rank 2. Compare every prefix to the independently predicted rank and confirm that total rank and coverage agree across streams. Rejections and duplicates must retain the preceding rank at a higher charged cost.

## Replay

From this directory build `g++ -std=c++17 -Wall -Wextra -Werror -pedantic -O2 replay.cpp -o /tmp/ic-followup-replay` and run the executable once with `coverage` and once with `rank`. Compare the output bytewise with the respective receipts.
