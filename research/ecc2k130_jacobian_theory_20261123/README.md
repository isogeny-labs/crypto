# ECC2K-130 Jacobian theory

[Report](REPORT.md) · [PDF](report.pdf) · [diagram](routes.svg) · [proof map](PROOF_MAP.md)

Continued work: [round02 report](round02/REPORT.md), [PDF](round02/report.pdf)
and [residue census](round02/local_check.txt). The new written necessity
result is [PCC-20261123-b7901e](../../pcc/claims/PCC-20261123-b7901e.json),
standing **unverified**: every F2-isogeny from the natural A to a principally
polarized representative has degree divisible by146505763881528721.
This constrains genus130 transfers; it is not a running-time bound or a
nonexistence result for larger-genus carriers.

Written conclusions, all with PCC standing **unverified**:

- [PCC-20261123-f0df24](../../pcc/claims/PCC-20261123-f0df24.json): the F2-isogeny class admits a principal polarization, using a concrete inert residue prime and Howe's theorem.
- [PCC-20261123-6e3ede](../../pcc/claims/PCC-20261123-6e3ede.json): a genus130 Jacobian in that class cannot retain integral zeta131; maximal F2-endomorphism order is excluded.
- [PCC-20261123-7b7fc5](../../pcc/claims/PCC-20261123-7b7fc5.json): zeta131 is absent from the universal Frobenius order, so the automorphism argument does not exclude all representatives.

No curve of genus130–300 or computable correspondence has been constructed.
The task remains open in [draft PR5079](https://github.com/isogeny-labs/crypto/pull/5079)
and the [existing construction problem](../../problems/koblitz-modular-carrier-transfer.md).

Replay exact arithmetic (C++17, no dependencies):

```sh
cd research/ecc2k130_jacobian_theory_20261123
g++ -std=c++17 -Wall -Wextra -Werror -O2 check.cpp -o /tmp/ecc2k130-check
/tmp/ecc2k130-check | diff -u check.txt -
```

The optimized replay and an UndefinedBehaviorSanitizer replay matched.
This is a producer check, not a signed execution proof or a Lean geometric proof.
`Arithmetic.lean` is explicitly uncompiled; `lean-attempt.txt` records missing Lean.
The required root Cargo test attempt could not execute because Cargo is absent
(`root-test-attempt.txt`). No Rust production sources changed.

Render the four-page PDF with Typst0.15.1:

```sh
python3 -c "import typst; typst.compile('report.typ', output='report.pdf')"
```

Python is used only for document rendering. The arithmetic is native C++.
The PDF was opened as page images and visually inspected on all four pages.
