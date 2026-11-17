import json, os
BASE = "/home/user/crypto/research/isogeny_fibers_20261010/round4/16_2_fast_4e"
out = {}
for n, f in [(131, "sample_n131.json"), (233, "sample_n233.json")]:
    d = json.load(open(os.path.join(BASE, f)))
    for b in d["bench"]:
        for r in b["rows"]:
            key = (n, b["set"], r["method"])
            out[key] = r["contended_wall_ns_per_check_median"]
            print(f"n={n} set={b['set']} method={r['method']} median_ns={r['contended_wall_ns_per_check_median']:.3f}")
    print(f"n={n} errors_total={d['errors_total']} chain_vs_table_disagreements={d['chain_vs_table_disagreements']} tnaf_vs_binary_disagreements={d['tnaf_vs_binary_disagreements']} library_oracle={d['library_oracle']['agree']}/{d['library_oracle']['checked']}")
    print(f"n={n} confusion={d['confusion']}")
print("CLAIM n=131 table members_of_G us:", round(out[(131,'members_of_G','closed_form_table')]/1000,3))
print("CLAIM n=233 table members_of_G us:", round(out[(233,'members_of_G','closed_form_table')]/1000,3))
print("CLAIM n=131 tnaf members_of_G us:", round(out[(131,'members_of_G','tnaf_ld_rP')]/1000,3))
print("CLAIM n=233 tnaf members_of_G us:", round(out[(233,'members_of_G','tnaf_ld_rP')]/1000,3))
