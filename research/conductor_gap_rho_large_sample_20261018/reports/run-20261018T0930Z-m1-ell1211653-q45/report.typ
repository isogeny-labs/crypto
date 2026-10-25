#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: m1-ell1211653-q45
Session `ECBS1h946e576a470c`, status `complete`, 1024 records; spec `ECS1h264ab8fe07bc` sha256 `264ab8fe07bcd17d…`; binary sha256 `ee0af723cf6c1064…`, git commit `d290558b2657bd320ea2a9964a91238924619763`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell=1211653 (2^20..), D=-19, hard kernel, q 45 bits: crater vs 3 floors, 128 shared targets.
Arms: rho-neg = rho.negation (reference), rho-neg-aa = rho.negation (control). Workloads: 4 curves × 128 `planted_shared` targets, target seed 2026101811, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 1024}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/m1-ell1211653-q45/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [crater], [icv1-fp45-t6688791-5bbdfc5b], [128], [0.9045], [0.0450], [0.018 / 2.530],
[rho-neg], [floor(seed1)], [icv1-fp45-t6688791-954cd116], [128], [0.9180], [0.0444], [0.097 / 3.325],
[rho-neg], [floor(seed2)], [icv1-fp45-t6688791-8d4d0b3f], [128], [0.9713], [0.0462], [0.020 / 2.786],
[rho-neg], [floor(seed3)], [icv1-fp45-t6688791-23a17747], [128], [0.9608], [0.0407], [0.092 / 2.028],
[rho-neg-aa], [crater], [icv1-fp45-t6688791-5bbdfc5b], [128], [0.9045], [0.0450], [0.018 / 2.530],
[rho-neg-aa], [floor(seed1)], [icv1-fp45-t6688791-954cd116], [128], [0.9180], [0.0444], [0.097 / 3.325],
[rho-neg-aa], [floor(seed2)], [icv1-fp45-t6688791-8d4d0b3f], [128], [0.9713], [0.0462], [0.020 / 2.786],
[rho-neg-aa], [floor(seed3)], [icv1-fp45-t6688791-23a17747], [128], [0.9608], [0.0407], [0.092 / 2.028],
)