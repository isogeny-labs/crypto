"""Launch all runs (SPEC.md section 9.1)."""
import sys, os, json, subprocess, argparse, shlex

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)


def launch(cmd_args, outfile, log):
    inner = "ulimit -t 600 -v 2000000; exec " + " ".join(shlex.quote(a) for a in cmd_args)
    r = subprocess.run(["bash", "-c", inner], cwd=ROOT, capture_output=True, text=True)
    status = None
    if r.returncode != 0:
        with open(outfile, "w") as f:
            json.dump({"status": "not_run", "returncode": r.returncode,
                       "stderr_tail": r.stderr[-2000:]}, f, sort_keys=True)
            f.write("\n")
        status = "not_run"
    else:
        try:
            status = json.load(open(outfile)).get("status", "done")
        except Exception:
            status = "unreadable"
    log.write("%s\t%d\t%s\n" % (os.path.relpath(outfile, ROOT), r.returncode, status))
    log.flush()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--instances", required=True)
    ap.add_argument("--outdir", required=True)
    ap.add_argument("--log", required=True)
    a = ap.parse_args()
    os.makedirs(a.outdir, exist_ok=True)
    insts = [json.loads(l) for l in open(a.instances) if l.strip()]
    log = open(a.log, "w")
    py = "python3"

    def ic(inst, s, arm, outfile):
        launch([py, "code/ic_run.py", "--instance-json", json.dumps(inst, sort_keys=True),
                "--s", str(s), "--arm", arm, "--out", outfile], outfile, log)

    def fam(f):
        return sorted([i for i in insts if i["family"] == f], key=lambda i: (i["p"], i["j"]))

    def out(name):
        return os.path.join(a.outdir, name)

    # 1. F1
    for inst in fam("F1"):
        for s in (1, 2):
            arms = ["A", "B", "C", "Bp"]
            rot = (inst["j"] + s) % 4
            arms = arms[rot:] + arms[:rot]
            for arm in arms:
                ic(inst, s, arm, out("F1_p%d_j%d_s%d_%s.json" % (inst["p"], inst["j"], s, arm)))
    # 2. N
    for inst in fam("F1"):
        if inst["p"] in (263, 523):
            ic(inst, 1, "N", out("F1_p%d_j%d_s1_N.json" % (inst["p"], inst["j"])))
    # 3. PC
    for inst in fam("F0"):
        for s in (1, 2):
            arms = ["A", "B", "C"]
            rot = (inst["j"] + s) % 3
            arms = arms[rot:] + arms[:rot]
            for arm in arms:
                ic(inst, s, arm, out("F0_p%d_j%d_s%d_%s.json" % (inst["p"], inst["j"], s, arm)))
    # 4. G
    for inst in fam("G"):
        of = out("G_p%d_j%d.json" % (inst["p"], inst["j"]))
        launch([py, "code/generic_run.py", "--instance-json", json.dumps(inst, sort_keys=True),
                "--s", "1", "--out", of], of, log)
    # 5. R
    for inst in fam("F1"):
        for w in range(5):
            of = out("R_p%d_j%d_w%d.json" % (inst["p"], inst["j"], w))
            launch([py, "code/rho_run.py", "--instance-json", json.dumps(inst, sort_keys=True),
                    "--w", str(w), "--out", of], of, log)
    # 6. determinism
    for inst in fam("F1"):
        if inst["p"] == 523 and inst["j"] == 0:
            ic(inst, 1, "B", out("repeat_F1_p523_j0_s1_B.json"))
    log.close()


if __name__ == "__main__":
    main()
