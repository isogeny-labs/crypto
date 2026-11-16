from pathlib import Path
import hashlib,json,ast,difflib
p=Path(__file__).parent;o=Path('/var/tmp/pencil-musl-build-supervision-20261031/build-receipted.py');s=o.read_text();(p/'original-build-receipted.py').write_text(s)
s=s.replace("p.add_argument('--commit',required=True);", "p.add_argument('--source-root',required=True);p.add_argument('--commit',required=True);")
s=s.replace("R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');T=", "ORIGINAL=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');R=Path(a.source_root);assert R.is_absolute() and R==R.resolve() and R.parent==Path('/var/tmp') and R.name.startswith('pencil-musl-source-') and R!=ORIGINAL;T=")
s=s.replace("head=command('head-before'", "assert command('source-top-level',['git','rev-parse','--show-toplevel']).strip()==str(R)\n assert command('source-detached',['git','rev-parse','--abbrev-ref','HEAD']).strip()=='HEAD'\n receipt['source_root']=str(R)\n head=command('head-before'")
s=s.replace("['git','ls-files','--others','--exclude-standard']", "['git','ls-files','--others']")
(p/'build-receipted-isolated.py').write_text(s);ast.parse(s)
(p/'amendment.patch').write_text(''.join(difflib.unified_diff(o.read_text().splitlines(True),s.splitlines(True),fromfile=str(o),tofile='build-receipted-isolated.py')))
(p/'static-validation.json').write_text(json.dumps({'python_ast_parse':'passed','execution_performed':False,'original_sha256':hashlib.sha256(o.read_bytes()).hexdigest(),'candidate_sha256':hashlib.sha256(s.encode()).hexdigest(),'source_gate_change':'untracked check additionally rejects ignored files; no exclusion added','git_worktree_created':False,'build_started':False},indent=2)+'\n')
