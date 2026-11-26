# Native analysis build and reproducibility receipt

- Analyzer source revision: `ab8d35cd9da71836037550c89e0b2125f563d4c1`.
- Disposable compile-repair overlay head:
  `aac697796c4d35ee2f494a84f85d5e429d4cc2e8`.
- Analyzer release binary SHA-256:
  `15b8eab4fce55741c3c6e0ddefb2b5969217cd00688a555823425263f2720e09`.
- Bootstrap: 10,000 paired resamples with frozen seed `2026112502`.

Two consecutive analyzer executions were byte-identical and matched the
retained evidence outputs:

| output | SHA-256 |
|:--|:--|
| `analysis.json` | `93c6248e1b29cc429b924d1f1ccbcf3b2c2b253559185ffec7f900b786d840b1` |
| `paired.csv` | `062c034a3197a445ae5a2169b69fba080f5754232ea87810ffb9a3a15cc1ddd1` |
| `figure.svg` | `05d39937587e4f309c903cafa2de841fe15d643c486de398186697a733a9b6dd` |
| `experiment.export.json` | `75864c85af0770af4600c6e26f4f1c2d8dc6b97b85335fa75557f1536e5ec3a4` |

Inkscape rendered the inspected 900x520 PNG with SHA-256
`05a0635c997d3f39d147d73729c23b02b98f7cb45b0cddbf159ecb06002cec0a`.
Typst 0.13.1 archive
`7d214bfeffc2e585dc422d1a09d2b144969421281e8c7f5d784b65fc69b5673f`
compiled the inspected two-page A4 PDF with SHA-256
`671aadb9c2308e4df962ba7bba7e84f9ce18a7d873f7334b32671e79b4971530`.
`pdfinfo` reports `JavaScript: no`.
