# Deviations from the frozen preset (recorded 2026-11-18)

1. **Memory cap.** `PRESET.txt` froze a 6 GB address-space limit (`ulimit -v`). The main ladders ran with a 4.5 GB cap (README, "msolve"), because the m=3 and m=4 ladders ran at the same time on one shared container. This deviation was not declared in an amendment before the runs. Cells that stopped with SIGSEGV at the cap ("seg") may have finished under the frozen 6 GB limit.
2. **Censored cell.** The 18-bit dense cell is censored (stopped at the cap or timed out). It gives a lower bound only, not a measured time or degree. Secondary run S1 (amendment 2, 11 GB cap) also stopped with SIGSEGV.
3. **Solver mode.** Amendment 1 changed the solver from `-l 2` to `-l 1` after `-l 2` crashed on every input with p > 2^16.

The kill verdict in README.md is `[pcc:unverified]` and should be read with these deviations.
