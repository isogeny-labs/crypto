from pathlib import Path
import json,hashlib,subprocess,datetime
r=Path('/workspace/relation-supervisor-live');s=r/'research/relation_fusion_20261117';p=r/'src/cryptanalysis/binary_radical_root.rs';old=p.read_text();a=old.index('    fn geometric_positive_controls(');b=old.index('    #[test]\n    fn factored_subspace_oracle_decides_as_subspace_oracle()',a)
(s/'root-fixture-repair/binary_radical_root-unformatted-387.rs').write_text(old)
helper=old[a:b].replace('crypto_lib::cryptanalysis::','crate::cryptanalysis::')
q=subprocess.run(['/home/agent/.cargo/bin/rustfmt','--edition','2021'],input=helper,text=True,capture_output=True,check=True)
formatted=''.join('    '+line if line.strip() else line for line in q.stdout.splitlines(keepends=True))+'\n';new=old[:a]+formatted+old[b:];p.write_text(new)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();source=json.loads((s/'ROOT_FIXTURE_SOURCE_387.json').read_text());source['schema']='actual-root-fixture-module-source/v2';source['parent_context']='ROOT_FIXTURE_SOURCE_387.json';source['format_and_import_change']='Format only helper; refer through crate::cryptanalysis (same original source in root and module harness). No algorithm/assertion/seed changes.';source['root_changes']={k:sha(r/k) for k in source['root_changes']};args=source['rustc_command'];args[args.index('root_fixture_modules_387')]='root_fixture_modules_389';args[args.index('/workspace/root-fixture-modules-387')]='/workspace/root-fixture-modules-389';(s/'ROOT_FIXTURE_SOURCE_389.json').write_text(json.dumps(source,indent=2)+'\n')
p=r/'docs/session-logs/2026-11-23.md';p.write_text(p.read_text()+f'''\n## {datetime.datetime.now(datetime.timezone.utc).isoformat()} —388actual-module tests pass;389style/import finalisation

### What was done (commands, files, features)
All7 original tests in the two actual changed source modules pass388, with zero failures/ignored/filtered and pinnedsource/ELF unchanged. Direct original lookup/map and curve identity tests are included, as are all5 binary radical root tests. This is module scope against frozen actual root352 public dependencies, not fullroot runtime. Formatted only new cfg(test) helper and used normal crate::cryptanalysis imports;389pins final source before recompile. [pcc:unverified]

### Decisions made (and why)
Keep all production oracle algorithms and original random cases/assertions, and every alias/value. Do not erase original380partial or384failures or claim full8216suite. Finalstyle/importedit warrants final7testcheck on exactpublishedsource.

### Open threads (unfinished, pick up next time)
Run final390 source-modulecheck, prepare scopedreport/PDF/archive/claims, publish toPR4989. Actual15×performancegate still requires canonicalprotocol access or explicit pending narrow waiver; no actual comparativeclocks collected.

### Errors (what went wrong, how it was handled)
No388test failures. Frozen557MiB350rootELF retained; current373MiBfree does not permit assuming a second fullroot rebuild fits. Exact module qualifier avoids overwriting it and labels dependency/scope limitations.
''')
with (s/'development-01/root-fixture-compile-389.stdout').open('xb') as o,(s/'development-01/root-fixture-compile-389.stderr').open('xb') as e:result=subprocess.run(['taskset','-c','0',*args],stdout=o,stderr=e,timeout=300)
print('compile_exit',result.returncode);raise SystemExit(result.returncode)
