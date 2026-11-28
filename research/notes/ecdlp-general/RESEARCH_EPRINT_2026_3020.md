# A Deterministic Subexponential-time Algorithm for the Discrete Logarithm Problem

Shuchang Liu, Bingsheng Zhang, and Kui Ren. Cryptology ePrint Archive, paper 2026/3020. The archived manuscript is dated 23 November 2026 and was submitted to ePrint on 24 November 2026. [ePrint record](https://eprint.iacr.org/2026/3020) · [Archived PDF](eprint-2026-3020.pdf) · [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)

## Statement and scope

Assuming the Cauchy–Hermite Boolean tensor contraction construction in OpenAI's Appendix D, Theorem 1.1 gives a deterministic worst-case time bound of $2^{O(b/\sqrt{\log\log b})}$ and space bound of $2^{O(b/(\log b)^{1/4})}$, where $b=\lceil\log_2 p\rceil$. The theorem covers prime-order elliptic curves over $\mathbb{F}_p$ for $p>3$ and the multiplicative group $\mathbb{F}_p^*$, including subgroup membership for an arbitrary base.

Section 3 studies affine-interval factor bases with deterministic coverage. Sections 4 and 5 use exact counting and terminal-preserving tensor contraction to recover and certify relations. The Prime31 summation-polynomial study tracks its own exact empty-target query and cold setup costs. [Prime31 study](../../prime31_summation_compression_20261122/README.md)

## Archived copy

The PDF is the supplied 25-page manuscript, preserved byte for byte. Its SHA-256 is `a3dce4f9939394d74dc1b273408d09c3e47586bb0a80fb0264a0d5a990fc1023`.
