# Amendments (dated; the original rule in PROTOCOL.md still governs the headline verdict)

## 2026-11-13 A1 — larger negative-control population for the one out-of-band cell
Reason: under the frozen rule 41/42 fibre cells were within band; the one exception is
16-bit, B rule p^1/4 (B=16), arm A2 (fibre, interval image set): W2 ratio 0.953, W3 ratio 0.885
against a band of 0.02/0.025. The band there came from 3 random seeds whose sd was ~0 at B=16
(few collisions), so the band under-states Poisson collision noise. Under the ORIGINAL rule this
cell makes the measurement verdict INCONCLUSIVE (not KILL); that reading is reported as such.
Follow-up (does not replace the original rule): 30 independent 16-bit curves, B target 16 and 28,
arms A1/A2/I against 10 random symmetric controls per curve; observable = additive coincidences
c_j = Wmax_j - W_j (lost j-sums). Pre-stated reading: fibre arm "sub-generic" only if its mean
c_2 and c_3 exceed the control mean by > 3 standard errors on the pooled 30 curves.

## 2026-11-13 A2 — FFT-free exact instrument for 26 bits
Reason: the prime-length FFT (Bluestein) at r ≈ 3.3e7 raised MemoryError under the 3 GB cap.
`code/widths_big.py` with L5_DIRECT=1 builds jA bitmaps by exact shift-OR (no floating point),
no spectrum. A/A: it is run at 18 and 20 bits and its W1..W3 and pinned widths must equal the FFT
instrument's rows exactly (same curves/seeds), else 26-bit rows are discarded.
