# Implicit three-mode transcript and scalar consistency

## Inputs and finite Fourier identity

Let r>=7 be a caller-certified prime, h=(r-1)/2, 3<=m<=64 divide h,
L=h/m and zeta have exact order m. Let the unknown nonzero scalar be k,
Y=k^m and initial sign epsilon=k^h in {1,-1}. At nonzero public radius b,
retain three modes of s_j=(k-b*zeta^j)^h:

    f0=sum_j s_j, fplus=sum_j zeta^j s_j, fminus=sum_j zeta^-j s_j.

Define sections by the formal identity

    (X-b)^h = sum_(e=0)^(m-1) X^e A_(b,e)(X^m).

Since zeta^h=1, expand (k-b*zeta^j)^h as (k*zeta^-j-b)^h.
The geometric sum of zeta^(j(t-e)) is m when e=t mod m and0 otherwise.
Consequently the exact finite-field identities are

    f0=m A_(b,0)(Y),
    fplus=m k A_(b,1)(Y),
    fminus=m k^(m-1) A_(b,m-1)(Y).

They hold also when one translated scalar is zero. The public collector exits
with verified identity/endpoint recovery in those cases. The circuit builder
accepts field-valued observations and represents inconsistency as empty support.
It does not certify that arbitrary user-supplied observations came from a point.
Public collection supplies that binding in the qualified path.

## A rational scalar chart

If any fplus_a is nonzero, put N=fplus_a and D=m A_(a,1)(Y).
Otherwise, if any fminus_a is nonzero, put N=m Y A_(a,m-1)(Y), D=fminus_a.
In either case k=N/D for a compatible scalar. The circuit retains equations

    Y^L-epsilon=0,
    N^m-Y D^m=0,
    m A_(b,0)-f0_b=0,
    m N A_(b,1)-fplus_b D=0,
    m N^(m-1) A_(b,m-1)-fminus_b D^(m-1)=0

for every retained radius b. Prefer a plus anchor, otherwise use a minus anchor.
The same chart works across all radii, including those preceding the chosen
anchor. No target-dependent inversion is used while building the circuit.

These equations imply D!=0: in the plus case N is a nonzero constant and
N^m=YD^m rules out D=0; in the minus case D is a nonzero constant already.
Y is nonzero because Y^L=epsilon. Set k=N/D. Then k^m=Y and
k^h=(k^m)^L=epsilon. Dividing the last equations by D and D^(m-1)
recovers exactly all three observed modes. Conversely any compatible scalar
satisfies all equations and the chart, by the Fourier identities.

Thus the common F_r-roots in Y are exactly the m-th powers of scalars with
the specified initial character and complete retained three-mode transcript.
Each anchored root determines exactly one compatible scalar. This is an
image-support equivalence, not an efficient root-extraction theorem.

If every fplus and fminus is zero, omit the chart and retain phase, mode0,
A_(b,1)=0 and A_(b,m-1)=0. Every F_r-root of Y^L=epsilon is an m-th power
in F_r: it satisfies Y^(2L)=1 and 2L=(r-1)/m. Any of its m scalar roots
has the required character and all retained modes. No unique scalar chart
is asserted in this case; phase_at returns None.

The phrase complete transcript here means all three retained Fourier modes
at the supplied radii, not all m individual signs when m>3. The source
collector observes those signs but retains their three summaries.

## What is added beyond the relative constraints

The earlier state retains f0, fplus*fminus, zero-mode equations and relative
plus correlations from the first product-nonzero anchor. These are necessary
consequences of an exact three-mode match. Therefore the new support is a
subset of that earlier support. The new scalar-consistency equation retains
absolute oriented information, and the chart can start from a single nonzero
mode even if the other mode vanishes. The exact checks compare complete
three-mode image sets with the earlier relative-support oracle.

## Compact construction and accounting

Circuit nodes are field constants, Y, addition, subtraction and multiplication.
Build the section vectors by binary powering of X-b in the rank-m algebra
F_r[Y][X]/(X^m-Y). A wrapped product multiplies by Y; no coefficient vector
in Y is constructed. Structurally equal gates are interned, constant operations
are folded, and repeated radii reuse sections. The phase and chart powers use
binary powering too. Construction never enumerates scalar/image roots.

For B<=4 radii, section construction takes O(B m^2 log h) gate attempts;
phase/chart and final constraints add O(log L+log m+B) attempts. Ordered-map
interning adds logarithmic lookup cost in stored nodes. Stored graph size is
bounded by those attempts and by the explicit node cap. Scalar specialization
uses one value per node. Polynomial specialization modulo supplied monic q of
degree d stores at most S*d coefficient slots and rejects a smaller cap before
work. This bound excludes engine temporaries, map/container overhead, input
copies and allocation metadata; it is not a process-RAM cap. Specialization
costs up to O(S) polynomial operations and reductions at degree d.

The public collector uses at most1+B*m source calls and checks its explicit
call cap before each orbit query. Charge source setup/calibration, each source
call, public point shifts, weight powers, graph construction, interning,
allocation, specialization, scalar inversion and final point verification.
The collector's source, curve, base and r must describe the same certified
subgroup; this is a caller contract, not a new validation service.

A supplied modulus q is only a specialization input. Small point checks and
large known-Y fixtures do not construct an annihilator from a public target.
The circuit removes dense phase expansion from representation construction;
efficiently extracting its common roots remains the next algorithmic task.
No default recovery adapter is changed and no timing comparison is claimed.

## Sources and qualification scope

Sources checked: signed.rs and oriented-image.rs at d77ec84673a7, the preceding
implicit-phase/observation bound, and the packed section derivation. FFTW's
https://www.fftw.org/fftw3_doc/The-1d-Discrete-Fourier-Transform-_0028DFT_0029.html
records unnormalized DFT/sign conventions; it is a complex-transform reference,
not a source for this finite-field recovery theorem. The orthogonality and
chart equivalence are derived above. KB unavailable: no MCP, wrapper lacks
executable. No novelty claim.

Deterministic controls enumerate scalars only in the fixture oracle. Public
transcript collection takes points, not scalar labels. Conditional phase tests
supply fixture Y=k^m explicitly and cannot establish unknown-scalar recovery.
