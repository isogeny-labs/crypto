# Guarded unknown-coefficient circuits

The public compiler now represents the original section predicate with five
normalized unknown Fp section coefficients, two forced inverse inputs and
acyclic quadratic gate equations. All15 fixed native arms completed, including
original indexed/group replay for ten toy sections and both original actual-q
cases. This is producer evidence PCC-20261123-7f4e38, unsigned/unverified.

At actual q213156431/a60824497,k20633, the recorded program has4597568
variables and4597589 equations. At k31 it has41493/41514. Seven input wires
does not mean seven total variables; degree2 gate equations do not bound
solving degree. The compiler was not used to discover the coefficients.
[pcc:unverified]

Read [DERIVATION.md](DERIVATION.md), [PROTOCOL.md](PROTOCOL.md),
[PROOF_MAP.md](PROOF_MAP.md), [FROZEN.md](FROZEN.md),
[REPRODUCE.md](REPRODUCE.md), [NEXT.md](NEXT.md) and
[the report](../reports/section-compiler-v1/report.pdf).

| Requirement | Status and evidence |
| --- | --- |
| Public compiler with unknown coefficients | Implemented; Spec,R,Q only, seven explicitly typed input roles |
| Explicit map and nonidentity guards | Implemented; two scalar inverse equations, classical degree5 rational torsion polynomial |
| Extension/projection | Written induction plus fixed native comparisons; no formal compiler theorem |
| Original witness | Replayed in all12 fixed section cases and accepted pencil tuple |
| Complete original program/assignment bytes | Retained locally, full archive117571164bytes/697a904ccfda; trusted-store publication pending |
| Independent or signed validation | Open; producer hashes/replays are not signed execution |
| Unknown-coefficient discovery and growing-family cost | Open; no solver campaign was run |

The full raw stream218010bytes/e9d3ccf6a00b and all24 exported program/witness
files are in the one private native archive described by native-v1/ARCHIVE.json.
SUMMARY.json is a labelled producer metadata/replay projection, not a substitute
for the raw bundle or an upload receipt. See PUBLISHING.md. [pcc:unverified]

Earlier source, reports, claims, ROUTE_CRITERIA.md/V3 and consumer98b6 source
pin retain their original bytes. No canonical performance row was added.
