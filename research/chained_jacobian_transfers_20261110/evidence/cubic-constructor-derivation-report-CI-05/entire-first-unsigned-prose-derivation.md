# Constructor arithmetic and Frobenius basis action at223

`lean/CubicConstructorFrobenius22301.lean` supplies exact constructor arithmetic, a no-cube-root certificate over the223 base residues, two characteristic-dependent basis identities and the fixed subspace of the stated diagonal action. Both unchanged original Lean4.35/4.34.1 gates pass13 audited declarations/0 trusted, covering12 new named theorems and one generated definition equation. Whole source/logs and twelve exact original PCC declaration bindings are in `evidence/cubic-constructor-frobenius-final-04`, datacf6114. All claims remain unsigned. [pcc:unverified]

## The original constructor expressions

The complete original common.rs has SHA256832fd28810909194875a9438e164ec1b45dcefbaa83257b4e3fad3095ef865d6. `Kf::new(q)` requires q mod12=7, starts candidate c=2 and stops at the first candidate with powmod(c,(q−1)/3,q) different from1. It stores w from that modular power, w2=w*w modq and the cubic size. These are the expressions being certified; the module does not prove Rust loop, powmod or compiler semantics. [pcc:unverified]

| Expression at q223 | Exact arithmetic certificate |
| --- | --- |
| Guard and exponent | 223 mod12=7; (223−1)/3=74. [pcc:unverified] |
| First candidate2 | 2^74 mod223=1, so the mathematical predicate skips2. [pcc:unverified] |
| Next candidate3 | 3^74 mod223=183≠1, so the predicate accepts3. [pcc:unverified] |
| Stored coefficients | w=183; w² mod223=39. [pcc:unverified] |
| Natural size expression | 223³=11,089,567. [pcc:unverified] |

This two-candidate certificate identifies the first successful mathematical predicate starting at2. Native powmod correspondence and the original mutable function/compiler remain separate. `no_base_cubic_root_three`/345fee also proves every x in Fin223 satisfies x³≠3. It reduces only223 base residues; it does not enumerate223³ coefficient triples. Constructing an extension field and a polynomial irreducibility object remains separate. [pcc:unverified]

## The basis coefficients follow from the stated cubic relation

In any commutative ring R with explicit characteristic223, the module proves3^74=183. If z³=3 is supplied, then

    z^223 = (z³)^74 z = 3^74 z = 183 z,
    (z²)^223 = (z^223)² = 39 z².

Theorems coefficient_power/f43f5f, generator_frobenius/216688 and squared_generator_frobenius/c14ecd prove these identities with their hypotheses explicit. The power-grouping lemma is valid in any semiring. The final coefficient proof uses standard characteristic-aware ring normalization. [pcc:unverified]

The concrete Fin223 coefficients183 and39 are distinct from1 and each cubes to1. Define the coefficient action

    T(a,b,c) = (a,183b,39c).

`basis_frobenius_three`/7c2720 proves T³ is the identity. `fixed_coefficient_subspace`/876dac proves

    T(a,b,c)=(a,b,c) if and only if b=0 and c=0.

This is the exact fixed-subspace statement for the declared coefficient operator. Identifying it with the full q-power map on a constructed extension requires the coefficient embedding, q-power and field bridges. Correspondence with the original UInt64/counter-bearing native function and Jacobian descent is also separate. [pcc:unverified]

## Complete attempts and proof custody

Source SHA256`e81532c9df6d5c416f26693ed7849d70e512d5225558adbab5147266f1e59738` is checked by both unchanged original gates with one compiler worker,120s each and original static proof memory defaults. Only propext, Classical.choice and Quot.sound occur; no sorry, new axiom or trusted native proof is accepted. The original PCC declaration extractor binds all twelve exact statements. [pcc:unverified]

The first two attempts fail in generic cast rewriting; whole sources/stdout/stderr/exits7762a0/b90bcb remain preserved, including erroneous downstream sorryAx outputs. They are unaccepted. Standard characteristic-aware ring normalization proves the same coefficient statement in the final attempt, whose direct compile and both audits pass. Earlier cubic square/nonroot and odd-subgroup components remain unchanged. [pcc:unverified]

The accompanying inspected PDF and coefficient-action diagram record the constructor predicates, explicit basis hypotheses and fixed base axis. Dedicated source-pinned CI retains the actual executing workflow and all partial/full original gate outputs. Actual geometric objects, full field/program correspondence and matched construction/transfer/solver/recovery costs remain separate; no timing or calibrated advantage is claimed. [pcc:unverified]

Sources checked: whole original common.rs powmod/Kf constructor/Frobenius; original CubicNormDescent/WordBridge scope; standard Lean semiring power and characteristic identities/Fin ring instances. Primary review644f82 records the exact sources. KB MCP unavailable, exact/negative/theorem wrappers127; no novelty or absence inference. [pcc:unverified]
