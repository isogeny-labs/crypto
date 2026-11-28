"""Launch one checked Sage process per repetition and retain every outcome."""

import hashlib
import json
import subprocess
import time
from pathlib import Path

HERE=Path(__file__).resolve().parent
SOURCE=HERE.parent/"xedni_fixed_curve_target_accounting_20261121"
SAGE="/Volumes/SSD990/cryptanalysis/sage"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(SOURCE/"run_sage.py")==p["source_table_runner_sha256"]
    assert sha(SOURCE/"audit-result.json")==p["source_audit_sha256"]
    rows=[]
    for rep in range(1,p["repetitions"]+1):
        start=time.monotonic_ns()
        try:
            cp=subprocess.run([SAGE,"-python",str(HERE/"worker_sage.py"),str(rep)],
                              capture_output=True,text=True,timeout=p["wall_cap_seconds_per_process"])
            row={"repetition":rep,"launcher_wall_ns":time.monotonic_ns()-start,
                 "exit_code":cp.returncode,"stdout":cp.stdout,"stderr":cp.stderr}
            if cp.returncode==0:
                try:
                    row["worker"]=json.loads(cp.stdout)
                    row["status"]="complete"
                except json.JSONDecodeError:
                    row["status"]="parse_error"
            else:
                row["status"]="error"
        except subprocess.TimeoutExpired as ex:
            row={"repetition":rep,"launcher_wall_ns":time.monotonic_ns()-start,
                 "status":"wall_cap",
                 "stdout":(ex.stdout or b"").decode(errors="replace") if isinstance(ex.stdout,bytes) else (ex.stdout or ""),
                 "stderr":(ex.stderr or b"").decode(errors="replace") if isinstance(ex.stderr,bytes) else (ex.stderr or "")}
        rows.append(row)
        print(json.dumps(row,sort_keys=True),flush=True)
    complete=[r for r in rows if r["status"]=="complete" and r["worker"]["status"]=="PASS"]
    summary={"schema":"xedni-process-fresh-target-result-v1",
             "status":"PASS" if len(complete)==p["repetitions"] else "INCOMPLETE",
             "processes":len(rows),"completed_processes":len(complete),
             "protocol_sha256":sha(HERE/"protocol.json"),
             "rows_sha256":sha(HERE/"process-rows.jsonl")}
    if complete:
        summary["target"]=complete[0]["worker"]["target"]
        summary["fixture_scalar_for_replay"]=complete[0]["worker"]["fixture_scalar_for_replay"]
    (HERE/"result.json").write_text(json.dumps(summary,indent=2,sort_keys=True)+"\n")


if __name__=="__main__":main()
