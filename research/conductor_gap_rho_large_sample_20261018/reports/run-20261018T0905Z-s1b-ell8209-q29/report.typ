#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: s1b-ell8209-q29
Session `ECBS1hb44547d4413b`, status `complete`, 18432 records; spec `ECS1h460301017b69` sha256 `460301017b69679b…`; binary sha256 `ee0af723cf6c1064…`, git commit `42ab2c57490a1eb4ee791e3b8d35563fce61b0e4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell=8209, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets.
Arms: rho-neg = rho.negation (reference), rho-plain = rho.plain (baseline), rho-neg-aa = rho.negation (control). Workloads: 6 curves × 1024 `planted_shared` targets, target seed 2026101803, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 18432}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/s1b-ell8209-q29/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [crater], [icv1-fp29-t47-14392892], [1024], [0.9631], [0.0153], [0.064 / 3.066],
[rho-neg], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [0.9621], [0.0153], [0.038 / 3.087],
[rho-neg], [floor1], [icv1-fp29-t47-1f934956], [1024], [0.9793], [0.0155], [0.040 / 2.691],
[rho-neg], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [0.9926], [0.0156], [0.037 / 3.016],
[rho-neg], [floor2], [icv1-fp29-t47-522e6830], [1024], [0.9691], [0.0157], [0.088 / 2.683],
[rho-neg], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [0.9389], [0.0152], [0.053 / 2.823],
[rho-neg-aa], [crater], [icv1-fp29-t47-14392892], [1024], [0.9631], [0.0153], [0.064 / 3.066],
[rho-neg-aa], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [0.9621], [0.0153], [0.038 / 3.087],
[rho-neg-aa], [floor1], [icv1-fp29-t47-1f934956], [1024], [0.9793], [0.0155], [0.040 / 2.691],
[rho-neg-aa], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [0.9926], [0.0156], [0.037 / 3.016],
[rho-neg-aa], [floor2], [icv1-fp29-t47-522e6830], [1024], [0.9691], [0.0157], [0.088 / 2.683],
[rho-neg-aa], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [0.9389], [0.0152], [0.053 / 2.823],
[rho-plain], [crater], [icv1-fp29-t47-14392892], [1024], [1.3215], [0.0205], [0.098 / 4.093],
[rho-plain], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [1.3509], [0.0216], [0.087 / 4.938],
[rho-plain], [floor1], [icv1-fp29-t47-1f934956], [1024], [1.3263], [0.0210], [0.087 / 4.074],
[rho-plain], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [1.3198], [0.0207], [0.068 / 4.111],
[rho-plain], [floor2], [icv1-fp29-t47-522e6830], [1024], [1.3566], [0.0221], [0.072 / 4.121],
[rho-plain], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [1.3318], [0.0219], [0.029 / 3.755],
)