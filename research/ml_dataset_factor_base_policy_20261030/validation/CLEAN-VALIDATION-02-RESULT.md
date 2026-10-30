# Repaired-source clean validation 02

The isolated restore-policy lint repair was validated against clean source
commit `c41bec150e16e4c4f9a00634002b633b821de262`. The complete source
package and its Git bundle had already passed fresh S3 readback. Its sole
change from package01 is patch
`e3852ec4bdf5b256a7a0360b085fc0ffdb02dc8ec9dcc3f7248b0c8b5bd28cbd`
in `src/ml_dataset/restore.rs`; source inventory, file modes, and all other
content hashes were unchanged.

The locked release build passed with clean Git, exact source preflight and
readback, and retained binary hashes. The build receipt SHA-256 is
`0f82585b6203ac020a0561234a7a9b779ceb7dda29724fd951b811b92edbac7a`;
the `crypto` binary SHA-256 is
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.
The build archive is `22728e52095abd23058a4b9d7d9b012c32a7dead544fcf6c64d0a070e2050b7a`
in the content-addressed S3 store, with fresh download byte comparison.

The prospective sequential validation contract completed without retries:

| Phase | Exact observed result | S3 archive SHA-256 |
| --- | --- | --- |
| Strict maintained Clippy | command0, wrapper0; source and binary readback true | `c416c383f9f55ef80fdfe34351e7ee22691ad2ef5474931a35963fbde802081c` |
| Native controller fixture | 31 passed, 0 failed, 0 ignored; command0, wrapper0 | `089a6f1513b72827f08e9a2cb1e1652fcf0ad20acb48b1ac2625fca0d6a5e453` |
| Dataset library scope | 479 passed, 0 failed, 2 ignored; command0, wrapper0 | `1df74c6302668296889c4d56f3f482ea19d7b6ec5d1598e246fdf8068ec1bae6` |
| Retained CLI surface | Four commands passed, zero solver calls | `5143223748939268c558f588cce3ccd607f5572118369e9c59761decf9125883` |

The dataset test log also contains the known FIFO safety child's separate
one-test passing summary. It was checked separately and not added to the
479-test parent count. Each archive above has a committed fresh S3 download
byte-comparison receipt. The four CLI commands checked `dataset --help`,
`dataset start --help`, `dataset capabilities`, and controller `--help`;
they establish command availability, not solver execution.

These are SOFTWARE readiness checks for the isolated source closure. The
seven prior native plan commands remain unbound preparation with 0 generated
cells. The next admission dependency is a measured native runtime-preflight
collector and frozen official child/umbrella protocols; no runtime estimate is
inferred from these build or help-command durations. The 26-cell workflow,
official export, V2 API fixtures, and full-library completion are separate
checks. Verified cloud dataset cells: 0.
