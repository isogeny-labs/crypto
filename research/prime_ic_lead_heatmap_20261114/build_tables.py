"""Render TABLES.md and REPORT.typ from leads.json (and checks/redteam.json if present).

Lays out recorded verdicts; computes nothing beyond counts. Run: python3 build_tables.py
"""
import json, pathlib, re, collections
HERE = pathlib.Path(__file__).resolve().parent
D = json.loads((HERE / "leads.json").read_text())
L = D["leads"]
INS = [r for r in L if r["verdict"] != "out-of-scope"]
COL = {"FB": "factor base", "ORACLE": "oracle/solver", "LIFT": "lifting/p-adic", "TRANSFER": "transfer/structure", "BARRIER": "barrier/control", "ENG": "engineering"}
def esc(s): return str(s).replace("|", "\\|").replace("\n", " ")
def row(r, extra=""):
    return f"| `{r['id']}` | {COL[r['column']]} | {r['source_kind']} | {r['evidence_strength']} | {esc(r['recorded_disposition'])[:60]} | {esc(r['why_this_verdict'])[:260]} | {esc(r['cheapest_next_test'])[:200]} |"
HDR = "| id | column | source | evidence | recorded | why (sources in leads.json) | cheapest next test |\n|---|---|---|---|---|---|---|"
out = ["# Tables generated from `leads.json` (2026-11-14)\n", "Every row is `[pcc:unverified]`. Full records, including `sources_checked` and `check_run_today`, are in `leads.json`.\n"]
c = collections.Counter(r["verdict"] for r in L)
out.append(f"Records: {len(L)} ({len(INS)} in scope, {c['out-of-scope']} out of scope). In scope: open {c['open']}, blocked {c['blocked']}, unexamined {c['unexamined']}, engineering {c['engineering']}, refuted {c['refuted']}.\n")
out.append("## Flagships (live exponent-level questions)\n\n" + HDR)
out += [row(r) for r in INS if r["flagship"]]
for v, title in [("open", "Open"), ("blocked", "Blocked"), ("unexamined", "Unexamined"), ("engineering", "Engineering / constant factor")]:
    out.append(f"\n## {title} (n = {sum(1 for r in INS if r['verdict']==v)})\n\n" + HDR)
    out += [row(r) for r in INS if r["verdict"] == v and not (v == "open" and r["flagship"])]
flips = [r for r in INS if r["verdict"] == "refuted" and re.search(r"open|proposed|next_step|unexamined|blocked", str(r["recorded_disposition"]), re.I)]
out.append(f"\n## Recorded as open or proposed, read as refuted today (n = {len(flips)}; atlas {sum(1 for r in flips if r['source_kind']=='atlas')}, idea records {sum(1 for r in flips if r['source_kind']=='idea-pr')}, crypto studies {sum(1 for r in flips if r['source_kind']=='crypto-study')})\n\n" + HDR)
out += [row(r) for r in flips]
checks = [r for r in L if (r.get("check_run_today") or {}).get("ran")]
out.append(f"\n## Desk and toy checks run today (n = {len(checks)}; stdlib Python, scratchpad, `[pcc:unverified]`)\n\n| id | what | result |\n|---|---|---|")
out += [f"| `{r['id']}` | {esc(r['check_run_today'].get('what',''))[:200]} | {esc(r['check_run_today'].get('result',''))[:260]} |" for r in checks]
oos = [r for r in L if r["verdict"] == "out-of-scope"]
out.append(f"\n## Out of scope (n = {len(oos)})\n\nNot prime-field E(F_p) ECDLP leads: " + ", ".join(f"`{r['id']}`" for r in oos) + ".\n")
rt = HERE / "checks" / "redteam.json"
if rt.exists():
    R = json.loads(rt.read_text())
    ch = [x for x in R if x.get("change")]
    out.append(f"\n## Red-team review (Opus) of the {len(R)} open, blocked, flagship and sampled desk-refuted records\n\n{len(ch)} proposed changes, applied to `leads.json` where accepted (see `checks/redteam.md`).\n\n| id | from | to | reason |\n|---|---|---|---|")
    out += [f"| `{x['id']}` | {x['current_verdict']}/{x['current_strength']}{' flag' if x.get('current_flagship') else ''} | {x['proposed_verdict']}/{x['proposed_strength']}{' flag' if x.get('proposed_flagship') else ''} | {esc(x['reason'])[:300]} |" for x in ch]
(HERE / "TABLES.md").write_text("\n".join(out) + "\n")
print("TABLES.md", len(out), "lines")
