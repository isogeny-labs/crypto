# Skeptic check, lane cuckoo (no pcc claim; [pcc:unverified])
Reran `ic_alt_cuckoo 8000000 200` and `cuckoo_model.py`. Output matches raw_output.txt/model_output.txt
exactly apart from ns timings and the time(1) footer; model output is byte-identical. Headline numbers checked
against raw output (loads, build success, kicks, bits/key, FPR, Bloom ratios, deletion, E5 GAE, hit rate 15374/8e6).
Lever correctly labelled memory_only, no counted-GAE gain claimed, beats_baseline=false is correct.
Minor: p83 gains and 1.12x at 32KiB are model extrapolations (disclosed); E5 uses random x trials and a single
20-rep mean without CIs, 63.2k vs 56.6k record is not a like-for-like. Policies compared under the same cost unit.
