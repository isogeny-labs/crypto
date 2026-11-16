import json, statistics as st
D = "research/isogeny_loop_glv_20261012/"
d = json.load(open(D + "bench_run2.json"))
arms = {a["name"]: a for a in d["arms"]}
print("arms:", list(arms.keys()), "rounds", d["rounds"], "pairs", d["pairs"], "correctness_all_pass", d.get("correctness_all_pass"))
for n, a in arms.items():
    rr = a["rounds_ns"]
    print(n, "median_ns stored", round(a["median_ns"], 3), "median(rounds_ns)", round(st.median(rr), 3), "n", len(rr), "min", round(min(rr), 3))
g = arms["glvgls4_proj"]["median_ns"] if "glvgls4_proj" in arms else None
for n in arms:
    pass
