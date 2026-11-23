from pathlib import Path
import shutil,hashlib,json,tarfile
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';pkg=Path('/workspace/flat-qualification-package-301');pkg.mkdir()
def copy(src,dst):
 p=pkg/dst;p.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(src,p)
for src,dst in [('flat-constructor-matrix-277','direct-277'),('flat-worker-matrix-281','worker-281'),('flat-context-269','context-269'),('flat-context-270','context-270'),('flat-constructor-before-268','before-268')]:shutil.copytree(Path('/workspace')/src,pkg/dst)
for k,d in [(274,'flat'),(280,'flat-worker'),(296,'flat-audit')]:
 for p in Path(f'/workspace/relation-native-qualifiers/{d}-{k}').iterdir():
  if p.is_file():copy(p,Path('bin')/f'{d}-{k}'/p.name)
for p in (s/'development-01').glob('flat-*'):copy(p,Path('logs')/p.name)
for p in Path('/workspace').glob('flat-root-hydration-276.*'):copy(p,Path('logs')/p.name)
for p in Path('/workspace').glob('root-syntax-*287*'):
 if p.is_file():copy(p,Path('root-recovery')/p.name)
for pattern in ['root-bhec-*-282.rs','root-bhec-*-283.rs','root-bhec-*283*.stderr','root-efd-*285.rs','root-hecdlp-*289.rs','flat-root-adapter-before-279.rs','flat-artifact-audit-initial-295.rs','sparse-before-root-274.txt','sparse-partial-root-275.txt']:
 for p in Path('/workspace').glob(pattern):
  if p.is_file():copy(p,Path('root-recovery')/p.name)
for d in ['root-parent-source-288','root-topic-candidates-290','root-qualified-c488-291','root-extra-candidates-292','root-other-parents-293','root-threeway-294']:
 shutil.copytree(Path('/workspace')/d,pkg/'root-recovery'/d)
for d in ['crates/certified-relations','crates/relation-replay-checker']:
 for p in (repo/d/'src').rglob('*.rs'):copy(p,Path('source')/p.relative_to(repo))
 for n in ['Cargo.toml','Cargo.lock']:
  p=repo/d/n
  if p.is_file():copy(p,Path('source')/d/n)
for d in ['src','native-supervisor/src','root-integration/tests']:
 for p in (s/d).rglob('*.rs'):copy(p,Path('source/research/relation_fusion_20261117')/p.relative_to(s))
for d in ['', 'native-supervisor','root-integration']:
 for n in ['Cargo.toml','Cargo.lock']:
  p=s/d/n
  if p.is_file():copy(p,Path('source/research/relation_fusion_20261117')/d/n)
for n in ['Cargo.toml','Cargo.lock','src/cryptanalysis/f4_fp/certified.rs','src/bin/hecx.rs','src/cryptanalysis/binary_hyperelliptic_index_calculus.rs','src/cryptanalysis/efd_representations.rs','src/cryptanalysis/hec_dlp.rs','src/cryptanalysis/hyperelliptic_index_calculus.rs','src/prime_ecc/efd/checked.rs','src/prime_ecc/efd/formats.rs','src/prime_ecc/efd/tests.rs']:copy(repo/n,Path('source')/n)
for pattern in ['FLAT_*.json','ROOT_BHEC_RECOVERY_283.json','ROOT_EFD_RECOVERY_285.json','ROOT_SOURCE_RECOVERY_289.json','HYPOTHESIS_FLAT_CONSTRUCTOR_268.md','FLAT_CONSTRUCTOR_REPORT_300.md','flat-constructor-report-300.*','visual-300/*']:
 for p in s.glob(pattern):
  if p.is_file():copy(p,Path('study')/p.relative_to(s))
for mode,k,base in [('missing-mutation',298,'direct-277'),('control-summary',299,'worker-281')]:
 src=Path(f'/workspace/flat-artifact-{mode}-{k}');changed=[]
 for p in sorted(src.rglob('*')):
  if not p.is_file():continue
  rel=p.relative_to(src);b=p.read_bytes();q=pkg/base/rel
  if not q.is_file() or q.read_bytes()!=b:
   copy(p,Path('canary-deltas')/mode/rel);changed.append(str(rel))
 (pkg/'canary-deltas'/mode/'delta-roster.json').write_text(json.dumps({'base':base,'changed_files':changed,'reconstruction':'Copy base directory and replace only listed files with these byte-exact deltas. No original inode is modified.'},indent=2)+'\n')
copy(__file__,Path('package.py'))
files=[{'path':str(p.relative_to(pkg)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(pkg.rglob('*')) if p.is_file()]
(pkg/'index.json').write_text(json.dumps({'schema':'flat-qualification-archive/v1','performance_comparison_collected':False,'signed_execution_verdict':False,'files':files},indent=2)+'\n')
archive=s/'flat-qualification-301.tar.xz'
with tarfile.open(archive,'x:xz',format=tarfile.GNU_FORMAT,preset=6) as tar:
 for p in sorted(pkg.rglob('*')):
  if p.is_file():
   info=tar.gettarinfo(str(p),str(p.relative_to(pkg)));info.mtime=1795392000;info.uid=info.gid=0;info.uname=info.gname='';info.mode=0o755 if p.stat().st_mode&0o111 else 0o644
   with p.open('rb') as f:tar.addfile(info,f)
digest=hashlib.sha256(archive.read_bytes()).hexdigest();(s/(archive.name+'.sha256')).write_text(f'{digest}  {archive.name}\n')
r={'schema':'flat-qualification-archive-receipt/v1','archive':archive.name,'sha256':digest,'bytes':archive.stat().st_size,'indexed_files':len(files),'uncompressed_bytes':sum(x['bytes'] for x in files),'complete_direct_cases':36,'complete_worker_calls':72,'performance_comparison_collected':False,'signed_execution_verdict':False,'15x_admitted':False,'native_matrix_executed_cases':0,'archive_policy':'Local Git evidence publication follows persistent user checkpoint/S3 waiver. Deterministic GNU tar metadata; xz single-thread preset6.','canaries':'Changed files and reconstruction rosters only; unchanged files are in direct-277/worker-281.','root_source_state':'Eight reviewed prerequisite files restored; six syntax paths remain. No full-root correctness claim.'}
(s/'FLAT_ARCHIVE_301.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
