# Errata to REPORT.md (after the validator's review)

The validator's review (`VALIDATION_REVIEW.md`, at commit 7fc37ffed7, disposition `validated`) found
reporting gaps; REPORT.md is corrected in the commit that adds this file. The measurement, the tables
of S, the band files and the replication are unchanged.

- **F1.** "1.8x to 2.8x … across the §5a band" was the 1/16 reading at w = 0. Now stated per weight:
  1.81–2.78x (w = 0), 1.73–2.77x (w = 0.1), 1.53–2.74x (w = 1) at 1/16; 1.40–1.75x at 1/2, w = 0. What
  holds across the band is the order.
- **F2.** Rows at the stop: 0.905–0.937 a per record (0.918–0.937 a was the range of the per-cell maxima).
- **F3.** The w = 1 dilution towards about 0.67 held only at 37.4 bits, |F_x| = 4096; inversions rose
  4.7–5.6x at 1/16, more than the pre-registered "about 4x" (previously called "as pre-registered").
- **F4.** Added the §5a list of unpriced counters with counts per arm, rho included, and that S is a
  lower bound on every arm.
- **F5.** Added the §3 class (engineering) and the §3a cost chain.
- **F6.** REPORT.pdf re-rendered from the corrected REPORT.md.
