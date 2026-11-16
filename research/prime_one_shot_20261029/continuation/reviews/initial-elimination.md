# Independent review: sufficient initial-elimination obligation

2026-10-30 UTC. Read-only mathematical review of `continuation/theory/initial-elimination-obligation.md`. No solver was developed or run.

The sufficiency argument is correct. The document appropriately labels the row-space bound as unproved and distinguishes certificate degree from final printed degree, productive degree and quotient dimension.

Let W_D be the span of every monomial multiple of every input generator whose total degree is at most D. Take D>=2 so that all quadratic input generators are included. Use columns in descending order of the specified degree-compatible monomial order. Suppose W_D contains a complete Gröbner basis G of the input ideal I.

Gaussian row reduction gives a basis H of W_D. For every g in G, its leading monomial is a pivot monomial of this row space: a nonzero linear combination of echelon rows has as its leading monomial one of their pivots. Therefore the leading monomials of H generate the whole initial ideal of I. Since H is contained in I, H is itself a Gröbner basis of I. It need not literally reproduce the particular promised G. This establishes the proposed identification step without a hidden search over subsets of rows.

Provenance and ideal equality matter. Every row must be generated from the original polynomials by the declared monomial multiplications and field row operations. All original generators must occur in the initial row space, so each is also in the linear span of H. Thus <H>=I. Buchberger verification alone would certify only that H is a basis of its own generated ideal; provenance and inclusion close that gap. Transformation matrices or replayable exact row operations retain this evidence with polynomial overhead in the row count.

If v=O(n), D=O(sqrt(n)+log n), and M=binom(v+D,D), then log M=O(sqrt(n)log n). There are at most a polynomial number of input generators times M rows. Generation, echelon reduction and provenance handling therefore use poly(n,M) field work. Coefficients lie in Fp with polynomial-bit representation, so field-bit costs preserve the exponent class.

All echelon polynomials have degree at most D. Their S-polynomials have degree at most2D. Because the order is degree compatible, ordinary polynomial reduction does not increase that degree. Moreover

    binom(v+2D,2D) <= binom(v+D,D)^2=M²,

as every monomial of degree at most2D can be split into two of degree at most D. Thus even full Buchberger verification is bounded by a fixed polynomial in M and n. Failure may return unknown; a verified row-space consequence cannot produce a false refutation. None of this proves that the hypothesized complete basis occurs at the proposed D.

For a positive target of quotient dimension at most L, a certified initial basis permits polynomial-in-L conversion and root extraction. On other inputs, first enumerate standard monomials with a hard stop at L+1 (or reject an evidently positive-dimensional leading ideal), then cap conversion and extraction. This makes the stated phase caps concrete without assuming advance recognition of the favorable target set.

One quantifier should be explicit in any eventual sufficient lemma: to obtain an **overall** exp(O(sqrt(n)log n)) oracle/reduction cost, the computable inverse yield of targets satisfying both the small-quotient and row-space promises must also be exp(O(sqrt(n)log n)). A proved exp(-o(n)) yield alone gives an exp(o(n)) total bound, without that sharper rate. For example, a constant fraction of the reviewed rho0/2 small-degree target set would suffice. An effective constant in D and a uniform, effective yield lower bound on a constant fraction of sampled bases are required by the capped selective-oracle reduction.

With this yield rate stated, the proposed obligation is a valid sufficient route. It is not a proof of small solving degree, an initial-basis construction with the promised bound, or a subexponential ECDLP algorithm.
