# Row 43 | 20261016-isogeny-large-ell: recompute headline claims from committed raw output.
# NOTE: the card lists no raw candidates; REPORT.md:77 names isogeny_algos/results/p13-large-ell.txt as raw output,
# which is committed. The "before" numbers are from results/p2-big.jsonl, as REPORT.md:57 names.
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C

STUDY = "isogeny_algos/docs/large-ell-20261016"
P13 = "isogeny_algos/results/p13-large-ell.txt"
P2 = "isogeny_algos/results/p2-big.jsonl"
txt = open(os.path.join(C.REPO, P13)).read().splitlines()

mont, weier, tree = {}, {}, {}
for line in txt:
    m = re.match(r"^(\d+) l=\s*(\d+): codomain velu\s+(\d+) us, sqrt\s+(\d+) .*?\s([\d.]+)x \| image velu\s+(\d+) sqrt\s+(\d+)\s+([\d.]+)x", line)
    if m:
        bits, l = int(m.group(1)), int(m.group(2))
        mont[(bits, l)] = {"cod_velu_us": int(m.group(3)), "cod_sqrt_us": int(m.group(4)), "cod_ratio_printed": float(m.group(5)),
                           "img_velu_us": int(m.group(6)), "img_sqrt_us": int(m.group(7)), "img_ratio_printed": float(m.group(8))}
        continue
    m = re.match(r"^(\d+) l=\s*(\d+): codomain xonly\s+(\d+) sqrt\s+(\d+) us\s+([\d.]+)x \| image xonly\s+(\d+) sqrt\s+(\d+) us\s+([\d.]+)x", line)
    if m:
        bits, l = int(m.group(1)), int(m.group(2))
        weier[(bits, l)] = {"cod_xonly_us": int(m.group(3)), "cod_sqrt_us": int(m.group(4)), "cod_ratio_printed": float(m.group(5)),
                            "img_xonly_us": int(m.group(6)), "img_sqrt_us": int(m.group(7)), "img_ratio_printed": float(m.group(8))}
        continue
    m = re.match(r"^(\d+) l=\s*(\d+) b=\d+ b'=\d+: codomain direct\s+(\d+) tree\s+(\d+) us \(([\d.]+)\) \| image direct\s+(\d+) tree\s+(\d+) us \(([\d.]+)\)", line)
    if m:
        bits, l = int(m.group(1)), int(m.group(2))
        tree[(bits, l)] = {"cod_direct_us": int(m.group(3)), "cod_tree_us": int(m.group(4)),
                           "img_direct_us": int(m.group(6)), "img_tree_us": int(m.group(7))}

def ratio(a, b):
    return a / b

p2 = [json.loads(l) for l in open(os.path.join(C.REPO, P2)) if l.strip()]
def p2med(algo, task, ell, bits):
    rows = [r for r in p2 if r.get("algo") == algo and r.get("task") == task and str(r.get("ell")) == str(ell) and str(r.get("p_bits")) == str(bits)]
    return rows[0]["median_ns"] if rows else None

# claim: Montgomery sqrt-Velu codomain at 10007 vs Velu
c256 = mont[(256, 10007)]; c511 = mont[(511, 10007)]
r256 = ratio(c256["cod_velu_us"], c256["cod_sqrt_us"]); r511 = ratio(c511["cod_velu_us"], c511["cod_sqrt_us"])
# before: p2-big velu_montgomery_projective / sqrt_velu_montgomery codomain
b256 = ratio(p2med("velu_montgomery_projective", "codomain", 10007, 256), p2med("sqrt_velu_montgomery", "codomain", 10007, 256))
b511 = ratio(p2med("velu_montgomery_projective", "codomain", 10007, 511), p2med("sqrt_velu_montgomery", "codomain", 10007, 511))

# crossover bracket for Montgomery codomain (p13): last l with ratio < 1 and first with ratio >= 1
def bracket(bits):
    ls = sorted(l for (b, l) in mont if b == bits)
    seq = [(l, ratio(mont[(bits, l)]["cod_velu_us"], mont[(bits, l)]["cod_sqrt_us"])) for l in ls]
    return seq

# image ratios use the printed unrounded ratio: integer us values (e.g. 5 vs 5 at l=101) are too coarse
img_all_ratio = {k: mont[k]["img_ratio_printed"] for k in mont}
img_all = list(img_all_ratio.values())
img_ge401 = [v for k, v in img_all_ratio.items() if k[1] >= 401]
w10007 = [ratio(weier[(b, 10007)]["cod_xonly_us"], weier[(b, 10007)]["cod_sqrt_us"]) for b in (256, 511)]

claims = [
    {"claimed": "Montgomery sqrt-Velu codomain 5.58x (256-bit) at l = 10007",
     "computed": f"{r256:.4f} (velu {c256['cod_velu_us']} us / sqrt {c256['cod_sqrt_us']} us)",
     "definition": "codomain velu us / codomain sqrt us from p13 line 256 l=10007", "files": [P13], "status": "match" if abs(r256-5.58)/5.58 <= 0.01 else "mismatch"},
    {"claimed": "5.27x (511-bit) at l = 10007",
     "computed": f"{r511:.4f} (velu {c511['cod_velu_us']} us / sqrt {c511['cod_sqrt_us']} us)",
     "definition": "codomain velu us / codomain sqrt us from p13 line 511 l=10007", "files": [P13], "status": "match" if abs(r511-5.27)/5.27 <= 0.01 else "mismatch"},
    {"claimed": "was 1.51x / 1.62x (Montgomery codomain before, l = 10007)",
     "computed": f"256-bit {b256:.4f}; 511-bit {b511:.4f} (p2-big velu_montgomery_projective / sqrt_velu_montgomery median_ns)",
     "definition": "p2-big.jsonl median_ns ratio of velu_montgomery_projective to sqrt_velu_montgomery, codomain, ell 10007",
     "files": [P2], "status": "match" if (abs(b256-1.51)/1.51 <= 0.01 and abs(b511-1.62)/1.62 <= 0.01) else "mismatch"},
    {"claimed": "crossover ~ l 150 (was ~ 1000)",
     "computed": "bracket p13 256-bit: " + json.dumps(bracket(256)) + " ; 511-bit: " + json.dumps(bracket(511)),
     "definition": "crossover (ratio = 1) is not interpolated by the study; no rule stated; only the bracket is computed",
     "files": [P13, f"{STUDY}/REPORT.md"], "status": "cannot_define"},
    {"claimed": "images 1.27-1.51x (Montgomery one-point image)",
     "computed": f"all l 101..10007: min {min(img_all):.4f} max {max(img_all):.4f}; l >= 401: min {min(img_ge401):.4f} max {max(img_ge401):.4f}",
     "definition": "printed image ratio (velu/sqrt) from p13, both bit lengths; range over l = 101..10007 (full) and l >= 401",
     "files": [P13], "status": "partial"},
    {"claimed": "Weierstrass sqrt-Velu 2.7-2.8x x-only Velu (codomain, l = 10007)",
     "computed": f"256-bit {w10007[0]:.4f}; 511-bit {w10007[1]:.4f}",
     "definition": "xonly codomain us / sqrt codomain us from p13 profile_weier at l = 10007",
     "files": [P13], "status": "match" if (round(w10007[0],1) in (2.7,2.8) and round(w10007[1],1) in (2.7,2.8)) else "mismatch"},
    {"claimed": "x-only Velu itself 15-25 % faster (before change)",
     "computed": "not computed: no pre-change x-only Velu record for the same harness in p2-big.jsonl (only velu_xonly_fast) or p13",
     "definition": "ratio of pre-change to post-change x-only Velu time; no pre-change record present", "files": [P2, P13], "status": "cannot_define"},
    {"claimed": "remainder-tree path slower than pair-by-pair to l = 250007",
     "computed": "codomain direct/tree: " + json.dumps({f"{b}/{l}": round(v['cod_direct_us']/v['cod_tree_us'],3) for (b,l), v in sorted(tree.items())})
                 + " ; image direct/tree: " + json.dumps({f"{b}/{l}": round(v['img_direct_us']/v['img_tree_us'],3) for (b,l), v in sorted(tree.items())}),
     "definition": "direct us / tree us for each size; tree slower where value < 1",
     "files": [P13], "status": "match" if all(v['cod_direct_us'] < v['cod_tree_us'] and v['img_direct_us'] < v['img_tree_us'] for v in tree.values()) else "mismatch"},
    {"claimed": "CSIDH-512 no change outside noise (sqrt-Velu leaves)",
     "computed": "no raw record: no profile_csidh output in p13-large-ell.txt; grep for CSIDH-512 sqrt-Velu timings in results/ not found",
     "definition": "CSIDH timing with and without sqrt-Velu leaves", "files": [P13], "status": "no_raw_in_repo"},
]

offrepo = C.grep_lines([STUDY], C.OFFREPO_PAT)
aa = C.grep_lines([STUDY], C.AA_PAT)
iso = C.grep_lines([STUDY], C.ISO_PAT) + C.grep_lines([P13], C.ISO_PAT)
row = {
    "row_no": 43,
    "slug": "20261016-isogeny-large-ell",
    "claims": claims,
    "identical": {"claimed": True, "n_compared": None, "n_equal": None, "n_differ": None,
                  "files": [P13], "status": "no_raw_in_repo"},
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": "no PROTOCOL file in " + STUDY,
    "first_raw_git_date": C.git_first_add(P13) + " | " + C.git_first_add(P2),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/43.py",
    "notes": ("Card RAW CANDIDATES was empty; the REPORT names committed raw output P13 (used here). "
              "identical claim '130/130 tests' is a cargo test count; no test-run output is committed, so n_compared is null. "
              "p13 has no verified field. CSIDH-512 claim has no raw record in the repo. "
              "No A/A or null-control lines in the study (aa_lines lists any matches). "
              "REPORT.md states the host as '4 vCPU shared VM, one run per setting, L0 wall' (not in the headline "
              "isolation field beyond that)."),
}
print(json.dumps(row, ensure_ascii=False))
