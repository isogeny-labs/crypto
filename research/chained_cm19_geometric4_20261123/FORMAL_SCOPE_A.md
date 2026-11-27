# Faithful quantified arithmetic leaf and remaining bridges

Actual target: all geometric group endomorphisms of J descend to F7,
then geometric primitivity and S4 monodromy of f4. The native finite
certificate and the following Lean module cover proper dependencies,
not this whole target. [pcc:unverified]

| Written dependency | Lean declaration | Quantification/hypotheses | Standing before first build |
| --- | --- | --- | --- |
| Frobenius coefficient recurrence modulo7 | ChainedCM19.Geometric4.coeffResidue | all n:Nat, the explicit Int-pair recurrence; first coefficient0 mod7, second in{1,2,4} | source only |
| no zero b_n for positive n | ChainedCM19.Geometric4.coefficientNonzero | all n:Nat, b_(n+1) !=0 | source only |
| actual F^n=a_n+b_nF | none | actual End(J), actual F7 beta and F2+3F+7=0 | readable induction pending full formal bridge |
| hF=Fh implies F7 definition | none | actual finite-extension-defined Hom, torsion-free Hom and natural relative Frobenius | primary facts and written proof pending |
| canonical action/quotient/primitivity | none | actual smooth nonhyperelliptic plane quartic and all geometric automorphisms | written proof pending |
| arithmetic/geometric constants and branch cases | none | actual function fields, regular Galois closure, cyclic tame inertia and RH | written proof plus native case leaf pending |
| rational theta uniqueness and differential obstruction | none | actual Pic2, canonical divisor, J(F7)[2]=0, h0=1, integral L identity | written proof plus native local leaf pending |

Shared crypto-leanb0c73236f22b988feba0a73e58e57e0b5272bbad and Mathlib
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2 remain the generic-library pins.
Prior resource-negative actual AddMonoid/Mathlib module is source-only;
its unchanged ENOMEM is not retried here. No missing geometry definition
is replaced by a custom group with an assumed conclusion. The new leaf
uses existing actual Nat/Int and the standard pinned Lean Omega theorem
producer; its compiled proof and #print axioms, if obtained, are required
before any formal-success claim. No new generic law is duplicated outside
the shared library. [pcc:unverified]
