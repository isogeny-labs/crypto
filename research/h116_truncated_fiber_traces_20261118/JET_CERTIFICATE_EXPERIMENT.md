# H116 fixed low-jet certificate experiment

Status: excluded-fixture theorem/control checkpoint. No gated H116 target or
label file was read, and this is not an ECDLP runtime claim.

## Question

Does the complete H116 recurrence actually require the full degree-`r`
rational map, or is a fixed prefix sufficient once the prefix is supplied with
a checkable curve-equation certificate? [pcc:unverified]

## Formal result

The Lean predicate `PowerSeriesPrefixEq n f g` means equality after
`PowerSeries.trunc n`, hence equality in degrees strictly below `n`. The new
endpoint

```text
BostanJetRecurrence.of_normalizedRationalMapJetCertificate_zmod
```

proves, for every prime `p>19`, that the complete seven-step recurrence follows
from exactly these premises. [pcc:unverified]

1. the reversed denominator has constant coefficient one;
2. `D*U=N` below degree eleven;
3. the cleared rational-map curve identity is true below degree ten.

The proof uses Mathlib truncation laws for multiplication, powers and
coefficient extraction. Differentiating the quotient congruence consumes one
coefficient, producing the first-order equation below degree ten.
Differentiating once more consumes one coefficient, producing equation (15)
below degree nine. Its coefficients zero through eight contain both base
identities and all five indexed recurrence steps. Unit-series cancellation is
proved by multiplication with the formal inverse, so it loses no precision.
[pcc:unverified]

The repository proof gate reports:

```text
PASS  60 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

No heartbeat/recursion limit, `sorry`, custom axiom, `native_decide` or copied
curve group law was added. Every audited theorem reports only `propext`,
`Classical.choice` and `Quot.sound`. [pcc:unverified]

## Excluded-fixture replay and controls

`h116_jet_certificate_verify.py` consumes the already excluded `p=31`, `r=37`
Vélu control and evaluates only the theorem's finite prefixes. It uses dense,
truncated polynomial arithmetic and records the number of coefficient slots
processed. The retained output is `JET_CERTIFICATE_CONTROL.json`.

Three mutations distinguish the precision boundaries:

- numerator coefficient two is inside both checks and must be rejected;
- numerator coefficient ten is the quotient's derivative guard: the
  degree-eleven quotient check must reject it while the curve identity below
  degree ten remains unchanged;
- numerator coefficient eleven is outside both prefixes and must leave both
  checks unchanged.

All controls passed. The replay obtained

```text
U mod X^11 = [1,0,20,0,13,7,2,4,8,14,5]
recurrence  = [1,0,20,0,13,7,2,4,8]
```

and the first nine entries agree. [pcc:unverified]

| exact measure | value |
| --- | ---: |
| quotient input precision | 11 coefficients per series |
| curve-identity precision | 10 coefficients |
| certified recurrence precision | 9 coefficients |
| low-jet `N,D` input slots | 22 |
| full excluded-map `N,D` input slots | 75 |
| dense verification additions | 1,106 |
| dense verification multiplications | 1,109 |
| dense verification inversions | 0 |

The operation counts cover the positive quotient construction, quotient
relation and cleared identity in the standalone replay. They count dense
coefficient slots even when a coefficient is zero, so they are deterministic
upper-bound-style implementation counts, not wall-clock timings.
[pcc:unverified]

Reproduction:

```text
python3 -m py_compile h116_jet_certificate_verify.py
python3 h116_jet_certificate_verify.py MAP_CERTIFICATE_CONTROL.json |
  diff -u JET_CERTIFICATE_CONTROL.json -
```

Evidence SHA-256 values:

```text
3151557ecc5c78a2651beed94b758e6efd5323ff19590d633e187c01d7e941c0  h116_jet_certificate_verify.py
95f2860bac5d98a1c3c7d1e5485b0c907cd45467a02a98ed70b3536e9e197ded  JET_CERTIFICATE_CONTROL.json
7d77482bed908cc294f8a5828a7227981df487ba66ba6e855ffc8b7789295bb9  lean/H116FiberTrace.lean
```

## Evidence boundary

This checkpoint proves fixed-size **verification**, not fixed-size
construction. The excluded replay first obtained the full degree-37 Vélu map;
the 22 input slots were then selected from it. No current theorem or experiment
constructs the correct eleven numerator/denominator coefficients from a generic
prime-field curve and kernel in sub-square-root time, and the low-jet
certificate alone does not prove that an arbitrary supplied prefix has the
intended kernel semantics. Those are the next construction and transfer
obligations. Consequently this result is not evidence of a subexponential
generic-prime-field ECDLP algorithm. [pcc:unverified]
