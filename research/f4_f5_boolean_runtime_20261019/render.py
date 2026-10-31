"""Render recorded native values only; no solver, timing admission or ratios."""
from pathlib import Path
import json
import shutil
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst
import fitz

HERE = Path(__file__).resolve().parent
BENCH = HERE.parents[1] / "benchmarks/20261019-boolean-native"
BENCH.mkdir(parents=True, exist_ok=True)
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 9})

def esc(text):
    return str(text).replace("\\", "\\\\").replace('"', '\\"')

def save(fig, name):
    fig.savefig(HERE / (name + ".svg"), bbox_inches="tight")
    fig.savefig(HERE / (name + ".png"), dpi=150, bbox_inches="tight")
    plt.close(fig)
    for ext in ["svg", "png"]:
        shutil.copy2(HERE / f"{name}.{ext}", BENCH / f"{name}.{ext}")

def compile_report(folder, title, paragraphs, images, tables):
    folder.mkdir(parents=True, exist_ok=True)
    content = ['#set page(paper: "a4", margin: 15mm, numbering: "1")',
        '#set text(font: "DejaVu Sans", size: 9pt)',
        '#set par(justify: false)', f'#heading(level: 1)[{title}]']
    for p in paragraphs:
        content.append(f'#text("{esc(p)}")\n\n')
    for index, image in enumerate(images):
        if index:
            content.append('#pagebreak(weak: true)')
        relative = Path(__import__("os").path.relpath(HERE / image, folder))
        content.append(f'#image("{relative}", width: 100%)')
    for caption, headers, rows in tables:
        content.append('#pagebreak(weak: true)')
        content.append(f'#heading(level: 2)[{caption}]')
        cells = headers + [str(cell) for row in rows for cell in row]
        content.append('#table(columns: ' + str(len(headers)) + ', inset: 4pt, ' +
            ', '.join(f'text("{esc(cell)}")' for cell in cells) + ')')
    source = folder / "REPORT.typ"
    source.write_text("\n".join(content))
    typst.compile(str(source), output=str(folder / "REPORT.pdf"), root=str(HERE))

def preparation_reports():
    graph = HERE / "preparation-failure.svg"
    doc = fitz.open(stream=graph.read_bytes(), filetype="svg")
    pdf = fitz.open("pdf", doc.convert_to_pdf())
    pix = pdf[0].get_pixmap(dpi=150)
    pix.save(HERE / "preparation-failure.png")
    for ext in ["svg", "png"]:
        shutil.copy2(HERE / f"preparation-failure.{ext}", BENCH)
    for tag, run, slug in [
        ("initial", "20261019-011623-boolean-native-compariso-9fc034", "20261019T011623Z"),
        ("optimization", "20261019-012707-boolean-native-optimizat-abf14c", "20261019T012707Z"),
    ]:
        folder = HERE / "reports" / ("run-" + slug)
        paragraphs = [f"Run {run}. Source preparation only; c7i.2xlarge on-demand. The instance terminated automatically.",
            "The working-tree patch included PR files outside the sparse checkout. git apply rejected the patch before compilation or timing. No native cost or speed ratio exists for this attempt.",
            f"Raw receipts: preparation-failures/{tag}-job.json, {tag}-status.json and {tag}-job.log. Corrected runs use complete checkouts; no measurement gate was weakened."]
        compile_report(folder, "Native comparison: source-preparation failure", paragraphs, ["preparation-failure.svg"], [])
        (folder / "REPORT.md").write_text("# Source-preparation failure\n\n" + "\n\n".join(paragraphs) + "\n\n![Preparation](../../preparation-failure.svg)\n")
        shutil.copy2(folder / "REPORT.pdf", BENCH / f"preparation-{tag}.pdf")
    quota_svg='''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="190" viewBox="0 0 1000 190">
<rect width="1000" height="190" fill="white"/>
<g font-family="DejaVu Sans" text-anchor="middle" fill="#173b50">
<text x="500" y="27" font-size="20">On-demand launch refused before boot</text>
<g fill="#edf4f7" stroke="#417086" stroke-width="2"><rect x="20" y="55" width="220" height="75" rx="8"/><rect x="265" y="55" width="220" height="75" rx="8"/><rect x="510" y="55" width="220" height="75" rx="8"/><rect x="755" y="55" width="220" height="75" rx="8"/></g>
<path d="M240 92H265 M485 92H510 M730 92H755" stroke="#417086" stroke-width="2"/>
<g font-size="15"><text x="130" y="85">c7i.2xlarge request</text><text x="130" y="108">same frozen protocol</text><text x="375" y="85">Shared32-vCPU quota</text><text x="375" y="108">launch rejected</text><text x="620" y="85">No instance or build</text><text x="620" y="108">no timing observation</text><text x="865" y="85">Separate Spot retry</text><text x="865" y="108">same type and gates</text></g>
<text x="500" y="166" font-size="14">No other job stopped; no account quota or isolation threshold changed.</text></g></svg>'''
    (HERE/"quota-failure.svg").write_text(quota_svg)
    svg=fitz.open(stream=quota_svg.encode(),filetype="svg")
    pdf=fitz.open("pdf",svg.convert_to_pdf());pdf[0].get_pixmap(dpi=150).save(HERE/"quota-failure.png")
    for ext in ["svg","png"]: shutil.copy2(HERE/f"quota-failure.{ext}",BENCH)
    for tag,run,stamp in [
        ("quota-baseline","20261019-013917-boolean-native-baseline-9a4ea0","20261019T013917Z"),
        ("quota-final","20261019-013933-boolean-native-batch-fin-aed805","20261019T013933Z")]:
        paragraphs=[f"Run {run}: the on-demand c7i.2xlarge request was rejected by the shared 32-vCPU Standard On-Demand quota before any instance booted. No compilation, timing or solver result exists for this attempt.",
            f"Raw launch/status/command receipts: preparation-failures/{tag}-*. Dedicated retries use the same c7i.2xlarge type in the separate Spot quota. The frozen inputs, one-worker/core reservation, output verification and admission gates remain unchanged. No other job was stopped and no account setting changed."]
        folder=HERE/"reports"/("run-"+stamp)
        compile_report(folder,"Native comparison: launch capacity refusal",paragraphs,["quota-failure.svg"],[])
        (folder/"REPORT.md").write_text("# Launch capacity refusal\n\n"+"\n\n".join(paragraphs)+"\n")
        shutil.copy2(folder/"REPORT.pdf",BENCH/f"{tag}.pdf")

def comparison_plot(data, boundary, name, title):
    fig, axes = plt.subplots(2, 2, figsize=(11.6, 7.5), layout="constrained")
    for ax, (n, random) in zip(axes.flat, [(17,0),(17,1),(23,0),(23,1)]):
        rows = [r for r in data["costs"] if r["boundary"] == boundary and r["n"] == n and r["random"] == random]
        values = [r for r in rows if r["cold"]["n"]]
        if values:
            y = list(range(len(values)))
            costs = [r["cold"]["median_ms"] for r in values]
            lo = [r["cold"]["min_ms"] for r in values]
            hi = [r["cold"]["max_ms"] for r in values]
            ax.barh(y, costs, color="#3e728f", xerr=[[m-l for m,l in zip(costs,lo)], [h-m for h,m in zip(hi,costs)]], capsize=2)
            ax.set_yticks(y, [r["arm"] for r in values])
            ax.set_xscale("log")
            ax.invert_yaxis()
        else:
            ax.text(.5,.5,"No admitted native samples",ha="center",transform=ax.transAxes)
        ax.set_title(f"n{n}, {'generator control' if random else 'planted'}; five attempted samples")
        ax.set_xlabel("Complete-call cold ms; median and recorded min/max")
        ax.grid(axis="x", alpha=.2)
    fig.suptitle(title + " · native matched fixtures · 2026-10-19")
    save(fig, name)

def timing_rows(data, n=23):
    rows=[]
    for r in data["costs"]:
        if r["n"] != n: continue
        c=r["cold"]
        rows.append([r["boundary"],r["arm"],"generator" if r["random"] else "planted",
            f'{c["median_ms"]:.4f}' if c["n"] else "missing",str(c["n"]),
            f'{r["comparison"]["reference_over_candidate"]:.3f}' if "reference_over_candidate" in r["comparison"] else "inconclusive"])
    return rows

def paired_plot(rows, before_key, after_key, name, title):
    values=[r for r in rows if r[before_key]["n"] and r[after_key]["n"]]
    fig,ax=plt.subplots(figsize=(10.8,max(3,0.56*len(values)+1.2)),layout="constrained")
    labels=[f'{r.get("arm","rootfind-mitm")} n{r["n"]} '+("generator" if r.get("random",0) else "planted")+(f' s{r["seed"]}' if "seed" in r else "")+(f' w{r["workers"]}' if "workers" in r else "") for r in values]
    for shift,key,color,label in [(-.17,before_key,"#7894a5","Reference"),(.17,after_key,"#286d91","Candidate")]:
        costs=[r[key]["median_ms"] for r in values]
        low=[r[key]["min_ms"] for r in values]
        high=[r[key]["max_ms"] for r in values]
        ax.barh([i+shift for i in range(len(values))],costs,height=.3,color=color,label=label,
            xerr=[[m-l for m,l in zip(costs,low)],[h-m for h,m in zip(high,costs)]],capsize=2)
    ax.set_yticks(list(range(len(values))),labels)
    if values: ax.set_xscale("log")
    ax.invert_yaxis();ax.set_xlabel("Native cold ms; median and recorded min/max; five attempted pairs")
    ax.set_title(title);ax.grid(axis="x",alpha=.2);ax.legend()
    save(fig,name)

def run_reports():
    initial_path=HERE/"initial-summary.json"
    if initial_path.exists():
        initial=json.loads(initial_path.read_text())
        paired_plot(initial["matrices"],"reference","candidate","native-single-write","Original kernel vs single-write; one worker, baseline host")
        folder=HERE/"reports/run-20261019T014036Z"
        paragraphs=["Run 20261019-014036-boolean-native-baseline-8d3eb7. Five A/A and A/B pairs, fixed outputs and unchanged isolation gates. Full records, missing samples and native bootstrap decisions are in initial-summary.json.",
            f'CPU: {initial["host"]["cpu_model"]}. Compiler: {initial["host"]["compiler"].strip()}. Exact identities are in fixture-identities.json.',
            "Complete-root and decomposition comparisons retain their separate boundaries. Oracle preparation is charged in cold costs. random=true is the unplanted generator control; n19 has 36 Boolean variables and its admission refusals are not timings."]
        compile_report(folder,"Native baseline and original single-write comparison",paragraphs,["native-single-write.svg"],[])
        (folder/"REPORT.md").write_text("# Native baseline\n\n"+"\n\n".join(paragraphs)+"\n")
        shutil.copy2(folder/"REPORT.pdf",BENCH/"native-baseline.pdf")
    opt_path=HERE/"optimization-summary.json"
    if opt_path.exists():
        opt=json.loads(opt_path.read_text())
        paired_plot(opt["matrices"],"before","after","native-vector-tables","Single-write baseline vs SIMD tables; same-host paired builds")
        generic=[r for r in opt["changes"] if "generalized" in r["arm"]]
        paired_plot(generic,"before","after","native-generalized-changes","Generalized solver: borrowed input, exact evaluation, wide Gray")
        folder=HERE/"reports/run-20261019T013858Z"
        paragraphs=["Run 20261019-013858-boolean-native-vector-gr-799a77. Exact frozen baseline and candidate builds on one c7i.2xlarge on-demand VM. Five A/A and A/B pairs; independent outputs/counts checked.",
            f'CPU: {opt["host"]["cpu_model"]}. Scalar/portable ablations, direct legacy-to-candidate ratios and all native decisions remain in optimization-summary.json. Samples are not pooled with another host.',
            "The generalized API dispatches to an existing SIMD Gray kernel and retains exact counts and scalar fallback. The matrix switch remains opt-in. These are bounded solver improvements; no asymptotic or whole-IC/rho claim is implied."]
        compile_report(folder,"Native vector-table and generalized-root optimizations",paragraphs,["native-vector-tables.svg","native-generalized-changes.svg"],[])
        (folder/"REPORT.md").write_text("# Native paired optimizations\n\n"+"\n\n".join(paragraphs)+"\n")
        shutil.copy2(folder/"REPORT.pdf",BENCH/"native-optimizations.pdf")

def profile_reports():
    path=HERE/"profiles-summary.json"
    if not path.exists(): return
    data=json.loads(path.read_text())
    fig,axes=plt.subplots(2,2,figsize=(10.8,6.5),layout="constrained")
    for ax,row in zip(axes.flat,data["rows"]):
        vals=[row[k]["median_ir"] for k in ["before","after"]]
        lo=[row[k]["min_ir"] for k in ["before","after"]]
        hi=[row[k]["max_ir"] for k in ["before","after"]]
        ax.bar(["Before","Candidate"],vals,color=["#7894a5","#286d91"],yerr=[[v-l for v,l in zip(vals,lo)],[h-v for h,v in zip(hi,vals)]],capsize=3)
        ax.set_title(row["arm"]+"; n23 planted, five pairs")
        ax.set_ylabel("Complete-call Ir (cold for Semaev)")
        ax.ticklabel_format(axis="y",style="sci",scilimits=(0,0))
    fig.suptitle("Auxiliary instruction profiles · Callgrind 3.19 · simulated AVX2")
    save(fig,"auxiliary-instructions")
    paragraphs=["Corrected local batch began with the02:19:48 UTC command receipt. Five alternating before/candidate pairs per arm; complete query Ir plus setup Ir for Semaev. Every output is verified and matching work/witnesses are checked by analyze-profiles.rs. This batch uses iteration1 before the later cutoff/typed policy.",
        "Recorded host: Xeon 6975P-C, shared managed x86-64 Linux host; Valgrind 3.19, simulated AVX2, merged workers and RAYON_NUM_THREADS=1. Counts are instruction evidence only. Local elapsed times, cycles and IPC are not supplied as native measurements. Native cloud results use their own admission and source manifests.",
        "Sources: profiles-corrected/ in the raw archive, corrected-profile-binaries.sha256, profiles-summary.json and run-profiles.sh. The corrected baseline is the original study's exact final executable, SHA256199eb9cf47d10245e105bf9f6bed431b3ec3afc106ba7d891619c317876b3d0e. Retain the earlier failed batch separately. First candidate hot-function annotation is in records/f4-candidate-hot-functions.txt; it is not a new native timing."]
    rows=[[r["arm"],str(r["before"]["median_ir"]),str(r["after"]["median_ir"]),f'{r["median_ir_reduction_percent"]:.4f}'] for r in data["rows"]]
    folder=HERE/"reports/run-20261019T021948Z"
    compile_report(folder,"Corrected auxiliary instruction profiles",paragraphs,["auxiliary-instructions.svg"],[("Five pairs; medians and derived reduction from Rust",["Arm","Before Ir","Candidate Ir","Reduction %"],rows)])
    (folder/"REPORT.md").write_text("# Corrected auxiliary profiles\n\n"+"\n\n".join(paragraphs)+"\n")
    shutil.copy2(folder/"REPORT.pdf",BENCH/"auxiliary-profiles.pdf")
    failed="""<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="170"><rect width="1000" height="170" fill="white"/><g font-family="DejaVu Sans" text-anchor="middle" fill="#173b50"><text x="500" y="30" font-size="20">First local profile batch: wrong baseline executable</text><text x="500" y="70" font-size="17">Matrix F4/F5 pair completed → generalized-native arm unknown → exit101</text><text x="500" y="105" font-size="15">Earlier records retained in profiles/; no pooling or native runtime claim.</text><text x="500" y="140" font-size="15">Corrected batch uses the original study's hashed final executable.</text></g></svg>"""
    (HERE/"profile-failure.svg").write_text(failed)
    svg=fitz.open(stream=failed.encode(),filetype="svg");pdf=fitz.open("pdf",svg.convert_to_pdf());pdf[0].get_pixmap(dpi=150).save(HERE/"profile-failure.png")
    for ext in ["svg","png"]:shutil.copy2(HERE/f"profile-failure.{ext}",BENCH)
    paragraphs=["The first local batch began with its02:14:56 UTC command receipt. A stale release-directory executable lacked the generalized-native arm and returned exit101. Earlier matrix pairs and the failed call remain under profiles/ in the raw archive; none is pooled with the corrected five-pair batch.",
        "Baseline/candidate selection: profile-binaries.sha256. The expected baseline was subsequently selected from the original study's verified final executable. Commands, stdout, stderr, profile dumps and exit receipts remain unchanged. This interrupted batch establishes no native speed result."]
    folder=HERE/"reports/run-20261019T021456Z"
    compile_report(folder,"Interrupted auxiliary profile batch",paragraphs,["profile-failure.svg"],[])
    (folder/"REPORT.md").write_text("# Interrupted local batch\n\n"+"\n\n".join(paragraphs)+"\n")
    shutil.copy2(folder/"REPORT.pdf",BENCH/"auxiliary-failure.pdf")

def final_report():
    path=HERE / "final-summary.json"
    if not path.exists(): return
    data=json.loads(path.read_text())
    ids=json.loads((HERE / "fixture-identities.json").read_text())
    comparison_plot(data,"complete_roots","phase3-complete-roots","Phase3: complete original root sets")
    comparison_plot(data,"decomposition","phase3-cold-decomposition","Phase3: decomposition, oracle setup charged")
    paired_plot(data["batching"],"before","after","native-semaev-setup","Semaev cold setup+query: scalar vs 256-addition batches")
    paragraphs=["Five interleaved native samples per method and fixture. Outputs are independently checked. Complete-root enumeration and first-witness decomposition have separate boundaries. Read the native JSON for all A/A, admission and paired-bootstrap decisions.",
        "Curves: " + "; ".join(f'n{r["n"]}: {r["slug"]}, {r["boolean_variables"]} Boolean variables, {r["factor_base_points"]} base points' for r in ids if r["n"] in [17,23]) + ". Full identities are derived by the native constructor in fixture-identities.json.",
        f'CPU: {data["host"]["cpu_model"]}; compiler: {data["host"]["compiler"].strip()}; reserved CPUs: {data["host"]["reserved_cpus"].strip()}. Full host/memory/ISA and source/binary manifests remain in the run receipts.',
        "Cold costs include fresh Semaev table/oracle preparation and query. Common curve/base construction is outside every solver boundary; these are not whole IC/rho pipeline times. Missing or capped runs are retained and do not decide that roots are absent.",
        "Final same-host run uses a dedicated c7i.2xlarge Spot VM, one Rayon worker and a complete reserved physical core. Host neighbours and frequency remain subject to A/A. Earlier hosts and attempted runs are separate evidence.",
        "The legacy random=true fixture is the unplanted curve generator, not a random group sample. TARGET_LABEL_CORRECTION.md corrects the earlier wording; raw flags and inputs are preserved. These particular targets do not establish an average-case ranking.",
        "Native source: final-summary.json; frozen protocol: FINAL_PROTOCOL.md. Baseline: 1340a6176. New table/setup switches remain opt-in. External Magma/msolve/FGb/PolyBoRi, Arm64 and GPU comparisons are not supplied by this run."]
    folder=HERE/"reports/run-20261019T014050Z"
    compile_report(folder,"Native Semaev setup and phase3 solver comparison",paragraphs,["native-semaev-setup.svg","phase3-complete-roots.svg","phase3-cold-decomposition.svg"],[])
    (folder/"REPORT.md").write_text("# Final native comparison\n\nRun 20261019-014050-boolean-native-batch-fin-43e52b.\n\n"+"\n\n".join(paragraphs)+"\n")
    shutil.copy2(folder/"REPORT.pdf",BENCH/"native-final.pdf")

def serial_report():
    path=HERE/"serial-clear-summary.json"
    if not path.exists():return
    data=json.loads(path.read_text())
    paired_plot([r for r in data["matrices"] if r["workers"]==1],"before","after","initial-serial-clearing","Initial serial clearing: one-worker observations")
    paired_plot([r for r in data["matrices"] if r["workers"]==4],"before","after","initial-serial-controls","Initial guard: four-worker controls include a regression")
    paragraphs=["Run 20261019-030905-boolean-native-serial-cl-25ad47. Same-binary KIC_GF2_SERIAL_CLEAR=0/1; single-write/vector tables stay enabled in both arms. Five baseline A/A and five A/B pairs for each F4/F5 matrix at n17 seed230 and n23 seeds230/231/232. Four-worker seed230 controls keep the original parallel path. Exact raw/canonical rows, rank and counted XOR/profile work agree.",
        f'Host: {data["host"]["cpu_model"]}; {data["host"]["compiler"].strip()}; c7i.2xlarge on-demand, reserved physical cores/SMT siblings: {data["host"]["reserved_cpus"].strip()}. The first condition queries the current Rayon pool. The parallel iterator remains selected with four workers; that does not establish unchanged cold runtime.',
        "All eight one-worker cells qualify faster. Primary n23 paired ratios are1.0770 F4 and1.0652 F5. The n17 F4 four-worker control qualifies slower at0.8436, CI[0.8284,0.8504]; the other three multiworker cells are neutral. This result blocks a regression-free claim for the unrestricted guard. A subsequent protocol narrows activation to explicitly configured one-worker processes before querying the registry; startup/scheduling is a hypothesis, not a proved diagnosis.",
        "Admission, first eligible of three attempts, five-pair exact bootstrap and max(2%, twice median absolute A/A noise) are unchanged. Rejected attempts and every slower/neutral/inconclusive result remain in serial-clear-summary.json. Negative-control movements are not interpreted as algorithm gains.",
        "This is an incremental matrix-call experiment. It does not measure complete recursive roots, decomposition, or a whole IC/rho pipeline. The eighteen-arm solver table retains its prior source and serial-clear-off flags; no complete-solver benefit is inferred from this kernel result. Source: SERIAL_CLEAR_PROTOCOL.md, run-serial-clear.sh, analyze-serial-clear.rs and runs/serial-clear/ in the raw archive. Both kernel-test switch states pass on the native host before timing."]
    rows=[]
    for r in data["matrices"]:
        c=r["comparison"]
        ratio=f'{c["reference_over_candidate"]:.4f}' if "reference_over_candidate" in c else "inconclusive"
        rows.append([r["arm"],str(r["n"]),str(r["seed"]),str(r["workers"]),f'{r["before"]["median_ms"]:.4f}',f'{r["after"]["median_ms"]:.4f}',ratio,c["decision"]])
    folder=HERE/"reports/run-20261019T030905Z"
    compile_report(folder,"Initial serial clearing and failed multiworker control",paragraphs,["initial-serial-clearing.svg","initial-serial-controls.svg"],[("All paired results, including the slower control",["Arm","n","Seed","Workers","Before ms","After ms","Ratio","Decision"],rows)])
    (folder/"REPORT.md").write_text("# Incremental serial row clearing\n\n"+"\n\n".join(paragraphs)+"\n\n|"+"|".join(["Arm","n","Seed","Workers","Before ms","After ms","Ratio","Decision"])+"|\n|"+"|".join(["---"]*8)+"|\n"+"\n".join("|"+"|".join(r)+"|" for r in rows)+"\n")
    shutil.copy2(folder/"REPORT.pdf",BENCH/"initial-serial-clearing.pdf")

def guard_report():
    path=HERE/"serial-guard-summary.json"
    if not path.exists():return
    data=json.loads(path.read_text())
    primary=[r for r in data["matrices"] if r["workers"]==1]
    controls=[r for r in data["matrices"] if r["workers"]==4]
    paired_plot(primary,"before","after","native-serial-clearing","Explicit-one-worker guard: incremental matrix calls")
    paired_plot(controls,"before","after","native-serial-negative","Refined guard: four-worker negative controls")
    paired_plot(primary,"legacy","after","native-legacy-to-candidate","Direct legacy to candidate; same binary, same host")
    control_status="All four multiworker controls are neutral under the frozen gate." if data["all_four_worker_controls_neutral"] else "At least one multiworker control moves outside the frozen neutral gate; no regression-free/default-change claim is admitted."
    paragraphs=["Run 20261019-040338-boolean-native-serial-gu-6ec5e9. Fresh same-binary confirmation after the initial four-worker regression. Five baseline A/A and five serial0/1 pairs per cell; each one-worker round additionally measures the legacy kernel with all three switches off. Exact input, raw/canonical output, rank, XOR and profile counts match. No host or earlier sample is pooled.",
        f'Host: {data["host"]["cpu_model"]}; {data["host"]["compiler"].strip()}; c7i.2xlarge on-demand; reserved cores/SMT siblings: {data["host"]["reserved_cpus"].strip()}. The refined guard caches the explicit RAYON_NUM_THREADS=1 configuration and checks the actual current pool before selecting serial work. With an explicit four-worker configuration, both switch states skip the registry query.',
        control_status,
        "All eight one-worker incremental cells qualify faster. Primary n23 ratios are1.0802 F4 and1.0730 F5. Direct same-host legacy to candidate ratios are1.1141 and1.1209, with CIs[1.0980,1.1297] and[1.0608,1.1402]. All12cells have five eligible pairs; eleven rejected attempts remain archived. These ratios do not reprice the earlier complete-solver table.",
        "Incremental pairs fix single-write/vector switches1 in both arms. Direct legacy ratios compare that round's all-off kernel with the candidate in the same binary, rather than multiplying previous gains. Admission, first eligible of three attempts, exact3125-resample CI and max(2%, twice A/A noise) are unchanged. Every slower, neutral or missing outcome remains in serial-guard-summary.json.",
        "The switch remains opt-in and requires explicit one-worker configuration; nested multiworker pools keep parallel iteration. Four worker/switch configurations pass the existing GF2 reference tests before native timing. These are matrix calls, not complete recursive roots or decomposition. The eighteen-arm cost table retains its earlier source and serial-clear-off flags. Source: SERIAL_GUARD_PROTOCOL.md, run-serial-guard.sh, analyze-serial-guard.rs and runs/serial-guard/ in the raw archive. The prior slower control is preserved in its own report."]
    ratio=lambda c:f'{c["reference_over_candidate"]:.4f}' if "reference_over_candidate" in c else "inconclusive"
    incremental=[[r["arm"],str(r["n"]),str(r["seed"]),str(r["workers"]),f'{r["before"]["median_ms"]:.4f}',f'{r["after"]["median_ms"]:.4f}',ratio(r["comparison"]),r["comparison"]["decision"]] for r in data["matrices"]]
    legacy=[[r["arm"],str(r["n"]),str(r["seed"]),f'{r["legacy"]["median_ms"]:.4f}',f'{r["after"]["median_ms"]:.4f}',ratio(r["legacy_over_candidate"]),r["legacy_over_candidate"]["decision"]] for r in primary]
    tables=[("Incremental pairs and four-worker controls",["Arm","n","Seed","Workers","Before ms","After ms","Ratio","Decision"],incremental),
        ("Direct legacy to candidate; one worker",["Arm","n","Seed","Legacy ms","Candidate ms","Ratio","Decision"],legacy)]
    folder=HERE/"reports/run-20261019T040338Z"
    compile_report(folder,"Fresh explicit-one-worker guard confirmation",paragraphs,["native-serial-clearing.svg","native-serial-negative.svg","native-legacy-to-candidate.svg"],tables)
    md="# Explicit-one-worker guard confirmation\n\n"+"\n\n".join(paragraphs)+"\n\n"
    for caption,headers,rows in tables:
        md+="## "+caption+"\n\n|"+"|".join(headers)+"|\n|"+"|".join(["---"]*len(headers))+"|\n"+"\n".join("|"+"|".join(row)+"|" for row in rows)+"\n\n"
    (folder/"REPORT.md").write_text(md.rstrip()+"\n")
    shutil.copy2(folder/"REPORT.pdf",BENCH/"native-serial-guard.pdf")

def policy_report():
    path=HERE/"policy-comparison-summary.json"
    if not path.exists(): return
    data=json.loads(path.read_text())
    policy=json.loads((HERE/"policy-summary.json").read_text())
    component=json.loads((HERE/"component.json").read_text())
    comparison_plot(data,"complete_roots","native-complete-roots","Final confirmation: complete original root sets")
    comparison_plot(data,"decomposition","native-cold-decomposition","Final confirmation: decomposition, oracle setup charged")
    primary=[r for r in policy["changes"] if r["seed"]==230]
    paired_plot(primary,"before","typed","native-typed-entry","Fresh confirmation: iteration1 JSON vs typed entry")
    paragraphs=[
        "Native F4/F5, F6 and both root-solving families; bounded public Koblitz m2 Boolean fixtures. Final same-host table: run 20261019-024352-boolean-native-policy-ty-fb16d2. Five admitted samples per arm, four primary fixtures, fresh paired planted holdouts231/232. Complete roots and first-witness decomposition have separate boundaries. Every output is checked against complete scalar roots or the original group equation.",
        f'Final host: {data["host"]["cpu_model"]}; {data["host"]["compiler"].strip()}; reserved CPUs {data["host"]["reserved_cpus"].strip()}; c7i.2xlarge on-demand, one Rayon worker, exclusive reserved physical core, isolation receipts and A/A. VM neighbours/frequency remain outside the reservation. Failed and contended attempts are retained, excluded, and counted in each JSON summary.',
        "n17: icv1-f2m17-tm101-00378d4e,16 Boolean variables,205 base points, group order 131,174. n23: icv1-f2m23-tm5197-1f85e9e1,22 Boolean variables,2071 base points, group order 8,393,806. Exact constructor output is fixture-identities.json. n19 is a 36-variable implementation-limit refusal, not an intermediate-size timing.",
        "Planted target uses seed230; generator control is the registered generator. The old driver flag random=true does not generate a random target for its single-target invocation. TARGET_LABEL_CORRECTION.md explicitly corrects that earlier label without rewriting frozen inputs or raw records. These cases do not establish an average-case ranking.",
        "Complete-root runs start from the prepared ANF or JSON representation for that API; system construction and JSON encoding are charged when rebuilt in the decomposition wrapper. Cold decomposition charges fresh target-dependent ANF construction, solving, lifting and verification; Semaev arms additionally charge fresh oracle/table preparation once. Common curve/base/context construction is outside every solver interval. Independent complete-root and final group-audit checks run after elapsed capture for all arms. Instrumented query timings include common driver metadata/wrapper overhead and are not bare-kernel latency. No whole-IC/rho ratio is supplied.",
        "The original single-write matrix route qualifies a small native improvement on the primary fixture. Extra SIMD table construction is neutral at both measured sizes and remains experimental. Semaev batch construction preserves every key, first witness, insertion order and abstract group charge; the n23 cold improvement is about 3.6x. Each before/after ratio is from its own same-host experiment; ratios across hosts are never multiplied or pooled.",
        "Iteration1 generalized JSON improvements borrow input, simplify exact evaluation and integrate the existing SIMD Gray kernel. Same-candidate ablations found SIMD slower at 16 variables and faster at 22; auto now keeps scalar below 22. Fresh confirmation measures that policy and the typed F2BoolPoly entry, which avoids JSON/BigUint conversion while retaining exact counts, output caps, budgets, original coordinates and root verification. This is engineering integration, not a new enumeration algorithm.",
        component["defaults"],
        "Fresh confirmation retains all outcomes: all 16 typed cells qualify faster than iteration1; n17 prepared JSON roots qualify faster on all four primary/holdout cases. n23 prepared JSON controls are neutral. One n23 planted JSON decomposition control regresses about 3.5% (before/after 0.9654); it is not hidden or described as regression-free. The typed route qualifies about 6x against that frozen wrapper on the same fixture.",
        "Native comparison/admission, statistics and ratios are computed by the Rust analyzers. Five paired ratios use all 3125 bootstrap resamplings with percentile indices78/3046; qualification additionally requires effect greater than max(2%, twice median absolute A/A noise). Charts show recorded medians and min/max; full CI, individual rounds and rejected attempts remain in JSON/raw archive. Python only renders these prepared values.",
        "Validation:13 standalone generalized-solver tests, strict Clippy, example build/tests, exact batched-map and binary/prime backend tests. Before the merge, 4,398 nonignored library tests were covered by passing executions: the8-thread suite had one unchanged time-budget failure, which passed serially; a separate serial full run passed4397 before the new map test. Post-merge latest-candidate serial run:4,397 passed,10 failed,118 ignored; nine failures were sandbox-denied network binds and one was an unchanged wall-threshold test. The network suite then passes38/38 with network access and the exact wall test passes serially. All 4,407 nonignored tests are covered by passing executions. The refined explicit-one-worker guard then passes a complete network-enabled serial library run:4,407 passed,0 failed,118 ignored, exit0. Its four worker/switch kernel configurations and SDK/Clippy/example checks pass. Earlier failing and interrupted invocations remain retained with their actual outcomes.",
        "Limits: no external Magma/msolve/FGb/Singular/PolyBoRi runtime comparison, Arm64/GPU measurement or whole-IC/rho promotion. The generalized native field/domain and full eight-coordinate S3 separator are functionally implemented; these m2 timings do not qualify m5 separator performance or the old20-case/8x hard all-roots research gate. The open problem record remains authoritative.",
        "Next measured bottlenecks: repeated F4/F5 matrix construction and recursive complete-root solving, plus the remaining native search kernel in typed generalized decomposition. The fresh typed wrapper is close to its prepared-input root cost; conversion/normalization was the dominant avoidable wrapper cost. Preserve output/accounting contracts and qualify any further layout or construction change with the same paired controls. Leaderboard component panels and browser are regenerated; tables A-C, curve facts and the IC/rho progress series are unchanged because the unit/boundary differs.",
        "Evidence: initial-summary.json and PROTOCOL.md (run 014036 baseline); optimization-summary.json and OPTIMIZATION_PROTOCOL.md (run 013858 first candidate); final-summary.json and FINAL_PROTOCOL.md (run 014050 Semaev); policy-summary.json, policy-comparison-summary.json and POLICY_PROTOCOL.md (run 024352 confirmation). Source/binary hashes, protocols, commands, every retry, preparation failures and corrected/failed auxiliary Callgrind profiles are retained. Callgrind Ir is not native runtime. See RAW_EVIDENCE.md for archive extraction and immutable job locations."
    ]
    evidence=paragraphs[-1]
    paragraphs.insert(5,"F6 is compared through the repository's first-witness decomposition APIs; no complete-original-root F6 API is benchmarked here. The original n23 planted system has two Boolean roots and the generator system has none. Exact complete-root output is a separate obligation from returning one group witness.")
    if (HERE/"serial-clear-summary.json").exists():
        paragraphs.append("The initial serial-clear matrix trial qualifies all eight one-worker cells faster but regresses the n17 F4 four-worker control (ratio0.8436). The unrestricted registry-query guard is not regression-free. serial-clear-summary.json preserves all outcomes. The refined source requires explicit one-worker configuration before querying Rayon; its fresh confirmation and direct legacy ablation have a separate protocol, summary and report. The eighteen-arm solver costs above precede both experiments with serial clearing off; they are not silently repriced.")
    if (HERE/"serial-guard-summary.json").exists():
        paragraphs.append("The fresh explicit-one-worker confirmation qualifies all eight one-worker incremental cells faster; all four four-worker controls are neutral. Primary n23 incremental ratios are1.0802 F4 and1.0730 F5. Direct legacy to candidate ratios on this same host are1.1141 F4 and1.1209 F5, rather than products of earlier experiments. All12cells have five eligible pairs; eleven rejected attempts remain. Activation stays opt-in. These matrix-call gains do not establish a complete-root or decomposition improvement.")
    rows=timing_rows(data)
    tables=[]
    for boundary in ["complete_roots","decomposition"]:
        for target in ["planted","generator"]:
            subset=[r for r in rows if r[0]==boundary and r[2]==target]
            tables.append((f"n23 {boundary}, {target}; five samples, ratio relative to F4",
                ["Method","Cold ms","n","F4 / method"],[[r[1],r[3],r[4],r[5]] for r in subset]))
    changes=[]
    for r in component["improvements"]:
        c=r["comparison"]
        ratio=f'{c["reference_over_candidate"]:.4f}' if "reference_over_candidate" in c else "inconclusive"
        changes.append([r["change"],r["target"],f'{r["before"]["median_ms"]:.4f}',f'{r["after"]["median_ms"]:.4f}',ratio,c["decision"]])
    tables.append(("Same-host paired changes; different rows are not pooled",["Change","Target","Before ms","After ms","Ratio","Decision"],changes))
    confirmations=[]
    for r in policy["changes"]:
        p=r["policy_change"];t=r["before_over_typed"]
        ratio=lambda c:f'{c["reference_over_candidate"]:.4f}' if "reference_over_candidate" in c else "inconclusive"
        confirmations.append([r["arm"],str(r["n"]),str(r["seed"]),"generator" if r["random"] else "planted",ratio(p),p["decision"],ratio(t),t["decision"]])
    tables.append(("Fresh cutoff and typed-entry confirmation, all holdouts",["Arm","n","Seed","Target","Old/JSON","Decision","Old/typed","Decision"],confirmations))
    images=["native-single-write.svg","native-vector-tables.svg","native-generalized-changes.svg","native-semaev-setup.svg","native-typed-entry.svg","native-complete-roots.svg","native-cold-decomposition.svg"]
    if (HERE/"serial-clear-summary.json").exists():images.extend(["initial-serial-clearing.svg","initial-serial-controls.svg"])
    if (HERE/"serial-guard-summary.json").exists():images.extend(["native-serial-clearing.svg","native-serial-negative.svg","native-legacy-to-candidate.svg"])
    compile_report(HERE,"Native F4/F5, F6 and root solvers",paragraphs,images,tables)
    shutil.copy2(HERE/"REPORT.pdf",BENCH/"REPORT.pdf")
    md="# Native F4/F5, F6 and root solvers\n\n"+"\n\n".join(paragraphs)+"\n\n"
    for caption,headers,items in tables:
        md+="## "+caption+"\n\n|"+"|".join(headers)+"|\n|"+"|".join(["---"]*len(headers))+"|\n"
        md+="\n".join("|"+"|".join(r)+"|" for r in items)+"\n\n"
    md+="\n".join(f'![Recorded native values]({name})' for name in images)+"\n"
    (HERE/"REPORT.md").write_text(md)
    folder=HERE/"reports/run-20261019T024352Z"
    run_paragraphs=paragraphs[:6]+paragraphs[7:11]+[evidence]
    compile_report(folder,"Fresh native cutoff, typed-entry and solver comparison",run_paragraphs,
        ["native-typed-entry.svg","native-complete-roots.svg","native-cold-decomposition.svg"],tables[:4])
    (folder/"REPORT.md").write_text("# Fresh native confirmation\n\n"+"\n\n".join(run_paragraphs))
    shutil.copy2(folder/"REPORT.pdf",BENCH/"native-policy.pdf")

if __name__=="__main__":
    preparation_reports()
    run_reports()
    profile_reports()
    final_report()
    serial_report()
    guard_report()
    policy_report()
