# RR9 D9 positive-kernel extraction plan

1. Freeze the five-arm protocol and source boundary. **Done in this commit.**
2. Add a native Rust candidate that reuses the existing sparse Macaulay and
   BM code, derives `P(A)/A v`, and directly checks `M v = 0`.
3. Add rank-one monomial consistency and a source-separated verifier for the
   seven recovered coefficients and original-curve eight-point sum.
4. Run unit/toy controls locally without citing timings.
5. Commit and push the exact candidate, verifier and IsoLab request; pass the
   checkpoint gate; run all frozen arms on EKS Spot.
6. Preserve raw rows and hashes, produce a descriptive report, diagram and
   PDF, and state the next branch from P2--P4 without promoting a component to
   an ECDLP result.

The cost chain is intentionally incomplete at this stage:

```text
C(existing B8 algebraic gate) = negative decision only; positive extraction unset
                              -> add verified sparse-kernel extraction
C(B8 component candidate)     = decision + extraction + exact relation replay
```

There is no incumbent-to-candidate performance-gain line until both arms compute the
same positive extraction. The generic 4+4 oracle remains a correctness and
cost reference, not an algebraic baseline silently replaced by a decision
test.
