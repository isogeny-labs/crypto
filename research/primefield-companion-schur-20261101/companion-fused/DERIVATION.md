# Deferred-reduction companion application

Derived candidate and syntax-checked prototype, 2026-11-01. No numerical
experiment has run. The capped Schur experiment and its frozen source remain
unchanged. This targets repeated intermediate decoding/reduction and scalar
wrap work in each source application, not the source-call count itself.

Let R=r-1, n be dimension, s generator width, and a,b distinct canonical
wraps. The existing checked polynomial algorithm computes, per column pair:

    P = first * shared,
    T_j = P_(n-1+j) + u P_(j-1), with P_(-1)=0,
    Q = second * reverse(T),
    Y_i = Q_i + v Q_(n+i), with out-of-range coefficients zero.

For primal application: first=k, shared=reverse(x), second=g, u=b,v=a.
For transpose: first=reverse(g), shared=x, second=reverse(k), u=a,v=b;
reverse the final output vector. Sum Y over all s columns and scale by
(a-b)^-1 modulo r. All these operations commute with final reduction mod r.

## One radix valid through the entire query

Use nonnegative integer lifts and choose beta=256^d strictly exceeding

    B = s n^2 R^4 (1+a)(1+b).

First-product coefficients are at most n R^2. Extraction coefficients T
are at most n R^2(1+u). Second-product coefficients are at most
n^2 R^3(1+u); wrapping multiplies that bound by (1+v). Summing s columns
and scaling by the canonical inverse, at most R, is bounded by B.
Every earlier coefficient is also bounded by B. Thus no cross-digit carry
occurs in any product, extraction, fold, sum or final scaling.

For w=8d, extraction works directly on packed integers:

    Tpack = (Ppack >> ((n-1)w))
              + u (Ppack & (2^((n-1)w)-1)) 2^w.

Reversal permutes fixed-width byte chunks; it does not decode coefficients.
Wrapping is

    Ypack = (Qpack & (2^(nw)-1)) + v (Qpack >> nw).

Accumulate these packed vectors and scale once by the modular inverse.
Only the n final coefficients are decoded and reduced. The n=1 and zero-wrap
cases obey the same bounds and formulas. The prototype asserts that each
packed intermediate vector fits its declared n-digit buffer.

## Exact intended ledger, still requiring a frozen check

A query uses one field subtraction, one inversion, zero separately executed
scalar field products, and n final coefficient reductions. Native work is
explicit, not treated as free:

- 2s convolution integer products, 2s scalar fold products, one packed scale;
- seven setup products to construct B: 4s+8 native products total;
- 3s+1 input encodings and (3s+1)nd packing bytes;
- snd intermediate serialization bytes and nd final serialization bytes;
- snd bytes on each side of fixed-chunk reversal; nd decoded bytes;
- 3s+2 integer shifts, 2s masks, 3s+2 additions, three subtractions;
- n coordinate-reversal slots in the primal direction, (2s+1)n in transpose.

Every native product records operand/result bit lengths. P and Q are not
serialized to byte buffers; their native result-bit volumes remain charged.
Counter formulas describe this prototype, not tested performance. Index,
loop and assertion operations are outside the named arithmetic/byte ledgers;
no total CPU/RAM inference is made. The radix is larger than in the earlier
per-product kernel. That tradeoff must be measured in separate native/byte
units, with exact output equality, rather than a mixed speed score.

## Next experiment

Freeze the complete retained generator/block contexts, both directions,
zero/one/seeded/basis vectors, n=1, zero wraps and maximum digits. Independently
check the carry bound, every extraction/fold identity, final products and
all field/native/byte counters, with radix/carry/sign/count corruption controls
and A/A. Only after that validation may this replace a source operator in a
new Schur comparison. The current Schur producer must not be patched or restarted.

## Pre-validation instrumentation revision

The unexecuted prototype now records operand and result bit-length volumes
for integer addition, subtraction, masks and shifts, in separate counters.
For shifts the data operand bit length and shift distance are separate;
shift-distance integers are not counted as data words. Multiplication retains
its operand-bit-length product and result-bit-length ledgers. These are
representation-volume descriptors, not a bit-operation or timing model.
The arithmetic operation-count formulas above are unchanged.

An optional trace callback exposes setup (bound, digit bytes, canonical
inverse), each column's P,T,Q,Y and accumulated packed value, and the final
scaled packed result. It permits independent coefficient-list comparison of
every intermediate in the frozen carry fixtures. Trace storage and callback
work are validation overhead outside the kernel arithmetic ledger. The
production path defaults to no trace. No numerical trial has run.

The per-vector buffer assertions only verify total packed size. The reason
individual digits cannot carry is the coefficient bound, which the new
reference must independently check; fitting a buffer alone is insufficient.
