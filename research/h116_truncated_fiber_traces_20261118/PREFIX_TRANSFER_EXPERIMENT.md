# H116 leading-parameter boundary control

Status: complete finite-field control under frozen protocol
`H116-PREFIX-TRANSFER-001`. This is not an elliptic-map or ECDLP experiment.
[pcc:unverified]

## Frozen question

For unit power series `f` and a nonzero correction `c X^m`, does multiplying by
the shared leading parameter move the first visible difference from degree `m`
to degree `m+1`, while inverse-square normalization moves it back to `m`? Is
the degree-`m` difference the closed-form value `-2 c f(0)^-3`? [pcc:unverified]

The protocol fixed five odd primes and four cutoffs before execution, for 20
cases total. It also froze a zero-correction control and a control that moves
the correction one degree later. No case was replaced. The protocol and both
implementations were committed at `bb78e0cabf` before result generation.
[pcc:unverified]

## Result

| measure | frozen prediction | observed |
| --- | ---: | ---: |
| cases | 20 | 20 |
| passing cases | 20 | 20 |
| local-parameter first-difference offset from `m` | 1 | min=mean=max=1 |
| inverse-square first-difference offset from `m` | 0 | min=mean=max=0 |
| boundary-delta matches | 20 | 20 |
| zero-correction controls | 20 | 20 |
| shifted-correction controls | 20 | 20 |

The first case `(p,m)=(23,1)` changes the local parameter at degree two and the
inverse square at degree one; observed and predicted boundary delta are both
20 modulo 23. The last case `(587,22)` changes them at degrees 23 and 22;
observed and predicted delta are both 411 modulo 587. Every intermediate case
has the same frozen offsets. [pcc:unverified]

The independent verifier imports no producer code. From the retained JSON it
reconstructs the case grid and checks each boundary value against
`-2 c 2^-3 mod p`; it reports 20 boundary matches, 20 zero controls and 20
shifted controls. A replay with the first observed boundary delta incremented
by one is rejected. [pcc:unverified]

## Interpretation boundary

The data show that the one-degree loss in the checked generic theorem is sharp
on this panel: asking for one more inverse-square coefficient would be false in
all 20 nonzero-correction cases. This supplies a measured control for the Lean
precision bookkeeping. It does not show that an elliptic Frobenius correction
has the assumed order, or that the regular unit factor in `x(z)` respects the
same prefix under substitution. Those remain the next proof obligations.
[pcc:unverified]

## Integrity and replay

```text
05328135bbbc1592d108a227faf2d5565893c5a6511708d05b8325ce3382b374  PREFIX_TRANSFER_PROTOCOL.json
6dcd1c277def9283505bedb4fc4394e8251e62eb4fe184c5f2c49ae7bd21fd09  h116_prefix_transfer_control.py
1e6995e018d0ee8afd337d80067a4a6ff93841f30dcdd04debd5a725faa3a038  h116_prefix_transfer_verify.py
2f4822b7a1baf0c9d0065941a5b81bfe11228012978c8afa5fa2aa8ab78a58e2  PREFIX_TRANSFER_RESULT.json
```

```text
python3 h116_prefix_transfer_control.py PREFIX_TRANSFER_PROTOCOL.json
python3 h116_prefix_transfer_verify.py PREFIX_TRANSFER_PROTOCOL.json PREFIX_TRANSFER_RESULT.json
```
