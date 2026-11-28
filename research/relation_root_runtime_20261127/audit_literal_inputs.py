"""File metadata audit only: verify existing literals against the exact Git tree."""
from pathlib import Path
import hashlib,json,re,subprocess,sys
ROOT=Path(__file__).resolve().parents[2]
roots=['src','crates','tools/factor-base','tools/volcano-cli','isogeny_algos','research/pencil_large_prime_20261029']
pattern=re.compile(r'(?:include(?:_str|_bytes)?!\s*\(\s*|#\[path\s*=\s*)"([^"\n]+)"')
seen=set();targets=set();outside=set()
while True:
    pending=[p for d in roots for p in (ROOT/d).rglob('*.rs') if p not in seen]
    if not pending:break
    for source in pending:
        seen.add(source)
        for literal in pattern.findall(source.read_text(errors='replace')):
            if '\\' in literal:continue
            p=(source.parent/literal).resolve()
            if not p.is_relative_to(ROOT):outside.add(str(p));continue
            targets.add(p.relative_to(ROOT).as_posix())
            if p.is_file() and p.suffix=='.rs':
                parent=p.parent.relative_to(ROOT).as_posix()
                if parent not in roots:roots.append(parent)
entries={}
for line in subprocess.check_output(['git','ls-tree','-r','HEAD','--',*sorted(targets)],cwd=ROOT,text=True).splitlines():
    meta,path=line.split('\t',1);mode,kind,blob=meta.split()
    if mode=='100644' and kind=='blob' and path in targets:entries[path]=blob
mismatches={};verified={};missing=[]
for name,expected in sorted(entries.items()):
    p=ROOT/name
    if not p.is_file():missing.append(name);continue
    h=hashlib.sha1(('blob '+str(p.stat().st_size)+'\0').encode());s=hashlib.sha256()
    with p.open('rb') as stream:
        for block in iter(lambda:stream.read(1048576),b''):h.update(block);s.update(block)
    actual=h.hexdigest()
    record={'git_blob':expected,'actual_git_blob':actual,'sha256':s.hexdigest(),'bytes':p.stat().st_size}
    if actual!=expected:mismatches[name]=record
    else:verified[name]=record
result={'schema':'literal-git-content-audit/v1','source_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'verified':verified,'mismatches':mismatches,'missing_tracked':missing,'untracked_or_parser_literals':sorted(targets-entries.keys()),'outside_root':sorted(outside),'complete_parser':False,'presence_alone_is_never_admission':True,'numerical_execution':False}
with Path(sys.argv[1]).open('x') as out:json.dump(result,out,indent=2);out.write('\n')
print(json.dumps({'verified':len(verified),'mismatches':mismatches,'missing_tracked':missing}))
raise SystemExit(bool(mismatches or missing))
