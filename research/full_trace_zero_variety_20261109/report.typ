#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Full trace-zero variety: production integration
2026-11-09 · Isogeny-labs/crypto · PR3744 · local unsigned validation

*Result:* Full group-trace membership, projection, coordinates, three actual
Boolean polynomial models, factor-base intersection and native S3 relation
integration are implemented. Ten focused tests and the root library type-check
pass locally. These are producer assertions with unverified independent standing.

#image("construction.svg", width: 100%)

== Establishing the algebraic object
Let k=F₂, K=F₂ⁿ, E₀/k: Y²Z+XYZ=X³+Z³ and W=Res(K/k)(E₀).
Define the scheme-theoretic kernel A of the descended group-sum map W → E₀.
After algebraic closure, W is E₀ⁿ and the map sums the factors.
The map from E₀ⁿ⁻¹ sending the last coordinate to minus the sum of the others
is an isomorphism onto A. Thus A is a smooth connected abelian variety of
dimension n−1. A splitting over k is not asserted.

Its k-points are exactly the kernel Tₙ of the Frobenius point trace.
E₀(F₂) is cyclic of order four, and trace restricts to multiplication by odd n,
so it is onto. The unique nontrivial geometric two-torsion point (0,1) has
nonzero trace, so the kernel has odd order. Therefore Tₙ=4E₀(K), of size \#E₀(K)/4.
This is standard restriction-of-scalars mathematics plus the written argument;
no new formal proof or independent signed verdict is supplied.

#pagebreak()
== Exact rational-point equations
The existing production halving derivation gives, for affine curve points,
membership exactly when Tr(x)=0 and Tr(y+xH(x))=0. Here H is the odd-degree
half-trace. A half exists iff lambda²+lambda=x has a solution; choose H(x).
Its abscissa u satisfies u²=y+(lambda+1)x, so the second bit is exactly Tr(u).
This includes the missing second halving test. The identity is separate.

In the polynomial basis, write Tr(x)=sum(tᵢxᵢ) with t₀=1, and q(x)=Tr(xH(x)).
q is quadratic Boolean ANF. Expand the original curve into n coordinate rows.
The raw model adds both trace rows. The compact model substitutes x₀ from
the first bit and y₀ from the second, raising the degree bound to three.
The factored model retains a helper h=q(x), keeping degree two.

#table(columns: (auto, auto, auto, auto, auto, auto), inset: 5pt,
  [n / mode], [Variables], [Rows], [Terms], [Degree], [AND aux],
  [83 raw], [166], [85], [53707], [2], [10292],
  [83 compact], [164], [83], [63644], [3], [20468],
  [83 factored], [165], [84], [53042], [2], [10127],
  [131 raw], [262], [133], [100717], [2], [25676],
  [131 compact], [260], [131], [117552], [3], [43517],
  [131 factored], [261], [132], [99684], [2], [25415],
)

These are constructed counts from one model per row, not solver timings.
The compact model removes two bits but adds terms and auxiliaries. All
variables are Boolean; equations live in F₂[v]/(vᵢ²+vᵢ). Together with the
identity branch, they represent all F₂-rational points. They are not a
positive-dimensional geometric defining ideal or an arbitrary-algebra chart.
Those remain distinct software deliverables.

ECC2K-130 uses n=131, polynomial z¹³¹+z¹³+z²+z+1 and full trace-kernel order
680564733841876926932320129493409985129. This is already the challenge subgroup.
The dimension130 and two-bit coordinate elimination do not shrink that group.

== Production path and guarantees
TraceZeroVariety validates curve shape, irreducibility and full cardinality;
it supplies exact point trace, idempotent projection P−[n mod4]Tr(P), the
different cofactor image tau(P)−P, and a checked codec.
The factor-base API preserves exactly the supplied points in Tₙ and reduces
the subspace basis by its x-trace bit. It does not silently replace them with
cofactor images. The root index-calculus adapter uses this API.

The native BigUint S3 path now admits n=131, attaches full membership and
curve equations to every summand and replays full points. The oracle handles
all intermediate-identity patterns for arity2 through9 and lifts x-tuples to
actual base points before original-group replay. Only complete exhaustion
returns Refuted; resource caps return Inconclusive. Existing CLI selection is
unchanged: these are explicit library entry points.

#pagebreak()
== Checks and retained incomplete searches
The focused facade imports actual production field, group, model, S3 and SAT
modules with no arithmetic mocks. Ten tests pass. Checks cover exhaustive
point traces, projection and codecs at n3/5/7/9 (kernel sizes1/11/29/127),
all model assignments at n3/5, actual n83/131 polynomial models on public
fixtures and torsion cosets, malformed admission and allocation caps,
factor-base intersection, complete n5 relation comparisons, certificate
replay and changed-statement rejection, and the following wide controls.

The degree131 full-membership SAT encoding rejects x=0, whereas the x-trace
bit alone admits the order-two point. Fully pinned P+(-P)+P=P controls give
SAT with original-group replay including an identity intermediate. At n83:
31540 variables, 92296 clauses, 421 XORs. At n131:78076,230560,661.
These check consistency of those witness encodings, not natural relation finding.

The separately bounded x-only-pinned n83 control returns *UNKNOWN* at its
one-second/2000-conflict cap. Its test passes by retaining that explicit
outcome; UNKNOWN is not a solution. An earlier unbounded control was
interrupted with SIGINT after90+ seconds; its output is preserved.
No natural degree131 relation, measured solving degree, yield, performance gain or
challenge recovery is reported.

Root command cargo check --lib --no-default-features passes. This does not
clear the earlier whole-root test failures or unrelated historical benchrec
gate. Matched experiment execution still needs task-specific checkpoint/S3
credentials; no EKS or paid job ran. This work is local development validation.

== Reproduction and evidence
README.md gives the exact locked cargo test/run commands and variable ordering.
The native exporter emits actual ECC2K-130 ANF when given a new output path;
otherwise it prints summary metadata. Infinity is explicit outside the affine
equations. checks/native-tests.txt and checks/root-cargo-check.txt retain
output; checks/commands.txt pins commands/environment; SOURCES.sha256 pins
source. The generated large equation export is not committed.

The integral telescoping identity has certificate IDC1h3fcd7aa52221c17e,
32 issued plus32 replay points over 2⁶¹−1. It is not a characteristic-two
membership certificate or formal theorem. Local evidence remains unsigned;
no independent PCC standing is asserted.

Flow: full models and local checks completed; unbounded wide control
interrupted; bounded control completed UNKNOWN; natural matched relation
finding, performance and full DLP not attempted. Existing performance graphs
retain their values; this study adds construction.svg only.

== Sources
Gorla–Massierer arXiv:1405.1059, section2 (trace-zero variety), and1403.0126
(representation caveats); Kosters–Yeo1503.08001; Galbraith–Granger–Merz–Petit,
SAC2020, Index Calculus; production curve.rs, is_in_4e and its point-halving
references. The prior trace_zero_relation_check_20261109 study preserves the
matched x-only layout audit. KB connector and executable are unavailable
(wrapper exit127); these primary and repository sources are the fallback.
