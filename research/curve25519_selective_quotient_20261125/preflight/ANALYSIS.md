# Native analysis build and reproducibility receipt

- Analyzer source revision: `66b3c839d02afbc372a61083e3a1d43e07386b7f`.
- Disposable compile-repair overlay head:
  `6deaa98307657bd03a81110cc46d280ca349434d`.
- Analyzer release binary SHA-256:
  `b3cfdad950c807e33e4e60affe60ff02b6be6de3282a6ad014dec53395053e07`.
- Bootstrap: 10,000 paired resamples with frozen seed `20261125`.

Two consecutive analyzer executions were byte-identical:

| output | SHA-256 |
|:--|:--|
| `analysis.json` | `c8bd28f47e1b49bdb946a5bae67286f9eeb61d793ec2ad6d5f88a205384c12a2` |
| `paired.csv` | `387f36a4f53c96bc315b56d8d020124ef83b79288defb25e0b01d54103fb9d48` |
| `figure.svg` | `95564269fff1a4b890f2b070fb20f57ea1e9c4a3210c9138de477c67b2dbc852` |
| `experiment.export.json` | `7cfb8f08b51cd0cd63539a5af740d0bedea9938294c9b402be4434c8ad6908f3` |

Inkscape rendered the 900x520 PNG with SHA-256
`4b0c9944459572090d299f81c929d86d4c739301d4d702ef3b465716ca9e586e`.
Typst 0.13.1 archive
`7d214bfeffc2e585dc422d1a09d2b144969421281e8c7f5d784b65fc69b5673f`
compiled the inspected two-page A4 PDF with SHA-256
`b3280270bf241cb9ea9288eefd8dc23edf6f793562a5450ee1ecc62c7b8a96eb`.
`pdfinfo` reports `JavaScript: no`.
