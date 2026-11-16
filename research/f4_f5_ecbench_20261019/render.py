"""Layout only: plot recorded values and Rust-prepared summaries."""
from pathlib import Path
import json
import shutil
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

HERE = Path(__file__).resolve().parent
BENCH = HERE.parents[1] / "benchmarks/20261019-f4-f5-recursive-ecbench"
BENCH.mkdir(parents=True, exist_ok=True)
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 9})
plt.rcParams["svg.hashsalt"] = "f4-f5-recursive-ecbench-20261019"
COLORS = ["#829bac", "#477e98", "#146185"]

def read(name):
    return json.loads((HERE/name).read_text())

def save(fig, name):
    for suffix, opts in [("svg", {"metadata": {"Date": None}}), ("png", {"dpi": 150})]:
        path = HERE/f"{name}.{suffix}"
        fig.savefig(path, bbox_inches="tight", **opts)
        shutil.copy2(path, BENCH/path.name)
    plt.close(fig)

def esc(text):
    return str(text).replace("\\", "\\\\").replace('"', '\\"')

def report(folder, title, paragraphs, images, tables, copy_name):
    folder.mkdir(parents=True, exist_ok=True)
    typ = ['#set document(date: none)', '#set page(paper: "a4", margin: 15mm, numbering: "1")',
           '#set text(font: "DejaVu Sans", size: 9pt)',
           '#set par(justify: false)', f'#heading(level: 1)[{title}]']
    md = ["# "+title, "", *[p+"\n" for p in paragraphs]]
    for p in paragraphs:
        typ.append(f'#text("{esc(p)}")\n\n')
    import os
    for index, name in enumerate(images):
        if index:
            typ.append('#pagebreak(weak: true)')
        path = os.path.relpath(HERE/name, folder)
        typ.append(f'#image("{path}", width: 100%)')
        md.extend([f'![{name}]({path})', ""])
    for title, headers, rows in tables:
        typ.extend(['#pagebreak(weak: true)', f'#heading(level: 2)[{title}]'])
        def cell(text):
            text=str(text).replace('_','_\u200b').replace('-','-\u200b')
            return f'text("{esc(text)}")'
        typ.append('#set text(size: 7.5pt)')
        typ.append('#table(columns: ('+', '.join(['1fr']*len(headers))+'), inset: 3pt, table.header('
            +', '.join(cell(c) for c in headers)+'), '
            +', '.join(cell(c) for row in rows for c in row)+')')
        typ.append('#set text(size: 9pt)')
        md.extend(["## "+title, "", "|"+"|".join(headers)+"|", "|"+"|".join(["---"]*len(headers))+"|",
                   *["|"+"|".join(str(c) for c in row)+"|" for row in rows], ""])
    (folder/"REPORT.typ").write_text("\n".join(typ)+"\n")
    (folder/"REPORT.md").write_text("\n".join(md).rstrip()+"\n")
    typst.compile(str(folder/"REPORT.typ"), output=str(folder/"REPORT.pdf"), root=str(HERE))
    shutil.copy2(folder/"REPORT.pdf", BENCH/copy_name)

def number(stat, field="median_ms"):
    return f'{stat[field]:.4f}' if field in stat else "missing"

def ratio(value):
    return f'{value["reference_over_candidate"]:.4f}' if "reference_over_candidate" in value else "inconclusive"

def paired_plot(rows, name, title):
    fig, ax = plt.subplots(figsize=(10, max(3.8, len(rows)*.42)))
    labels = [f'{r["arm"]} n{r["n"]} s{r["seed"]} '+("generator" if r["random"] else "planted") for r in rows]
    for variant, key in enumerate(["before", "prepared", "candidate"]):
        positions = [i+(variant-1)*.23 for i in range(len(rows))]
        ax.barh(positions, [r[key].get("median_ms", 0) for r in rows], height=.22, color=COLORS[variant], label=["Baseline", "Prepared", "Quadratic tail"][variant])
        for pos,r in zip(positions,rows):
            if r[key].get("n"):
                ax.hlines(pos,r[key]["min_ms"],r[key]["max_ms"],color="black",linewidth=.7)
    ax.set_yticks(range(len(rows)), labels)
    ax.invert_yaxis(); ax.set_xscale("log"); ax.set_xlabel("Native complete-call ms; median bars and min/max lines; five paired rounds")
    ax.set_title(title); ax.legend(fontsize=8); ax.grid(axis="x", alpha=.2)
    save(fig, name)

def costs_plot(data, boundary, name):
    fig, axes = plt.subplots(2, 2, figsize=(11, 8.5))
    for ax, (n, random) in zip(axes.flat, [(17,0),(17,1),(23,0),(23,1)]):
        rows = [r for r in data["costs"] if r["boundary"]==boundary and r["n"]==n and r["random"]==random]
        ax.barh(range(len(rows)), [r["cold"].get("median_ms", 0) for r in rows], color=COLORS[2])
        for pos,r in enumerate(rows):
            if r["cold"].get("n"):
                ax.hlines(pos,r["cold"]["min_ms"],r["cold"]["max_ms"],color="black",linewidth=.7)
        ax.set_yticks(range(len(rows)), [r["arm"] for r in rows], fontsize=8)
        ax.invert_yaxis(); ax.set_xscale("log"); ax.grid(axis="x", alpha=.2)
        ax.set_xlabel("Cold ms; median bars, min/max lines; five samples")
        ax.set_title(f'n{n}, '+("generator control" if random else "planted"))
    fig.suptitle("Latest policies enabled: "+("complete original roots" if boundary=="complete_roots" else "first-witness decomposition, setup charged"))
    fig.tight_layout(); save(fig, name)

def local_report():
    audit = read("local-smoke.audit.json")
    assert audit["ok"]
    fig, ax = plt.subplots(figsize=(9, 3.5))
    ax.bar(["Native audit records", "Verified DLP recovery records"], [audit["records"],audit["verified_records"]], color=COLORS[2])
    ax.set_ylabel("Recorded completed cases"); ax.set_title("Local eleven-arm plumbing: L0, no native speed claim")
    save(fig, "local-recovery-controls")
    report(HERE/"reports/run-20261019T1652Z", "Local complete-recovery correctness controls",
        ["Eleven arms, two public n11 targets; every one of the22 records verifies, and all22 audit replays reproduce. This is a plumbing/correctness session at L0, not a performance result.",
         "The separate integration test recovers and replays32 DLP cases across Koblitz and generic binary curves. New Gray work appears as unpriced native counters; old method defaults remain unchanged.",
         "Source0acfc55b7f; local CPU/compiler and binary hashes are adjacent. Raw sealed session: runs/local-smoke; audit: local-smoke.audit.json. Shared local elapsed time is not quoted."],
        ["local-recovery-controls.svg"], [], "local-recovery-controls.pdf")
    rows = [json.loads((HERE/f"diagnostic-{arm}-n{n}-{policy}.jsonl").read_text())
            for arm in ["solve-f4", "solve-f5"] for n in [17,23] for policy in ["baseline","prepared","quadratic-tail"]]
    fig, axes = plt.subplots(1,2,figsize=(11,4))
    for ax, field, label in [(axes[0], "calls", "Macaulay matrix calls"), (axes[1], "word_ops", "Counted elimination word XORs")]:
        for i, policy in enumerate(["baseline","prepared","quadratic-tail"]):
            selected = [r for r in rows if r["matrix_runtime"]==policy]
            ax.bar([j+(i-1)*.23 for j in range(len(selected))], [r["profile"][field] for r in selected], width=.22, color=COLORS[i], label=policy)
            ax.set_xticks(range(len(selected)), [f'{r["arm"]}\nn{r["n"]}' for r in selected])
        ax.set_yscale("log"); ax.set_ylabel(label); ax.legend(fontsize=7)
    fig.suptitle("Local deterministic work diagnostics; generator controls, not speed")
    fig.tight_layout(); save(fig,"local-matrix-work")
    table = [[r["arm"],r["n"],r["matrix_runtime"],r["profile"]["calls"],r["profile"]["word_ops"],r["stats"]["tail_assignments_tested"]] for r in rows]
    report(HERE/"reports/run-local-work-diagnostics", "Construction and bounded-tail work diagnostics",
        ["Same n17/n23 generator systems; baseline, prepared layout/top-degree/linear-tail/direct-packing package, and bounded quadratic-tail package. Every original root set verifies equal. Shared-host native wall values are retained raw but not admitted or quoted.",
         "The n23 complete-root tree uses1022 matrices before,511 with prepared policies and63 with the quadratic tail. The tail also performs4194304 Gray assignments. Those are different work units; fewer matrices alone is not a speed result. Both matrix engines show the same count pattern here.",
         "These diagnostics motivated the frozen dedicated-host runs; they do not substitute for five paired native timings or full DLP recovery. Sources: diagnostic-*.jsonl, frozen baseline and candidate manifests."],
        ["local-matrix-work.svg"], [("Recorded work, including enumeration",["Arm","n","Policy","Matrices","Word XORs","Gray assignments"],table)], "local-matrix-work.pdf")

def native_report(summary="native-summary.json", prefix="", job="20261019-190236-boolean-adaptive-native-d28a77", source="9bed104b0a", folder="run-20261019T190236Z"):
    if not (HERE/summary).exists(): return
    data = read(summary)
    for boundary, name in [("complete_roots","latest-complete-roots"),("decomposition","latest-decomposition")]: costs_plot(data,boundary,prefix+name)
    for boundary, arms in [("roots",["solve-f4","solve-f5"]),("decomposition",["decomp-f4","decomp-f5"])]:
        rows = [r for r in data["changes"] if r["arm"] in arms]
        paired_plot(rows,prefix+"paired-"+boundary,"Fresh same-host baseline / prepared / quadratic-tail: "+boundary)
    changes = [[r["arm"],r["n"],r["seed"],"G" if r["random"] else "P",number(r["before"]),number(r["candidate"]),ratio(r["comparison"]),r["comparison"]["decision"]] for r in data["changes"]]
    controls = [[r["arm"],r["n"],number(r["before"]),number(r["after"]),ratio(r["comparison"]),r["comparison"]["decision"]] for r in data["four_worker_controls"]]
    phases=[]
    for r in data["changes"]:
        if r["seed"]!=230: continue
        for policy,key in [("baseline","before_profile"),("prepared","prepared_profile"),("quadratic-tail","candidate_profile")]:
            phases.append([r["arm"],r["n"],"G" if r["random"] else "P",policy,
                *[number(r[key].get(k,{})) for k in ["build_ns","reduce_ns","readback_ns"]],
                str(r[key].get("calls",{}).get("median","missing"))])
    cost_rows=[]
    for boundary in ["complete_roots","decomposition"]:
        names=[r["arm"] for r in data["costs"] if r["boundary"]==boundary and r["n"]==17 and r["random"]==0]
        for arm in names:
            values=[next(r["cold"] for r in data["costs"] if r["arm"]==arm and r["n"]==n and r["random"]==random) for n,random in [(17,0),(17,1),(23,0),(23,1)]]
            cost_rows.append([boundary,arm,*[number(v)+f' ({v["n"]})' for v in values]])
    paragraphs = [f"Run {job}, source {source}. Dedicated c7i.2xlarge, one worker and a reserved complete physical core; four-worker controls reserve two cores/SMT siblings. Exact source/compiler/CPU/ISA and binary manifests remain in the raw tree. Baseline: frozen a78 algorithm with all three previously qualified GF2 switches. Candidate: identity-bound prepared construction and bounded quadratic-tail policies.",
        "Original n17/n23 fixtures have16/22 Boolean variables on icv1-f2m17-tm101-00378d4e / icv1-f2m23-tm5197-1f85e9e1. Planted seed230 and fixed generator control plus planted holdouts231/232; random=true means generator here. n19's36-variable admission gap remains separate. No fixture or baseline is replaced by the full-DLP standard subspaces.",
        "All eighteen arms are rerun with this source's declared policies. Complete original roots and first-witness decomposition remain separate; common curve/base construction is outside component timings. Fresh target ANF, solving, lifting and internal verification are charged to decomposition; Semaev fresh table setup is charged once. Independent original-root/group audits run after elapsed capture.",
        f'Five paired rounds, first eligible of at most three attempts, exact original outputs, exact3125-resample CI and max(2%, twice A/A noise) gate. Rejected attempts retained: {len(data["rejected_attempts"])}. All four multiworker controls neutral: {data["all_four_worker_controls_neutral"]}. Every slower/neutral/missing cell remains in native-summary.json.',
        "Prepared is a combined active-layout/top-degree/linear-tail/direct-packing ablation. Quadratic-tail retains an algebraic root reduction, then exhaustively completes at most16 free quadratic variables using the existing Gray kernel; original coordinates, root order, caps and verification remain enforced. This is a hybrid engineering change, not a novel enumeration algorithm. Other split rules/custom node oracles keep recursion.",
        f"The matrix switches stay opt-in. Source/protocols: {source}, PROTOCOL.md and the frozen runner/protocols in the raw tree. Sources0acfc55b7f,71c4c99d85 and9bed104b0a are separate experiments. The final adaptive variants preserve cheap degree-two refutations below16 original variables; legacy policy names remain for replay. Full-table setup uses the same exact sorted tuples serially under an explicit one-worker resource contract. No cross-host pooling, external-fastest, Arm64/GPU, whole-IC/rho or hard-family8x conclusion follows from component calls."]
    tables = [("All paired recursive-solver outcomes",["Arm","n","Seed","Target","Before ms","Candidate ms","Ratio","Decision"],changes),
              ("Latest native complete-call ms (sample count)",["Boundary","Method","n17 planted","n17 generator","n23 planted","n23 generator"],cost_rows),
              ("Four-worker matrix controls",["Arm","n","Before ms","After ms","Ratio","Decision"],controls),
              ("Instrumented matrix phase medians, ms; separate nonadditive observations",["Arm","n","Target","Policy","Build ms","Reduce ms","Readback ms","Calls"],phases)]
    if data.get("full_table_four_worker_controls"):
        rows=[[r["arm"],r["n"],number(r["before"]),number(r["after"]),ratio(r["comparison"]),r["comparison"]["decision"]] for r in data["full_table_four_worker_controls"]]
        tables.append(("Full-table four-worker cold-call controls",["Arm","n","Before ms","After ms","Ratio","Decision"],rows))
    report(HERE/"reports"/folder,"Native F4/F5, F6 and root solvers", paragraphs,
           [prefix+n for n in ["paired-roots.svg","paired-decomposition.svg","latest-complete-roots.svg","latest-decomposition.svg"]],tables,prefix+"native-rerun.pdf")

def profile_report():
    if not (HERE/"profiles-summary.json").exists(): return
    data=read("profiles-summary.json")
    fig,ax=plt.subplots(figsize=(9,4))
    for variant,key in enumerate(["before","prepared","candidate"]):
        ax.barh([i+(variant-1)*.24 for i in range(len(data["rows"]))],[r[key].get("median_ir",0) for r in data["rows"]],height=.23,color=COLORS[variant],label=["Baseline","Prepared","Quadratic tail"][variant])
    ax.set_yticks(range(len(data["rows"])),[r["arm"] for r in data["rows"]]);ax.invert_yaxis();ax.set_xscale("log")
    ax.set_xlabel("Client-request region instructions (Ir); five paired complete-root calls")
    ax.set_title("Auxiliary n23 planted instruction work; simulated AVX2");ax.legend(fontsize=8);ax.grid(axis="x",alpha=.2)
    save(fig,"complete-root-instructions")
    rows=[[r["arm"],str(r["before"].get("median_ir","missing")),str(r["prepared"].get("median_ir","missing")),str(r["candidate"].get("median_ir","missing")),str(r["complete"])] for r in data["rows"]]
    report(HERE/"reports/run-20261019T1713Z","Complete-root instruction profiles",[
        "Original n23 planted seed230, F4/F5 complete-root calls. Five interleaved baseline/prepared/quadratic-tail profiles per arm; every root set verifies equal. Local Xeon6975P-C, Valgrind3.19, merged threads, one worker, simulated AVX2. This is instruction-count evidence and supplies no native time, cycles or IPC.",
        "The first-tail implementation reduces median call Ir from4.525/4.556 billion to410.240/413.511 million for F4/F5. Prepared-only medians are1.505/1.523 billion. All five rounds per arm complete without failures. These are different implementations' complete instruction work; elimination word counts alone omit the newly explicit Gray assignments.",
        "The hot-function annotation identifies remaining matrix reduction, column lookup and direct packing work. LOOKUP_PROTOCOL.md freezes a subsequent dispatch refinement before its profiles. Neither profile batch reprices the dedicated native or ecbench clocks. Sources: profiles-summary.json, runs/profiles raw data, binary/source manifests and PROFILE_PROTOCOL.md."],
        ["complete-root-instructions.svg"],[("Recorded complete-call medians",["Arm","Baseline Ir","Prepared Ir","First-tail Ir","Five pairs"],rows)],"complete-root-instructions.pdf")
    if (HERE/"lookup-summary.json").exists():
        lookup=read("lookup-summary.json")
        fig,ax=plt.subplots(figsize=(9,3.5))
        for i,key in enumerate(["before_ir","after_ir"]):
            ax.barh([j+(i-.5)*.3 for j in range(len(lookup["rows"]))],[r.get(key) or 0 for r in lookup["rows"]],height=.29,color=COLORS[i+1],label=["First tail","Lookup refinement"][i])
        ax.set_yticks(range(len(lookup["rows"])),[r["arm"] for r in lookup["rows"]]);ax.invert_yaxis()
        ax.set_xlabel("Complete-root Ir; five interleaved pairs");ax.set_title("Column representation selected outside product loops");ax.legend(fontsize=8)
        save(fig,"column-lookup-instructions")
        report(HERE/"reports/run-lookup-instructions","Column-index dispatch refinement",[
            "Same original n23 planted complete-root fixture and first-tail policy in both binaries. The representation dispatch moves outside the product term loop; all three lookup types preserve exact support, fallback, root order, caps and deterministic work counters.",
            f'The frozen profile gate requires five complete pairs and more than2% lower median Ir on both arms. Retain refinement: {lookup["retain_refinement"]}. All rows and failures remain in lookup-summary.json. This is not native-time qualification; retained changes require fresh full native/ecbench panels.',
            "Valgrind merged-thread simulated AVX2 Ir, one worker; binary hashes, commands and original audits in runs/lookup-profiles/. Source: LOOKUP_PROTOCOL.md. No cross-host ratio multiplication or runtime extrapolation.",
            "The refined F4 matrix driver accounts for51.69% of complete-call Ir in the retained function annotation. That symbol includes inlined work and does not isolate reduction from construction; the native build/reduce/readback timers provide the phase diagnosis. Annotation: records/f4-refined-hot-functions.txt."],
            ["column-lookup-instructions.svg"],[("Frozen instruction gate",["Arm","Before Ir","After Ir","Pairs","Retain"],[[r["arm"],str(r["before_ir"]),str(r["after_ir"]),r["pairs"],str(r["retain"])] for r in lookup["rows"]])],"column-lookup-instructions.pdf")

def ecbench_report(summary="ecbench-summary.json", prefix="", job="20261019-190253-boolean-adaptive-ecbench-cbeb18", source="9bed104b0a", folder="run-20261019T190253Z"):
    if not (HERE/summary).exists(): return
    data=read(summary)
    fig, axes=plt.subplots(2,2,figsize=(11,9))
    table=[]
    for row, session in enumerate(data["sessions"]):
        arms=session.get("arms",[])
        for ax,key,title in [(axes[row,0],"cold_solve","Cold complete DLP solve"),(axes[row,1],"process","Whole child process")]:
            ax.barh(range(len(arms)),[a[key].get("median_ms",0) for a in arms],color=COLORS[2]);ax.invert_yaxis()
            for pos,a in enumerate(arms):
                if a[key].get("n"):
                    ax.hlines(pos,a[key]["min_ms"],a[key]["max_ms"],color="black",linewidth=.7)
            ax.set_yticks(range(len(arms)),[f'{a["name"]} [{a["admitted"]}/{a["requested"]}]' for a in arms],fontsize=8);ax.set_xscale("log");ax.set_xlabel("Recorded native ms; L2 rows only; min/max lines")
            for pos,a in enumerate(arms):
                if not a[key].get("n"):
                    ax.text(.02,pos,"no admitted clock",transform=ax.get_yaxis_transform(),fontsize=7,color="#9a4433")
            ax.set_title(f'n{session["n"]}: {title}');ax.grid(axis="x",alpha=.2)
        for arm in arms:
            table.append([session["n"],arm["name"],number(arm["cold_solve"]),number(arm["process"]),arm["verified"],arm["admitted"],arm["requested"],str(arm["complete"])])
    fig.tight_layout();save(fig,prefix+"ecbench-cold-recovery")
    comparisons=[]
    prices=[]
    counters=[]
    phases=[]
    for session in data["sessions"]:
        for arm in session.get("arms",[]):
            prices.append([session["n"],arm["name"],*[f'{v:.5g}' if v is not None else 'missing' for v in arm["s_sensitivity_mean"]],str(arm["lower_bound"]),len(arm["unquantified"])])
            for unit,count in arm["unpriced_counts"].items():
                counters.append([session["n"],arm["name"],unit,count["n"],count["median"],count["min"],count["max"]])
            for phase,time in arm["phase_wall"].items():
                phases.append([session["n"],arm["name"],phase,number(time),number(time,"min_ms"),number(time,"max_ms"),time["n"]])
        for entry in session.get("comparisons",[]):
            result=entry["result"]
            if result:
                wall=result["wall"]
                point=f'{wall["median_ratio"]:.4f}' if wall.get("median_ratio") is not None else "missing"
                interval='..'.join(f'{v:.4f}' for v in wall["ci95"]) if wall.get("ci95") else "missing"
                comparisons.append([session["n"],entry["pair"],wall["status"],point,interval,str(wall.get("outside_noise")),str(wall.get("pairs_below_level", "missing"))])
    paragraphs=[f"Run {job}, source {source}. Complete one-target cold DLP recovery on dedicated c7i.2xlarge; native ecbench L2 requirement, one worker,8 independent public targets per curve,5 measured rounds plus1 warmup,11 interleaved arms. Sources/specs and complete audit certificates remain adjacent/raw. Missing, failed and below-level rows are retained; a launch is not a result.",
        "All methods within each session share the same public targets and standard polynomial subspace: dimension5 at n17 and6 at n23. These bases differ from the component experiment's Frobenius-invariant bases. The curve identities are icv1-f2m17-tm101-00378d4e and icv1-f2m23-tm5197-1f85e9e1. This two-size panel is not a scaling claim.",
        "The pipeline solve clock includes factor-base construction, oracle prepare, relation collection, linear algebra, target descent/recovery and internal verification. Whole child process additionally includes adapter construction, report metadata, common workload construction and startup. Both clocks are recorded; the native harness's comparison gate uses the pipeline solve clock. Independent parent verification/replay, compilation and warmup are outside these clocks.",
        "F4/F5 baseline uses the prior recursive policy with qualified GF2 kernels. Sources0acfc55b7f/71c4c99d85 use candidate matrix_runtime=quadratic-tail; corrected source9bed104b0a uses adaptive-quadratic-tail, with a different method identity. F6 names kernel_runtime=native-v1. Generalized typed roots and both Semaev table backends participate. Environment-override refusal remains intact; no admission threshold is relaxed.",
        "Wall ratios below are B/A from ecbench's own paired bootstrap and A/A gate. Admission, completeness and outside_noise must all support a gain; partial/failed sessions do not become successful comparisons. Process medians are supplementary observed clocks, not a replacement gate.",
        "Counted S, generic floor and matched strong rho are retained in table-n*.txt and ecbench-summary.json. Gray assignments are explicitly counted unpriced; reference native unpriced counters are retained too. Sensitivity bands at0/0.1/1 GAE per recorded operation are diagnostic and deduplicate generalized solver-count aliases. An unquantified counter stays unquantified. No ratio of lower bounds is a full-cost ranking.",
        "Exact original-output and replay checks, query-count comparisons, source/binary hashes, failures and job resources remain in the evidence. All-L2 coverage is separate from execution/verification completeness; a fully verified session can have below-level timing rows. Low-sample clocks are descriptive; no threshold is relaxed. New matrix policies stay opt-in; no external-fastest, other-hardware or rho-beating claim is made."]
    for session in data["sessions"]:
        paragraphs.append(f'n{session["n"]}: {session.get("records_total","missing")} total records, {session.get("warmups","missing")} warmups; replay audit OK {session.get("audit",{}).get("ok")}; all requested L2 timing coverage {session["complete"]}.')
        for entry in session.get("comparisons",[]):
            if entry["pair"] in ["f4","f5"] and entry.get("result"):
                wall=entry["result"]["wall"]
                point=f'{wall["median_ratio"]:.4f}' if wall.get("median_ratio") is not None else 'missing'
                paragraphs.append(f'n{session["n"]} {entry["pair"]}: candidate/baseline pipeline wall {point}, CI95 {wall.get("ci95")}, gate {wall["status"]}, outside A/A noise {wall.get("outside_noise")}. Ratios above1 are regressions, below1 are reductions in time; admission/completeness still govern interpretation.')
    report(HERE/"reports"/folder,"Complete cold DLP runtime through ecbench",paragraphs,[prefix+"ecbench-cold-recovery.svg"],
        [("Recorded clocks and admission",["n","Arm","Solve ms","Process ms","Verified","L2","Requested","All L2"],table),
         ("Native harness comparisons; B/A",["n","Pair","Wall gate","Median ratio","CI95","Outside noise","Below L2"],comparisons),
         ("Partly priced S sensitivity; all entries diagnostic",["n","Arm","0 GAE/op","0.1 GAE/op","1 GAE/op","Lower bound","Unknown entries"],prices),
         ("Explicit unpriced operation ledgers; different units are not equated",["n","Arm","Unit","Samples","Median count","Min","Max"],counters),
         ("Admitted phase clocks; recorded ms",["n","Arm","Phase","Median ms","Min ms","Max ms","Samples"],phases)],prefix+"ecbench-full-recovery.pdf")

def screen_reports():
    for summary, prefix, folder, source in [
        ("adaptive-screen-summary.json","screen-v0-","run-local-adaptive-screen","frozen-adaptive-screen/koblitz_groebner.rs (pre-version guarded source)"),
        ("versioned-screen-summary.json","screen-versioned-","run-local-versioned-screen","9bed104b0a")]:
        if not (HERE/summary).exists(): continue
        data=read(summary);rows=data["work_rows"]
        fig,ax=plt.subplots(figsize=(10,4))
        for i,key in enumerate(["baseline_word_xors","failed_policy_word_xors","adaptive_word_xors"]):
            ax.bar([j+(i-1)*.23 for j in range(len(rows))],[r[key] for r in rows],width=.22,color=COLORS[i],label=["Baseline","Failed all-width policy","Adaptive degrees"][i])
        ax.set_xticks(range(len(rows)),[f'{r["engine"]} n{r["n"]}' for r in rows]);ax.set_yscale("log");ax.set_ylabel("Recorded solver word XORs");ax.set_title("One matched public target per curve; work/correctness only");ax.legend(fontsize=8)
        save(fig,prefix+"word-work")
        table=[[r["n"],r["engine"],r["solver_calls"],r["baseline_word_xors"],r["failed_policy_word_xors"],r["adaptive_word_xors"],str(r["query_counts_and_answer_equal"])] for r in rows]
        report(HERE/"reports"/folder,"Cheap degree-two refutations: local work screen",[
            f'Source {source}. Same registered curves, standard bases and public target0/round1 as the full panel; one warmup and one measured round, four arms per curve. All16 executions verify, including8 warmups; all8 measured replays reproduce. Gate passed: {data["gate_passed"]}. The native replay-all mode excludes warmups. This corrects earlier prose equating total verified records with measured replays. No L0 elapsed-time or pipeline speedup claim follows.',
            "The failed all-width degree-three policy spends3.000 billion word XORs on the n23 F4 target; the guarded policy spends62.421 million, against79.418 million baseline. The old native full-pipeline regression remains in its own report; logical work counts here are not native wall time.",
            "Both original16/22-variable fixtures retain identical complete roots and all checked deterministic counters against source71c4c99d85. The final corrected policy receives its own identity; pre-version screen records keep their sealed bytes and frozen-source replay. Sources: the adjacent Rust-prepared summary, ADAPTIVE_PROTOCOL.md and the raw screen tree."],
            [prefix+"word-work.svg"],[("Matched target work",["n","Engine","Calls","Baseline XORs","Failed XORs","Adaptive XORs","Queries/answer equal"],table)],prefix+"work-screen.pdf")
    fig,ax=plt.subplots(figsize=(10,2.8));ax.axis("off")
    for x,text in [(0.02,"Source/spec manifests\nwritten"),(.36,"Default global lock\nopen denied"),(.70,"No session created\nexit2 retained")]:
        ax.text(x,.5,text,transform=ax.transAxes,bbox={"boxstyle":"round,pad=.7","facecolor":"#e9eef1","edgecolor":"#477e98"},fontsize=10)
    save(fig,"local-screen-lock-failure")
    report(HERE/"reports/run-local-screen-lock-failure","Local screen preflight failure",[
        "The first L0 screen stopped before creating a session: /tmp/crypto-bench.lock was not writable, exit2. Source/spec/binary manifests and stderr remain in runs/adaptive-screen. No solver result or elapsed-time comparison was produced.",
        "The separate retry uses a private workspace lock, --cpus none and --allow-busy for correctness/work only. No isolation gate is bypassed for a native claim; the dedicated final panels retain their original gates. Both attempts remain evidence."],
        ["local-screen-lock-failure.svg"],[],"local-screen-lock-failure.pdf")

def aggregate():
    images=["local-matrix-work.svg"]
    for name in ["latest-complete-roots.svg","latest-decomposition.svg","ecbench-cold-recovery.svg"]:
        if (HERE/name).exists(): images.append(name)
    paragraphs=["Continuation of the original Boolean matrix study: direct cached F4/F5 packing, bounded quadratic recursive tails, specialized column lookup, adaptive degree scheduling, serial one-worker full-table setup, a fresh eighteen-arm native comparison and full ecbench DLP recovery. Final source9bed104b0a and frozen baseline a78cc8906; earlier sources0acfc55b7f/71c4c99d85 remain separate intermediate evidence, including their full-pipeline regressions.",
        "Matrix-F4/F5 here are degree-bounded algebraic solvers with splitting; the new tail is an explicit hybrid with the existing scalar Gray search. Complete roots and first-witness decomposition are separate tasks. Full ecbench recovery additionally includes every DLP phase and uses separately declared standard-subspace bases.",
        "Validation:4,414 release library tests passed,0 failed,118 ignored;17 end-to-end integration tests passed, including48 exact policy/generalized recoveries/replays. Standalone SDK13 tests and strict Clippy pass. The legacy n17 session verifies528 records and reproduces440 measured runs with the corrected current binary. Both local adaptive screens verify16 records each (8 warmups) and reproduce all8 measured runs; the pre-version screen uses its frozen source. Strict root Clippy still fails on one unchanged PMAC lint; its separately retained scoped diagnostic passes with only that existing lint allowed, without waiving the strict failure. Merged-owner backlog integration with pinned Cairn9.1.0 passes13/15; two tests fail on an unchanged baseline quintic card with an unbound artifact placeholder. Cover assessment verification and refreshed-lockfile metadata checks pass.",
        "Native comparisons and admission are prepared in Rust; Python only lays out recorded summaries. Five same-host paired rounds and A/A remain mandatory; Semaev setup is cold charged. The ecbench harness supplies its own sealed-session audit and comparison admission. No local unisolated wall, cross-host product, lower-bound ratio or stage-only result is promoted to a full-pipeline speedup.",
        "The raw archive, exact commands and source/binary/host manifests preserve reproducibility. All new policies are explicit and opt-in, preserving old method defaults and frozen evidence. External fastest comparisons, Arm64/GPU, n19's36-variable matrix admission, and the old hard-family/8x research gates remain open."]
    if (HERE/"native-summary.json").exists():
        native=read("native-summary.json")
        for arm in ["solve-f4","solve-f5"]:
            row=next(r for r in native["changes"] if r["arm"]==arm and r["n"]==23 and r["seed"]==230 and r["random"]==0)
            paragraphs.insert(1,f'n23 planted {arm}, complete roots: baseline {number(row["before"])}ms; final {number(row["candidate"])}ms; paired baseline/final {ratio(row["comparison"])}; CI95 {row["comparison"].get("ci95","missing")}; decision {row["comparison"]["decision"]}. Five same-host paired rounds; all other inputs and controls remain in the native run report.')
    if (HERE/"ecbench-summary.json").exists():
        ecbench=read("ecbench-summary.json")
        for session in ecbench["sessions"]:
            for entry in session.get("comparisons",[]):
                if entry["pair"] in ["f4","f5"] and entry.get("result"):
                    wall=entry["result"]["wall"]
                    point=f'{wall["median_ratio"]:.4f}' if wall.get("median_ratio") is not None else "missing"
                    paragraphs.append(f'Full ecbench n{session["n"]} {entry["pair"]}, candidate/baseline cold solve time: {point}; CI95 {wall.get("ci95")}; admission {wall["status"]}, outside A/A noise {wall.get("outside_noise")}. All requested L2 timing coverage: {session["complete"]}; {wall.get("pairs_below_level", "missing")} of {wall.get("pairs", "missing")} matched pairs below L2. Process clocks, partial prices and every failure remain in its run report.')
    report(HERE,"F4/F5 construction, recursive roots and complete recovery",paragraphs,images,[],"REPORT.pdf")

if __name__=="__main__":
    local_report()
    profile_report()
    screen_reports()
    native_report("iteration1-native-summary.json", "iteration1-", "20261019-165506-boolean-recursive-native-188e81", "0acfc55b7f", "run-20261019T165506Z")
    ecbench_report("iteration1-ecbench-summary.json", "iteration1-", "20261019-165520-boolean-recursive-ecbenc-9d61aa", "0acfc55b7f", "run-20261019T165520Z")
    native_report("iteration2-native-summary.json", "iteration2-", "20261019-174306-boolean-final-native-c19491", "71c4c99d85", "run-20261019T174306Z")
    ecbench_report("iteration2-ecbench-summary.json", "iteration2-", "20261019-174315-boolean-final-ecbench-25cc20", "71c4c99d85", "run-20261019T174315Z")
    native_report()
    ecbench_report()
    aggregate()
