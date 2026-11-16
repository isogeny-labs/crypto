#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E38: a second pre-registered confirmation of the pivot rules F, S and H. It stopped at the pilot.

*Run report, 2026-11-02.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: §9.46 (pre-registration, commit `c20e4fa066`, before any code) and §9.47 (results). Code: `149dac81b9` (probe records of the pivot set, border targets and updates; controls `bump_update` = C_F^mid and `bump_pivot` = its fallback, in `src/cryptanalysis/chain_pivot_rules.rs`; probe hooks in `klein_v4::chain_quotient`) and `da3e7e5363` (examples `e38_checks`, `e38_stage_a`, `e38_tables`; `run_e2e.sh`, `identity.sh`). The binaries were built once from a clean `git archive` export of `da3e7e5363`, checked blob by blob (4,675 files, 0 mismatches), and pinned by sha256 (`experiments/45_pivot_rules_confirm2/binary_sha256.txt`, `binary_commit.txt`). Records are in `experiments/45_pivot_rules_confirm2/`. Host: the 4-vCPU Claude Code container. isolab-eks is unreachable from it and `conductor` is not installed. *Operation counts only, no wall-time claim.*

*This is a confirmation of a hypothesis that E36 and E37 generated.* Both measured F (each bucket's first-arrived row), S and H ahead of A and E25 at 0, 0.1 and 1 GAE on every row they ran (E37: all 32 rows). In both, a registered control failed its positive check, so the kill criterion voided the rank. E38 changed only the F controls:
- C_F (E37's misfiled arrival) is now judged by the extended check `P_ext`, which also accepts a changed pivot set or border-target set.
- C_F^mid is a new control that keeps the structure intact by construction and corrupts one read pivot row. It is judged by `P_mid`.

== Outcome
- *The registered kill fired at G2.176, the pilot.* C_F^mid could not be aimed on any of the 80 pilot calls: its target set $U$ is empty. $U$ is the set of pivot columns that the left-looking normal forms read and whose bucket held one row; on every call, every column the normal forms read was contested. The registered fallback C_F^mid-b draws from the same set $U$, and it was also never aimed (0/80 on stream P2). §9.46 says a failed fallback stops E38. *No rank is claimed.* As registered, the main control run, Stage A and the end-to-end runs were *not* taken.
- The defect is in the C_F^mid design, not in the rules. The registration predicted it would be aimed on at least 95 % of calls; it was aimed on none.
- *The F-control repair (i) worked on both pilots.* E37's misfile, judged by `P_ext`, passed its positive check on 80/80 (P1) and 80/80 (P2) calls; in E37 it passed 0/320 under the strict check. On every call the pivot set and the border targets both changed, and the strict check never held. It was detected on 56/57 and 58/58 of the non-empty calls.
- C_SH (S, H) and the calibration C_0 passed both pilots: positive 80/80, and non-empty detection 0.982–1.000.
- Identity (G2.177, the binary clauses): 19/19 existing configurations are byte-identical under the E38 binaries. These are:
  - E37's eleven identity configurations;
  - E37's stored end-to-end records at $p = 101$ seed 2, salt `0xE37E0001`, for GLS A, F, S, H and B″ and for E24 F;
  - E37's `checks.json`, rerun from E37's unchanged `e37_checks`;
  - E37's `walk_e24.json`, rerun from E37's unchanged `e37_walk_e24`.

#figure(image("figures/controls.svg", width: 100%), caption: [Pilot controls (80 calls each on streams P1 `0xE38D0001` and P2 `0xE38D0101`; 57 and 58 calls with a non-empty clean set). The bars show the registered positive check, detection over all calls and detection over non-empty calls. C_F^mid (P1) and its fallback (P2) were never aimed: $U = emptyset$ on every call. Source: `experiments/45_pivot_rules_confirm2/pilot.json`, `pilot_p2.json` via `summary.json`.])

== Pilot results (registered data)
#table(columns: 7, align: (left, left, right, right, right, right, left),
  [control], [check], [P1 positive], [P1 non-empty det.], [P2 positive], [P2 non-empty det.], [pilot],
  [C_F (i)], [`P_ext`], [80/80], [56/57], [80/80], [58/58], [pass],
  [C_F^mid (ii) / -b], [`P_mid`], [0/80 (unaimed)], [0/57], [0/80 (unaimed)], [0/58], [*fail*],
  [C_SH (S)], [strict], [80/80], [57/57], [80/80], [58/58], [pass],
  [C_SH (H)], [strict], [80/80], [56/57], [80/80], [58/58], [pass],
  [C_0 F/S/H/L (cal.)], [strict], [80/80], [56/57 each], [80/80], [57–58/58], [—],
)
Kept checks on both pilots: E31's perturbed coefficient changed every non-empty set under F, S, H and L; the F and L digests differ from A's on 80/80 calls; no clean-run identity failure.

== Verdicts
#table(columns: 3, align: (left, left, left),
  [id], [result], [verdict],
  [G2.176], [C_F^mid 0/80 (never aimed), fallback 0/80], [*falsified: the kill fired; E38 stops*],
  [G2.177], [19/19 configurations byte-identical; Stage A and end-to-end clauses not run], [holds where run],
  [G2.178–G2.189], [not run (stop rule)], [not tested],
)

== Why $U$ is empty, and what to register next
The normal forms start at the border targets, which are low-degree monomials late in the column order, and they read only columns at or to the right of their target. In F's echelon those late buckets always hold more than one row: about 334 of the 384 pivot columns are contested, and the uncontested ones lie early in the order, where no normal form reaches. The mechanism is inferred from the pilot records and E36/E37's counts. No column-by-column trace was taken.

A control that keeps the structure provably intact can instead act on a *contested* read column $l$ *after* its bucket's updates. The pivot row of $l$ is then never used again by the echelon, so changing its last-column entry leaves every other row, the pivot set and the border targets exactly as they were, and it alters a row that the normal forms read. A next registration (E39) could use that control together with C_F under `P_ext`, which has now passed on 160/160 pilot calls.

== Distance to rho and limitations
No new measurement of F, S or H was taken, so E38 adds no band and no distance to rho. The last measured values are E37's descriptive ones (§9.45). On the closest GLS row ($p = 397$, seed 1, arm B), F is 3.93·10³× the matched rho at 0 GAE and 8.40·10⁴× at 1 GAE. Rho is ahead of every row at every weight.

Limitations:
- Pilot sizes are 80 calls.
- The emptiness of $U$ is established on 160 calls of E25's ten curves only.
- The main-session commit `699c05df9a` (rustfmt and clippy, no behaviour change) landed after the build SHA and is not in the binaries.
- Building required moving some of this run's own dependency artifacts from tmpfs to disk to stay under the memory cgroup limit. This changed only where files were stored, not the build inputs or flags.
