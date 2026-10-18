#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: s1a-ell8209-q29
Session `ECBS1h4d33a6cf0f60`, status `complete`, 18432 records; spec `ECS1hbbcf385ccb3e` sha256 `bbcf385ccb3e627b…`; binary sha256 `ee0af723cf6c1064…`, git commit `42ab2c57490a1eb4ee791e3b8d35563fce61b0e4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell=8209, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets.
Arms: rho-neg = rho.negation (reference), rho-plain = rho.plain (baseline), rho-neg-aa = rho.negation (control). Workloads: 6 curves × 1024 `planted_shared` targets, target seed 2026101801, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 18432}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/s1a-ell8209-q29/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [crater], [icv1-fp29-t47-14392892], [1024], [0.9778], [0.0158], [0.056 / 3.058],
[rho-neg], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [0.9517], [0.0149], [0.077 / 2.659],
[rho-neg], [floor1], [icv1-fp29-t47-1f934956], [1024], [0.9749], [0.0155], [0.062 / 2.726],
[rho-neg], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [0.9793], [0.0153], [0.081 / 2.790],
[rho-neg], [floor2], [icv1-fp29-t47-522e6830], [1024], [0.9730], [0.0152], [0.069 / 3.122],
[rho-neg], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [0.9469], [0.0149], [0.086 / 2.784],
[rho-neg-aa], [crater], [icv1-fp29-t47-14392892], [1024], [0.9778], [0.0158], [0.056 / 3.058],
[rho-neg-aa], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [0.9517], [0.0149], [0.077 / 2.659],
[rho-neg-aa], [floor1], [icv1-fp29-t47-1f934956], [1024], [0.9749], [0.0155], [0.062 / 2.726],
[rho-neg-aa], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [0.9793], [0.0153], [0.081 / 2.790],
[rho-neg-aa], [floor2], [icv1-fp29-t47-522e6830], [1024], [0.9730], [0.0152], [0.069 / 3.122],
[rho-neg-aa], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [0.9469], [0.0149], [0.086 / 2.784],
[rho-plain], [crater], [icv1-fp29-t47-14392892], [1024], [1.3298], [0.0218], [0.056 / 3.702],
[rho-plain], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [1.3391], [0.0219], [0.105 / 4.031],
[rho-plain], [floor1], [icv1-fp29-t47-1f934956], [1024], [1.3277], [0.0220], [0.066 / 4.107],
[rho-plain], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [1.3177], [0.0210], [0.075 / 3.941],
[rho-plain], [floor2], [icv1-fp29-t47-522e6830], [1024], [1.2995], [0.0215], [0.092 / 4.249],
[rho-plain], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [1.3285], [0.0211], [0.082 / 3.695],
)