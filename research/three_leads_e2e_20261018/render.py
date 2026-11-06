"""Render frozen Rust observations; no experiment, fit, or statistical analysis."""
import argparse
import csv
import hashlib
import json
import re
from pathlib import Path
from urllib.parse import urljoin
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst
import fitz
from PIL import Image, ImageDraw

HERE=Path(__file__).resolve().parent
FIG=HERE/"figures"
FIG.mkdir(exist_ok=True)
plt.rcParams.update({"font.family":"DejaVu Sans","font.size":9,"svg.fonttype":"none","svg.hashsalt":"three-leads-e2e-20261018","axes.spines.top":False,"axes.spines.right":False})
BASE="https://github.com/aburan28/crypto/blob/72d25b274af3b8f018baed492696eea4a0d1971e/research/three_leads_e2e_20261018/"
BLUE,ORANGE,GREEN="#235789","#ad4b16","#26734a"
def load(name):return json.loads((HERE/name).read_text())
def quote(s):return json.dumps(s,ensure_ascii=False)
def inline(value):
    out=[]
    for token in re.split(r"(`[^`]+`|\*\*[^*]+\*\*|\[[^\]]+\]\([^)]+\))",value):
        if not token:continue
        if token.startswith("`"):out.append(f'#text(font:"DejaVu Sans Mono",size:8pt,{quote(token[1:-1])})')
        elif token.startswith("**"):out.append(f'#strong[{inline(token[2:-2])}]')
        elif token.startswith("["):
            label,target=re.match(r"\[([^\]]+)\]\(([^)]+)\)",token).groups()
            out.append(f'#link({quote(target if "://" in target else urljoin(BASE,target))})[{inline(label)}]')
        else:out.append(f'#text({quote(token)})')
    return "".join(out)
def save(fig,path):
    fig.savefig(path.with_suffix(".svg"),bbox_inches="tight",metadata={"Date":None})
    fig.savefig(path.with_suffix(".png"),bbox_inches="tight",dpi=150)
    plt.close(fig)
def plots():
    data=load("results/report-data.json")
    fig,axes=plt.subplots(1,2,figsize=(11,4.4),constrained_layout=True)
    for ax,prefix,title in [(axes[0],"genus-","Field-size cohort"),(axes[1],"order-genus-","Actual subgroup-size cohort")]:
        rows=[r for r in data["genus_tables"] if r["group"].startswith(prefix)]
        sizes=[int(re.search(r"genus-(\d+)",r["group"]).group(1)) for r in rows]
        ax.vlines(sizes,[r["native_band_ratio_range"][0] for r in rows],[r["native_band_ratio_range"][1] for r in rows],color="#b0b0b0",linewidth=7,label="Native-charge sensitivity")
        ax.vlines(sizes,[r["arithmetic_ratio_range"][0] for r in rows],[r["arithmetic_ratio_range"][1] for r in rows],color=BLUE,linewidth=2,label="12 arithmetic prices")
        ax.plot(sizes,[r["declared_default_ratio"] for r in rows],"o-",color=ORANGE,label="Declared S/M=.75, I/M=10")
        ax.axhline(1,color="#555555",linestyle="--",linewidth=1)
        ax.set(xlabel="Nominal field-size level" if prefix=="genus-" else "Exact prime-subgroup bits",ylabel="Cold IC / negation rho",yscale="log",xticks=sizes,title=title+" · 180 targets per level")
    axes[0].legend(fontsize=7,frameon=False)
    save(fig,FIG/"genus_cost_bands")
    rows=load("results/coverage-summary.json")
    fig,ax=plt.subplots(figsize=(10,4.6),constrained_layout=True)
    for prefix,marker,color,label in [("binary-q","o",BLUE,"Primary b outside subfield; 3 curves/q"),("binary_coset1_control-q","s",GREEN,"Supplementary opposite trace coset"),("binary_descended_control-q","^",ORANGE,"Descended b; trace-zero a"),("binary_descended_coset1_control-q","v","#7b547f","Descended b; trace-one a")]:
        selected=[r for r in rows if r["cell"].startswith(prefix)]
        selected.sort(key=lambda r:int(r["cell"].rsplit("q",1)[1]))
        ax.plot([int(r["cell"].rsplit("q",1)[1]) for r in selected],[r["structured"] for r in selected],marker+"-",color=color,label=label,markersize=4)
    ax.axhline(.375,color="#444444",linestyle="--",linewidth=1,label="3/8 proposed generic limit")
    ax.axhline(.3934693402873666,color="#777777",linestyle=":",linewidth=1,label="1-exp(-1/2) occupancy approximation")
    ax.set(xscale="log",xlabel="Subfield size q",ylabel="Full-group reachable fraction",ylim=(-.01,.58),title="Complete binary pair sumsets · finite coverage is not fixed by the trace bound")
    ax.legend(fontsize=7,frameon=False,ncol=2)
    save(fig,FIG/"binary_coverage")
    xrows=load("results/random-x-null/coverage-summary.json")
    fig,axes=plt.subplots(1,2,figsize=(11,5.6),constrained_layout=True)
    odd=[r for r in rows if r["cell"].startswith("odd-p")]
    for x,r in enumerate(odd):
        axes[0].vlines(x,r["bootstrap_95"][0],r["bootstrap_95"][1],color=BLUE,linewidth=2)
        axes[0].plot(x,r["delta"],"o",color=BLUE)
    axes[0].axhline(0,color="#777777",linewidth=1)
    axes[0].set(xticks=list(range(len(odd))),xticklabels=[r["cell"].replace("odd-","") for r in odd],ylabel="Structured minus matched-point coverage",title="Odd-characteristic primary census · 3 curves/cell")
    axes[0].tick_params(axis="x",rotation=60)
    pkm=[r for r in rows if r["cell"].startswith("pkm_")]
    xpos={r["cell"]:x for x,r in enumerate(pkm)}
    for selected,marker,color,label in [(pkm,"o",BLUE,"Same actual point count"),([r for r in xrows if r["cell"].startswith("pkm_")],"x",ORANGE,"Uniform x sets")]:
        axes[1].scatter([r["z"] for r in selected if r["z"] is not None],[xpos[r["cell"]] for r in selected if r["z"] is not None],marker=marker,color=color,s=24,label=label)
    axes[1].axvline(1.84,color="#777777",linestyle=":",linewidth=1)
    axes[1].axvline(-1.84,color="#777777",linestyle=":",linewidth=1)
    axes[1].set(yticks=list(range(len(pkm))),yticklabels=[r["cell"].replace("pkm_","") for r in pkm],xlabel="Nominal paired-prime standardized mean z",title="18 PKM cells · 23 primes/cell · 32 controls/prime")
    axes[1].tick_params(axis="y",labelsize=6)
    axes[1].legend(fontsize=7,frameon=False)
    save(fig,FIG/"coverage_controls")
    events=load("results/genus-cycle-events.json")
    events=[r for r in events if r["group"].startswith("order-genus-")]
    fig,ax=plt.subplots(figsize=(9,4.3),constrained_layout=True)
    sizes=[int(r["group"].rsplit("-",1)[1]) for r in events]
    ax.bar(sizes,[r["solves_after_first_graph_cycle"] for r in events],width=2.7,color=ORANGE,label="Needed a later useful dependence")
    ax.bar(sizes,[r["solves_at_first_graph_cycle"] for r in events],bottom=[r["solves_after_first_graph_cycle"] for r in events],width=2.7,color=BLUE,label="Solved at first graph cycle")
    ax.set(xticks=sizes,xlabel="Exact prime-subgroup bits",ylabel="Verified target cases",title="First graph cycle versus actual solve · 180 targets per level")
    ax.legend(fontsize=8,frameon=False)
    save(fig,FIG/"signed_cycle_stopping")

def markdown(source,path):
    lines=source.splitlines();i=0;out=[]
    while i<len(lines):
        line=lines[i]
        if not line.strip():i+=1;continue
        if line.startswith("#"):
            level=len(line)-len(line.lstrip("#"));out.append(f'#heading(level:{level})[{inline(line[level:].strip())}]\n');i+=1
        elif line.startswith("|"):
            table=[]
            while i<len(lines) and lines[i].startswith("|"):
                cells=[x.strip() for x in lines[i].strip("|").split("|")]
                if not all(re.fullmatch(r"[-: ]+",x) for x in cells):table.append(cells)
                i+=1
            n=len(table[0]);widths="("+",".join("1fr" for _ in range(n))+")"
            if n==3:widths="(1.1fr,.8fr,3.3fr)"
            header=",".join("["+inline(x)+"]" for x in table[0]);body=",".join("["+inline(x)+"]" for row in table[1:] for x in row)
            out.append(f'#table(columns:{widths},inset:4pt,stroke:.3pt+rgb("cccccc"),table.header({header}),{body})\n')
        else:
            para=[]
            while i<len(lines) and lines[i].strip() and not lines[i].startswith(("#","|")):
                para.append(lines[i].strip());i+=1
            out.append(inline(" ".join(para))+"\n\n")
    path.write_text("\n".join(out).rstrip()+"\n")

def main_report():
    plots()
    body=HERE/"report-body.typ";markdown((HERE/"README.md").read_text(),body)
    source=['#set page(paper:"a4",margin:(x:17mm,y:16mm),numbering:"1",header:align(right,text(size:8pt)[Fresh ECDLP experiments · 2026-10-19]))','#set text(font:"DejaVu Sans",size:9.1pt)','#set par(leading:.55em,spacing:.8em)','#show heading.where(level:1):set text(size:18pt)','#show heading.where(level:2):set text(size:12pt)','#show table.cell:set text(size:8pt)','#show link:set text(fill:rgb("235789"))','#include "report-body.typ"','#pagebreak()','#heading(level:1)[Recorded observations]']
    for name,caption in [("genus_cost_bands","Price sensitivity, not a confidence interval. Cold work includes setup, transfer, roots, elimination and source verification. Each point uses 180 matched targets; both cohorts contain 1,260 targets."),("binary_coverage","Primary and supplementary complete finite sumsets. Descended-b controls demonstrate why the trace kernel alone does not establish a 3/8 limit."),("coverage_controls","The two PKM nulls answer different questions. Odd-cell bars are native curve-resampling intervals. Standardizations and intervals do not prove universal random equivalence."),("signed_cycle_stopping","The signed collector distinguishes graph cycles from useful target dependencies. Repeated split divisor classes are discarded in the primary arm.")]:
        source.append(f'#figure(image("figures/{name}.svg",width:100%),caption:[{inline(caption)}])')
    if (HERE/"CLAIMS.md").exists():
        markdown((HERE/"CLAIMS.md").read_text(),HERE/"claims-body.typ");source.extend(['#pagebreak()','#include "claims-body.typ"'])
    (HERE/"report.typ").write_text("\n".join(source)+"\n")
    typst.compile(str(HERE/"report.typ"),output=str(HERE/"report.pdf"),root=str(HERE))

def individual_reports():
    rows=load("results/session-summaries.json");root=HERE/"reports";root.mkdir(exist_ok=True)
    source=['#set page(paper:"a4",margin:(x:17mm,y:16mm))','#set text(font:"DejaVu Sans",size:9.3pt)','#set par(leading:.5em,spacing:.7em)','#show table.cell:set text(size:8.4pt)']
    eligible=[]
    for row in rows:
        session=load(f'sessions/{row["name"]}/session.json')
        if session["status"]!="complete" or session["records_written"]!=row["records"]:raise RuntimeError("Unclosed/mismatched session "+row["name"])
        target=root/("run-"+row["session_id"]);target.mkdir(exist_ok=True)
        observation=json.dumps(row,sort_keys=True,ensure_ascii=False,indent=1)+"\n"
        digest=hashlib.sha256(observation.encode()).hexdigest()
        marker=target/"render-source.sha256"
        if marker.exists() and marker.read_text().strip()!=digest:raise RuntimeError("Changed immutable report "+row["name"])
        if not marker.exists():
            (target/"observations.json").write_text(observation)
            fig,ax=plt.subplots(figsize=(8.2,3.7),constrained_layout=True)
            arms=row["arms"]
            if len(arms)>1:
                labels=[a["arm"] for a in arms];ys=list(range(len(arms)))
                ax.barh(ys,[a["mean_GAE"] for a in arms],color=[BLUE if "rho" in a["arm"] else ORANGE for a in arms],alpha=.8)
                ax.hlines(ys,[a["minimum_GAE"] for a in arms],[a["maximum_GAE"] for a in arms],color="#222222",linewidth=1)
                ax.set(yticks=ys,yticklabels=labels,xscale="log",xlabel="Cold source-addition equivalents; native work unpriced",title=row["name"])
            else:
                ax.axis("off")
                for x,label in [(0.05,"Public model\nand planted target"),(.38,arms[0]["arm"]+"\ncontrol"),(.71,"Recovered scalar\nchecked on E")]:
                    ax.text(x,.55,label,transform=ax.transAxes,va="center",bbox={"boxstyle":"round,pad=.6","fc":"#edf2f5","ec":BLUE},fontsize=10)
                for x in [.27,.60]:ax.annotate("",xy=(x+.09,.55),xytext=(x,.55),xycoords="axes fraction",arrowprops={"arrowstyle":"->","color":BLUE})
                ax.text(.04,.17,"One control arm: no within-session rho comparison.\nAll curve targets and complete native costs are retained.",transform=ax.transAxes,fontsize=9)
                ax.set_title(row["name"])
            save(fig,target/"plot");marker.write_text(digest+"\n")
        if eligible:source.append('#pagebreak()')
        source.extend([f'#heading(level:1)[{inline(row["name"])}]',inline("Session "+row["session_id"])+"\n",inline(row["field_model"]+"; isolation "+row["isolation"])+"\n",f'#image("{target.relative_to(HERE)}/plot.svg",width:100%,height:105mm,fit:"contain")'])
        head='table.header([Arm],[Attempts],[Verified],[Mean GAE],[Min],[Max])'
        cells=[]
        for a in row["arms"]:
            cells.extend([inline(a["arm"]),str(a["attempts"]),str(a["verified"]),f'{a["mean_GAE"]:.3f}',f'{a["minimum_GAE"]:.3f}',f'{a["maximum_GAE"]:.3f}'])
        source.append('#table(columns:(1.4fr,.6fr,.6fr,.9fr,.9fr,.9fr),inset:4pt,stroke:.3pt+rgb("cccccc"),'+head+','+','.join('['+x+']' for x in cells)+')')
        if any(a["other"] for a in row["arms"]):source.append('#text(fill:rgb("9b341b"))[Includes capped/exhausted attempts. Consumed work is not a completed-solving cost.]')
        source.extend([inline("Min/max are observed target spread, not confidence intervals. Means and price conversions were computed by the Rust analysis, not the renderer.")+"\n",inline("Target seed: "+str(row["target_seed"])+". Example algorithm seed: "+row["algorithm_seed_example"])+"\n",inline("Exact models, parameters, caps, execution order and verification outcomes: "+row["source"])+"\n",inline("Replay receipt: replay-receipts/"+row["name"]+".json. Raw records and replay inputs are in the evidence archives.")])
        eligible.append((row,target))
    path=HERE/"session-reports.typ";path.write_text("\n".join(source)+"\n")
    bundle=HERE/"session-reports.pdf";typst.compile(str(path),output=str(bundle),root=str(HERE))
    doc=fitz.open(bundle)
    if len(doc)!=len(eligible):raise RuntimeError(f"Report layout overflow: {len(doc)} pages for {len(eligible)} sessions")
    index=["# Individual session reports","","Every session has a one-page report, its SVG/150-dpi PNG graph and the native observations. Extract `../runs.tar.xz` at the study root; the paths below then resolve inside `reports/`.","","| Session | Report path |","|:--|:--|"]
    inspection=HERE/"pdf-inspection";inspection.mkdir(exist_ok=True)
    sheets=[];sheet=None;draw=None
    for n,(row,target) in enumerate(eligible):
        text=doc[n].get_text()
        if row["session_id"] not in text or row["name"] not in text:raise RuntimeError("Missing report identity "+row["name"])
        if not (target/"report.pdf").exists():
            single=fitz.open();single.insert_pdf(doc,from_page=n,to_page=n);single.save(target/"report.pdf",deflate=True);single.close()
        if n%40==0:
            if sheet is not None:sheet.save(inspection/f"sessions-{n//40-1:02}.png")
            sheet=Image.new("RGB",(1500,3000),"white");draw=ImageDraw.Draw(sheet)
        pix=doc[n].get_pixmap(matrix=fitz.Matrix(.34,.34),alpha=False)
        thumb=Image.frombytes("RGB",(pix.width,pix.height),pix.samples);thumb.thumbnail((290,350))
        col=(n%40)%5;rr=(n%40)//5;sheet.paste(thumb,(col*300,rr*375));draw.text((col*300+5,rr*375+351),f'{n+1}: {row["session_id"]}',fill="black")
        index.append(f'| {row["session_id"]} · {row["name"]} | `run-{row["session_id"]}/report.pdf` |')
    if sheet is not None:sheet.save(inspection/f"sessions-{(len(eligible)-1)//40:02}.png")
    (root/"INDEX.md").write_text("\n".join(index)+"\n")
    (inspection/"all-page-text-checks.json").write_text(json.dumps({"session_pages":len(doc),"all_identities_present":True,"one_page_per_session":True},indent=1)+"\n")
    doc.close()

def aa_report():
    data=load("results/instruction-AA.json")
    target=HERE/"instruction-AA-report";target.mkdir(exist_ok=True)
    fig,ax=plt.subplots(figsize=(8.2,3.5),constrained_layout=True)
    ax.plot([p["round"] for p in data["pairs"]],[p["ratio"] for p in data["pairs"]],"o-",color=BLUE)
    ax.axhline(1,color="#555555",linestyle="--")
    ax.set(xlabel="Measured pair (one warmup pair excluded)",ylabel="Repeat / reference retired instructions",title="Identical-rho instruction A/A control")
    ax.ticklabel_format(useOffset=False,style="plain",axis="y")
    save(fig,target/"plot")
    source=['#set page(paper:"a4",margin:18mm)','#set text(font:"DejaVu Sans",size:10pt)','#heading(level:1)[Retired-instruction A/A control]',inline("ECBS1h825e5ad6666e; same actual-40-bit subgroup model, workload and binary. Five measured identical-rho pairs and one warmup pair; all answers verify, all ten measured records replay identically. Primitive and logical native counters agree exactly."),'#image("plot.svg",width:100%)',inline("Frozen ratio range: 0.9999703014–1.0000120823. Median: 0.9999992766. These are user-space process instruction counts, not qualified wall times. L0; no isolation speed claim."),inline("Source: ../results/instruction-AA.json; sealed spec, host, plan and records in ../instruction-AA-session/; audit ../correctness/instruction-AA-replay.json.")]
    (target/"report.typ").write_text("\n\n".join(source)+"\n")
    typst.compile(str(target/"report.typ"),output=str(target/"report.pdf"),root=str(HERE))

if __name__=="__main__":
    ap=argparse.ArgumentParser();ap.add_argument("--individual",action="store_true");ap.add_argument("--main",action="store_true");args=ap.parse_args()
    if args.individual:individual_reports()
    if args.main or not args.individual:main_report();aa_report()
