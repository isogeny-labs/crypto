# fes-f2 vs inherited-f4 at degree 13, Runpod (2026-11-15)

Study: [`research/fes_k13_validated_20261112/`](../../research/fes_k13_validated_20261112/RESULTS.md)
(frozen protocol, session `ECBS1h695ef8e76c34`, independent validation). Report
`REPORT.pdf`; figures `fig1-mean-s-per-arm.svg/.png`, `fig2-band.svg/.png`. The study
directory is the source of truth; these are copies. `session/`, `validation/` and
`VALIDATION.json` are byte copies of the evidence the gate reads.

On `icv1-f2m13-t181-515ee569` (r = 2003), 48 measured runs per arm on a Runpod cpu5m pod
(AMD EPYC 9575F container, shared host, no L2 gate): paired S ratio fes-f2 / inherited-f4
0.7081 [0.6683, 0.7503]; band S at w = 0 / 0.1 / 1 for fes-f2 15.177 / 3081.763 / 30681.037,
inherited-f4 15.174 / 4621.699 / 46080.422, matched rho-sfstrong-v2 2.584 / 2.677 / 3.513.
H1 is not supported: the order flips at w = 0, so the comparison is unresolved inside the
band. H2 is supported. Distance remaining, fes-f2 / rho-sfstrong-v2: 33.30. No wall figure
is usable (A/A wall control failed, no L2 gate). Finding F5, the binary-to-source link,
was closed on 2026-11-15 by an independent rebuild and a 288/288 identical replay. `[pcc:unverified]`
