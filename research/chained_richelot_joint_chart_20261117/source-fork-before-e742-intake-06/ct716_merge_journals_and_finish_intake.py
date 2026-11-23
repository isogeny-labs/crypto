from pathlib import Path
from datetime import datetime,timezone
import collections,hashlib,json,lzma,re,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_joint_chart_20261117';p=s/'source-fork-before-e742-intake-06'
f=p/'COMPLETE_OLD_SOURCE_METADATA_INVENTORY.json';b=f.read_bytes();z=f.with_name(f.name+'.xz');z.write_bytes(lzma.compress(b));assert lzma.decompress(z.read_bytes())==b
(p/'COMPLETE_OLD_METADATA_INVENTORY_LOSSLESS_STORAGE.json').write_text(json.dumps({'original_path':f.name,'original_bytes':len(b),'original_sha256':hashlib.sha256(b).hexdigest(),'original_mode':f.stat().st_mode&0o7777,'whole_compressed_path':z.name,'whole_bytes_compared':True},indent=2)+'\n');f.unlink()
j=r/'docs/session-logs/2026-11-17.md';base=j.read_text()
with tarfile.open(p/'entire-original-source-PCC-journal-policy-tools-and-index.tar.xz') as t:old=t.extractfile('docs/session-logs/2026-11-17.md').read().decode()
pattern=re.compile(r'(?m)^(?:#{1,3} )?2026-11-17(?:T|[ ~])[0-9~][^\n]*$')
def norm(b):
 b=b.strip();b=re.sub(r'\n\s*---\s*$','',b);return b.strip()
def split(b):
 starts=[m.start() for m in pattern.finditer(b)];assert starts
 return b[:starts[0]].strip(),[norm(b[a:z]) for a,z in zip(starts,starts[1:]+[len(b)])]
pref,a=split(base);_,b=split(old);ca=collections.Counter(a);cb=collections.Counter(b);want=ca|cb
def stamp(v):
 line=v.splitlines()[0];m=re.search(r'2026-11-17(?:T|[ ~]+)(\d\d):(\d\d)(?::(\d\d))?',line)
 return (tuple(map(int,[m[1],m[2],m[3] or '00'])) if m else (0,0,0),line)
blocks=sorted(want.elements(),key=stamp);merged=pref+'\n\n---\n\n'+'\n\n---\n\n'.join(blocks)+'\n'
_,actual=split(merged);assert collections.Counter(actual)==want
receipt={'time':datetime.now(timezone.utc).isoformat(),'main_base':'94479f7e3eb9de9ae59b764d2b2b50b3238f0a44','old_source_checkpoint':'0bc406900e2ef9756215d3623a3b34db8b68da2b','full_base_journal_bytes':len(base.encode()),'full_source_journal_bytes':len(old.encode()),'base_blocks':len(a),'source_blocks':len(b),'union_multiset_blocks':len(blocks),'counter_max_union_roundtrip_equal':True,'normalized_full_block_multiplicities':[{'sha256':hashlib.sha256(v.encode()).hexdigest(),'base':ca[v],'source':cb[v],'output':want[v]} for v in sorted(want)],'baseline_claim':'PCC-20261117-ae44b4','CI_claim':'PCC-20261117-db3c55'}
(p/'FULL_JOURNAL_MULTISET_INTAKE.json').write_text(json.dumps(receipt,indent=2)+'\n')
entry=f'''\n\n---\n\n{datetime.now(timezone.utc).isoformat()}
# Canonical13 intake and final4485 CI closure before actual divisor14

## What was done
PCC-20261117-ae44b4: pinned current main94479f7e, complete index tree5660ed0822b4 equality with partial-clone write-tree --missing-ok;4117 physical metadata blobs/3376 claims/1352 ledger rows match main. Full old1292-row source fork remains public0bc40690 and whole archive;11 raw old claims moved intact, no row rechaining or trust/policy edits. Unit13 native42a9c2/inspected report28a4bb imported unchanged. PCC-20261117-db3c55 retains all21 final4485 workflow/32 check records and four whole logs:17 failures/9success/6skips; validation86tests/1failure/90 provider-truncated diagnostics,20 explicit download403 and6migration403 lines. Main12 run44014491905 reports13zero phases; full7291140B providerZIP still awaits complete retrieval/output comparison. EpisodeE-9e564213b742 records merged partial, without green-CI assertion; revertcheck2026-12-01T04:03:30Z. [pcc:unverified]

## Decisions made
Keep old/frozen Root/Consumer indices and old shared objects untouched. New untagged regular source13 follow-up has no workflow edits; observe actual checks after50min, no closed4485 push. Preserve both complete journal block multisets with maximum multiplicities, not heading-key deduplication. Current goal brief was materialized and run; explicit Chained user objective stays active. Next14 will reuse unchanged Richelot/dual/closed-fibre and17-witness bodies on the literal Cpi1_j1 monic model; freeze its scope/source before calls. [pcc:unverified]

## Open threads
Regular13 publication, compact persistent3904 queue, prospective14/PDF, actual nontrivial divisor correspondence, both dual[2] orientations and prime109/113 scalar cases. Full main12 providerZIP custody remains open. PaidSTOP e42949, lockedVLT1-8edec422, KB/Conductor127, original fullLP/EKS requirements and unsigned verification remain; do not launch paid jobs or access the vault. [pcc:unverified]

## Errors
Normal mixed resets stopped on unmaterialized tracked blobs, including an ENOSPC promisor download. Exact closed incomplete10711040B new download cache removed with full SHA/size/no-open-descriptor receipt; all scientific packets/indexed packs and old failed1.8MB cache retained. Standard read-tree -i plus soft reset and write-tree --missing-ok avoid unrelated blob existence hydration;14 missing current trees fetched narrowly. First narrow tree negotiation503 retained, ordinary no-op negotiation succeeds; no auth/header/provider change.59 owned already-packed loose objects removed by normal prune-packed after all decompressed object hashes checked, freeing6171223 compressed bytes; old objects untouched. One goal-brief call raced materialization and failed2, later exact materialized invocation succeeds. Baseline151 missing-type errors remain, not repaired or counted green. [pcc:unverified]
'''
j.write_text(merged+entry)
print('journal full blocks',len(a),len(b),'union',len(blocks),'bytes',j.stat().st_size,'freed inventory bytes',len(f.read_bytes()) if f.exists() else len(lzma.decompress(z.read_bytes())))
