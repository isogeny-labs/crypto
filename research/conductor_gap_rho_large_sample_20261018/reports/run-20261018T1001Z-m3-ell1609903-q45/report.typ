#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: m3-ell1609903-q45
Session `ECBS1h51432cd8311e`, status `complete`, 1024 records; spec `ECS1h64a1f9d0e994` sha256 `64a1f9d0e994ce39…`; binary sha256 `ee0af723cf6c1064…`, git commit `f300174628b4fbd71ba925d8edae0bc105e6e00b`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell=1609903 (2^20..), D=-19, hard kernel, q 45 bits: crater vs 3 floors, 128 shared targets.
Arms: rho-neg = rho.negation (reference), rho-neg-aa = rho.negation (control). Workloads: 4 curves × 128 `planted_shared` targets, target seed 2026101813, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 1024}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/m3-ell1609903-q45/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [crater], [icv1-fp45-t9290211-d1c818b3], [128], [0.9588], [0.0476], [0.068 / 3.218],
[rho-neg], [floor(seed1)], [icv1-fp45-t9290211-20343fd2], [128], [0.9133], [0.0443], [0.050 / 2.492],
[rho-neg], [floor(seed2)], [icv1-fp45-t9290211-56b81cc9], [128], [0.8909], [0.0413], [0.060 / 2.349],
[rho-neg], [floor(seed3)], [icv1-fp45-t9290211-931d8f76], [128], [0.8808], [0.0379], [0.090 / 1.789],
[rho-neg-aa], [crater], [icv1-fp45-t9290211-d1c818b3], [128], [0.9588], [0.0476], [0.068 / 3.218],
[rho-neg-aa], [floor(seed1)], [icv1-fp45-t9290211-20343fd2], [128], [0.9133], [0.0443], [0.050 / 2.492],
[rho-neg-aa], [floor(seed2)], [icv1-fp45-t9290211-56b81cc9], [128], [0.8909], [0.0413], [0.060 / 2.349],
[rho-neg-aa], [floor(seed3)], [icv1-fp45-t9290211-931d8f76], [128], [0.8808], [0.0379], [0.090 / 1.789],
)