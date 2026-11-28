# Native analysis receipt

- Analyzer source commit: `65fb7db2b4` on the task branch.
- Disposable compile-repair overlay head:
  `a1af7b241870d1dc29d4145b699023e64c05cd17`.
- Analyzer release binary SHA-256:
  `7834285c35b635fb21a00d44dc80249bec16865a554647f98230d03474d8f531`.
- Bootstrap: 10,000 paired resamples, frozen seed `20261125` at `d=19`.

Two consecutive analyzer executions were byte-identical for all four native
outputs:

| output | SHA-256 |
|:--|:--|
| `analysis.json` | `9be74149d58a7d85eb079a59150ea6d715870e6ac27f57ba10915d66596dfa39` |
| `paired.csv` | `d97775fd272a46a2eba1bb832a881f4ff8129c7a727bac6f9a8109ad219482b3` |
| `figure.svg` | `b97dde24d16a99283df6c3c4d52d2d2b6bb2fd0c3ea0c91fb4806c4925e5a497` |
| `experiment.export.json` | `7ed28b3326cac62e8b58408e784d2fe397ad4ba5a7b16870af497771e29ac4fb` |

The Inkscape-rendered PNG SHA-256 is
`3c4e33b78a631ebcedfe538f2f69805734a22e2a763b867a4a140005a232c83b`.
The inspected two-page A4 Typst PDF SHA-256 is
`b18d6b97c69346fb5f15110918acc434fa8602e2b8020a737111a8e925bf112f`;
`pdfinfo` reports `JavaScript: no`.
