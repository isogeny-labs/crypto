import json

path = "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk/VERIFICATION.json"

explanation = (
    "No scientific number differed. I classed the two mismatches below as nondeterminism and a replicator race. "
    "The other two are timing/cosmetic, or a check on a different file.\n\n"
    "1. Skipped cells (nondeterminism, fixed). The preregistered rule \"skip if the prior cell of the same arm took more than 200 s\" "
    "reads wall-clock time. The per-cell hard limit (timeout 840 s) also depends on wall time. replicate2 ran under load of about 15, "
    "so the prior cells took 539/533/225/218 s (original: 199/195/191/164 s). It therefore skipped a1 b36 i1/i2, kpure b40 i2 and "
    "kpureT b40 i2, which the original ran. Those four raw files in replicate2 were carried over from the original, so its matching "
    "results were partly by carry-over.\n\n"
    "   Every cell is seeded with setrand, so there is no randomness problem. The fix is in cellrun.sh, run.sh, run_supp.sh and a new "
    "skip.list, and is recorded in DEVIATIONS.md items 18-20:\n"
    "   - The skip set is frozen as skip.list (a1 b40 i1/i2), which is what the original run skipped. The rule's threshold is unchanged "
    "and the preregistration is untouched.\n"
    "   - The timeout is now CELL_TIMEOUT, default 3000 s.\n"
    "   - run.sh and run_supp.sh gained D, JOBS, REPLICATE and PARALLEL_CONTROLS environment variables.\n"
    "   - There is one longest-first cell pool with no per-b barrier. The controls and the a1c supplement run beside it when PARALLEL_CONTROLS=1.\n\n"
    "2. Reduced check \"fail=1 on cell_nc1_b24_i1.txt\" (race in the replicator, not a result difference). replicate2's check compared "
    "reduced/raw/ with replicate2/raw/. The full run was rewriting replicate2/raw/cell_nc1_b24_i1.txt from 19:22:31 to 19:25:40, and the "
    "check ran at 19:23:12 on a partial file. Once the files were complete, all three copies (original, replicate2 raw, replicate2 reduced) "
    "agreed on all 50 lines after dropping the per-trial ms field.\n\n"
    "3. Wall-clock columns (meanT, us/step, ms, done_ms, time_*.sec) and the seconds quoted in skip messages are timing/cosmetic.\n\n"
    "4. The items VERIFICATION.json lists as matching (results.json, opcounts and so on) did match, partly by carry-over (item 1).\n\n"
    "Verification of the fix. I ran the full fixed pipeline twice in fresh directories on an idle 4-core machine with JOBS=4.\n"
    "- replicate3/ (serial controls) took 1542 s. replicate4/ (PARALLEL_CONTROLS=1) took 1180 s, about 19.7 min.\n"
    "- compare_full.sh gives fail=0 for both: 89 of 89 cell files identical after dropping wall-clock fields.\n"
    "- curves, identity, chains and exhaustive are byte-identical.\n"
    "- results.json, out/results.json, out/costs.csv and out/opcounts* are byte-identical.\n"
    "- summary.csv is identical without the two timing columns.\n"
    "- MISSING is exactly {a1 b40 i1, a1 b40 i2}.\n\n"
    "The scientific numbers in results.json and RESULTS.md stand unchanged. The verdict-label criticisms in VERIFICATION.json "
    "(prediction_confirmed overstated, F1 and F2 formally triggered, A1 unusable) are separate from this replication mismatch and were not touched."
)

files = [
    "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk/cellrun.sh",
    "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk/run.sh",
    "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk/run_supp.sh",
    "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk/skip.list",
    "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk/compare_full.sh",
    "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk/DEVIATIONS.md",
    "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk/replicate4/",
]

fixed_commands = [
    "D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk; R=$D/replicate_new; rm -rf $R && mkdir $R && cd $D && cp *.gp *.sh *.py skip.list $R/ && rm -f $R/analyze_frozen_v1.gp && cd $R && D=$R REPLICATE=1 JOBS=4 PARALLEL_CONTROLS=1 ./run.sh > run.log 2>&1   # full reproduction, about 20 min wall on an idle 4-core machine (1180 s measured)",
    "D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk; cd $D && ./compare_full.sh $D/replicate_new | tail -2   # expect 'full replication fail=0' (run only after the previous command has finished)",
]

reconciliation = {
    "diagnosis": {
        "cause": "nondeterminism_fixed",
        "explanation": explanation,
    },
    "needs_rereplication": False,
    "fixed_commands": fixed_commands,
}

with open(path) as f:
    data = json.load(f)

data["reconciliation"] = reconciliation

with open(path, "w") as f:
    json.dump(data, f, indent=2, ensure_ascii=False)
    f.write("\n")

print("keys:", list(data.keys()))
