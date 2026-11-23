# Retained Weil weighted decomposition study entry

The original entry below is retained verbatim for provenance. It is not admitted to the benchmark table: the study does not yet supply the validation gate artifacts and protocol-matched rho reference. Its stage counts and unqualified timing remain in the original study.

| date | entry | measured | headline (as reported) | host | study |
|:--|:--|:--|:--|:--|:--|
| 2026-10-19 | [`20261019-weil-weighted-decomposition`](20261019-weil-weighted-decomposition/) | Native weighted slots directly on cubic Weil restrictions; 12 synthetic curves, two seeds, c=1/2/4 and c=1 A/A | 36/36 recoveries verified through group order 1,027,286,801; p=1009 c=4 needed 1152/1177 residuals vs c=1 3044/3063. Primary replay byte-identical; PARI fixture checks passed. Stage counts only; complete S, IC/rho and end-to-end speedup null; no external fastest-tool comparison | AMD EPYC 9V74 KVM, 4-CPU quota, 16 GiB, L0 concurrent; no admitted wall time | [study](../research/weil_weighted_decomposition_20261019/), [result PDF](20261019-weil-weighted-decomposition/REPORT.pdf), [larger-run PDF](20261019-weil-weighted-decomposition/RUN_REPORT.pdf) |
