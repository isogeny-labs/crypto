"""Whole closed data and a fresh current scientific-input guard; custody only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,subprocess,shutil
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
base='4cce14d072ae6c1c5f8588b3e955269d6256dbc5';assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-odd-product-raw-handoff-01';handoff=json.loads((d/'handoff.json').read_text());assert len(handoff['attempts'])==9
objects=[]
for row in handoff['attempts']:
    label=row['label']
    for name,size,h,target in [(label+'.tar.xz',row['archive_bytes'],row['archive_sha256'],'whole-raw.tar.xz'),(label+'-members.json.xz',row['complete_index_xz_bytes'],row['complete_index_xz_sha256'],'members.json.xz')]:
        p=d/name;assert p.stat().st_size==size and hashlib.sha256(p.read_bytes()).hexdigest()==h
        objects.append(dict(path=str(p.relative_to(r)),bytes=size,sha256=h,requires_original_data_migration=size>65536,consumer_path=str((cs/'evidence'/('closed-ct64-'+label)/target).relative_to(c)),role='whole-closed-archive'if target=='whole-raw.tar.xz'else'whole-complete-index'))
for attempt,size in [('01',120636),('02',120752)]:
    source=cs/'evidence'/('odd-product-maps-acceptance-'+attempt)/'all-native-records.json.xz'
    raw=source.read_bytes();assert len(raw)==size;name='odd-product-maps-'+attempt+'-native-records.json.xz';(d/name).write_bytes(raw)
    objects.append(dict(path=str((d/name).relative_to(r)),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),requires_original_data_migration=True,consumer_path=str(source.relative_to(c)),role='whole-complete-native-fixture-no-filtering'))
assert len(objects)==20
g=s/'evidence/public-n4-odd-product-current-guard-06';g.mkdir()
current=json.loads(Path('/workspace/scratch/ct63-odd-product-maps-native-02/before.json').read_text());assert len(current)==2485
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
for line in(cs/'ci_odd/reference-01.sha256').read_text().splitlines():
    h,p=line.split('  ',1);p=(c/p).resolve();assert hashlib.sha256(p.read_bytes()).hexdigest()==h;current[str(p)]=h
selected=[cs/'ci_odd/reference-01.sha256']
for name in ['ODD_SOURCE_ISOGENY_PROTOCOL_01.md','ODD_SOURCE_DERIVATION_01.md','ODD_SOURCE_PROOF_MAP_01.md','ODD_PRODUCT_GLUING_PROTOCOL_01.md','ODD_PRODUCT_CHAIN_DERIVATION_01.md','ODD_PRODUCT_PROOF_MAP_01.md']:
    selected.append(cs/name)
for folder in ['odd-product-models','odd-product-maps','reports/odd-product-01','reports/odd-source-01','reports/chain-kernels-01','reports/alternative-gluing-01','reports/kernel-census-01','reports/bitangents-01','reports/two-primary-01','evidence/odd-product-chain-bindings-01']:
    selected +=[p for p in(cs/folder).rglob('*')if p.is_file()]
selected +=[c/x['claim_path']for x in json.loads((cs/'evidence/odd-product-chain-bindings-01/claims.json').read_text())]
selected +=[cs/'evidence'/('odd-product-maps-acceptance-'+a)/'all-native-records.json.xz'for a in ['01','02']]
for p in selected:current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
physical=json.loads(Path('/workspace/scratch/ct62-odd-source-full-01/before.json').read_text());assert len(physical)==640
for p,h in physical.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h;current[p]=h
(g/'current-inputs.json').write_text(json.dumps(current,indent=2)+'\n')
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),current_guard_count=len(current),native_runtime_inputs=2485,latest_published_CI=639,latest_physical_full=640,
    additional='current new models/maps/SourceE/Rust/Lean/README/written/proofmap/two exact bindings/all existing affected reports/whole complete map fixtures',
    old_guards='05 historical3096 released;NOT reused;older independent full captures are closed artifacts,latest physical source640 is selected',
    hold='Do not edit selected physical bytes until exact genuine bot FF and final guards;new later work may be added without mutating this frozen set',scope='fresh actual-current union,not a stale historical guard'),indent=2)+'\n')
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');shutil.copyfile(__file__,d/'prepare-handoff.py')
large=[x for x in objects if x['requires_original_data_migration']]
(d/'scope.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,whole_closed_archives=9,complete_member_indices=9,whole_complete_native_fixtures=2,objects=20,requires_original_migration=len(large),small_whole_Git_objects=20-len(large),current_guard_count=len(current),guard_path=str(g.relative_to(r)),all_bytes_exact=True,remote_availability='pending unchanged genuine trusted publisher',no_split_prune_threshold_verifier_or_trust_change=True,control_plane='whole API/log losslessXZ/push/FF/manifests direct;no recursive publication archive'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','NINE whole closed captures+NINE complete indices+TWO complete large native fixture files20objects/'+str(len(large))+' require unchanged original artifact migration/'+str(20-len(large))+' small whole Git objects. Every source/member/raw/archive/index hash roundtrip;all failed/successful kernel/source13/newmodel/map01+02 captures intact,no split/prune/policy change. Fresh guard06 freezes actual2485native+current639CI/self+latest640physicalfull+current Rust/Lean/written/two exact bindings and all affected report assets,actual'+str(len(current))+' hashes. Oldguard05 historical/released/NOT reused. HOLD selected bytes until genuine bot FF/finalguards. Latest3de8 fullb2b04d/CI5fee4184native50digests;new9722d7/9561ce/written1fd6e7/two partial52bindings/report13cfa7. Genuine remote availability still pending;unsigned/no cost/global-root claim.']
for p in [d/'handoff.json',d/'objects.json',d/'scope.json',g/'custody.json']:cmd+=['--proof','attestation:'+str(p.relative_to(r))]
print(subprocess.check_output(cmd,cwd=r,text=True).strip(),len(current),len(large),20-len(large),flush=True)
entry='''

## STAMP — actual different-product Jacobian52 and complete data handoff

Consumer3de8c174/265paths/bff7b2/6876d6 normallypublished0. Actual source13de87bd/writtenb9a91f/six partial bindings/report1ea6c9;independentfullb2b04d640guards/13phases/four actual controls;TEN-workflow CI5fee41/all84native50canonicaldigests settled. No unchanged redispatch. [pcc:unverified]

Fresh Aprime model9722d7/all2478guards matchesALL6 old equations/kernels/origins/quotients,all135productkernels/4096pairings/all6newHLP models smooth,THREEsquare(1,0)(1,1)(2,1)+THREEnonsquare retained. Native01e1e62b and API02_9561ce/all2485guards giveALL3complete newPhiNorm/both2 and actual F=PhiNew gamma Nold,G=PhiOld delta NormNew BOTH52/all224old+224newCclasses/36source/public5/exactell/exceptional controls. Reusable tagged original odd-source recovery159985 delta NormNew passes/manualAPI images match/no scalar witness. RationalF kernels2,4,2/G2,2,2/allC4 retained. [pcc:unverified]

Written1fd6e7 audits global Ciani/V4/Weil restriction/polarizations/both52/degree140608 and each new F223 endpoint different from old via disjoint-j products+old unique2kernel. Two faithful exact partial bindings4d1338/95d4e6 passbothoriginalgates/no axioms/zero trusted,52total. First redundant-rfl failurec4ffa9 whole retained. TWO59335-byte final report pages ALLinspected13cfa7. Actual geometric interpretation remains written/partial formal. [pcc:unverified]

NINE new whole closed archives+NINE complete indices+TWO complete native fixtures are prepared as20 objects. Fresh currentguard06 holds COUNT current scientific/source/proof/report/fixture/physical640 inputs until exact genuine botFF/finalguards. Oldguard05 historical3096 released/NOT reused. Whole control-plane API/logXZ/push/FF/manifests remain direct,no recursive archive. All original bytes preserved;remote availability pending. [pcc:unverified]

Next actual unchanged publisher/remote-byte replay/exactFF/genuineconsumerprojections,then source checkpoint and strict clean independent current new-model/map replayCI. After that,transport oldGold through gamma on2torsion for full geometric52kernel types and/or actual new-target publicIC/cost coverage under fresh protocols. Original benchmark S3checkpoint/livebudget/capacity/complete costs/strongest refs remain unresolved;no speed claim. Root authentic fixture/main append-only ledger/signing/reviews/conflict/fullgeometry/vault8edec422/KBecdb gaps persist.20:04root3877conflicting/1519readymergeable;next21:00routine unlessmaterialsignal. Continueaftereachrung. [pcc:unverified]
'''.replace('STAMP',datetime.now(timezone.utc).isoformat()).replace('COUNT',str(len(current)))
for p in [s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md',r/'docs/session-logs/2026-11-13.md']:
    with p.open('a')as f:f.write(entry)
q=s/'QUEUE.md';q.write_text('Current actual different-product52 checkpoint: PUBLIC_TARGET_CHECKPOINT.md.\n'+entry+'\nEarlier entries remain historical.\n\n'+q.read_text())
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
Path('/workspace/scratch/ct64-root-publication-expected.json').write_text(json.dumps([x['path']for x in large],indent=2)+'\n')
