"""One-session Git plumbing: publish immutable local objects through gh REST.

Does not move refs, sign claims, rewrite history, or print credentials.
This is repository management, not a research/benchmark execution harness.
"""
import base64
import datetime
import json
import pathlib
import re
import subprocess
import sys
import tempfile

repo, expected, head = sys.argv[1:]
assert re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+',repo)
assert all(re.fullmatch(r'[0-9a-f]{40}',s) for s in (expected,head))

def git(*args):
    return subprocess.check_output(['git',*args])

def api(endpoint,body):
    with tempfile.NamedTemporaryFile('w',suffix='.json',delete=False) as f:
        json.dump(body,f); name=f.name
    try:
        p=subprocess.run(['gh','api','--method','POST',f'repos/{repo}/{endpoint}','--input',name],text=True,capture_output=True)
        if p.returncode:
            raise RuntimeError(p.stderr.strip())
        return json.loads(p.stdout)
    finally:
        pathlib.Path(name).unlink()

def identity(header):
    m=re.fullmatch(r'(.*) <([^<>]+)> (\d+) ([+-]\d{4})',header)
    assert m,header
    name,email,stamp,offset=m.groups()
    minutes=int(offset[1:3])*60+int(offset[3:])
    if offset[0]=='-':minutes=-minutes
    tz=datetime.timezone(datetime.timedelta(minutes=minutes))
    date=datetime.datetime.fromtimestamp(int(stamp),tz).isoformat()
    return dict(name=name,email=email,date=date)

commits=git('rev-list','--reverse',f'{expected}..{head}').decode().splitlines()
uploaded=set()
for commit in commits:
    raw=git('cat-file','commit',commit).decode()
    header,message=raw.split('\n\n',1)
    meta={}
    for line in header.splitlines():
        key,_,val=line.partition(' ')
        meta.setdefault(key,[]).append(val)
    assert 'gpgsig' not in meta,'signed commits need a different transport'
    assert len(meta.get('parent',[]))==1
    parent=meta['parent'][0]
    entries=[]
    paths=git('diff','--name-only','-z',parent,commit).decode().split('\0')
    for path in filter(None,paths):
        item=git('ls-tree',commit,'--',path).decode().strip()
        if not item:
            entries.append(dict(path=path,mode='100644',type='blob',sha=None));continue
        row=item.split('\t',1)[0].split()
        mode,kind,sha=row
        assert kind=='blob',(path,kind)
        if sha not in uploaded:
            content=git('cat-file','blob',sha)
            got=api('git/blobs',dict(encoding='base64',content=base64.b64encode(content).decode()))
            assert got['sha']==sha,(path,sha,got['sha'])
            uploaded.add(sha)
        entries.append(dict(path=path,mode=mode,type=kind,sha=sha))
    base_tree=git('rev-parse',parent+'^{tree}').decode().strip()
    tree=api('git/trees',dict(base_tree=base_tree,tree=entries))
    assert tree['sha']==meta['tree'][0],('tree mismatch',commit,tree['sha'])
    body=dict(message=message,tree=tree['sha'],parents=meta['parent'],
              author=identity(meta['author'][0]),committer=identity(meta['committer'][0]))
    got=api('git/commits',body)
    assert got['sha']==commit,('commit mismatch; no refs moved',commit,got['sha'])
    print(json.dumps(dict(commit=commit,tree=tree['sha'],changed_paths=len(entries),objects_published=True)),flush=True)
print(json.dumps(dict(head=head,commits=len(commits),refs_moved=False)),flush=True)
