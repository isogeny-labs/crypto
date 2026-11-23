#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Component construction of anchor-supported relations
16 November 2026 · hec_fibre_pencil_20261116 / anchor gate

== Exact construction and bounded checks
The signed fibre matrix is grounded incidence. Build its off-anchor graph and sum rows on each ungrounded component. These sums have zero off-anchor coefficients. Combine them using component sizes to enforce zero total fibre coefficient. The resulting rows span the entire anchor-supported difference intersection. Its dimension is (k−delta_O)−(c−delta), where k/c count off/full ungrounded components and delta_O/delta are ranks of their size functionals over the coefficient field. This is classical rank-nullity, with no novelty or speed claim.

All179424exhaustive synthetic configurations pass: n=1..4 vertices, every edge/ground column absent/off-anchor/anchor, primes3/5/1000003. There are11742configurations with full closed components and5432with a characteristic-divisible off-component size. Each output passes independent full/off Gaussian rank difference, off-column zero and containment in the original row space. Categories overlap; these are finite controls, not statistical samples.

The curve gate passes690policies on69eligible divisors across genera2,3,4, preserving three excluded inputs. All original fibre/group/point-add checks remain; original stdout is byte-identical to CONTROL.tsv. Ten policies per eligible divisor use five H values and two catalogue orders. Source5e05b98b844e, pre-run checkpoint20261116T064255Z; source/import/binary freeze passes, both once-only binaries exit0. Curve stderr is intentional diagnostics; synthetic stderr is empty.

== Anchor survival in the recorded panel
#image("../../ANCHOR-COUNTS.svg",width:100%)
The figure shows canonical-order homogeneous rank availability. All H=8 policies have zero rank. Rotated genus2/H16 retains positive rank in3of24pencils; its canonical order retains none. Full-catalogue rank is recovered where at least two fibres exist. All policies and zero cases remain in ANCHOR-SUMMARY.tsv. Exact deterministic counts; no timing or uncertainty estimate. Rank sums are not pooled multi-pencil rank.

#pagebreak()
== A distinct affine consequence
Homogeneous relations cancel the supplied-divisor column. A decomposition may retain it. Apply an odd-prime cyclic projection pi to the sum over an ungrounded off-component C, killing the omitted branch-place 2-torsion. Then

$|C| pi([D]) + sum_P w_C(P) pi([P-infinity]) = 0.$

Under a valid cyclic prime projection, a component with invertible size therefore gives an anchor decomposition of D. Zero homogeneous rank does not rule out that possibility. This affine interpretation was derived after the homogeneous gate from its retained component diagnostics; it is not a pre-registered scalar-recovery endpoint.

#table(columns:5,inset:4pt,
[g / field],[Eligible],[H],[Canonical rank / component],[Rotated rank / component],
[2 / F101],[24],[8],[0 / 0],[0 / 0],
[2 / F101],[24],[16],[0 / 6],[3 / 8],
[2 / F101],[24],[59(all)],[24 / 24],[24 / 24],
[3 / F61],[23],[8],[0 / 0],[0 / 0],
[3 / F61],[23],[16],[0 / 3],[0 / 2],
[3 / F61],[23],[35(all)],[20 / 23],[20 / 23],
[4 / F31],[22],[8],[0 / 0],[0 / 0],
[4 / F31],[22],[16],[0 / 3],[0 / 3],
[4 / F31],[22],[18(all)],[0 / 5],[0 / 5])

Each cell counts eligible pencils with positive homogeneous rank / available ungrounded off-component. Algebra-control prime1000003 exceeds component sizes, but is not the group order. The component equation is not an actual subgroup scalar solve. All H=0 and H=g+1 controls have neither rank nor available component.

== Cost decision and next required construction
The construction avoids dense full-catalogue elimination, but still pays catalogue setup, scan/evaluations/inverses, fibre generation and component assembly. H8 supplies no useful output in this panel; increasing H yields some affine decompositions with almost no homogeneous rank. These observations do not establish a complete tenfold premise against Compact. No timing campaign follows from this gate.

A useful successor must integrate affine component descent with the actual subgroup projection and target-seeded anchors, charge failed pencils and source/target work, and compare against existing random point-add descent and Compact. The checked component law is a building block; the mathematical tenfold objective remains open.

== Evidence boundary and custody
Native scientific computation only. ANCHOR-SYNTHETIC.tsv, ANCHOR-CURVE-DIAGNOSTICS.tsv and original-stdout replay remain. ANCHOR-PROTOCOL.md and ANCHOR-FREEZE.sha256 identify the pre-run contract. Post-run anchor_reduce.rs summarizes the retained counters and renders this graph; no parameter or curve selection changes. Both PDF pages are visually checked.

These bounded controls share native field/group code. No independent scientific validation, machine-checked full group certificate, benchmark admission or performance claim. Existing canonical timing graphs remain unchanged. The earlier literature/KB receipt and inaccessible primary-PDF gap remain in THEORY.md/SESSION.md; classical identities are derived explicitly, without priority claims. PR4357 remains unmerged.
