#set page(paper: "a4", margin: 19mm)
#set text(font: "Liberation Sans", size: 9pt)
= Quartic twist two-line A/A repeat
UTC start `2026-10-22 05:52:55.464`; same native producer and frozen protocol as the first mixed run; Intel Xeon 6975P-C, 32 logical CPUs; 16 Rayon threads; L0 shared host.

All 42 records and 126 IC answers succeeded again. Raw SHA-256: `6338077c4dd2218c33c135a834622bd5cf74c6f95e93418ea1d61ae6f445ca1a`. The non-timing raw diff is empty, and the native summary is byte-identical to the first. The independent second audit replays every target's hit count, every selected group relation, all encoded matrices and scalar answers. The figure is the identical count graph reconstructed from the matched raw data.

#image("quartic_mixed_lines_v1.png", width: 100%)

This is a determinism control for the operation counts, not an isolated wall-time test. See `README.md` for the command, links and limitations, and `QUARTIC_TWIST24_REPORT.md` for the conclusion.
