#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: hyperelliptic IC round fourteen, fruitless-cycle window

*Card* `CARD-HEIC-ROUND14-1` (sha256 `5fde0d1c…4e93c5`), commit pin `2fc38dc7` (re-pinned from `6a3d95cd`; only the note and the card changed); binary `hyperelliptic_rho_cycle_window` sha256 `fdad7013…9a575e`, built from `6a3d95cd` (`examples/hyperelliptic_rho_cycle_window.rs`). Receipt `../../run1/receipt.json`: status completed, all nine commands exit 0.

*Host and isolation.* Intel Xeon @ 2.30 GHz, 4 logical CPUs, kernel 6.18.44-fc-v153, x86-64 (a different host from rounds 9–13). `tools/isolated_bench.py`, reserved CPU 3, preflight `--max-other-cpu 1.0`; record `contended: false`, wall 596.6 s. All values are counted operations, deterministic in the seed and independent of the host.

*Arms and workload.* Full-width distinguished-point rho: plain; folded on `±` classes with a passive window of the last 32 walk hashes (round thirteen's walk, instrumented: it counts steps inside a short cycle and changes nothing); folded with the window active (escape on the first repeat). Seeds `0xC0FFEE + t`, t < 40, on the thread's 18 instances; 18 × 40 × 3 runs, every recovered `k` verified.

#figure(image("swalk.svg", width: 100%), caption: [Left: folded/plain walk operations per row, without early escape (round thirteen, reproduced exactly by the passive arm) and with the active window. Right: share of the folded walk's steps the passive window saw inside a short cycle. Source: `../../run1/R14.txt`.])

*Recorded verdict lines* (printed by the binary):
- C14-1: passive trapped share on the two largest rows 78.1% and 71.6% (bound ≥ 50%); the passive arm reproduces round thirteen: *holds*.
- C14-2: active folded/plain, median over the 7 rows with N ≥ 10^5: 0.752 (bound 0.80): *holds*.
- C14-3: active folded `S_walk`, median over those rows: 1.045 (bound 1.05): *holds*.
- C14-4: active fruitless share at most 10% on every row: max 32.0%; every run verified: *falsified*.

*Observations.* Most of the folded walk's steps on the largest rows were spent inside fruitless cycles longer than two, escaped only at a gap limit of about √N; escaping on the first repeat brings the largest row from 4.15× the plain walk to 0.686×. On the smallest rows the active window costs more than it saves (active/plain 0.93–1.08 against about 0.70 without the window), and its fruitless share reaches 32%. Because C14-4 failed, the active folded walk does not become the reference. A plausible reason, to be tested in round fifteen: a genuine rho cycle is shorter than the window on the smallest groups, so the window escapes a useful collision as if it were fruitless.

*Missing.* No cross-method comparison. The window's unpriced work (up to 32 hash comparisons per step) is reported but not priced (§5a). One seed set per row; no intervals. No validator report, so no `benchmarks/README.md` row (validation gate).
