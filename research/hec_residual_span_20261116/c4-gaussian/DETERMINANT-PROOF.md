# Balanced Gaussian determinant lemma: a proof using standard realizations

This note supplies a mathematical proof of the earlier Hdet, with the realization assumptions and convention checks explicit. It has not been independently peer-reviewed or formalized; do not replace retained experimental certificates with a claim of signed verification. No source, run or measurement is changed. [pcc:unverified]

## Statement

Let C/Fp be smooth y²=f(x), f odd of degree9, p odd and p=1 mod4. Fix i in Fp with i²=-1 and sigma(x,y)=(-x,i*y). Let J=Jac(C), and let G(T) be the degree4 characteristic factor of p-power Frobenius on one Gaussian eigenspace, using the weight-one convention (complex roots of absolute value sqrt(p)). Then its constant coefficient d0 satisfies

    d0=u*p², u in {1,-1,i,-i}.

Ordinarity is not required. This is not an assertion that the candidate reconstructed from only trace and N is unique on every input. Uniqueness and scalar-action qualifications remain separate.

## Standard realization inputs

Use the usual compatible Frobenius realizations of an abelian variety: weight-one etale characteristic polynomials agree with crystalline Frobenius, including the decomposition by an endomorphism defined over Fp. One can identify the Gaussian factors by mixed traces of sigma^j*pi^n; this prevents silently comparing geometric inverse Frobenius with the p-power endomorphism. The Dieudonne/crystalline convention here has F=1 on the etale height-one group and F=p on the multiplicative height-one group, as in Weinstein Theorem3.3.2. Thus the determinant valuations below concern roots of absolute value sqrt(p), not their inverses. These are invoked standard comparison facts, not re-proved by an experiment.

The endomorphism sigma²=-1 on J gives O=Z[i] action. The Gaussian characteristic factors lie in Q(i)[T]: mixed traces and the projectors (1±sigma/i)/2 recover their power sums, then Newton identities recover coefficients. Their roots are Frobenius eigenvalues, hence algebraic integers, so their coefficients lie in Z[i]. This argument concerns the rational factors, not an assumed integral splitting at the prime2.

## Ranks and dimensions

On regular differentials omega_j=x^j dx/y, j=0,1,2,3,

    sigma*omega_j = i*(-1)^j*omega_j.

There are two eigenforms of each sign. The induced Lie action is dual to the regular-differential action and still has multiplicities2+2 (swapping signs, if conventions swap, changes nothing).

The canonical principal polarization is sigma-invariant, so its adjoint sends sigma to sigma^-1=-sigma. Therefore the nondegenerate alternating pairing on a weight-one realization pairs opposite Gaussian eigenspaces and vanishes on each individually: if both arguments have eigenvalue i, invariance would multiply the pairing by -1. Consequently both Gaussian eigenspaces have rank4, since the total rank is8.

## Local determinant valuations

Since p splits in O and2 is invertible in Zp, O tensor Zp is Zp times Zp. Its two idempotents split the p-divisible group J[p^infinity] into two groups of height4, each dimension2. The dimension assertion follows from the Lie multiplicities above; the height assertion follows from the compatible rank4 decomposition. This splitting is defined over Fp, so Frobenius preserves each component. Over W(Fp)=Zp the Witt Frobenius is the identity: F on each Dieudonne lattice is an ordinary linear map.

For either lattice M, FV=p implies pM is contained in FM. Thus M/FM is killed by p. By the Dieudonne dimension theorem its Fp-dimension is2. Smith normal form over Zp therefore gives

    v_p(det F on M) = length_Zp(M/FM) = 2.

By compatibility, these are exactly the valuations of d0 at the two primes of Z[i] above p. A possible interchange of the two factors has no effect because both valuations are2. No ordinary slope decomposition was used.

## Global conclusion

Each of the four complex roots of G has absolute value sqrt(p), so |d0|=p² and norm_Q(i)/Q(d0)=p4. Integrality makes its ideal supported only at the two primes over p. The preceding local calculation assigns exponent2 to each. Hence (d0)=(p²) in Z[i]. Their quotient is a Gaussian unit, proving d0=u*p².

This also explains why a balanced signature matters. An unbalanced split action can give unequal determinant valuations, and its quotient by p² need not be integral. For p=3 mod4 the split-idempotent proof does not apply. No genus2 DLP transfer or tenfold total-cost conclusion follows.

## Sources and remaining review boundary

Jared Weinstein, AWS Lecture Notes: Modular curves at infinite level, section3.3, Theorem3.3.2(2),(3),(5), https://math.bu.edu/individuals/jsweinst/AWS/AWSLectureNotes.pdf: contravariant Dieudonne rank/dimension/base-change statements and etale/multiplicative convention examples. PDF retrieved by curl after browser502; source hash retained separately. The elementary Smith-normal-form and Gaussian-ideal arguments above are supplied here. Standard compatible etale/crystalline realization and Weil-integrality inputs are explicitly invoked; this note does not constitute an independently checked treatment of those foundations or the entire Gaussian reconstruction algorithm.

KB query `Gaussian Frobenius determinant balanced signature Dieudonne dimension`, lexical revision9b2119e8fbac482b98b50521c0d604bb, returned unrelated root-lattice proposals, not a determinant result. This does not establish novelty. The lemma is a standard-theory consequence, not a mathematical priority claim. Existing THEORY.md and measured reports preserve their historical conditional statements; read this proof note as a subsequent argument, not a retroactive experimental gate.
