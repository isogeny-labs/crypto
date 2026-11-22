"""Retain actual original publisher evidence and genuine consumer projections."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110'
c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
d=s/'evidence/public-n4-common-origin-publication-01'
base='2c0cc5c9c956b21723a2330daf02f6d49d2fbc44';bot='aecf9b255ff6d9ca47501972c40ac9f04db8082e'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot
log=json.loads(lzma.decompress((d/'whole-log-api.json.xz').read_bytes()))['structuredContent']['content'].encode()
assert log.index(b'8 remote objects replayed')<log.index(b'[detached HEAD aecf9b255]')
jobs=json.loads(lzma.decompress((d/'jobs-api.json.xz').read_bytes()))['structuredContent']['jobs']
assert len(jobs)==1 and jobs[0]['id']==132270673745 and jobs[0]['conclusion']=='success'
assert lzma.decompress((d/'whole-job.log.xz').read_bytes())==log
p=s/'evidence/public-n4-common-origin-raw-handoff-01/.artifacts.json'
manifest=json.loads(p.read_text());assert len(manifest['files'])==8;shutil.copyfile(p,d/'canonical-manifest.json')
objects=[]
for name,entry in manifest['files'].items():
    path=str((p.parent/name).relative_to(r));raw=subprocess.check_output(['git','show',base+':'+path],cwd=r)
    assert len(raw)==entry['bytes'] and hashlib.sha256(raw).hexdigest()==entry['sha256']
    objects.append(dict(path=path,**entry))
enc=[]
for source,label in [('/workspace/scratch/ct70-root-handoff-publication-01','normal-publication'),('/workspace/scratch/ct70-raw-publication-discovery-01','source-discovery'),('/workspace/scratch/ct70-common-origin-artifact-ff-01','exact-ff')]:
    target=d/label;target.mkdir()
    for src in sorted(Path(source).rglob('*')):
        if not src.is_file():continue
        out=target/src.relative_to(source);out.parent.mkdir(parents=True,exist_ok=True);raw=src.read_bytes()
        if src.suffix in ['.log','.txt','.stdout','.json']:
            z=lzma.compress(raw);assert lzma.decompress(z)==raw;stored=out.with_suffix(out.suffix+'.xz');stored.write_bytes(z)
            enc.append(dict(raw_path=str(out.relative_to(d)),stored_path=str(stored.relative_to(d)),raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),exact_roundtrip=True))
        else:out.write_bytes(raw)
assert all((d/'exact-ff'/(n+'.exit')).read_text().strip()=='0'for n in ['fetch','exact-ff','artifact','wiring'])
preservation=json.loads(Path('/workspace/scratch/ct70-common-origin-artifact-ff-01/dirty-preservation.json').read_text())
assert preservation['current_union_count']==4230 and preservation['all_consumer_dirty_bytes_preserved'] and preservation['all_selected_symlink_types_and_targets_preserved']
(d/'lossless-encodings.json').write_text(json.dumps(enc,indent=2)+'\n');shutil.copyfile(__file__,d/'original-recorder.py')
summary=dict(at=datetime.now(timezone.utc).isoformat(),run=43537722838,job=132270673745,event='pull_request_target',conclusion='success',source=base,bot=bot,exact_bot_parent=base,exact_changed_paths=10,actual_remote_byte_replays_BEFORE_bot=8,objects=objects,all_source_Git_blob_hashes_match_genuine_manifests=True,normal_FF=0,artifact=0,wiring=0,all4230_selected_inputs_and_both_repositories_owned_dirty_bytes_and_symlink_types_targets_unchanged=True,source_hold08_released_after_FF_terminal=True,whole_UTF8_log_bytes=len(log),whole_UTF8_log_sha256=hashlib.sha256(log).hexdigest(),whole_control_plane='whole API JSON/log losslessXZ/normal push/discovery/exactFF/manifests direct,no recursive publication archive',scope='7 WHOLE closed captures+7 WHOLE complete indices+ONE WHOLE final PDF15objects:8 genuineS3+7smallwholeGit;includes whole94filepending scientificsource/report/754CI/PCCledger/resume/base0a+patch snapshot;all failed/success/fullCI/transport/rootclock/JSONcontrol originals intact;not signed execution/math/cost')
(d/'publication.json').write_text(json.dumps(summary,indent=2)+'\n')
members={str(x.relative_to(d)):dict(bytes=x.stat().st_size,sha256=hashlib.sha256(x.read_bytes()).hexdigest())for x in sorted(d.rglob('*'))if x.is_file()}
(d/'members.json').write_text(json.dumps(members,indent=2)+'\n')
statement='ACTUAL43537722838/job132270673745/pull_request_target SUCCESS EIGHT remote byte replays BEFOREaecf9b255/directparent2c0cc5c9/EXACT10paths. SEVENwholeclosedcaptures+SEVENcompleteindices+ONEwholefinal79399-bytePDF15objects:8genuineS3/7smallwholeGit. Everycanonicalentrymatches sourceGitblob. Includeswhole94file unpublishedscientificsource/report/754CI/PCCledger/resume/base0a+patch snapshot/restorablebeforeconsumercommit;no recursivepublicationarchive. NormalFF/artifact/wiring0/ALL4230freshguard08selectedinputs+BOTHereposowned dirtybytes+BOTHreportlinktypes/targets unchanged;hold08 RELEASEDafterterminal. Whole427955-byteUTF8log/allwholeAPI losslessXZ/push/discovery/FF/manifests DIRECT/no policychange. Wholefailed/success/current730full/14CI/originaltransport/rootclock/nativecompat/JSONcontrols retained;unsigned/no math/execution/cost/globalrootgreen.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for name in ['publication.json','members.json','whole-job.log.xz']:cmd+=['--proof','attestation:'+str((d/name).relative_to(r))]
root_claim=subprocess.check_output(cmd,cwd=r,text=True).strip();print('actual',root_claim,flush=True)
dest=cs/'evidence/genuine-common-origin-publication-01';dest.mkdir();shutil.copyfile(d/'publication.json',dest/'publication.json')
projected=[];data_objects=json.loads((p.parent/'objects.json').read_text());groups={}
assert len(data_objects)==15
for obj in data_objects:
    target=c/obj['consumer_path'];raw=target.read_bytes();source_name=Path(obj['path']).name
    assert len(raw)==obj['bytes'] and hashlib.sha256(raw).hexdigest()==obj['sha256']
    if obj.get('consumer_alias'):
        alias=c/obj['consumer_alias'];assert alias.is_symlink() and os.readlink(alias)==obj['alias_relative_target'] and alias.resolve()==target
    if source_name in manifest['files']:
        entry=manifest['files'][source_name];assert len(raw)==entry['bytes'] and hashlib.sha256(raw).hexdigest()==entry['sha256']
        groups.setdefault(target.parent,{})[target.name]=entry
        projected.append(dict(path=obj['consumer_path'],entry=entry,genuine_remote=True,role=obj['role'],alias=obj.get('consumer_alias')))
    else:
        assert raw==subprocess.check_output(['git','show',bot+':'+obj['path']],cwd=r) and len(raw)<65536
        projected.append(dict(path=obj['consumer_path'],bytes=len(raw),sha256=obj['sha256'],normal_Git_source=bot+':'+obj['path'],genuine_remote=False,role=obj['role']))
for folder,managed in groups.items():
    assert not (folder/'.artifacts.json').exists()
    (folder/'.artifacts.json').write_text(json.dumps(dict(schema=manifest['schema'],files=managed),indent=2)+'\n')
    (folder/'.gitignore').write_text('\n'.join(sorted(managed))+'\n')
    for name in managed:assert subprocess.run(['git','check-ignore','-q',str((folder/name).relative_to(c))],cwd=c).returncode==0
assert len(projected)==15 and sum(x['genuine_remote']for x in projected)==8
(dest/'projection.json').write_text(json.dumps(dict(root_actual_claim=root_claim,root_bot=bot,objects=projected,all_original_physical_bytes_intact=True,eight_genuine_S3_objects=True,seven_small_whole_objects_Git=True,canonical_whole_PDF='docs/research-artifacts/chained-transfer/common-origin-IC-01/report.pdf',research_relative_symlink_unchanged=True,no_threshold_verifier_or_trust_change=True),indent=2)+'\n')
checks=[('original-canonical-PDF-local-verify',['python3','tools/artifacts/artifacts.py','verify','docs/research-artifacts/chained-transfer/common-origin-IC-01/report.pdf'],0),('original-PCC-final-report',['python3','tools/pcc/pcc.py','verify','pcc/claims/PCC-20261114-6271ce.json','--json'],2),('original754inputs',['sha256sum','--check','experiments/chained-transfer-public-target/ci_common_origin/reference-01.sha256'],0)]
for label,command,expected in checks:
    v=subprocess.run(command,cwd=c,capture_output=True);raw=v.stdout+v.stderr
    (dest/(label+'.log.xz')).write_bytes(lzma.compress(raw));(dest/(label+'.exit')).write_text(str(v.returncode)+'\n');assert v.returncode==expected,raw[:2000]
    if label=='original-PCC-final-report':
        result=json.loads(v.stdout);assert result['verdict']=='not_covered' and all(x['verdict']=='not_covered' and not x['reasons']for x in result['proofs'])
cmd=['python3', 'tools/pcc/pcc.py', 'claim', 'new', '--kind', 'statement', '--claim-type', 'data', '--statement', 'GENUINE7capture consumerprojection15objects/8actual43537722838/aecf9b255 canonicalS3entries+7smallwholeGit;every originalbyte/SHA/sourceGitblob exact/physicaloriginalsintact. Wholefinal79399-bytePDFcanonicaldocs/genuineentry+unchangedresearchrelativeGitlink6271ce underprospectived7586b;originalartifactlocalverify0/originalPCCreport2notcovered/no perproof failures/all754CIinputs0. Includeswhole94fileunpublished scientificsource/report/754CI/PCCledger/resume/base0a+patch checkpoint andcomplete current730full/14CI/originaltransport/rootclock/nativecompat/JSONcontrolcaptures/allfailures. Originalpolicies/workflows/verifier/keyring/trust/claimpins/source unchanged;unsigned/no math/execution/cost/globalrootgreen.']
for x in sorted(dest.iterdir()):cmd+=['--proof','attestation:'+str(x.relative_to(c))]
consumer_claim=subprocess.check_output(cmd,cwd=c,text=True).strip();print('genuine',consumer_claim,flush=True)
canonical=c/'docs/research-artifacts/chained-transfer/common-origin-IC-01'
with (canonical/'README.md').open('a')as f:f.write('\nActual publication '+datetime.now(timezone.utc).isoformat()+':43537722838/job132270673745 SUCCESS/8remote byte replays beforeaecf9b255. This whole79399-bytePDF nowhasgenuinecanonicalmanifest/SHA2a5befb0 matches sourceGitblob/6271ce. Originalartifactlocalverify0/PCCreport2notcovered/no perproof failures;hold08 releasedonlyafterexactFF/all4230guards+BOTHreportlinktypes. This supersedesearlierpendingavailability. [pcc:unverified]\n')
