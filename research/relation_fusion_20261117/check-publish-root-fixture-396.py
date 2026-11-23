from pathlib import Path
import json,hashlib,re,subprocess,datetime
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';git=['git','-c','gc.auto=0','-c','maintenance.auto=false'];parent='41f837ac6bb345f8efab9c7c41f66d45d515dc0f';base='816f9c3a942efed569049618c056d12a0c56d693';sha=lambda p:hashlib.file_digest(p.open('rb'),'sha256').hexdigest()
ledger=(repo/'pcc/ledger.jsonl').read_bytes()
for h in [base,parent]:assert ledger.startswith(subprocess.check_output(git+['show',h+':pcc/ledger.jsonl'],cwd=repo))
q=subprocess.run(['python3','tools/pcc/pcc.py','ledger','verify','--base',base],cwd=repo,capture_output=True);assert q.returncode==0,q.stderr
for ext,b in [('stdout',q.stdout),('stderr',q.stderr)]:(s/f'development-01/root-fixture-ledger-396.{ext}').write_bytes(b)
for n in ['ROOT_FIXTURE_REPORT_391.md','ROOT_FIXTURE_REPLAY_394.md']:
 q=subprocess.run(['python3','tools/pcc/pcc.py','lint-prose',str(s/n)],cwd=repo,capture_output=True);assert q.returncode==0,(n,q.stdout)
p=Path('/workspace/base-root-fixture-log-396.md');p.write_bytes(subprocess.check_output(git+['show',parent+':docs/session-logs/2026-11-23.md'],cwd=repo))
def lint(p):
 q=subprocess.run(['python3','tools/pcc/pcc.py','lint-prose',str(p)],cwd=repo,capture_output=True,text=True);return [x.split(': line ',1)[1] for x in (q.stdout+q.stderr).splitlines() if ': line ' in x]
a,b=lint(p),lint(repo/'docs/session-logs/2026-11-23.md');assert a==b,(a,b)
for n in ['SUPERVISED_SYNTHETIC_SOURCE_253.json','FLAT_MATCHED_SYNTHETIC_SOURCE_366.json']:
 d=json.loads((s/n).read_text())
 for p,h in d['source_sha256'].items():assert sha(repo/p)==h,(n,p)
 if 'tool_sha256' in d:
  for p,h in d['tool_sha256'].items():assert sha(Path(p))==h,(n,p)
d=json.loads((s/'ROOT_UNIT_SOURCE_348.json').read_text());changed=[p for p,h in d['source_sha256'].items() if sha(repo/p)!=h];assert changed==['src/cryptanalysis/binary_radical_root.rs'],changed
final=json.loads((s/'ROOT_FIXTURE_SOURCE_389.json').read_text())
for p,h in final['root_changes'].items():assert sha(repo/p)==h,p
before=(s/'root-fixture-repair/binary_radical_root-before-387.rs').read_text();after=(repo/'src/cryptanalysis/binary_radical_root.rs').read_text();assert before.split('#[cfg(test)]')[0]==after.split('#[cfg(test)]')[0]
assert json.loads((s/'root-fixture-repair/old/curve_aliases.json').read_text())==json.loads((repo/'src/cryptanalysis/curve_aliases.json').read_text())
# Preserve the original seeded loop literally except its corrected explanatory comment.
start='                for i in 0..200 {';oldloop=before[before.index(start,before.index('fn factored_subspace_oracle_decides_as_subspace_oracle')):];oldloop=oldloop[:oldloop.index('                assert!(yes > 10')]
newloop=after[after.index(start,after.index('fn factored_subspace_oracle_decides_as_subspace_oracle')):];newloop=newloop[:newloop.index('                // The bounded scan above')]
assert newloop==oldloop.replace('// Plant targets as x of an S₄ root over the base, half the time.','// Attempt a bounded S₄-root scan over the base, half the time.')
q=subprocess.run(git+['diff','--check','--','.',':(exclude)research/relation_fusion_20261117/development-01/*'],cwd=repo,capture_output=True);assert q.returncode==0,q.stdout
for n in ['ROOT_FIXTURE_OUTCOME_390.json','ROOT_FIXTURE_EXTRACTION_393.json']:
 d=json.loads((s/n).read_text());assert d['15x_admitted'] is False and d['full_root_suite_qualified'] is False
out={'schema':'root-fixture-publication-check/v1','published_parent':parent,'canonical_base':base,'ledger_prefixes_chain_pass':True,'legacy_ineligible_claim_payloads_preserved':True,'scoped_prose_lints_zero':2,'inherited_shared_log_findings':len(b),'new_log_findings':0,'frozen253_and366_source_tools_unchanged':True,'root348_changed_paths':changed,'final389_source_pins_match':True,'production_oracle_prefix_unchanged':True,'all_alias_values_metadata_unchanged':True,'original_200_case_loop_literal_preserved_except_comment':True,'code_whitespace_excluding_rawlogs_clean':True,'report_pdf_pages_inspected':1,'actual_module_tests_pass':7,'fresh_extracted_module_tests_pass':7,'full_root_suite_qualified':False,'global_benchmark_gate':'Fails unchanged historical null-exit documentary record; ROOT_BENCHMARK_GATE_396.json','new_comparative_performance_observations':0,'signed_execution_verdict':False,'15x_admitted':False};(s/'ROOT_FIXTURE_PUBLICATION_396.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
paths=subprocess.check_output(git+['diff','--name-only'],cwd=repo,text=True).splitlines()
for p in (repo/'pcc/claims').glob('PCC-20261123-*.json'):
 if subprocess.run(git+['ls-files','--error-unmatch',str(p.relative_to(repo))],cwd=repo,capture_output=True).returncode:paths.append(str(p.relative_to(repo)))
for p in s.iterdir():
 m=re.search(r'_(\d+)\.',p.name)
 if p.is_file() and ((m and 378<=int(m[1])<=397) or p.name in ['reclaim-extraction-379.py','run-root-full-380.py','replay-root-failures-384.py','prepare-root-repair-385.py','prepare-geometric-controls-386.py','integrate-root-fixtures-387.py','run-root-modules-388.py','format-root-fixture-389.py','run-root-modules-390.py','render-root-fixture-391.py','polish-root-report-391.py','package-root-fixture-392.py','extract-root-fixture-393.py','check-publish-root-fixture-396.py'] or p.name.startswith(('root-fixture-report-391.','root-fixture-qualification-392.'))):paths.append(str(p.relative_to(repo)))
for folder in ['root-fixture-repair','visual-391']:
 for p in (s/folder).rglob('*'):
  if p.is_file():paths.append(str(p.relative_to(repo)))
for p in (s/'development-01').iterdir():
 m=re.search(r'-(\d+)\.',p.name)
 if p.is_file() and m and 380<=int(m[1])<=396:paths.append(str(p.relative_to(repo)))
subprocess.run(git+['add','--']+[p for p in sorted(set(paths)) if not p.endswith('root-fixture-repair/Cargo.lock')],cwd=repo,check=True)
subprocess.run(git+['add','-f','research/relation_fusion_20261117/root-fixture-repair/Cargo.lock'],cwd=repo,check=True)
print('staged_paths',len(set(paths)))
