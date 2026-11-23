# Scaling checks for the prime-field follow-ups

## Uniform residual coverage

Let the subgroup have prime order $r$. Freeze the base and labels before a uniform residual $R$ is drawn. For a base of size $B$ and $m$ distinct points per signed representation, the number of residuals that can have an informative representation is at most $2^m\binom{B}{m}$. With ordered disjoint slots of sizes $B_i$, it is at most $2^m\prod_i B_i$. These are upper bounds even when sign choices collide or the recovery denominator vanishes.

Consequently, fixed $m$ with a base of size $r^{o(1)}$ gives single-draw success probability at most $r^{-1+o(1)}$ in this model. For independent uniform residuals, expected attempts are at least $r^{1-o(1)}$. A union bound also bounds success of a fixed number of draws when each draw is uniform conditional on its past and the prepared base. The argument charges only attempts and leaves any setup cost out. It does not cover conditioned base construction paired with adaptive, nonuniform residual selection.

For explicit split support with left and right sets $L$ and $U$, at most $|L||U|$ group elements are sums. To obtain coverage $c>0$ requires $|L||U|\ge cr$. The larger support has at least $\sqrt{cr}$ elements. A representation storing one entry per distinct abscissa can combine at most two odd-order group elements per finite abscissa, so the explicit support stage still has order $\sqrt r$ group-element scale for constant coverage. Sharing the table across many targets changes amortization. These inequalities leave implicit support counting open.

## Comparison with the tensor-oracle lead

[PR #4668](https://github.com/isogeny-labs/crypto/pull/4668) states an exact tensor-counting bound and applies it to the growing-arity group reduction in [PR #4665](https://github.com/isogeny-labs/crypto/pull/4665). That route addresses a different scaling regime. Its degree condition, recurrence, and conversion from relation probability to useful rank and total work require the checks in those proof PRs. The fixed-arity experiments here provide no independent end-to-end validation of them.

There is a practical distinction within #4668's written schedule. It picks the initial clique size $K_0$ as a power of two with $\log_2 K_0$ divisible by eight, and applies its root construction when $K_i^{64}\le s$ for the Boolean edge count (s). Even the smallest positive choice $K_0=2^8$ sets that applicability threshold at $s\ge2^{512}$. The proof calls for a sufficiently large initial size, so this number is a lower bound for the written schedule, with no assertion that it is a lower bound for other constructions. A practical oracle experiment needs a separately costed small-size grouping schedule or another implementation. Component measurements on prime31 cannot establish a crossover for this proof schedule. [pcc:unverified]

Primary sources for the tensor lead are the [Anthropic construction](https://github.com/anthropics/formal-math/blob/16a5ce41304fcdaea4e0be7c132e1c361dc26da5/eth/README.md), the [OpenAI exact-counting preprint](https://github.com/openai/math/tree/main/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026), and the [Bochum prime-field reduction](https://github.com/bochum-crypto/subexponential-ecdl/blob/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f/subexponential-ecdl.pdf).
