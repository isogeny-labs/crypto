# H116 prefix-transfer boundary source gate

No boundary-control result was generated before this source gate.
[pcc:unverified]

```text
05328135bbbc1592d108a227faf2d5565893c5a6511708d05b8325ce3382b374  PREFIX_TRANSFER_PROTOCOL.json
6dcd1c277def9283505bedb4fc4394e8251e62eb4fe184c5f2c49ae7bd21fd09  h116_prefix_transfer_control.py
1e6995e018d0ee8afd337d80067a4a6ff93841f30dcdd04debd5a725faa3a038  h116_prefix_transfer_verify.py
```

The producer computes truncated inverses by a coefficient recurrence and then
squares them. The verifier does not import that implementation; it checks the
retained first-difference indices and boundary coefficient against the
closed-form first variation `-2 c f(0)^-3`. [pcc:unverified]

The frozen result disposition is all-or-nothing over 20 cases. There are no
replacement cases or adaptive cutoffs. [pcc:unverified]
