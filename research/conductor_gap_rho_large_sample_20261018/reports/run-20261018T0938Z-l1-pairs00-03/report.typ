#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: l1-pairs00-03
Session `ECBS1h9fa6d138102f`, status `complete`, 192 records; spec `ECS1h45fae6bcee1a` sha256 `45fae6bcee1ad824…`; binary sha256 `ee0af723cf6c1064…`, git commit `d290558b2657bd320ea2a9964a91238924619763`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 0..3 of cm_orbits: crater vs floor vs crater twins, 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101821, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/l1-pairs00-03/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair0-crater], [icv1-fp53-t33555417-7e2a3b7a], [16], [1.0323], [0.1388], [0.190 / 2.040],
[rho-neg], [pair0-crater-twin(u=3)], [icv1-fp53-t33555417-b31145d8], [16], [0.9557], [0.1212], [0.337 / 1.976],
[rho-neg], [pair0-floor], [icv1-fp53-t33555417-ade0b980], [16], [1.1499], [0.1394], [0.201 / 1.982],
[rho-neg], [pair1-crater], [icv1-fp53-t33568641-0d2a2a59], [16], [0.9675], [0.1361], [0.085 / 2.070],
[rho-neg], [pair1-crater-twin(u=3)], [icv1-fp53-t33568641-498ed63c], [16], [0.9651], [0.0890], [0.432 / 1.626],
[rho-neg], [pair1-floor], [icv1-fp53-t33568641-272bd5be], [16], [1.0944], [0.1629], [0.042 / 2.543],
[rho-neg], [pair2-crater], [icv1-fp53-t33600281-7acbe863], [16], [0.7606], [0.0892], [0.207 / 1.397],
[rho-neg], [pair2-crater-twin(u=3)], [icv1-fp53-t33600281-aad96b3d], [16], [1.2101], [0.1224], [0.558 / 2.283],
[rho-neg], [pair2-floor], [icv1-fp53-t33600281-8389a3a2], [16], [0.9645], [0.1082], [0.117 / 1.545],
[rho-neg], [pair3-crater], [icv1-fp53-t33623877-90323303], [16], [1.0109], [0.1425], [0.099 / 2.025],
[rho-neg], [pair3-crater-twin(u=3)], [icv1-fp53-t33623877-ee2584c0], [16], [0.9161], [0.0612], [0.514 / 1.336],
[rho-neg], [pair3-floor], [icv1-fp53-t33623877-87bc6afd], [16], [0.9100], [0.1063], [0.148 / 1.786],
)