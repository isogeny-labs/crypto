# Canonical coverage boundary

Schema, flow and file-hash validation passes. The protocol-aware checker
retains four findings because it assumes a rectangular3-arm by4-field by
2-seed panel (24 cells). The frozen v7 plan instead prescribes b/u/x only
at52bits and u only at56/58/60bits: exactly12 cells. All12 ran: ten verified
recoveries and two60-bit timeouts. No cells were omitted or excluded.

The unchanged frozen native analyzer and the separate strict
`../check-frontier.rs` verify this exact contract, including its stopping
rule. `../frontier-audit-v7.txt` records the passed audit and eight rejected
mutation controls; its sibling hash file binds source and raw rows. The
canonical findings remain in protocol-check.txt. This is not a full
canonical protocol-checker pass or an independent scientific validation.
