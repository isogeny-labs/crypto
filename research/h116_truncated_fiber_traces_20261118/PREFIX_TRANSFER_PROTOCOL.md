# H116 prefix-transfer boundary protocol

Protocol: `H116-PREFIX-TRANSFER-001`. Frozen before executing either producer
or verifier. [pcc:unverified]

## Question

Is the one-degree loss in Lean theorem `PowerSeriesPrefixEq.leading_X_inv_sq`
observable and sharp in finite fields? [pcc:unverified]

For each `p` in `{23,31,43,211,587}` and `m` in `{1,2,10,22}`, define a unit
series `f` with constant coefficient two and add a nonzero correction `c X^m`,
where `c=5m+7 mod p`. Compare:

```text
local parameters:              X f  versus  X(f+cX^m)
normalized double-pole terms:  f^-2 versus  (f+cX^m)^-2.
```

The frozen prediction is that all 20 local-parameter pairs first differ at
degree `m+1`, while all normalized inverse-square pairs first differ at degree
`m`. The boundary delta must be `-2 c f(0)^-3 mod p`. No case may be replaced.
[pcc:unverified]

## Controls

- Setting `c=0` must leave both series identical.
- Moving the same correction from degree `m` to `m+1` must move the two first
  differences to `m+2` and `m+1` respectively.
- The verifier uses only the retained JSON and the closed-form boundary law; it
  does not import the producer.

This experiment checks a tight finite-field boundary of the generic theorem.
It does not instantiate an elliptic formal group, prove a map identity or
establish an ECDLP speedup. [pcc:unverified]
