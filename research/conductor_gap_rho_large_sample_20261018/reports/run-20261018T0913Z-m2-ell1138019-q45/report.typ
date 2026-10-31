#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: m2-ell1138019-q45
Session `ECBS1h997ff8aee43b`, status `complete`, 1024 records; spec `ECS1h544555e68970` sha256 `544555e68970c791…`; binary sha256 `ee0af723cf6c1064…`, git commit `d290558b2657bd320ea2a9964a91238924619763`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell=1138019 (2^20..), D=-19, hard kernel, q 45 bits: crater vs 3 floors, 128 shared targets.
Arms: rho-neg = rho.negation (reference), rho-neg-aa = rho.negation (control). Workloads: 4 curves × 128 `planted_shared` targets, target seed 2026101812, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 1024}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/m2-ell1138019-q45/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [crater], [icv1-fp45-t10055507-57efa2c7], [128], [0.9669], [0.0452], [0.048 / 2.159],
[rho-neg], [floor(seed1)], [icv1-fp45-t10055507-466680bc], [128], [0.9185], [0.0473], [0.110 / 2.518],
[rho-neg], [floor(seed2)], [icv1-fp45-t10055507-53f8dd81], [128], [0.9434], [0.0461], [0.087 / 2.984],
[rho-neg], [floor(seed3)], [icv1-fp45-t10055507-561646d3], [128], [0.9423], [0.0458], [0.124 / 2.265],
[rho-neg-aa], [crater], [icv1-fp45-t10055507-57efa2c7], [128], [0.9669], [0.0452], [0.048 / 2.159],
[rho-neg-aa], [floor(seed1)], [icv1-fp45-t10055507-466680bc], [128], [0.9185], [0.0473], [0.110 / 2.518],
[rho-neg-aa], [floor(seed2)], [icv1-fp45-t10055507-53f8dd81], [128], [0.9434], [0.0461], [0.087 / 2.984],
[rho-neg-aa], [floor(seed3)], [icv1-fp45-t10055507-561646d3], [128], [0.9423], [0.0458], [0.124 / 2.265],
)