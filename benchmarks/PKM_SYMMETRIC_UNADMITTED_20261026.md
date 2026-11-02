# Symmetric PKM tower, F4 stage: retained benchmark record (unadmitted)

AGENTS.md ("Performance results also land in `benchmarks/`") requires a browsing copy of
this solver-stage timing. [`20261026-pkm-symmetric-tower/`](20261026-pkm-symmetric-tower/)
holds it: the report PDF, the graph as SVG, and a 150 dpi PNG from the same `plot.py`.
The study directory is the source of truth. The row below is **not** in the admitted table
of [`README.md`](README.md). It has no replay certificate, validator report or matched rho
reference (no `VALIDATION.json`), and a solver-stage diagnostic has no rho to match. So
under the 2026-10-19 validation gate it stays here until it validates.

| date | entry | measured | headline (as reported) | host | study |
|:--|:--|:--|:--|:--|:--|
| 2026-10-26 | [`20261026-pkm-symmetric-tower`](20261026-pkm-symmetric-tower/) | F4 stage (`f4_fp`, grevlex, p = 786433, two random targets) of the plain S₃-chain Kummer tower against the symmetric tower (Kummer membership and S₅ in e₁..e₄), m = 4, N = 12 | 1.10·10¹¹ → 4.01·10⁵ counted field operations (2.7·10⁵×), 21,619 → 3,663 columns (5.9×), 66–70 s → 8–9 ms per system (7,600–8,900×, paired per target). D 7 → 8 (the symmetric input already has degree 8), last productive degree 6 → 5. Solver-stage diagnostic: no pipeline priced | cloud container, 4 vCPU, L0 (shared VM, nothing else running during the timed rerun) | [study](../research/pkm_symmetric_lead1_20261026/README.md), [report](20261026-pkm-symmetric-tower/report.pdf) |
