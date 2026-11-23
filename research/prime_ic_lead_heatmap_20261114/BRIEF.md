# Brief: chase the prime-field index-calculus lead corpus (2026-11-14)

You are one worker in a parallel chase. Goal: for every lead assigned to you, establish the
CURRENT verdict with evidence, run the cheapest discriminating desk/toy check when it is cheap
(pure Python, <= 3 CPU-minutes, scratchpad only), and write one JSON record per lead.
The records feed a "heat map" figure (verdict colour x evidence strength) like
/home/user/crypto/docs/ic/factor-base-encyclopedia/figures/taxonomy.svg.

## Where things are
- crypto repo (decrypted studies): /home/user/crypto. Key readers:
  - research/prime_field_lead_assessment_20261107/atlas_survey.md  (full prime-field atlas survey, 2026-11-07, lead by lead)
  - research/lead_sweep_20261110/leads.json (173 atlas leads: id, disposition, title) and research/lead_sweep_20261110/<LEAD-ID>/RESULT.md where present
  - research/prime_ic_leads_20261110/README.md (equation-budget filter, §5 ideas closed on paper, §7 leads)
  - research/prime_field_ic_closure_20261112/{prime-field-ideas.md,closure-ledger.md,chase-synthesis.md} (15 ideas, routes A1-A11, B1-B13)
  - research/prime_leads_chase_20261113/README.md and L1-pkm .. L5-fibre (five-lab chase, D(8,3) question)
  - research/leads_20261112/README.md and lanes/lane_P_prime.md (P1-P5, S1-S3)
  - research/leads_20261111/README.md (13 leads; mostly binary/extension, note which are prime)
  - research/ecdlp_subexp_ideas_20261110/README.md, research/iwasawa_leads_20261113/REPORT.md,
    research/prime_moonshot_20261110/ideas-generator.md, research/representation_leads_20261109/README.md
  - docs/ic/factor-base-encyclopedia/README.md (A1-A14 prime families, verdict table), docs/ic/LEADERBOARD.md
  - RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md, docs/ecdlp-hardness-corpus/{ATTACK_GRAPH.md,WEAKNESSES.md}
  - docs/session-logs/2026-11-1{0,1,2,3,4}.md (grep for a lead id to find the latest work on it)
- cryptanalysis repo: /home/user/cryptanalysis (grep for lead ids / mechanisms there too).
- crypto-autoresearcher atlas: /home/user/crypto-autoresearcher is VAULT-ENCRYPTED (ciphertext). Do not try to read it.
  The only plaintext atlas content is: the survey above, and the idea-campaign PR records extracted at
  /tmp/claude-0/-home-user/c2970330-965d-56e2-8425-0faf587c0f40/scratchpad/ideas-corpus/files/
  (ideas/<wave>.md wave notes, ledger/proposals/IDEA-20261109-*.yaml records). Index with PR number and
  merge state: .../ideas-corpus/idea_index.json ; triage: .../ideas-corpus/pr-triage-20261110.md
- research-kb MCP and ecdb are NOT available in this container; say so under sources_checked. Use grep.

## Rules (repository contract, non-negotiable)
- Everything you write is `[pcc:unverified]`. No benchmark launchers, no isolab/EKS/RunPod, no cargo builds,
  no edits to repository files. Toy checks: pure Python 3 (no sympy/numpy installed; stdlib only), run from the
  scratchpad, <= 3 CPU-minutes each, seeds fixed, command + output summary recorded in the JSON.
- Distance to rho is "distance remaining", never pass/fail. Relabelling and accounting changes are not improvements.
- Do not invent numbers. Every quoted number names its file path. If a claim can't be sourced, say "unsourced".
- Prefer the NEWEST evidence (session logs 2026-11-12..14 and the chase/closure studies override the 11-07 survey).
- Sabotage is assumed: if two sources disagree, record both and say which is newer / independently re-run.

## Verdict vocabulary (exactly one per lead)
- "refuted"      : negative result or obstruction (theorem, proof, measured, or toy-scale with wide margin) for the mechanism as stated
- "engineering"  : constant-factor only; exponent unchanged; may improve the IC baseline but not the rho gap at scale
- "open"         : live question, inconclusive evidence, a decisive test exists and is not done
- "blocked"      : open but blocked on a resource (host, memory, missing paper, missing records, admission)
- "unexamined"   : no evidence either way, nothing run or derived yet
- "out-of-scope" : not a prime-field E(F_p) ECDLP lead (binary, Koblitz, F_{p^k} cover/Jacobian, pure formula verification, quantum). One line why.

## Evidence strength (exactly one)
"proof" (written/machine-checked argument), "measured" (counted/charged runs, replayed), "toy" (local toy runs, not replayed),
"desk" (derivation or literature only), "none".

## Heat-map column (exactly one)
"FB" factor-base families | "ORACLE" decomposition/solver oracles | "LIFT" lifting and p-adic | "TRANSFER" transfers, covers, structure
| "BARRIER" barriers, spectra, complexity, controls | "ENG" engineering and accounting

## Output: one JSON file at the path given in your assignment, a JSON array of records, schema:
{
 "id": "<lead or IDEA id>", "label": "<= 38 chars for the figure cell, start with the id's short tail or a 2-6 word name>",
 "column": "FB|ORACLE|LIFT|TRANSFER|BARRIER|ENG", "source_kind": "atlas|idea-pr|crypto-study",
 "pr": <int or null>, "pr_state": "MERGED|OPEN|null",
 "recorded_disposition": "<as recorded in the source, verbatim field or short quote>",
 "verdict": "...", "evidence_strength": "...", "flagship": <true if this is one of the few live exponent-level questions>,
 "exponent_delta": "<e.g. '0 (self-declared)', 'unknown', 'needs D(8,3)=9'>",
 "distance_to_rho": "<quoted figure with path, or 'not quantified'>",
 "mechanism": "<one sentence>",
 "why_this_verdict": "<2-5 sentences with file paths>",
 "check_run_today": {"ran": <bool>, "what": "...", "command": "...", "result": "...", "runtime_s": <n>} or null,
 "cheapest_next_test": "<one sentence; name the cell/size/host class>",
 "sources_checked": ["repo:path", ...], "notes": "<disagreements, scope caveats>", "pcc": "unverified"
}
Write valid JSON (use python json.dump). Be exhaustive over YOUR list; do not add other leads. When done, reply with
a 10-line summary: counts per verdict, the flagship ids, and anything that contradicts the recorded disposition.
