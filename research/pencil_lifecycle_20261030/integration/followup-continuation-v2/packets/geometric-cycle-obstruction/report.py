from pathlib import Path
import json,hashlib,collections
D=Path(__file__).parent;v=json.loads((D/'selection.json').read_text());counts=dict(collections.Counter(x['status']for x in v['observations']));status={'schema':'research.fixed-geometric-cycle-construction/v1','status':'no_cycle_in_retained_fixture_graph','p':739,'retained_records':60,'cases_inspected':420,'categories':counts,'predecessor_attempts':0,'native_runs':0,'terminal_verified':False,'planted_label_construction_planned':True,'operation_cap':{'archive_cases':420,'predecessor_attempts':840,'native_timeout_seconds':120},'scientific_execution':False,'independent_review':'pending'};(D/'status.json').write_text(json.dumps(status,indent=2)+'\n')
report='''# Fixed geometric cycle construction: retained fixture graph is a forest

Inspected the complete frozen set of 60 archived p=739 known-answer records, containing 420 case divisors. The deterministic selection extracted 114 distinct split-divisor edges. They form a forest: there is no cycle in this retained edge set.

The remaining cases were 126 nonsplit or repeated-root cases, 120 signed duplicates, and 60 degree skips. Generator equality was checked across all 60 archived records. Each accepted case carries its archived scalar relation s=(a+b*k) mod r. No new target, trajectory extension or scalar search was run.

The prospective construction would select an existing cycle, realize each selected divisor as an original-G transition through one of two algebraic predecessors (D-cG)/2, recover from the propagated pair, and pass the signed divisor to the existing graph adapter. Its labels would deliberately use the known answer to plant a target-bearing closing edge. This would be a known-answer geometric software fixture, not a collector search result. That construction did not start because the required cycle was absent.

Observed resource boundary: 420 of 420 permitted archived cases inspected; 0 of 840 permitted predecessor attempts; 0 native executions. No Q=kG terminal check or geometric stopping claim is reported. Existing synthetic equation controls and earlier recovery failures remain separate unchanged evidence.

The concrete missing input is an additional geometric relation set containing a signed cycle that supplies a target constraint. This finite forest observation does not exclude cycles in other relations or prove that the proposed original-G inverse-branch construction can reach a future cycle. A first unsigned cycle may also only anchor a component; terminal verification remains a separate check.

Reproduction: run select.py in a fresh private copy with the original archived path available; selection.json contains the input hash and all 420 dispositions. archive.jsonl preserves the exact input. Frozen PLAN.md preceded inspection. No source producer files were changed. Independent review pending.

Sources and interfaces: prior graph-integration packet and native SignedGraph; CCS recovery paper, sections 2, 6 and 7, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf. The construction uses the established difference-pair recovery interface. No novelty claim.
''';(D/'REPORT.md').write_text(report)
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="800" height="220"><rect width="800" height="220" fill="white"/><g font-family="sans-serif" fill="#20344b"><text x="15" y="30" font-size="19">420 retained cases → 114 unique split edges → forest</text><rect x="15" y="52" width="228" height="27" fill="#84adc6"/><text x="255" y="72">114 accepted edges</text><rect x="15" y="90" width="252" height="27" fill="#c2ccd5"/><text x="280" y="110">126 nonsplit / repeated root</text><rect x="15" y="128" width="240" height="27" fill="#d4dbe1"/><text x="270" y="148">120 signed duplicates</text><rect x="15" y="166" width="120" height="27" fill="#e3e8eb"/><text x="150" y="186">60 degree skips; 0 predecessor attempts</text></g></svg>''';(D/'coverage.svg').write_text(svg)
typ='#set page(paper:"a4",margin:18mm)\n#set text(size:10pt)\n#text(size:16pt,weight:"bold")[Geometric cycle fixture boundary]\n#image("coverage.svg",width:100%)\n'
for para in report.split('\n\n')[1:]:typ+='\n#text('+json.dumps(para,ensure_ascii=False)+')\n'
(D/'report.typ').write_text(typ)
