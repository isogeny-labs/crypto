#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= Historical genus-3 cold-target audit
*PCC standing: unverified. Historical Python observations; no admitted performance claim.*

The frozen record contains 64 separately initialized one-target trials on the restricted cubic-extension cover family. The reported mean cold wall times are 0.0566323933 seconds for the cover implementation and 0.0787853895 seconds for negation-aware BSGS. The recorded ratio of means is 1.3911718161; the producer's bootstrap interval is [1.3019612161, 1.4815007697]. All these are preserved producer observations [pcc:unverified].

#image("recorded-wall.svg", width: 100%)

== Evidence and local checks
Source: historical/cold_target_audit_q1061_v65.json and historical/source.tar. The archive preserves the five measured Python files and their pre-run hashes. Native Rust checks replay the recorded scalars on the original elliptic curve, the subgroup and Hasse witness, source hashes, means, medians, ratio and win count. The checker is local and unsigned; it is not an independent PCC verdict or execution proof.

== What was cold
The cover intervals include construction, factor-base enumeration, fresh relations, exact matrix solving, target descent and answer checking for each target. BSGS builds a fresh table for each target. Known curve/order/generator inputs, runtime imports and common target generation are excluded for both. No DLP precomputation is shared. The record is not a fresh-process startup benchmark.

== Fixture and prior art
The recorded field is F_1061[alpha]/(alpha^3+alpha+65), with E: Y^2=X(X-alpha)(X-alpha^1061). The cover over F_1061 is y^2=(t^3+t+65)(t^4-2t^2-520t+1). The recorded subgroup order is 298607311. Formal repository identity registration remains pending.

The construction is prior art: Joux--Vitse, Cover and Decomposition Index Calculus, Appendix A.1, equations (6)--(8). Internal context: research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md, sections 9.6, 11.1 and 11.7; research/cubic_extension_transfer_20261025/README.md.

== Unresolved admission
Counted cold costs, matched strong rho, native solver timings, calibrated bands, qualified isolation/A/A, an external reference and independent signed replay are missing. Original bootstrap samples and physical timing provenance are not replayed by the checker. No accepted dashboard, benchmark-index, registry or leaderboard row is promoted. The study README and admission problem define the next native implementation and measurement obligations.
