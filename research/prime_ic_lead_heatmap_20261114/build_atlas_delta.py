"""Write ATLAS_DELTA.md: atlas leads whose verdict today differs from the recorded disposition family."""
import json, pathlib, re
HERE = pathlib.Path(__file__).resolve().parent
L = json.loads((HERE / "leads.json").read_text())["leads"]
MAP = {"refuted": "closed_obstructed (or exhausted_at_scale where the obstruction is toy-scale only)", "engineering": "open_next_step with next_step = constant-factor measurement only; or closed_dominated",
       "open": "open_next_step", "blocked": "open_blocked", "unexamined": "open_unexamined", "out-of-scope": "(no change; not a prime-field lead)"}
def fam(disp):
    s = str(disp)
    for k in ("closed_obstructed", "closed_known", "closed_dominated", "exhausted_at_scale", "open_blocked", "open_unexamined", "open_next_step"):
        if s.startswith(k): return k
    return s[:30]
rows = []
for r in L:
    if r["source_kind"] != "atlas" or r["verdict"] == "out-of-scope": continue
    f = fam(r["recorded_disposition"])
    same = (r["verdict"] == "refuted" and f.startswith("closed")) or (r["verdict"] == "refuted" and f == "exhausted_at_scale") \
        or (r["verdict"] == "open" and f == "open_next_step") or (r["verdict"] == "blocked" and f == "open_blocked") \
        or (r["verdict"] == "unexamined" and f == "open_unexamined") or (r["verdict"] == "engineering" and f in ("open_next_step", "closed_dominated", "closed_known"))
    if not same: rows.append((r, f))
out = ["# Lead-atlas delta from this study (to apply in `crypto-autoresearcher` once `KEY` is available)\n",
       "The atlas (`knowledge/leads/ecdlp/`) is AES-256-SIV vault-encrypted and `KEY` was not set in this",
       "container, so nothing was edited there. This file lists, for every atlas lead whose verdict today",
       "differs in kind from its recorded disposition, the proposed disposition, the reason and the evidence",
       f"paths (all `[pcc:unverified]`; details in `leads.json`). {len(rows)} leads.\n",
       "| lead | recorded | verdict today (evidence) | proposed disposition | reason | evidence to add |", "|---|---|---|---|---|---|"]
for r, f in rows:
    srcs = "; ".join(s for s in r["sources_checked"] if ":" in s and "research-kb" not in s)[:300]
    out.append(f"| `{r['id']}` | {f} | {r['verdict']} ({r['evidence_strength']}) | {MAP[r['verdict']]} | {r['why_this_verdict'].replace('|','/')[:320]} | {srcs.replace('|','/')} |")
out.append("\n## Also to add to the atlas\n")
out.append("- `IDEA-20261109-1130d0` (merged idea record, `ideas/mythos2-20261109-torsion` wave): the construction is empty (alpha = 0 mod (pi-1) forces N | q); mark the record refuted/proof and link it from `LEAD-TRF-anomalous-padic-lift`.")
out.append("- The mis-set falsifiers listed in `README.md` section 4 (`179c5d`, `410419`, `e78107`, `00e2fd`, `876571`, `fde21e`, `5114b3`): correct the records before any is admitted to a run.")
out.append("- New evidence rows for the flagship cells 0.1 / 0.2: crypto PR #4151 (`d_F >= 6` at W = 1, t = 12), `research/prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md` (D(8,3) demoted to diagnostic; positive sparse relation-finder panel is the lead-changing test).")
(HERE / "ATLAS_DELTA.md").write_text("\n".join(out) + "\n")
print("ATLAS_DELTA.md:", len(rows), "atlas deltas")
