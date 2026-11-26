# Frozen protocol: matched closure degree of binary `S_3` edge lifts

Frozen 2026-11-26 before any closure scan. The support census in `README.md`
preceded this protocol and is not closure evidence.

## Question

Does lifting a mixed product edge (`ac` or `bc`) reduce closure refutation
degree or complete matrix work relative to the standard `ab` lift, with the
same draws, variables, equations, degree cap, and resource caps?

## Arms

1. `chain_s3`: original cubic chain.
2. `quad_ab`: `w=x_a x_b`, the existing generic quadratisation.
3. `quad_ac`: `w=x_a x_c`, candidate C1.
4. `quad_bc`: `w=x_b x_c`, matched alternative.
5. `e2_aux`: existing line-norm quadratisation.

The three `quad_*` arms must have identical variable and equation counts per
draw. A mismatch invalidates that draw. `chain_s3` and `e2_aux` are contextual
controls, not size-matched claims against every arm.

## Cells and draws

Reuse the deterministic instances and exact undecomposability/planted checks
of `research/representation_leads_20261109/PROTOCOL.md`:

```text
(n,m,ell) in {(11,3,3), (11,3,4), (13,3,3), (11,4,3)}
(a,b) in {(0,1),(1,1)}
```

Per cell: 16 exhaustively undecomposable targets and four planted satisfiable
targets. The export must be regenerated only after `quad_ac` and `quad_bc` are
added to `Rep::PROTOCOL`, then committed and pinned by SHA-256 before a scan.

## Instrument and caps

Use the existing native `examples/replead_closure.rs` and its dense/sparse
closure engines. Dense scan degrees 2 through 6 while columns are at most
70,000; for an unbuilt degree, use the sparse engine with at most
1,500,000,000 entries. Preserve censored rows as unmeasured. Arm order rotates
per draw; analysis masks arm names until the decision is computed.

Record per arm and degree:

- smallest closure refutation degree `D*`;
- columns actually built, rank, products, and nonzero entries;
- wall/CPU time and peak RSS as practicality data, not the primary metric;
- planted-assignment evaluation and any false refutation.

## Decision

- **Degree effect:** a candidate has strictly smaller `D*` than `quad_ab` on
  at least 12 of 16 unsatisfiable draws in every `m>=4` cell, with no cell in
  which `quad_ab` is smaller on at least 12 of 16. Advance it to the `ell=7`
  scale gate.
- **Support-only effect:** `D*` is equal on at least 12 of 16 draws in every
  cell, but the candidate uses fewer built nonzeros or products on at least 12
  draws in every `m>=4` cell. Report only the measured component reduction;
  do not call it a solving-degree improvement.
- **No effect:** equal `D*` and no support-work rule above.
- **Mixed/censored:** any other result, including more than four censored draws
  in a cell. No winner.
- **Void:** any planted draw is refuted, any planted assignment fails, an
  allegedly matched count differs, or the regenerated system digest differs
  from the pre-run pin.

## Blocker

The inherited protocol requires a working checkpoint upload before dispatch.
No closure run was launched in this authoring environment; no checkpoint token
was available. A local uncustodied scan does not discharge this protocol.
