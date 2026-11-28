#!/usr/bin/env python3
"""Round78 full-source Rust ablation: isolate root and division specialties.

Compares the Round77 baseline against source-exact variants:
  rootonly: modified linear-root extraction, original Round77 division
  root_short: rootonly + short-dividend early return
  full: complete Round78 producer
Every case performs entire solve + independent verifier; reduced only bounded
module and the known-prime admission check. 13 alternated trial rotations.
"""
from pathlib import Path
import ast
import csv
import statistics
import subprocess
import tempfile

repo = Path(__file__).resolve().parents[2]
script = ast.parse((Path(__file__).with_name("round78_full_ab.py")).read_text())
constants = {}
for node in script.body:
    if isinstance(node, ast.Assign) and len(node.targets)==1 and isinstance(node.targets[0],ast.Name):
        if node.targets[0].id in {"cargo", "main"}:
            constants[node.targets[0].id] = ast.literal_eval(node.value)
cargo = constants["cargo"]
main = constants["main"]

source_path = "src/cryptanalysis/prime_root_trace.rs"
before = subprocess.check_output(["git","show","5b1b404b4fe0632d4add0a1b20209430c0a8a810:"+source_path],cwd=repo,text=True)
after = (repo/source_path).read_text()
def extract(source, signature):
    start=source.index(signature)
    brace=source.index("{",start)
    depth=0
    for i in range(brace,len(source)):
        if source[i]=="{": depth+=1
        elif source[i]=="}":
            depth-=1
            if depth==0:return source[start:i+1]
    raise ValueError("unclosed "+signature)
div_sig="fn div(&mut self, a: &[u64], b: &[u64])"
old_div=extract(before,div_sig)
new_div=extract(after,div_sig)
rootonly=after.replace(new_div,old_div,1)
assert rootonly!=after and "        // Synthetic division by a monic linear factor." in new_div
short_start=new_div.index("        // A degree-short dividend has zero quotient")
short_end=new_div.index("        // Division by X^k is a coefficient split.",short_start)
short_body=new_div[short_start:short_end]
assert old_div.count("        let mut r = a.to_vec();")==1
short_div=old_div.replace("        let mut r = a.to_vec();",short_body+"        let mut r = a.to_vec();")
root_short=rootonly.replace(old_div,short_div,1)
variants={"before":before,"rootonly":rootonly,"root_short":root_short,"full":after}

with tempfile.TemporaryDirectory(prefix="rootfind-ablation-") as tmp:
    tmp=Path(tmp)
    bins={}
    for label,source in variants.items():
        folder=tmp/label
        (folder/"src/cryptanalysis").mkdir(parents=True)
        (folder/"Cargo.toml").write_text(cargo)
        (folder/"src/main.rs").write_text(main)
        (folder/"src/cryptanalysis/prime_root_trace.rs").write_text(source)
        (folder/"src/cryptanalysis/prime_root_bounded.rs").write_text("// out of scope\n")
        subprocess.run(["cargo","build","--release","--quiet"],cwd=folder,check=True)
        bins[label]=folder/"target/release/rootfind-isolated-ab"
    rows=[]
    labels=list(variants)
    for trial in range(13):
        rotated=labels[trial%len(labels):]+labels[:trial%len(labels)]
        if trial%2:rotated=list(reversed(rotated))
        results={}
        for label in rotated:
            output=subprocess.check_output([str(bins[label])],text=True)
            results[label]={(r["prime"],r["case"]):r for r in csv.DictReader(output.splitlines())}
        keys=results["before"].keys()
        for label in labels:
            assert results[label].keys()==keys
        for key in keys:
            checks={(results[label][key]["checksum"],results[label][key]["completed"]) for label in labels}
            assert len(checks)==1,(trial,key,checks)
            rows.append(dict(trial=trial,prime=key[0],case=key[1],**{
                label+"_ns":results[label][key]["ns_per_solve_verify"] for label in labels
            }))
    out=repo/"research/amm_digit_perf_20261118/round78_ablation_results.csv"
    with out.open("w",newline="") as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)
    print("prime,case,rootonly_speedup,root_short_speedup,full_speedup")
    for key in sorted({(r["prime"],r["case"]) for r in rows}):
        subset=[r for r in rows if (r["prime"],r["case"])==key]
        med={label:statistics.median(float(r[label+"_ns"]) for r in subset) for label in labels}
        print(f"{key[0]},{key[1]},{med['before']/med['rootonly']:.4f},{med['before']/med['root_short']:.4f},{med['before']/med['full']:.4f}")
    print(f"verified {len(rows)} output-matched trials over {len(labels)} whole-producer variants, each with 8 solve+verify instances")
