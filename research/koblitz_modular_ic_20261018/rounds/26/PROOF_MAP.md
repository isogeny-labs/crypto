# R26 proof fidelity

Pinned Lean4.34.1/mathlibd13f23b. Source
lean/KoblitzPrym26.lean SHA256
962c2be137636fa886575f29581af0a826c7e822eb7a2c422384347e295786b7.
Lean02 compiles16 named theorems. AuditAxioms enumerates17 declarations,
including generated diamondNorm.eq_1:0 failing,0 trusted-native. Only standard
propext/Classical.choice/Quot.sound foundations occur. Finite arithmetic uses
kernel decide, not native_decide. Lean01 is rejected and retained.

| Exact scope | Named proofs / evidence | Remaining bridge |
| --- | --- | --- |
| CRT and primitive diamond signs | crt_delta,diamond_two_fifth,shared_rational_diamond_fifth,primitive_sigma | R15/R16 modular identification/specialization written |
| All nontrivial diamond norms mod11 and trivial controls | nontrivial_diamond_norm,trivial_diamond_norm; CHAR26-02 | Not the full unknown Jacobian lattice |
| Minus norm zero,difference double,6 recovers,nonzero detection | norm_zero_of_minus,difference_double,six_recovers_difference,difference_detects_nonzero | Actual target inclusion/evaluation not constructed |
| Difference lies in minus/norm kernel,Frobenius compatibility | difference_is_minus,difference_norm_zero,difference_equivariant | Conditional actual action/commutation as stated |
| General quadratic function norm,normalized AS/trace | quadratic_function_norm,normalized_trace_one,normalized_AS_relation | Source substitution from R25,not arbitrary-divisor reverse |
| T universal/source/base identities,T^5=sigma | Native TRANSPORT26-01 and SOURCEORBIT26-01 | QR moduli preservation and proper normal-curve extension written |
| Transported proper cusp charts,complete lifted base-entry divisors | R24 charts,R23 Z5,R22 base divisors and exact T inverse | DVR/divisor pullback interpretation written |
| Prescribed RR degree128/dimension68 | R15 genus61 and RR deduction | Full basis still not constructed;no Lean RR axiom |

The group algebra is over integral groups with11-torsion; it does not
identify minus functions in characteristic2. No full divisor,curve,modular
factor,polarization or complexity theorem is claimed formalized.
