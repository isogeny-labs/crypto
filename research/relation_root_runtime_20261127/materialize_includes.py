"""Metadata-only batch restoration of literal inputs from this exact Git tree."""
from pathlib import Path
import hashlib,json,re,subprocess
ROOT=Path(__file__).resolve().parents[2]
STUDY=Path(__file__).resolve().parent
GIT=['git','-c','gc.auto=0','-c','maintenance.auto=false']
roots=['src','crates','examples','tools/factor-base','tools/volcano-cli','isogeny_algos','research/pencil_large_prime_20261029']
pattern=re.compile(r'(?:include(?:_str|_bytes)?!\s*\(\s*|#\[path\s*=\s*)"([^"\n]+)"')
seen=set(); restored={}; absent=set(); outside=set(); rounds=[]
with (STUDY/'raw'/'hydration-batches.jsonl').open('x') as log:
    while True:
        missing=set()
        sources=[p for root in roots for p in (ROOT/root).rglob('*.rs') if p not in seen]
        for source in sources:
            seen.add(source)
            for literal in pattern.findall(source.read_text(errors='replace')):
                if '\\' in literal: continue
                target=(source.parent/literal).resolve()
                if not target.is_relative_to(ROOT): outside.add(str(target)); continue
                if not target.is_file(): missing.add(target.relative_to(ROOT).as_posix())
        if not missing: break
        entries={}
        # ls-tree does not read any blob contents or fetch them.
        for line in subprocess.check_output(GIT+['ls-tree','-r','HEAD','--',*sorted(missing)],cwd=ROOT,text=True).splitlines():
            meta,path=line.split('\t',1)
            mode,kind,blob=meta.split()
            if mode=='100644' and kind=='blob': entries[path]=blob
        absent.update(missing-entries.keys())
        if not entries: break
        request={'round':len(rounds),'requested':entries,'recorded_before_restore':True}
        log.write(json.dumps(request)+'\n'); log.flush()
        result=subprocess.run(GIT+['restore','--worktree','--ignore-skip-worktree-bits','--',*sorted(entries)],cwd=ROOT)
        log.write(json.dumps({'round':len(rounds),'exit_code':result.returncode})+'\n'); log.flush()
        if result.returncode: raise SystemExit(result.returncode)
        for path,blob in entries.items():
            restored[path]={'git_blob':blob,'sha256':hashlib.sha256((ROOT/path).read_bytes()).hexdigest()}
        rounds.append(len(entries))
        # Restored Rust files can themselves refer to more literal inputs.
        for path in entries:
            if path.endswith('.rs') and (ROOT/path).parent.as_posix() not in roots:
                roots.append(path.rsplit('/',1)[0])
    receipt={'schema':'literal-source-hydration/v2','source_commit':subprocess.check_output(GIT+['rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'restored':restored,'round_sizes':rounds,'absent_tracked_inputs':sorted(absent),'outside_root_not_restored':sorted(outside),'dynamic_macros':'compiler authoritative','numerical_execution':False}
    with (STUDY/'INCLUDE_HYDRATION.json').open('x') as stream: json.dump(receipt,stream,indent=2); stream.write('\n')
    print(json.dumps({'restored_files':len(restored),'rounds':rounds,'absent_inputs':len(absent)}))
