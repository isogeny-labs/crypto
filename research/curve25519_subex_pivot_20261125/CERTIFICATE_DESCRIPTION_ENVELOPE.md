# Factorization certificate description envelope

This note gives the explicit description bound implemented by
`factorization_certificate::description_envelope`. It applies to a certificate
already accepted by the standalone verifier. It is a producer-side counting
argument and not a theorem-assistant proof. `[pcc:unverified]`

## Parameters

Let:

- `M` be the old edge count;
- `K` be the packet count;
- `L` be the maximum packet length;
- `tau` be the certified tile period `2^(i+1)`;
- `R` be the label radix;
- `Q` be the number of generated modes;
- `T` be the number of retained exponent tuples;
- `d` be the extension degree; and
- `p` be the coefficient precision, so coefficients lie modulo `2^p`.

Every packet has at most `L` slots. For a fixed packet, let `a_g` be the
number of left and right incidences assigned to group `g`. Then

```text
sum_g a_g <= 2L.
```

Convexity of `2^a` moves mass to one group, so

```text
sum_g 2^(a_g) <= 2^(2L) + R - 1.
```

Thus, writing

```text
S = 2^(2L) + R - 1,
H = T R K,
```

there are at most `H` entry tables and at most `T K S` packet-entry ring
elements. This is deliberately worst case: it charges the maximally
concentrated arity to every packet and tuple.

## Source bounds for modes and tuples

The schema-2 verifier accepts only a power-of-two tile period `2 <= tau <= L`.
Every retained mode has nonempty columns in one residue class modulo `tau`,
and the verifier rejects two modes with the same `(packet, weight, residue)`.
Since every mode weight lies in `1..p`, this gives, without trusting the
producer's mode count,

```text
n = K tau,
Q_b <= n                         for each weight b,
Q <= n (p - 1).
```

The independent tuple check reconstructs exactly every nonnegative exponent
vector of total weighted degree below `p`, subject also to packet/group
capacities. Dropping those extra capacities can only increase the count. For
at most `n` variables of each positive weight, the weighted generating
function at `exp(-theta)` gives

```text
ln T <= theta p + 2n/theta.
```

Choosing `theta=sqrt(n/p)` and converting to base two is covered by the
integer source bound

```text
log2 T <= 5 ceil(sqrt(n p)).
```

The verifier recomputes all three ceilings from `K`, `tau`, and `p`, checks
the mode tile keys and exact tuple coverage, and rejects a certificate whose
actual `Q` or `T` exceeds them. This is the same local source count used by
`admission::dimensions`, now bound to serialized factorization evidence.
When the separate active-call premises provide `L=2^a`, `tau=2^(i+1)`,
`D=L/2^i`, and `K L<=3M`, they also give

```text
n D = 2 K L <= 6 M.
```

The packing-capacity and recursive-precision premises remain separate
obligations; this certificate does not manufacture them. `[pcc:unverified]`

## Ring elements and scalar atoms

Packet weights, mode rows, and entry values contribute at most

```text
E_w = K L^2,
E_m = Q L,
E_v = T K S,
E   = E_w + E_m + E_v
```

ring elements. Mode-column scalars contribute another `Q L` residues. Each
ring element has `d` coordinates of fewer than `p` bits. The verifier
rejects duplicate row or column slots within a mode, making each `QL` term
load-bearing rather than an assumption about producer output.

The schema's non-coefficient scalar atoms are bounded by

```text
I = 6 + K L + 2M + 4Q + 2QL + TQ + H(Q + 2).
```

The final term includes the deliberately repeated `beta` vector and the
group/packet headers in every JSON entry table.

## JSON byte ceiling

Let `W = ceil(p/32)`, the maximum number of serialized `u32` limbs in one
coefficient. The primitive JSON atom count is bounded by

```text
A = I + (E d + Q L) W.
```

The implementation bounds JSON containers and named fields by

```text
C = 8 + 2K + KL + E(d + 1) + 3Q + 3QL + T + 3H,
F = 13 + 6Q + 4H.
```

Every primitive number, delimiter, comma, key, and colon is covered by the
coarse fixed allowance

```text
JSON bytes <= 64 (A + C + F + 1).
```

The factor 64 is intentionally generous. The purpose is a simple uniform
description envelope, not compression or a tight model of `serde_json`.
`[pcc:unverified]`

## Canonical binary ceiling

For a canonical length-prefixed encoding, let

```text
B = bitlength(max(M,K,L,tau,R,Q,T,H,2^(2L))).
```

The implemented bit ceiling is

```text
(E d + Q L) p + I B + 64(C + F + 1).
```

This retains a 64-bit framing allowance per container and field. Together
with the checked source bounds above, it proves existence of this finite
description ceiling from the certificate dimensions. The separate recursion
induction must still bound those dimensions over a complete call tree.
`[pcc:unverified]`

## Native evidence and boundary

The standalone verifier checks that actual weight, mode-row, entry-table, and
entry-value counts fit these formulas. A duplicate-mode-slot mutation is
rejected. The positive controls cover every tested partition orbit and four
label schedules through five edges. At five edges the largest computed ceilings
are:

```text
JSON bytes              32,519,488
canonical binary bits   19,154,282
observed JSON bytes         43,460
```

For the checked two-edge fixture they are 1,227,392 JSON bytes and 725,994
canonical bits, versus a 2,172-byte JSON payload. Its recomputed source fields
are `tau=2`, `n=4`, `Q<=16`, and `log2 T<=25`. The gap records the
`2^(2L)` worst-case concentration and uniform framing constants.
`[pcc:unverified]`

The standalone module now exposes `verify_source_dimensions`, which takes the
source coordinates `(a,u,i)` and independently reconstructs
`(M,K,L,tau,D,p)` from an already accepted serialized certificate. It requires
uniform packet length and checks `K L<=3M`, `nD<=6M`, and `p u<=34MD`, then
recomputes the same mode and tuple ceilings. The production controls compare
that result field-for-field with the separate BigUint `NodeDimensions` audit.
Among the 596 three- through five-edge partition-shape certificates, 285 meet
the source packing premise; all 285 agree exactly. The remaining 311 are useful
algebraic factorization shapes but both refinements reject them at the
packing-capacity boundary. The correlated two-edge fixture is rejected there
as well. This pinned split prevents a finite factorization control from being
misreported as an admitted recursive node. It still does not prove that a
graph or the source packing routine produced the certificate.
`[pcc:unverified]`

The native verifier exposes this refinement as
`factorization-certificate-check [CERTIFICATE.json] --source A U I`; omitted
source arguments preserve the ordinary algebraic certificate check.

This closes a finite description-counting obligation, not the entire
subexponential theorem. A full result still needs the recursive-history and
precision induction, plus the external contraction/reduction theorems. It
does not imply a practical Curve25519 attack. `[pcc:unverified]`
