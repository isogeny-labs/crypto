import json,glob,os,sys
R=sys.argv[1]; out=sys.argv[2]
rows=[json.load(open(f)) for f in sorted(glob.glob(R+"/m3_p*.json"))]
tab=[]
for r in rows:
    tab.append(dict(p=r["p"],t=r["t"],idx=r["idx"],seed=r["seed"],a=r["a"],b=r["b"],g=r["g"],x_R=r["x_R"],degF=r["degF"],
      bound=r["bound_3t_plus_5"],d_E1=r["d_NS_E1_unique_pointwise"],
      d_E1_rank=(r["d_NS"].get("1") or {}).get("d"),d_E2=(r["d_NS"].get("2") or {}).get("d"),d_E8=(r["d_NS"].get("8") or {}).get("d"),runtime=r["runtime_s"]))
json.dump(tab,open(out+"/table.json","w"),indent=1)
for t in tab: print(t["p"],t["t"],t["idx"],t["degF"],t["bound"],t["d_E1"],t["d_E1_rank"],t["d_E2"],t["d_E8"],round(t["runtime"],1))
