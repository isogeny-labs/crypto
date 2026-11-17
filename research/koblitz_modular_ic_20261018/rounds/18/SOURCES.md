# Primary sources and their exact use

[Damien Robert, The arithmetic of theta groups and biextensions of abelian
varieties, Theorem1.1 proof](https://www.normalesup.org/~robert/pro/publications/notes/2023-08-thetagroup.pdf)
was read23 October2026. A follow-up fetch seeking the detailed symmetric-lift section timed out; no unread passage from that fetch is used. It provides the symmetric-lift framework. Our
characteristic-2 discrepancy formula, actual character values and cover
matrix obstruction are separate calculations in this round.

[Ben Moonen, The theta group of a line bundle,8.10–8.15](https://www.math.ru.nl/~bmoonen/BookAV/ThetaGr.pdf)
provides the correspondence between line-bundle descent and a theta-group
kernel lift. THETA.md exposes the symmetric class identification and does
not claim these scheme arguments are formalized in Lean.

The actual level77 CM type and modular interpretation are frozen Round15
CM.md and Round16 CM.md. The integral Tate existence argument and its
primary source remain Round17 CM.md/SOURCES.md. This round computes actual
deck formulas, conductor residues and map candidates; it does not replace
that existence argument with an evaluated factor inclusion. The live LMFDB
77.2.l.a endpoint was unavailable through the web tool; we do not infer new
database facts from it. The retained earlier database row remains frozen.

The arithmetic inputs are Round17's frozen q8 circuits, Q24 models, G16
cover, G17 kernel, G18 factored functions and CM17 incidence. Q25/Q27 reuse
the earlier source-cone semantics and add explicit universal identities.
All source-code and proof bindings are hashed in the native ledger. Search
exhaustion, full field semantics, function divisors and proper modular
normalization remain outside the Lean formalization unless individually
listed in PROOF_MAP.md.
