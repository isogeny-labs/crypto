# Analyzer receipt

- Native analyzer source revision: `ab1a18f2f9`.
- Disposable compile-repair analyzer head:
  `c8414c3f4f2e5d49e52f5a872f07f9f3ec95bb20`.
- Analyzer binary SHA-256:
  `36baef79f42abce93149e183a88854bd942edbcd5165f225a56babe2622886ad`.

Two executions over the retained rows produced byte-identical artifacts:

- `analysis.json`: `3ce66e2eae51558ee299c627d57eee2026c52ac9263491993311868facdc5c7c`
- `experiment.export.json`: `69300f7e8cd8bd19c0ab3bfe24f7ba2a4361276b3ce9d6c9d8f42c1a4229f9da`
- `figure.svg`: `33036d45158813729bf607ed75e4e5d6a25789f180e569e1de762e7f1dc19692`
- `paired.csv`: `381d8f7490502a9d56328d1bbc2a29cee8cd8fb3f34f5638d75b7dec7a29b2fd`

The rendered PNG has SHA-256
`f8b839934b27bcb536efb5e7da1444395dea15dfba3eaa3ea952d969994cfda1`.
The analyzer accepts only the frozen `d=7` stage and enforces exact pairing,
row counts, planted correctness, self-test status, and predecessor
reproduction before reporting cost gates.
