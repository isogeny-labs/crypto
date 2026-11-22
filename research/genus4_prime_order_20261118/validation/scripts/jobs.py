#!/usr/bin/env python3
"""Write out/jobs.txt: one shell command per line for the nonce plan (out/plan.json), longest first, run by
run_jobs.sh with 4 workers.  Replays use the validator's rebuild with the flags of run.sh / search.sh /
search_chunks.sh, one cell per invocation (--curves 1 --curve-seeds P=cs --targets n --target-from k).
Harness jobs (val_po) print f_irreducible per seed, the search's jacobian_order for drawn rejected seeds,
the instances (f, G, T, t) of all 120 part-B keys and the 5 Q keys, and recovered logs for the replayed cells."""
import json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import OUT
from select_check import seed_list

V = "/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/val4po"
B, H = f"{V}/bin/genus_ladder.val", f"{V}/bin/val_po"
R = f"{OUT}/replay"
plan = json.load(open(os.path.join(OUT, "plan.json")))
rc = json.load(open(os.path.join(OUT, "recompute.json")))
jobs = []  # (est_seconds, cmd)
q = plan["Q"]
jobs.append((700, f"{B} run --primes 16411 --curves 1 --curve-seeds 16411={q['cs']} --targets 1 --seeds 1 --max-h 1 --arms Q --cc 2.0 "
                  f"--genera 4 --threads 1 --rho-cap 5e8 --out {R}/q_{q['cs']}_k0.jsonl > {R}/q_{q['cs']}_k0.log 2>&1"))
for name in plan["S"]:
    P, lst = seed_list(name)
    stem = name[:-len(".jsonl.gz")]
    jobs.append((400, f"{B} run --primes {P} --genera 4 --max-h 1 --curves 100000 --targets 1 --seeds 1 --arms n --threads 1 "
                      f"--curve-seeds {P}={'/'.join(map(str, lst))} --out {R}/{stem}.jsonl > {R}/{stem}.log 2>&1"))
for c in plan["B"]:
    p, cs, k0, cc = c["p"], c["cs"], c["k0"], c["cc"]
    tag = f"b_p{p}_{c['arm']}_{cs}_k{k0}"
    jobs.append((300 if p == 32771 else 90,
                 f"{B} run --primes {p} --curves 1 --curve-seeds {p}={cs} --targets {c['targets']} --target-from {k0} --max-h 1 "
                 f"--arms x --cc {cc} --genera 4 --threads 1 --seeds 2 --sge 4,1.5 --out {R}/{tag}.jsonl > {R}/{tag}.log 2>&1"))
for a in plan["A"]:
    tag = f"a_cc{a['cc']}_{a['cs']}_k{a['k']}"
    jobs.append((120, f"{B} run --primes 16411 --curves 1 --curve-seeds 16411={a['cs']} --targets 1 --target-from {a['k']} --max-h 2 "
                      f"--arms x --cc {a['cc']} --genera 4 --threads 1 --seeds 2 --sge 4,1.5 --sge 8,2 --sge 16,3 --sge 32,4 "
                      f"--out {R}/{tag}.jsonl > {R}/{tag}.log 2>&1"))
# harness
jobs.append((60, f"printf 'irr 16411 1 2000\\nirr 32771 1 3000\\n' | {H} > {R}/h_irr.jsonl 2> {R}/h_irr.log"))
for p, v in plan["ord"].items():
    for s in v["drawn"]:
        jobs.append((100 if p == "32771" else 35, f"echo 'ord {p} {s}' | {H} > {R}/h_ord_{p}_{s}.jsonl 2> {R}/h_ord_{p}_{s}.log"))
lines = []
inst = {}
# orders of the chosen curves come from the instance rows (recompute checked r == order, h = 1)
from common import rows_of, D
for p in (16411, 32771):
    for row in rows_of(f"{D}/b_p{p}_x.jsonl.gz")[0]:
        if row["kind"] == "instance":
            inst[(p, row["curve_seed"])] = row
for (p, cs), row in sorted(inst.items()):
    for k in range(6):
        lines.append(f"inst {p} {cs} {cs + k * 1000003} {row['order']} 1 {row['r']}")
jobs.append((30, f"printf '%s\\n' {' '.join(repr(l) for l in lines)} | {H} > {R}/h_inst_all.jsonl 2> {R}/h_inst_all.log"))
for c in plan["B"]:
    p, cs, k0, cc = c["p"], c["cs"], c["k0"], c["cc"]
    row = inst[(p, cs)]
    ls = [f"inst {p} {cs} {cs + k * 1000003} {row['order']} 1 {row['r']} x {cc} 4 1.5 1,2" for k in range(k0, k0 + c["targets"])]
    tag = f"h_log_p{p}_{c['arm']}_{cs}_k{k0}"
    jobs.append((180 if p == 32771 else 60, f"printf '%s\\n' {' '.join(repr(l) for l in ls)} | {H} > {R}/{tag}.jsonl 2> {R}/{tag}.log"))
row = inst[(16411, q["cs"])]
jobs.append((700, f"echo 'inst 16411 {q['cs']} {q['cs']} {row['order']} 1 {row['r']} Q 2.0 4 1.5 1' | {H} > {R}/h_log_q_{q['cs']}.jsonl 2> {R}/h_log_q_{q['cs']}.log"))
jobs.sort(key=lambda j: -j[0])
os.makedirs(R, exist_ok=True)
with open(os.path.join(OUT, "jobs.txt"), "w") as f:
    for _, c in jobs:
        f.write(c + "\n")
print(len(jobs), "jobs, estimated", sum(j[0] for j in jobs) / 60, "CPU minutes")
