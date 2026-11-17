from pathlib import Path
import csv,json,hashlib,collections,html,datetime
D=Path(__file__).resolve().parent;P=D.parent;I=P.parent.parent;H=Path('/home/ec2-user/crypto-pr2304-persist/research')
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def put(n,v):(D/n).write_text(json.dumps(v,indent=2)+'\n')
src=H/'four_followups_review_20261022/results/standard-targets.csv';data=list(csv.DictReader(src.open()));groups=collections.defaultdict(list)
for r in data:groups[(r['lead'],r['arm'])].append(r)
rows=[]
for (lead,arm),items in sorted(groups.items()):
 status=collections.Counter(r['status'] for r in items);verified=sum(v for k,v in status.items() if k.startswith('priced_source_verified'));capped=status['incomplete_cap_unpriced']
 rows.append({'lead':lead,'arm':arm,'historical_rows':len(items),'distinct_workload_ids':len({r['workload_id'] for r in items}),'priced_source_verified_rows':verified,'capped_rows':capped,'scope':'historical_row_recount_not_new_execution','source_path':str(src),'source_sha256':h(src),'statuses':dict(status)})
assert len(data)==12850
for r in rows:
 if r['lead'] in ['factor_base','projective_law']:assert r['distinct_workload_ids']==1260 and r['priced_source_verified_rows']==1260
pencil=next(r for r in rows if r['arm']=='online_identity');assert pencil['priced_source_verified_rows']==543 and pencil['capped_rows']==1
n19path=H/'kummer_differential_collector_20261022/n19-labelled-fallback/results/analysis/summary.json';n19=json.loads(n19path.read_text());assert n19['candidate_verified']==n19['baseline_verified']==n19['rho_verified']==1260
plans=[]
for f in sorted((I.parent.parent/'pencil_large_prime_20261029/prepared_native_plans').glob('plan-*.json')):
 j=json.loads(f.read_text());plans.append({'path':str(f),'sha256':h(f),'workloads':len(j['workloads']),'executions':len(j['executions'])})
# Absolute repository path avoids ambiguity from report location.
if not plans:
 root=Path('/home/ec2-user/crypto-pencil-evidence-recovery-20261105')
 for f in sorted((root/'research/pencil_large_prime_20261029/prepared_native_plans').glob('plan-*.json')):
  j=json.loads(f.read_text());plans.append({'path':str(f),'sha256':h(f),'workloads':len(j['workloads']),'executions':len(j['executions'])})
assert len(plans)==7 and sum(x['workloads'] for x in plans)==1260 and sum(x['executions'] for x in plans)==22680
obligations=[
 {'lead':'factor_base','component':'existing five arms','state':'historical cohorts retained','next':'Reuse results; additional graph policies are separate prospective arms, with no ready launch identified here.'},
 {'lead':'projective_law','component':'wrapper and retained state','state':'historical cohorts retained','next':'Complete compiled-source invariant and exceptional-branch obligations; boundary normalization inversions remain in records.'},
 {'lead':'pencil','component':'fresh matched cohort','state':'1260 targets frozen; execution incomplete','next':'Reconcile combined-v8 terminal; qualify exporter+four parity tests; build two production bins; run existing plans, replay and retained export.'},
 {'lead':'kummer','component':'differential and N19 cohorts','state':'historical cohorts retained','next':'Reconcile current signed-driver qualification; remaining exceptional-event controls remain separate.'},
 {'lead':'formal','component':'branch multiplicity','state':'13 named readbacks and machine review retained','next':'Reuse branch-controls-results and branch-controls-machine-review; preserve prior failed controls.'},
 {'lead':'formal','component':'pivot','state':'17 named readbacks retained','next':'Independent result/kernel review remains marked incomplete in saved summary; do not repeat producer by default.'},
 {'lead':'formal','component':'find root','state':'8 substantive modules compiled; earlier Controls failed','next':'Reconcile root-owned corrected Controls admitted successor before another execution.'},
 {'lead':'formal','component':'nonbranch','state':'corrected control source and30-name plan only','next':'Existing draft contains stale standalone approval wording; no current execution admission inferred.'},
 {'lead':'runtime','component':'managed/exporter qualification v5','state':'sealed source only,15 exact controls','next':'Current publication/review/ledger/lease/runtime admission remains root-owned.'},
 {'lead':'runtime','component':'production two-bin v3','state':'sealed source only','next':'Requires all15 qualification results and matching4501-file materialization; no executable invocation in build packet.'}]
put('inventory.json',{'schema':'research.preserved-cohort-inventory/v1','snapshot_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'method':'CSV row and distinct-ID recount plus read-only saved result inspection','scientific_executions':0,'new_cost_ratios':[],'statistical_inference':None,'historical_rows':rows,'n19_saved_summary':n19,'n19_summary_source':str(n19path),'n19_summary_sha256':h(n19path),'frozen_current_plans':plans,'obligations':obligations,'pencil_overlap':'Arm rows overlap. online_identity has543 verified and1 capped; neither arm row totals nor historical rows are fresh1260 enrollment.','independent_arithmetic_replay_this_inventory':False})
with (D/'historical-rows.csv').open('w',newline='') as f:
 keys=[k for k in rows[0] if k!='statuses'];w=csv.DictWriter(f,fieldnames=keys);w.writeheader();w.writerows({k:r[k] for k in keys} for r in rows)
with (D/'obligations.csv').open('w',newline='') as f:w=csv.DictWriter(f,fieldnames=list(obligations[0]));w.writeheader();w.writerows(obligations)
paths=[src,n19path,H/'projective_law_followup_20261022/independent_complete_law/COHORT_RECHECK_20261027.json',P/'state-and-repair-closure/phase-accounting/result.json',P/'pivot-control-results-20261111/summary.json',P/'branch-controls-machine-review-20261111/review.json',P/'find-root-results-20261111/summary.json',P/'nonbranch-proof-attempt-plan-20261111/PLAN.md',P/'target-stream-implementation/summary.json',P/'storage-forest-adapter/native-matrix-review/result.json',P/'exporter-v4-qualification-v5-20261111/manifest.json',P/'pencil-production-two-bin-build-v3-20261111/manifest.json',I/'four-lead-inventory/README.md',I/'five-followups/README.md']
# Existing PDF locations are pointers; sparse absence is recorded, not called loss.
paths += [H/'four_followups_review_20261022/REPORT.pdf',H/'projective_law_followup_20261022/REPORT.pdf',H/'kummer_differential_collector_20261022/n19-labelled-fallback/REPORT.pdf',P/'pivot-control-results-20261111/report.pdf',P/'branch-controls-results-20261111/report.pdf',I/'reports/reconciliation-v1/report.pdf']
put('sources.json',{'files':[{'path':str(f),'locally_present':f.is_file(),'sha256':h(f) if f.is_file() else None,'bytes':f.stat().st_size if f.is_file() else None} for f in paths],'note':'Absent historical PDF paths may be sparse or externally stored. No remote custody claim. Source summaries and standardized rows were locally inspected.'})
# One count graphic; rates, ratios and confidence intervals are intentionally absent.
chart=[r for r in rows if r['lead'] in ['factor_base','projective_law'] or r['arm'] in ['online_identity','differential_uncached']]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="960" height="590" viewBox="0 0 960 590"><rect width="960" height="590" fill="white"/><g font-family="sans-serif" fill="#172535"><text x="24" y="30" font-size="21">Preserved historical target rows</text><text x="24" y="54" font-size="13">Recount, not new runs. Each bar is a separate arm; overlapping arms must not be summed.</text>']
for i,r in enumerate(chart):
 y=79+i*43;count=r['priced_source_verified_rows'];label=r['lead']+'/'+r['arm'];svg.append(f'<text x="24" y="{y+18}" font-size="13">{html.escape(label)}</text><rect x="340" y="{y}" width="{count/1260*480:.2f}" height="25" fill="#4c7294"/><text x="{350+count/1260*480:.2f}" y="{y+18}" font-size="13">{count}</text>')
svg+=['<text x="340" y="490" font-size="13">Source-verified historical rows per arm (0 to1260)</text><text x="24" y="524" font-size="13">Pencil online_identity also has1 capped row. Other pencil arms overlap this enrollment.</text><text x="24" y="548" font-size="13">N19 separately records1260 candidate/baseline/rho verifications; see inventory.json.</text><text x="24" y="572" font-size="13">No cost ratios inferred. Current software and full-cohort obligations: obligations.csv.</text></g></svg>'];(D/'historical-coverage.svg').write_text(''.join(svg))
put('graph-data.json',{'rows':chart,'units':'historical_source_verified_rows','uncertainty':'not a statistical estimate; exact count of preserved rows'})
# Read-only custody recheck of all50 saved phase-accounting inputs.
prior=json.loads((P/'state-and-repair-closure/phase-accounting/result.json').read_text());raw={}
for name,expected in prior['inputs'].items():
 f=Path(name);current=h(f) if f.is_file() else None;raw[name]={'saved_sha256':expected,'current_sha256':current,'bytes':f.stat().st_size if f.is_file() else None,'matches':current==expected if current else None}
assert len(raw)==50 and all(v['matches'] is True for v in raw.values())
put('raw-source-hashes.json',{'schema':'research.preserved-raw-hash-recheck/v1','files':raw,'matched':50,'scalar_replay':False,'new_scientific_runs':0})
