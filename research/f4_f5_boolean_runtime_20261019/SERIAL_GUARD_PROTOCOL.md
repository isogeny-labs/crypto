# Fresh guard confirmation after the four-worker negative control

Observed in run20261019-030905-boolean-native-serial-cl-25ad47: all eight
one-worker cells qualify faster; primary n23 paired ratios are1.0770 F4 and
1.0652 F5. The four-worker n17 F4 negative control is slower, ratio0.8436
with CI[0.8284,0.8504]. Other four-worker controls are neutral. Retain every
result. Do not claim the unrestricted guard is regression-free.

Hypothesis: asking Rayon for the current pool size can initialize its global
registry on the calling thread, changing cold-start behavior even when the
subsequent iterator remains parallel. Scheduling or VM effects are also
possible; the exact cause is not established. Narrow the opt-in switch to
processes explicitly configured with RAYON_NUM_THREADS=1, cache that decision,
and still require the actual current pool to have one worker before choosing
the serial iterator. With an explicit four-worker setting, both switch states
cache false and skip the registry lookup. Nested multiworker pools retain the
parallel path. An unset or automatic worker setting leaves the switch inactive.

Use one fresh dedicated c7i.2xlarge, on-demand (same-type Spot fallback only
for a launch refusal), TTL90m, full source checkout and60-second cooldown.
Same native isolation, first eligible of at most three attempts, fixed source
and executable hashes, output checks, and bootstrap qualification as before.
No cross-host pooling or multiplication of prior gains.

Repeat the previous12 matrix cells: F4/F5, one worker at n17 seed230 and n23
seeds230/231/232, and four-worker negative controls at n17/n23 seed230.
Five baseline A/A and five same-binary serial0/1 pairs in every cell. Keep
single-write/vector switches1 for these incremental pairs. Additionally take
one legacy-kernel sample per round at one worker (all three switches0) and
compare directly with that round's candidate. That gives a same-host complete
legacy-to-candidate matrix ratio without multiplying experiments. Keep every
earlier failed control and gate unchanged. Report all neutral/slower results.

Run the ten GF2 reference tests for serial0/1 at one and four workers before
timing. The four-worker negative controls must not be interpreted as gains:
their effective hot path is intentionally identical. A repeated material
control regression blocks a regression-free claim/default change. This is
still a matrix-call study, not a recursive complete-root or whole-IC timing.
