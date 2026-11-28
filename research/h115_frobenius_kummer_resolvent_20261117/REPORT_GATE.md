# H115 illustrated-report gate

Date: 2026-11-18 UTC

This gate covers documentation only. It does not execute
`h115_producer --run`, score a predicate, create a measurement packet or
alter the checkpoint/EKS requirement.

## Render

Command:

```text
./render_h115_report.sh
```

Environment:

```text
pandoc 3.1.11.1
XeTeX 3.141592653-2.6-0.999996 (TeX Live 2025/dev/Debian)
SOURCE_DATE_EPOCH=1794960000
```

The report source is `H115_REPORT.html`; `mechanism.png` is the embedded
illustration and `mechanism.svg` remains its editable source. The Lua filter
sets print page breaks and explicit table widths. The TeX header limits table
font size and spacing. Neither helper changes report claims or numeric data.

## Artifact checks

```text
PDF pages: 3
PDF page size: A4, 595.28 x 841.89 points
PDF encrypted: no
PDF JavaScript: no
```

SHA-256:

```text
H115_REPORT.html       a1d17a4882a80529fdf33cfe09ddd4f9c0f5cb7b4b305de3466da2fc96569d18
H115_REPORT_RENDER.lua e07e4dd341269bcf45e49cde59e8afb1a835b97d638361360ef1d3b8c8280229
H115_REPORT_HEADER.tex 509ab0dcfca3fd21e87f19c373e6ff9d8cc4f9aa0abcc59ea8f83cfc2bea1103
render_h115_report.sh  e472df60a41f7eeb1fc9603dbee1ddfc03efdd2851c4d99f980cd7d65e044259
H115_REPORT.pdf         d9a41ef39eed5f7701b7e52e165cbf3806e45dee289e24a04806a4e1519d8cce
```

`xdvipdfmx` generates a random PDF trailer identifier even with a fixed source
date. The render script replaces only that fixed-length identifier with the
first 128 bits of the frozen HTML source hash. This leaves all byte offsets
unchanged and makes the resulting document byte-for-byte reproducible. The
render-script and PDF hashes above are from the deterministic double-render
check.

Two consecutive renders were compared with `cmp`; the exit code was zero.

## Visual inspection

All three pages were rasterized with Poppler at 105 DPI, and page 3 was also
rasterized at 180 DPI. Manual inspection found:

- page 1: title, status, mechanism diagram, proved algebra and open cost bridge
  are legible; no clipping;
- page 2: equations, cohort table, test size and controls are legible; no
  clipping or missing symbols;
- page 3: evidence and cost tables wrap inside their columns; sources and the
  no-claim conclusion are legible; no clipping.

An earlier render was rejected because monospaced Greek glyphs were missing
and two tables clipped. It was not committed. The checked artifact uses
DejaVu Sans/DejaVu Sans Mono and explicit print column widths.

## Claim boundary

The report states that the exact representation reduction is from `r^2` to
`r`, while the frozen implementation still performs `Theta(r)` enumeration.
It explicitly records that no H115 candidate score, selected predicate,
decoder result, timing or subexponential algorithm exists.
