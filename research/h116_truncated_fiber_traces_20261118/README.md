# H116 truncated Frobenius-fiber traces

H116 tests a component needed by H115: whether the fixed low-order moments of
the degree-`r` fiber of `phi = V - 1` can be recovered from a short Laurent jet
of the `x`-coordinate map, without constructing or enumerating that fiber.

The proposed identities are frozen as hypotheses until proved and independently
checked. In particular, an efficient moment evaluator is not by itself an ECDLP
algorithm. The separate digit-predicate and decoder obligations remain open.
[pcc:unverified]

## Requested scope and evidence plan

| Requirement | Frozen evidence |
| --- | --- |
| A different non-generic lead | Relative fiber traces/residues of the special endomorphism `V-1`, not a generic walk or relation collection |
| Rigorous algorithm | Native candidate with exact field-operation counters and a direct degree-`r` enumeration reference |
| Controls | identical A/A reference, planted polynomial positive control, characteristic-bound rejection control, coordinate isomorphisms and shuffled target order |
| Scaling | five deterministically selected public toy curves with `p > 19`, all nonidentity targets and three generator labels |
| Independent check | separate cryptanalysis verifier source and tamper tests before scoring |
| Formal proof | Lean identity layer for logarithmic derivatives/Newton sums and residue extraction, with the Frobenius-jet and cost premises kept explicit if their library instances are unavailable |
| Subexponential claim | admissible only if the moment construction, one fixed exact digit predicate, the complete decoder and every setup/recovery cost are all bounded by `r^o(1)` |

## Frozen algebraic target

Let a separable odd-degree isogeny have coordinate map

```text
phi(x,y) = (R(x), y R'(x) / a),   R = N/D,
F_u = N - uD.
```

For a non-2-torsion target `Q=(u,v)`, H116 tests the following coefficient
identities, with fiber points counted with their algebraic multiplicities:

```text
X_j(Q)-X_j(Q0)
  = [x^(-j-1)] d/dx log((R-u)/(R-u0)),

Y_j(Q)
  = a v [x^(-1)] x^(j-1)/(R-u).
```

For `phi=V-1` on `E/F_p`, the formal endomorphism identity

```text
V - 1 = [t-1] - pi,    pi(z)=z^p
```

predicts that the required fixed-order jet is computable from a truncated
formal-group calculation. For the first implementation, the Bostan et al.
normalized-isogeny recurrence is used only when its denominators are units;
the frozen cohort requires `p > 2J+3` with `J=8`. The exact general small-
characteristic formal-group construction is an open successor, not silently
substituted into this run. [pcc:unverified]

## Cost boundary

The candidate target for fixed `J` is `O(M(J) log p)` base-field operations,
including construction of the scalar `[t-1]` jet. The direct control performs
`Theta(rJ)` extension-field work after constructing the kernel generator. The
run records cold setup, genuinely reusable setup, per-target work, recovery and
verification separately. No stage ratio will be reported as an end-to-end
ECDLP ratio. [pcc:unverified]

Files are added in this order: literature and protocol freeze, producer and
Lean source, independent verifier binding, checkpointed EKS execution, raw
export, report/diagram/PDF, and only then any claim record.
