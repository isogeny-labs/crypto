# N83 a=0 full-D18 public fixture export — 2026-11-14 UTC

The ordered first-pair D18 four-summand exporter now addresses each point in
the sealed primary public corpus. The source commit
`cfa916b8d169962c9b906828afe0899136865d69` produced the first two
natural-query XCNFs from the same 5,418,738-point full-D18 base, with 32,643
folded columns. Its binary SHA-256 was
`93df8af0693ece07978bed0835d4933b0f2fd74325f561eeccdb82d668ce6f74`.
The input object SHA-256 was
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`;
the public corpus SHA-256 was
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`.

| Fixture | Public point `(x,y)` | XCNF SHA-256 | Bytes |
| --- | --- | --- | ---: |
| 0 | `(1834913121178197242856238, 8797964190732846213981377)` | `bafed0f783ad93fcc91dcbd4c0ab45a505fc4ee3f2b0607a561e63995d0c89c3` | 50,520,608 |
| 1 | `(5809312440517013698248638, 3568442427236746854022671)` | `e0e29d928ebedee1177eefb5aa1d88d247a5e28329bffa9b93da5aefac167fe9` | 50,520,592 |

Both formulas have 329,808 variables, 2,807,501 CNF clauses, and 3,901 XOR
rows. Fixture 0 exactly reproduced the previously retained ordered-D18
natural formula. The independent byte audit checked raw lengths and SHA-256
values against these receipts, both public points against the sealed corpus,
the same points against the K2048 exports, and distinct formulas for the two
points. Its four-row result is `independent-audit.json`. Raw XCNFs and
process transcripts remain under
`/Volumes/SSD990-2/tmp/n83-a0-d18-public-fixture-20261114/`.

These are export and identity receipts. They supply common natural inputs for
the next matched solver, relation-yield, rank, and complete IC/rho panel.
Each solver attempt needs its own committed lock and admitted host; retain
failed and capped attempts in that panel.
