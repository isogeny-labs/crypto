# Prior exponent studies (addendum to the frozen protocol)

Written on 2026-11-10, after the coordinator asked for it. The protocol `PROTOCOL.md` was frozen on
2026-11-08 (sha256 in `data/frozen.sha256`), and most grid cells were measured before this file was
written. This file is therefore not part of the preregistration. It does not change the grid, the
estimator or the thresholds. The frozen file is left unchanged so that its hash still holds.

| directory | curve family and factor base | exponent found | covers m = 3 or 4 on a prime-field subgroup base? |
|:--|:--|:--|:--|
| `research/ic_exponent_20260926/` | binary Koblitz curves over F_2^n, n up to 61 (r from 2^18 to 2^47.2); the Koblitz collection thread's m = 3 workflow | ratio to batch rho, top-end fit beta = 0.137 [-0.10, 0.37] in r; not the cost of one decomposition system against the base size | no: binary field, no multiplicative-subgroup base |
| `research/ic_m4_exponent_audit_20260928/` | binary Koblitz curves F_2^9 to F_2^19, F_2-subspace base, chained m = 4 Semaev system | c = 0.985 bits per unit n [0.983, 1.018] for the word-XOR cost of the m = 4 decomposition; "closed for this engine at these sizes" | m = 4 yes, but on a binary-field subspace base, not a prime-field subgroup base |
| `research/ic_exponent_top_20260930/` | binary Koblitz curves, r at least 2^36 | none: withdrawn before anything ran (re-declared as `ic_single_target_20260930`) | no |
| `research/ic_exponent_falsifiers_20261013/` | prime curve p = 946177, PKM 2-power Kummer tower t = 8 | not a cost exponent: the exponent sum of a decomposition over mu_{2^t} is indistinguishable from random (test C4) | no: it studies the exponent of the base abscissae, not the cost of the system; m = 3, enumeration |
| `research/ecdlp_exponent_assessment_20261018/` | only an artifact manifest is in Git (`REPORT.pdf`, `LEADS.pdf`, two PNGs, in the artifact store); the PDFs were not retrievable on this machine | not readable here | unknown; not checked |

None of the four readable studies measures the cost T(E, m, L) of one decomposition system against
the size of a prime-field multiplicative-subgroup base, for any m. The closest is the m = 4 audit,
whose base is an F_2-subspace of a binary field. The question of this study stays open in full,
and the grid was not narrowed.
