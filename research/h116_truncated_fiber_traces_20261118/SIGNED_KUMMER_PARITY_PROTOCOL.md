# H116 signed-y Kummer parity protocol

Protocol ID: `H116-SIGNED-KUMMER-PARITY-001`

Filed after the experiment and Lean proof excluding every negation-invariant
x-only parity decoder, before evaluating a signed-y cover. [pcc:unverified]

## Nonsymmetric cover family

On the same five discovery curves and first public generator `P`, use anchors
`A_j=[j]P`, `1<=j<=8`. For every nonempty subset `S` of size at most three,
define

```text
D_S: z^2 = product_{j in S} (y - y(A_j)).
```

The split/inert bit is the Legendre symbol of the displayed product at
`Q=[k]P`; zeros abstain. Test both polarities as predictions of `k mod 2`.
There are 184 fixed rules. Unlike the previous x-covers, these functions are
not invariant under `Q -> -Q`, so the audited pairing obstruction does not
apply.

## Controls and decision

Use the exact 1,520-target, five-curve discovery census, statistic, deterministic
tie-break and 4,096 within-curve family-wise label shuffles from
`H116-KUMMER-PARITY-001`. Retain exact and exact-on-covered counts, branch
coverage, per-curve performance and a byte-identical replay.

Only a calibrated discovery association may nominate one unchanged rule for a
new large-curve OOS protocol. Abstention is not exactness. A null result rejects
only this bounded signed-y cover family; it does not reinstate the x-only
route or establish a generic impossibility. [pcc:unverified]
