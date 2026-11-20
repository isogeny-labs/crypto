# H116 rational separation capacity source gate

This gate seals both implementations before the first affected panel run for
protocol `H116-RATIONAL-SEPARATION-CAPACITY-001`.

```text
d58dd2215ca4922dbfc00480dabc218a59ec78e9afcd63bed5257bd04a9ce4fa  RATIONAL_SEPARATION_CAPACITY_PROTOCOL.json
efaafe9bf3107a0b7d643367eaca8543de473ee3e18b0ca5236309422b14eda6  h116_rational_separation_capacity.rs
70e4edd6b81426edf95f722172352d1a5bb6c6c9dea0965d91564c1a4cc9b75d  h116_rational_separation_capacity_verify.py
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
469ef0cd5ca96329a9543b4f1afbc55f0c58027e927151dbe56429917df2126c  h116_pointwise_charpoly_verify.py
```

The producer compiled with
`rustc 1.100.0 (0ef1bc386 2026-11-09)` using
`--edition=2021 -O -D warnings`; its selftest passed.  The independent verifier
compiled under Python 3.12.14 and its separate `F_3` polynomial selftest
passed.  `rustfmt` is not installed, so no formatting-tool claim is made.

Three pre-data failures are retained:

```text
d91ca1365626c49b9a5abc0bc77ed61b028d15a6e441d1ebb13fd9eb256feb46  RATIONAL_SEPARATION_CAPACITY_FIRST_BUILD_FAILURE.txt
b76630ce224ec5c97287876c7c5e98658bafe424b6f021ac81e31c17912e414a  RATIONAL_SEPARATION_CAPACITY_SECOND_BUILD_FAILURE.txt
6ae09c01dc30df2a124bee02373d32e14e21226d933420f7e3a24f0ac3da4fe3  RATIONAL_SEPARATION_CAPACITY_FIRST_SELFTEST_FAILURE.txt
```

They record a positional TSV formatting error, two missing iterator-count type
annotations, and a degree-one-only unit-test fixture issue.  No panel command
had run and no affected result existed.  The final changes did not alter any
frozen sample, polynomial, check, control, or acceptance rule. [pcc:unverified]
