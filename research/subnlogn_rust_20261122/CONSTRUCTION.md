# Construction map and conditional cost accounting

## Status and sources

The experimental Rust implementation contains a reconstruction of the integer
multiplication construction, including concrete recursive swap and layer stages.
This is a source reconstruction with execution receipts, **not a verified native
sub-`n log n` runtime**. The complete published network has not been exercised,
and the filesystem tape interpreter has an identified cost gap. The earlier
bounded `motif.rs` and `synthetic.rs` references remain separate components; their
checks do not establish the new construction. [pcc:unverified]

The primary source is *Integer multiplication below n log n*, September 23, 2026,
at `openai/math` commit `4eaee87548250b2682affda7d05d947c05554bae`. Links below
pin that revision. The manuscript's claimed bound is

$$
O\!\left(n(\lg n)^{1-\kappa}\right),\qquad
\kappa=2^{-182},\quad \lg n=\max\{\lceil\log_2n\rceil,1\}.
$$

It concerns a fixed finite alphabet and fixed number of one-dimensional tapes.
It is not a unit-cost RAM or filesystem bound. See [the machine model][s2] and
[the complete accounting][s8]. The knowledge-base MCP and repository wrapper
were unavailable; this review used the pinned manuscript, repository sources,
and the prior evidence directory. It makes no novelty claim. [pcc:unverified]

## Source-to-code map

All module paths below are relative to `isogeny_algos/src/subnlogn/`. This table
describes source correspondence, not a certificate of correctness. [pcc:unverified]

| Source | Rust component | Constructed operation |
| --- | --- | --- |
| [Section 2][s2] | `tape.rs`, elementary routines in `swap.rs` | Sequential encoded tapes, local counters, copies, rewinds, stack restoration, digit moves, controlled rotations and rational unit scalings. |
| [Section 3][s3] | `tensor.rs`, rational labels and event generator in `swap.rs` | Published `h=100` roles, all three stages, source/sink frames, arbitrary scratch restoration, residual bases and rational edge matrices. |
| [Section 4][s4] | `swap.rs` | Exact lower-triangular factorization, admissible-prime search, one recursive interchange per pivot, row splitting, parking, radix padding and general-width assembly. |
| [Section 5][s5] | `tensor.rs` | Binary basis changes, packed selected-bit additions, exceptional-record repair, directional children, endpoint phases and final normalized-layer truncation. |
| [Section 6][s6] | `transforms.rs`, `ring.rs` | Retained named-slot layouts, signed polynomial twiddles, forward/opposite transforms, signed Kronecker packing and Gaussian-integer ring products. |
| [Section 7][s7] | `certified.rs`, `resampling.rs` | Directed scalar enclosures, ordered Gaussian expansion, finite Neumann compression, and coordinate permutations retained in the transform identity. |
| [Section 8][s8] | `parameters.rs`, `crt.rs`, `assembly.rs` | Literal parameter selection, triangular CRT placement, chirps, source transforms, finite error admission, both source-volume scales, rounding and carries. |
| [Arithmetic prerequisite][mca], [Section 9][s9] | `fastint.rs`, `fastdiv.rs` | Independent recursive Fermat-ring multiplication and exact reciprocal-based division; the division schedule differs from the supplement. |

The grouped-rectangle supplement [Section 11][s11] explains the broader
descriptor interface. The layer engine uses its complete rectangular case;
this note does not claim a separate general grouped-stream parser. The
transposition consequence in Section 10 is not a prerequisite for integer
assembly and is not exposed as another completed algorithm. [pcc:unverified]

## Recursive stages and their invariants

For address swaps, `factor` constructs $LAR=\Pi$, where the two factors are
invertible lower triangular and $\Pi$ has exactly $\operatorname{rank}A$
pivots. The executor first applies $D\leftarrow R^{-1}D$ and
$H\leftarrow LH$. Each pivot performs

$$
D_j\leftarrow D_j+H_i,\qquad
(H_i,D_j)\leftarrow(D_j,H_i),\qquad D_j\leftarrow H_i-D_j.
$$

It then applies $H\leftarrow L^{-1}H$ and $D\leftarrow RD$. Thus the
only recursive work is one interchange per pivot. The fixed rational compiler
also searches an admissible odd prime and checks the published total rank;
these are actual finite algorithms, although their full-sized execution is
beyond the current resource ceiling. [s4] [pcc:unverified]

A node splits whole rows into $W_{\rm b}$ roles. Each child retains the
complete original chunk ranges: unused digits become spectators. Its descriptor
collapses the intervals around its two selected fields, and only the row count
is divided by $W_{\rm b}$. Consequently its logical volume is exactly
$V/W_{\rm b}$. With $s_{\rm b}$ rank children, the intended recurrence is

$$
F_k\le(s_{\rm b}/W_{\rm b})F_{k-1}+O(1),\qquad
F_k=O((m^k)^\tau),\quad m=10^6,\quad\tau=1-2^{-50}.
$$

High-digit joining, padding the row count, base-$m$ pieces and inverse
joining remove the power-width restriction. Inactive roles are appended to a
parking stack and restored backwards without traversing ancestors. Temporary
copies are charged to parent volume, never substituted for child volume in the
recurrence. [s4] [pcc:unverified]

The complex layer similarly executes all published roles and edges. Sparse
completed binary bases provide concrete coordinate-addition programs; there
are $s_{\rm c}$ directional children per network node. Decreasing edges use
the inverse kernel with its phase corrections. The middle stage executes the
inverse scalar schedule. Scratch rows may contain arbitrary data.
Packed basis changes use eight ordered rotations and sort only extracted
exceptional records. Repair is $T_0S_0^{-1}$ on **current** exceptional
addresses, followed by checked reinsertion. These details are necessary for
the source recurrence and cannot be replaced by unrestricted scatter or zero
scratch assumptions. [s3] [s5] [pcc:unverified]

## Precision and complete numerical assembly

Complex values contain exact signed integer numerators over a caller-specified
$2^p$. Scalar constants use directed integer intervals, Machin's formula,
binary splitting and bit-burst exponential evaluation. No floating-point value
decides a disk, precision or rounding predicate. Gaussian windows and the
finite Neumann series preserve the imported truncation locations; compression
selects an increasing subsequence instead of executing the two Fourier
coordinate permutations. [s7] [pcc:unverified]

Layer arithmetic remains exact until the complete normalized layer finishes.
The accelerated branch checks the published admission condition

$$
\Delta=C_0d^{20}\le p,\qquad
C_0=128m\bigl(s_{\rm c}+64(W_{\rm c}+m+1)^3\bigr)^2.
$$

The individual-kernel diagnostic branch uses a separate conservative guard.
Intermediate halves and encoded widths are checked; a failed guard is an
error. The final layer truncation is toward zero. Runtime checks supplement
the admission argument; they are not its replacement. [s5] [pcc:unverified]

`assembly::Plan` connects radix digits, padded CRT placement, Gaussian maps,
chirp convolution, pointwise multiplication, the opposite source transform,
both factors of $S=\prod_i s_i$, and the radix carry scan. Its precision
certificate retains unsimplified finite error quantities and requires the
final absolute coefficient error to be strictly below $1/2$. Intermediate
disk margins are also checked. Signed Kronecker packing writes and extracts
fixed-width fields, avoiding repeated shifts of a growing packed integer.
These are construction obligations implemented in source; the certificate
does not independently prove its underlying lemmas. [s6] [s8] [pcc:unverified]

## Amended arithmetic cost argument

The manuscript imports an established $O(N\log N)$ integer multiplier.
`fastint.rs` instead implements the independent Schönhage–Strassen-style
Fermat-ring prerequisite described in [Modern Computer Arithmetic][mca].
Its intended bit-operation bound is
$M(N)=O(N\log N\log\log N)$, with a fixed schoolbook cutoff.
This prerequisite does not replace the rank-saving top-level construction.
`fastdiv.rs` uses full-width Newton refinement, costing
$O(M(N)\log N)$, rather than claiming the supplement's sharper
precision-doubling bound. These are mathematical operation analyses, not
native timing results. [pcc:unverified]

For scalar calls on the used resampling family, balanced rational trees have
$O(p\log p)$ bits and there are logarithmically many bit-burst/restoration
stages. The refinement argument requires only $O(\log p)$ extra guard bits:
interval errors grow polynomially in $p$, and the positive Gaussian diagonal
stays uniformly below one. Hence these concrete prerequisites give
$p(\log p)^{O(1)}=O(p^{1+\delta})$ scalar work for fixed
$\delta=1/16$. This claim is restricted to the constructed parameter
family; arbitrary oversized rational encodings or arguments approaching a
public scalar API's disk boundary need separate accounting. [s7] [pcc:unverified]

More specifically, argument restoration uses $h$ squarings with $2^h\le16p$;
the resulting interval width is $p^{O(1)}2^{-\mathrm{work}}$. For the positive
Gaussian diagonal, $\beta^2\le1/4$ and $\alpha\ge2$ give

$$
\log D'\le-\frac{23}{42}\alpha^2+\frac43\le-\frac67,
\qquad D'<\frac12.
$$

Thus the diagonal's disk test has a fixed margin, rather than demanding
unbounded precision near one. Explicit disk checks and adapter validation add
soft-linear scalar work to their scans. Their fixed polylogarithmic factors
must be included in the amended accounting. [s7] [pcc:unverified]

A more explicit sufficient guard can be obtained from the implemented interval
operations. Write $w=p+g$, $H=\lceil\log_2(16p)\rceil$, $D=w+H$, and
$F=2^{20}(p+1)^3D^3$. The two Machin arctangent enclosures give a pi interval
of width at most $2\cdot2^{-w}$. The phase argument width is at most
$6\cdot2^{-w}$; a real argument with numerator/denominator at most $p$ has
width at most $(2p+2)2^{-w}$. The balanced bit-burst products have component
endpoint error below $20D^3 2^{-w}$: each new factor obeys the conservative
recurrence $\eta_{j+1}\le5\eta_j+14\cdot2^{-w}$ and there are at most
$\lceil\log_2D\rceil$ factors. [pcc:unverified]

During argument restoration, absolute error for phase/negative arguments, or
relative error for positive arguments, obeys
$\eta_{j+1}\le5\eta_j+2\cdot2^{-w}$ while $\eta_j\le1/2$.
Since $2^h\le16p$, the restored error is bounded by
$R=2^{17}p^3D^3 2^{-w}$. The sufficient condition $2^g>4F$ implies
$R<1/128$, validating that induction, and bounds the final interval widths
by $F2^{-w}$. The positive diagonal's fixed disk margin then also satisfies
its upper-endpoint admission. One sufficient choice is

$$
G(p)=64+16\lceil\log_2(p+2)\rceil.
$$

For $L=\lceil\log_2(p+2)\rceil$ and $p\ge2$, this gives
$D\le32(p+2)$ and $F\le2^{35+6L}<2^{G(p)-2}$.
The initial guard $32+2\operatorname{bitlength}(p)$ reaches a sufficient
guard within three doublings. This is an unsigned interval-error analysis of
the used family, not a formal verification of the implementation.
[pcc:unverified]

The restriction to that family matters. For example, a generic positive call
with $p=128$, ratio $1/2^{1000}$ and zero binary scale initially encloses the
argument by $[0,2^{-w}]$ for every $w\le998$. Its lower exponential endpoint
is one and its upper endpoint exceeds one, so the generic disk test can
neither accept nor reject at those precisions. Valid rational inputs can also
approach the disk boundary arbitrarily closely. Their encoding size and
distance to the boundary are additional parameters; the $O(\log p)$ guard
claim is not made for that unrestricted API. [pcc:unverified]

Put $V=Tp=\Theta(n)$, $p=6\lceil\log_2n\rceil$,
$\epsilon=2^{-75}$, $c=2^{-56}$, and
$\lambda'=1-2^{-54}$. The source's normalized costs, before host overhead,
are as follows. The table is a conditional accounting map. [s8] [pcc:unverified]

| Operation | Cost divided by $V$ |
| --- | --- |
| Prefix moves and individual rounds | $dK$ |
| Transform chunk exchanges | $pK^{\tau-1}$ |
| Simultaneous rounds | $\ell d^{\lambda'}$ |
| CRT and axis layouts | $d^2(1+\ell^\tau)$ |
| Gaussian line maps | $dp^{1/2+\delta}\alpha$ |
| Chirps, twists and scalar products | $dp^\delta$ |
| Packed polynomial products with this prerequisite | $\log(rp)\log\log(rp)$ |
| Final scales, rounding and sequential I/O | $O(p^\delta+1)$ |

The additional ring factor satisfies $\log\log(rp)=O(\log p)$, since
$\log r=\Theta(p^{1-\epsilon})$. It is not a factor $\log n$.
The source's smallest exponent margin is
$\epsilon c(1-\tau)=2^{-181}=2\kappa$. Therefore a fixed additional
power of $\log p$, where justified by the arithmetic analysis, is absorbed:

$$
Vp^{1-2\kappa}(\log p)^C
=O(Vp^{1-\kappa})
=O\!\left(n(\log n)^{1-\kappa}\right).
$$

This argument is conditional on the stated stream, precision and prerequisite
costs. It cannot absorb arbitrary polynomial overhead in $p$, or a factor
depending logarithmically on total data volume at every bit. [s8] [pcc:unverified]

## Native limitations and remaining obligations

The literal selector preserves $d=\lfloor b^{2^{-75}}\rfloor$ and
$K=\lfloor d^{2^{-56}}\rfloor$, using exact integer roots. On current
32/64-bit targets every representable nontrivial input length gives $d=1$,
so this route selects a base case. Explicit-parameter transforms are diagnostic
executions of the numerical path, not executions of the published parameter
family. Full $h=100$ rank-saving branches remain unexercised; their role
counts exceed $2^{64}$, and the published precision guard also exceeds native
descriptor capacity before that branch becomes feasible. [s3] [s8] [pcc:unverified]

The extra finite admissions do not remove the eventual mathematical family:
$\Delta=o(p)$, reserved row axes are $O(\log d)$, $K/\log p\to\infty$,
$r$ exceeds every fixed polynomial in $p$, and $\gamma=o(b)$. The conservative
final numerical error is $O(2^{-b+\gamma}\operatorname{poly}(b))$, which tends
to zero. These observations address mathematical admission, not native
representability. [s5] [s8] [pcc:unverified]

The tape interpreter uses amortized ripple counters, but role reopening and
flushing serialize complete `Big` length/cursor headers for each row. With
fixed chunk width and payload and growing prefix/row count, this can introduce
$\Theta(V\log V)$ native header work. Decimal segment filenames add further
growing overhead. The abstract fixed-tape machine keeps role heads parked;
its recurrence does not pay these operations. Movement receipts and
`header_bits` must remain distinct. Ignoring header work does not prove a
native runtime bound. [s2] [s4] [pcc:unverified]

The remaining obligations are concrete. [pcc:unverified]

1. Independently certify the full finite-network labels, rank totals, endpoint
   identities and recursive restoration invariants, beyond feasible fixtures.
2. Independently validate the used-family scalar guard, finite error certificate
   and prerequisite recurrence arguments against the pinned implementation,
   including their composition across all calls.
3. Remove or rigorously account for native header, segment-addressing, allocator
   and descriptor costs through a faithful fixed-tape lowering. The present
   interpreter does not discharge this obligation.
4. Preserve explicit base-case/transform provenance, native resource failures
   and unexercised-branch status in public receipts. Add the separately pinned
   final native-observation receipt when available; this note asserts no new
   test counts or performance measurements.

[s2]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/02-streams.tex
[s3]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/03-motifs.tex
[s4]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/04-swap.tex
[s5]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/05-layers.tex
[s6]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/06-transforms.tex
[s7]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/07-resampling.tex
[s8]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/08-assembly.tex
[s9]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/09-exact-arithmetic.tex
[s11]: https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/11-grouped-rectangles.tex
[mca]: https://members.loria.fr/PZimmermann/mca/mca-cup-0.5.9.pdf
