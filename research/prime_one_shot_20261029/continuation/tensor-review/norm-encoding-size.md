# Independent review: size of the direct norm-to-tensor translation

2026-10-30 UTC. Narrow representation/cost review only. No code, solver construction or experiment.

The proposed clarification is correct for the literal translation of the retained coefficient/remainder system. It explains why the contraction theorem alone does not establish a subexponential-in-field-bitlength solver for that system. It is not a lower bound against another encoding or algorithm.

Let b=ceil(log2 p), with b=log2 r+O(1) in the whole-prime-order setting. In the balanced parameter regime,

    t=log2 d=b/m+O(log m)=Theta(b/m),
    mt=Theta(b).

The retained shared-remainder construction exposes Theta(mt) field coefficient/state words. Straightforward dense coefficient products and congruences use O(tm²) scalar field additions/multiplications: each length-m convolution has O(m²) monomial operations, repeated over O(t) stages. Intermediate arithmetic words and fan-out copies must also be charged.

Bochum v0.2 Lemma8 and Corollary1 (p11) supply O(b)-edge Boolean tensor encodings per canonical scalar modular-arithmetic constraint, including word interfaces and intermediate words. Applying that composition result directly gives

    S=O(tm²b)=O(mb²)

Boolean edges. This is the size of this direct translation, not a claimed optimum. Even its explicit Theta(mt) canonical b-bit state-word interfaces already contain Theta(mtb)=Theta(b²) Boolean ports in that representation. That observation cannot be promoted to a lower bound for arbitrary tensor descriptions or symbolic representations.

Substituting the direct size bound into Theorem D.1 gives at best the displayed guarantee

    poly(S+L_in) 2^O(mb²/sqrt(log(mb²))),

where the gadget description length is polynomial. This expression does not establish 2^o(b) time. At balanced m=Theta(sqrt(b)), its stated exponent scale is b^(5/2)/sqrt(log b). The explicit-state interface count also prevents this particular uncompressed word-by-word representation from having only O(mb) edges when m=Theta(sqrt(b)).

These are upper-bound/representation statements. An upper bound of that form is not a proof that actual execution takes that long, a worst-case lower bound, or evidence that every instance is difficult. More compact gadgets for the whole polynomial structure, elimination of states, another symbolic representation or the separately proposed Macaulay lemma could change the analysis; none follows automatically from D.1. Small quotient degree alone also does not remove these Boolean state words from the direct encoding.

The contrast with the reviewed external hash-membership construction is concrete. Its point-validity, point-sum and per-slot hash-membership checks use only O(1) scalar word constraints per point, with O(b)-bit words. Lemma11 (p15) therefore obtains O(mb) edges. That compactness is an additional structural property of its encoding; it cannot be transferred to a length-m coefficient convolution repeated over t stages merely because both use the same scalar arithmetic gadget.

Accordingly D.1 is materially relevant external mathematics, but it does not by itself close the shared-remainder norm solver obligation. This clarification supplies no new variant or novelty claim.
