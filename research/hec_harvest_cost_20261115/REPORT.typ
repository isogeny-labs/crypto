#set page(paper:"a4",margin:15mm)
#set text(font:"Helvetica",size:9.5pt,fill:rgb("233746"))
#set par(leading:.6em)
#text(size:21pt,weight:"bold")[What a 10× harvesting cost case requires]

Derived conditional model · genus 2–4 · 15 November 2026

*Result.* Shrinking an algebra core is insufficient without pricing the enlarged relation
search. The following threshold jointly optimizes the initial factor base and algebra.
No measured gain, attainable retention law or double-large-prime model is established.

$ T(B) = A k^sigma B^(-alpha) + D W rho^beta B^beta $

Here k is relations per new base variable, ρ is the retained fraction of that new base,
W is a relative algebra/row-weight penalty, and all exponents and constants are positive.
Setup, filtering, memory, descent and verification remain omitted and must be charged later.

$ frac(T^*, T_0) = k^(frac(sigma beta,alpha+beta))
 W^(frac(alpha,alpha+beta)) rho^(frac(alpha beta,alpha+beta)) $

For a model gain G, the required retention threshold is

$ rho <= G^(-frac(alpha+beta,alpha beta)) k^(-sigma/alpha) W^(-1/beta). $

At equality, initial base growth is $(G k^sigma)^(1/alpha)$; retained core relative
to the original optimum is $(G W)^(-1/beta)$. The relation pool grows by k times
the initial-base growth. This comparison is to the model's continuous interior optimum.

#image("THRESHOLDS.svg",width:100%)

*Finite-domain gates.* B must fit the available candidate-base cap N. For the separate
one-large-prime birthday illustration, the supplied support ratio L/B₀ is 10,000 and
the conservative guard is √(2kB/L) ≤ 0.1. N is an allowed candidate-base cap, not an
assertion of the full point population; L must be supplied consistently for an actual curve.
If L varies with B or constants/row weights change, the separable model must be revisited.

*Native checks.* 7,776 parameter combinations satisfy objective balance, perturbed-minimum,
threshold, base/core scaling and gain-direction checks at relative tolerance 10⁻⁹.
There are 32 illustrative threshold rows and 96 domain cases: 50 pass the specified
algebraic/domain guards and 46 fail. None is an experimentally admitted solver setting.
The saturated one-large-prime genus-2 interior formula is invalid (α = 0).

*Prior art and boundary.* The accessible Avanzi–Thériault source text, Section 4.1,
contains the zero/one-large-prime tradeoffs; its conclusion excludes double-large-prime
harvesting analysis. This generic calculus re-derives those cases and makes the required
10× threshold and domain checks explicit. It is established modeling, not new asymptotics.

*Next admission evidence.* An observed ρ(k), row weights W(k), preserved target information,
valid collection regime, finite base/pool fit and complete costs against the strongest
same-target control. Existing double-large-prime cold losses remain unchanged.

Protocol 947754886e preceded numerical reduction. Native Rust source, CSVs, checks and
hashes are retained. Source transcription provenance and limitations are in PROTOCOL.md.
