# Odd regular Hermite controls: conditional contract

This fresh worker implements the unexecuted derivation in
research/odd_regular_plan_20261115_v3/PROOF.md, whose independent static
review is research/exact_btf_review_20261115_v3/ODD_PLAN_REVIEW.md. Author
performed AST parsing only, no import, solver, arithmetic oracle or benchmark.

The declared fields are GF3,GF5,GF9=GF3[t]/(t²+1),GF25=GF5[t]/(t²+2).
Extension coordinates a0+p*a1 are canonical. Both quadratic moduli have no
base-field root. Direct two-coordinate multiplication tables are checked
against coefficient-vector convolution/reduction on EVERY pair; all triples
check associativity/distributivity and every nonzero inverse is checked.
Every integer polynomial/derivative constant embeds modulo the CHARACTERISTIC,
not as a canonical extension integer. In particular 3 is zero in GF9.

Use C=X³+X and C=X³+X+1. Their discriminants -4 and -31 are nonzero for
p3/p5, proving squarefreeness over the algebraic closure; literal finite-root
derivative checks audit these fixed models but are not a general gcd substitute.
All rational curve points are explicitly listed. Before any comparative
solving, select first lexicographic representatives of declared guard categories
(ramified,zeroa,targetcollision,r1zero/nonzero,Delta0/nonzero,regular), then
fill with lexicographic candidates up to16cells perGF3/GF5/GF9 and8GF25.
Selection uses only algebraic guards, not outputs or timing. At most100000
selection candidates are examined per field. Missing categories are recorded.

On a!=z,ya!=0,G(a)!=0, the proof's invertible two-by-two equations yield
r=r0+r1k,s=s0+s1k. The implementation builds these constants directly.
N=P²-CG² is formed by literal convolution, divided exactly by target X-z,
then by (X-a)². The formal k coefficients N0,N1,N2 are each divided by
the fixed combined denominator and replayed for every derived k. The cubic
sum S=h²-z-2a-2r0-2r1k is checked and is not treated as fixed. Delta and
r1 are recorded; the worker does NOT use Delta division to solve, so no
zero-Delta branch can be discarded. Ya0 regular fibers are checked empty.
A=z is explicitly excluded from this chart; G(a)=0 remains UNKNOWN.

The independent dense oracle traverses EVERY (k,r,s) in each admitted cell,
up to139096 raw states before target-collision skips. It does not reuse the
derived affine formula. Instead it directly evaluates the target-imposed P(a),
P'(a), G(a), and N'(a)=2P(a)P'(a)-C'(a)G(a)²-2C(a)G(a)h. A regular selected
point/double-norm solution exists iff the literal selected point equation and
this derivative vanish. Every such survivor then receives independent literal
norm convolution and both exact divisions. Thus this prefilter preserves the
FULL dense parameter oracle while avoiding unnecessary norm convolutions;
raw-state and literal-norm-evaluation counts are separate. Compare complete
sorted(k,r,s) sets, not only counts. Ramified ya0 gives no regular solution.

Each derived regular candidate is retained with k,r,s,t,P,N,H,Q. Dsrc={a,a+1}
has two distinct elements. Build every one of its four cubic multisets by
independent factor multiplication. Q has all its roots in Dsrc with repetitions
iff (A_D modQ)³=0; the primary-factor argument and degree-three multiplicity
bound prove this in all four fields. Every support result is compared with
membership in the full independent multiset index. Supported cubics emit all
distinct ordered five-tuples after appending two copies ofa. Different k fibers
are retained. Store every support remainder and cubic residue, including all
rejections. Source/target overlap is recorded and not silently filtered.

At least one supported regular norm parameter is required globally; per-field
positives and missing guard categories remain observed coverage, not assumptions.
Output is complete only for these frozen selected norm components. Common-factor,
other points/targets/h, original auxiliary/nonrational/ghost and global root
coverage remain UNKNOWN. No NIST benchmark, novelty or qualified speed claim.

Budgets: <=200000 raw triples, <=2000000 checked predicates,80s internal work,
84s post-serialization demotion,88s final print,90s external whole process,
4GiB per process and1core. Full raw payload has32MiB cap; overflow makes the
result inconclusive and retains an omission hash. Artifact gzip is exclusive.
Exactly six result fields are printed, with compact summaries only; full
certificates remain in raw-controls.json.gz. RSS is one process high-water mark,
not aggregate descendants. A final serialized report is constructed before
completion acceptance; late output clears conditional completion/exact positive
counts. The runner additionally charges imports/startup/artifact/final stdout.

Next: independently replay the stored norm/support/source certificates and
diagnose guard/positive-field gaps before a fresh changed variant. Compare a
cheaper construction only on matching conditional contracts. Generic outer
enumeration remains large; no binary Fp-affinity or universal cost bound applies.
