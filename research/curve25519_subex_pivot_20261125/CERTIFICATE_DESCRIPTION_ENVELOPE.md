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
I = 5 + K L + 2M + 4Q + 2QL + TQ + H(Q + 2).
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
F = 12 + 6Q + 4H.
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
B = bitlength(max(M,K,L,R,Q,T,H,2^(2L))).
```

The implemented bit ceiling is

```text
(E d + Q L) p + I B + 64(C + F + 1).
```

This retains a 64-bit framing allowance per container and field. It therefore
proves existence of a polynomial-overhead binary description once `Q` and
`T` receive their separate source bounds; it does not itself bound either
quantity. `[pcc:unverified]`

## Native evidence and boundary

The standalone verifier checks that actual weight, mode-row, entry-table, and
entry-value counts fit these formulas. A duplicate-mode-slot mutation is
rejected. The positive controls cover every tested partition orbit and four
label schedules through five edges. At five edges the largest computed ceilings
are:

```text
JSON bytes              32,519,360
canonical binary bits   19,154,209
observed JSON bytes         43,450
```

For the checked two-edge fixture they are 1,227,264 JSON bytes and 725,921
canonical bits, versus a 2,162-byte JSON payload. The gap records the
`2^(2L)` worst-case concentration and uniform framing constants.
`[pcc:unverified]`

This closes a finite description-counting obligation, not the entire
subexponential theorem. A full result still needs independently justified
source bounds for `Q`, `T`, recursive histories, and the external
contraction/reduction theorems. It does not imply a practical Curve25519
attack. `[pcc:unverified]`
