"""Bounded read-only ancestry indexing, not a scientific experiment."""
from pathlib import Path
import hashlib, json, resource, sys, time

ROOT = Path('/workspace/generalized_roots')
HERE = Path(__file__).resolve().parent
LEGACY = 'lab/ledger_sealed_20261021_v9'
VERIFY = 'research/ledger_epoch_s3_20261021_v1/PARENT_VERIFICATION.json'
VERIFY_SHA = '89c5a0f3668a55907519fbfc9145914556057d55478b102df95fe6530a646f50'
STATE_SHA = '0d4d843b27fcfb469c7c08cfa21d76739e8f54c33b337a0a189f5130c63568cb'
ADMISSIONS = ['research/'+v+'/ADMISSION_FROZEN.json' for v in
              ['s3_cached_native_20261022_v1','s3_full_native_20261022_v1',
               's3_nearword_shadow_20261023_v1','s3_source_activation_20261024_v1']]

def sha(path):
    h=hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda:stream.read(1048576),b''): h.update(chunk)
    return h.hexdigest()

def local(name):
    if not isinstance(name,str): raise ValueError('path must be text')
    p=(ROOT/name).resolve()
    if not p.is_relative_to(ROOT): raise ValueError('path escapes project')
    return p

def read(path):
    def pairs(items):
        out={}
        for k,v in items:
            if k in out: raise ValueError('duplicate JSON key')
            out[k]=v
        return out
    with path.open() as stream:
        return json.load(stream,object_pairs_hook=pairs,
                         parse_constant=lambda x:(_ for _ in ()).throw(ValueError('nonfinite JSON')))

def need(value,message):
    if not value: raise ValueError(message)

def authenticated(name,digest,trusted,evidence):
    if digest in trusted.get(name,set()):return
    for parent,hashes in trusted.items():
        if not parent.endswith('/ADMISSION_FROZEN.json'):continue
        path=local(parent);actual=sha(path)
        if actual not in hashes:continue
        admission=read(path)
        if admission.get('required_sha256',{}).get(name)==digest:
            evidence.append(dict(path=parent,sha256=actual,binding='trusted child admission authenticates receipt'))
            return
    raise ValueError('metadata receipt is not authenticated by a verified child registration')

def add_epoch(name,seen):
    need(name not in seen,'ancestry cycle');seen.add(name)

def bind_receipt(entry,receipt):
    need(receipt['journal_sha256']==entry['journal_sha256'],'journal receipt mismatch')
    need(receipt['last_record_sha256']==entry['terminal_sha256'],'terminal receipt mismatch')
    need(receipt['sealed_path']==entry['path'],'path receipt mismatch')
    need(isinstance(receipt.get('state_sha256'),str),'receipt lacks state hash')
    return dict(path=receipt['sealed_path'],state_sha256=receipt['state_sha256'],
                journal_sha256=receipt['journal_sha256'],terminal_sha256=receipt['last_record_sha256'],
                kind='path-receipt')

def modern(entry):
    need(isinstance(entry.get('state_sha256'),str),'modern parent lacks state hash')
    need(isinstance(entry.get('journal_sha256'),str),'modern parent lacks journal hash')
    terminal=entry.get('terminal_record_sha256',entry.get('last_record_sha256'))
    need(isinstance(terminal,str),'modern parent lacks terminal hash')
    return dict(path=entry['sealed_path'],state_sha256=entry['state_sha256'],
                journal_sha256=entry['journal_sha256'],terminal_sha256=terminal,kind='sealed-path')

def relocation_binding(entry,admission,verification):
    need(entry.get('sealed_parent_path')==LEGACY,'unknown relocation parent')
    need(admission['parent_path']==LEGACY,'admission path mismatch')
    need(admission['parent_journal_sha256']==entry['parent_journal_sha256'],'admission journal mismatch')
    need(admission['parent_terminal_sha256']==entry['terminal_sha256'],'admission terminal mismatch')
    required=admission['required_sha256']
    need(required[VERIFY]==VERIFY_SHA and required[LEGACY+'/state.json']==STATE_SHA,'admission byte binding mismatch')
    need(verification.get('canonical_terminal_verified') is True and
         verification.get('all_suffix_records_verified') is True,'legacy chain unverified')
    need(verification.get('state_matches_terminal') is True,'legacy terminal state unverified')
    journal=verification.get('journal_sha256',verification.get('parent_journal_sha256'))
    terminal=verification.get('last_record_sha256',verification.get('terminal_sha256'))
    need(journal==entry['parent_journal_sha256'] and terminal==entry['terminal_sha256'],'verification relocation mismatch')
    return dict(path=LEGACY,state_sha256=STATE_SHA,journal_sha256=journal,
                terminal_sha256=terminal,kind='relocation-with-frozen-admission')

def next_parent(state,verified_legacy_root=False):
    program=state['program']
    if program is None:return None
    need(isinstance(program,dict),'unknown program/root schema')
    if 'sealed_parent' not in program:
        need(verified_legacy_root,'unverified missing parent key')
        return None
    parent=program['sealed_parent']
    need(parent is None or isinstance(parent,dict),'parent must be object or explicit null')
    return parent

def normalize(entry,trusted,evidence):
    need(isinstance(entry,dict),'parent must be object')
    forms=int('sealed_path'in entry)+int('path'in entry and'receipt'in entry)+int(entry.get('schema')=='ledger_epoch_relocation_v1')
    need(forms==1,'ambiguous or unsupported parent schema')
    if 'sealed_path' in entry: return modern(entry)
    if 'path' in entry and 'receipt' in entry:
        path=local(entry['receipt']);digest=sha(path);authenticated(entry['receipt'],digest,trusted,evidence);receipt=read(path)
        evidence.append(dict(path=str(path.relative_to(ROOT)),sha256=digest,binding='trusted child source plus entry journal/terminal/path'))
        return bind_receipt(entry,receipt)
    if entry.get('schema')=='ledger_epoch_relocation_v1':
        need(entry.get('sealed_parent_path')==LEGACY,'unknown relocation parent')
        for name in ADMISSIONS:
            p=local(name);digest=sha(p)
            if digest not in trusted.get(name,set()): continue
            admission=read(p)
            verify_path=local(VERIFY);need(sha(verify_path)==VERIFY_SHA,'verification receipt changed')
            verification=read(verify_path)
            result=relocation_binding(entry,admission,verification)
            evidence.extend([dict(path=name,sha256=digest,binding='trusted child registration'),
                             dict(path=VERIFY,sha256=VERIFY_SHA,binding='trusted admission required_sha256')])
            return result
        raise ValueError('no trusted child registration binds a relocation admission')
    raise ValueError('unsupported parent schema')

def semantic(spec,identity):
    command=[('<id>' if a==spec['id'] else a) for a in spec['command']]
    if '-m' in command: command[command.index('-m')+1]='<module>'
    files=identity['files_sha256']
    return dict(command=command,code=sorted(h for n,h in files.items() if Path(n).suffix in ['.py','.cpp','.hpp','.rs']),
                inputs=sorted(files[n] for n in spec['input_paths']),timeout=spec['timeout_seconds'],
                memory=spec['memory_limit_mb'],cpu=spec['cpu'])

def build():
    resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
    started=time.monotonic();trusted={};evidence=[];epochs=[];items=[];seen=set()
    program=read(ROOT/'lab/research_program.json');entry=program.get('sealed_parent')
    active=read(ROOT/'lab/ledger/state.json')
    need(sha(ROOT/'lab/research_program.json')==active['program']['manifest_sha256'],'active manifest changed')
    for item in active['experiments'].values():
        for n,h in item.get('registered_identity',{}).get('files_sha256',{}).items(): trusted.setdefault(n,set()).add(h)
    del active
    while entry is not None:
        parent=normalize(entry,trusted,evidence);name=parent['path'];add_epoch(name,seen)
        path=local(name)/'state.json';need(sha(path)==parent['state_sha256'],'sealed state changed')
        state=read(path)
        need(state['schema']=='research_ledger_state_v1','state schema')
        epochs.append(dict(**parent,state_bytes=path.stat().st_size,experiments=len(state['experiments'])))
        for eid,item in state['experiments'].items():
            identity=item.get('registered_identity')
            fingerprints=set(a['fingerprint'] for a in item['attempts'])
            if item.get('registered_fingerprint'): fingerprints.add(item['registered_fingerprint'])
            need(identity is not None or not fingerprints,'historical fingerprint lacks semantic identity')
            sem=None if identity is None else semantic(item['spec'],identity)
            sem_sha=None if sem is None else hashlib.sha256(json.dumps(sem,sort_keys=True,separators=(',',':'),allow_nan=False).encode()).hexdigest()
            items.append(dict(epoch=name,id=eid,status=item['status'],fingerprints=sorted(fingerprints),
                              registered_fingerprint=item.get('registered_fingerprint'),semantic_sha256=sem_sha))
            if identity:
                for n,h in identity['files_sha256'].items(): trusted.setdefault(n,set()).add(h)
        entry=next_parent(state,name==LEGACY and parent['state_sha256']==STATE_SHA)
        del state
    result=dict(schema='typed_historical_duplicate_index_v1',epochs=epochs,experiments=items,
                evidence=evidence,root_reached=True,original_bytes_unmodified=True,
                elapsed_seconds=time.monotonic()-started,peak_self_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                journal_bytes_read=0,method='stream full-state SHA then parse one state at a time under4GiB; no raw state copy to disk')
    data=json.dumps(result,sort_keys=True,separators=(',',':')).encode();need(len(data)<=262144,'compact index cap')
    with (HERE/'INDEX.json').open('xb') as stream:stream.write(data)
    receipt=dict(index_sha256=hashlib.sha256(data).hexdigest(),index_bytes=len(data),epochs=len(epochs),
                 experiments=len(items),full_history=True,root_reached=True,old_files_modified=False,
                 source_sha256=sha(Path(__file__)),evidence=evidence,peak_self_rss_kib=result['peak_self_rss_kib'])
    with (HERE/'INDEX_RECEIPT.json').open('x') as stream:json.dump(receipt,stream,indent=2)
    print(json.dumps(receipt))

def fixtures():
    good=dict(path='lab/example',journal_sha256='j',terminal_sha256='t')
    receipt=dict(sealed_path='lab/example',journal_sha256='j',last_record_sha256='t',state_sha256='s')
    tests=[]
    need(bind_receipt(good,receipt)['state_sha256']=='s','valid receipt fixture');tests.append('valid path-receipt')
    modern_good=dict(sealed_path='lab/example',state_sha256='s',journal_sha256='j',terminal_record_sha256='t')
    need(modern(modern_good)['state_sha256']=='s','modern fixture');tests.append('valid modern')
    relocation=dict(schema='ledger_epoch_relocation_v1',sealed_parent_path=LEGACY,parent_journal_sha256='j',terminal_sha256='t')
    admission=dict(parent_path=LEGACY,parent_journal_sha256='j',parent_terminal_sha256='t',required_sha256={VERIFY:VERIFY_SHA,LEGACY+'/state.json':STATE_SHA})
    verification=dict(canonical_terminal_verified=True,all_suffix_records_verified=True,state_matches_terminal=True,journal_sha256='j',last_record_sha256='t')
    need(relocation_binding(relocation,admission,verification)['state_sha256']==STATE_SHA,'relocation fixture');tests.append('valid relocation')
    need(next_parent({'program':None}) is None and next_parent({'program':{'sealed_parent':None}})is None,'root fixture');tests.append('valid explicit null roots')
    need(next_parent({'program':{}},True)is None,'verified legacy root fixture');tests.append('valid hash-bound legacy root missing parent key')
    try:next_parent({'program':{}})
    except ValueError:tests.append('rejected unverified missing root key')
    else:raise ValueError('unverified missing key accepted')
    changed=dict(receipt,state_sha256='altered')
    original_digest=hashlib.sha256(json.dumps(receipt,sort_keys=True).encode()).hexdigest()
    changed_digest=hashlib.sha256(json.dumps(changed,sort_keys=True).encode()).hexdigest()
    try:authenticated('receipt.json',changed_digest,{'receipt.json':{original_digest}},[])
    except ValueError:tests.append('rejected altered receipt statehash despite unchanged journal/path')
    else:raise ValueError('unauthenticated changed statehash accepted')
    seen=set();add_epoch('one',seen)
    try:add_epoch('one',seen)
    except ValueError:tests.append('rejected ancestry cycle')
    else:raise ValueError('cycle accepted')
    for key in ['parent_journal_sha256','parent_terminal_sha256','parent_path']:
        bad=dict(admission);bad[key]=None
        try:relocation_binding(relocation,bad,verification)
        except (KeyError,ValueError):tests.append('rejected relocation '+key)
        else:raise ValueError('relocation corruption accepted')
    try:normalize(dict(modern_good,path='lab/example',receipt='irrelevant'),{},[])
    except ValueError:tests.append('rejected ambiguous format')
    else:raise ValueError('ambiguous metadata accepted')
    try:next_parent({'program':7})
    except ValueError:tests.append('rejected invalid program')
    else:raise ValueError('invalid program accepted')
    for key in ['journal_sha256','last_record_sha256','sealed_path','state_sha256']:
        bad=dict(receipt);bad[key]=None
        try:bind_receipt(good,bad)
        except (KeyError,ValueError):tests.append('rejected '+key)
        else:raise ValueError('corruption accepted '+key)
    for entry in [{},{'schema':'unknown'}, {'sealed_path':'x'}, {'schema':'ledger_epoch_relocation_v1','sealed_parent_path':'wrong'}]:
        try:normalize(entry,{},[])
        except (KeyError,ValueError):tests.append('rejected unsupported/incomplete')
        else:raise ValueError('invalid parent accepted')
    try:local('../outside')
    except ValueError:tests.append('rejected path escape')
    else:raise ValueError('path escape accepted')
    result=dict(schema='typed_ancestry_metadata_fixtures_v1',tests=tests,passed=len(tests),scientific_executions=0)
    with (HERE/'FIXTURES.json').open('x') as stream:json.dump(result,stream,indent=2)
    print(json.dumps(result))

if __name__=='__main__':
    if sys.argv[1]=='fixtures':fixtures()
    elif sys.argv[1]=='index':build()
    else:raise ValueError('mode must be fixtures or index')
