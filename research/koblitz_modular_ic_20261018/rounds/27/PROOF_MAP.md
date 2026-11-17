# R27 proof and computation boundaries

Lean4.34.1, mathlibd13f23b723b8a846827a245b89c10fc7d3f11612.
`lean09` rebuilds both modules from frozen sources and audits35namedtheorems,
0failing,0trustednative. Earlier successful `lean03` audited18Collectorclaims.
Retain ALLfailedcompile/runtime/gate attempts; see FAILURES.md/raw manifests.

| Claim | Evidence | Formalization boundary |
|---|---|---|
| Fixed function/divisor differences, norm-one ratios and Hilbert90 | Collector27 kernel proofs; NORM01 actual substitutions | Universal field/group identities, actual substitutions native |
| Free involution and genus61/31/Prym30 | 20proper-chart transports,NORM01,ramification table; Collector numeric RH consequences | Proper curves,torsors,RH and divisors written, not Lean scheme proofs |
| All4original and8registered divisors | LAURENT02,LAMBDA14,SS5/Z5R22-R26 and degree balances | Actual finite identities native; complete proper-DVR bridge written |
| Complete68RR/34invariant sections | RR-O01,RR-export01,ACTION01 native exact rank/recombinations | RR completeness/jet injection uses written degree theorem |
| Actual source class exactorder11 | ACTION01 L(D')=0 with complete68basis; actual principal exponents | Torsion27 proves integral11/T6/moving4/recovery3 identities; nonprincipality native+RR written |
| Closed sourceP67/quotientP33 embeddings and prime saturated ideals | QUADRICS03 everyquadratic kernel/pivot; Prop11 finite-field theorem | Lean numericdegree/count lemmas only; veryampleness/quadratic generation external theorem, not formal schemes |
| MixedPrym24/type(1^18,5^6) overF2 | POLARIZATION.md; LOWGENUS01 E2=10E/residualrecovery | Conditional additive coprime2/5 and numeric counts Lean; proper Picard/polarization theory written |
| Actual cusp reverse evaluator | REVERSE02 full integral5x5witness on formal arbitrarycoefficients | Native exact matrix products plus actualorder11bridge; not arbitraryordinary classes |
| Node187 proper local separator | NODE01 80actualfunctions/conjugates; exact group-law lift identities | Not an original-ring h-power membership proof; no fullsaturation claim |
| Genus7/13 particular quotient exclusions | LOWGENUS01 irreducible/parity; LMFDB +retained modularbridge | Conditional modular decomposition/good reduction argument written |
| F32map search | MAPBUILD02 full455equations, prescribedRRspace; MAPCHECK controls/solver records | No globalnonexistence from caps/nonunit; anymapstillneeds11image anddescent |

Neither module assumes a curve inclusion, a nonzero CM projection, a saturation
claim or a timing result as an axiom. Onlypropext,Classical.choice,Quot.sound
occur in the transitiveaudit. The actual remaining mathematical obligations
are explicit; a numericdegree lemma is not presented as a proper-geometryproof.
