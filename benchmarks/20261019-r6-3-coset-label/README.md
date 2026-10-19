# R6.3: trace label of E/4E and constant-time test costs

Browse copy of the result. Full derivation, certificates, raw samples, and limits: [`research/.../r6_3_coset_label/RESULTS.md`](../../research/isogeny_fibers_20261010/round6/r6_3_coset_label/RESULTS.md).

- The trace pair labels all four cosets of E/4E across 44,740,508 exhaustive points over odd n=5..25 (zero exceptions), plus 10,000 sampled points at each of six larger fields. The trace label is a set bijection, not a group homomorphism. Two polynomial proof cores have accepted identity certificates; the trace identity itself is outside that certificate schema.
- Constant-time `is_in_4e` variants cost 2.5–5.0× the table version in contended wall time and 2.0–3.7× in counted word operations; they miss the predeclared <2× bar and are not proposed as the default. Against the library implementation, best CT wall ratios are 1.23–1.57×.
- Toy curve only; large fields are sampled; timings were contended and no deployed-curve security claim is made.

Figures: `labels.svg` summarizes coset labels; `cost.svg` shows the test cost comparisons.
