"""Static substitution and syntax audit. No numerical producer imports."""
from pathlib import Path
import ast, hashlib, json
D=Path(__file__).resolve().parent
records=json.loads((D/'integration.json').read_text())
deps={}
for record in records:
    raw=(D.parent/record['source']).read_bytes()
    deps[record['source']]=hashlib.sha256(raw).hexdigest()
    text=raw.decode()
    for before,after in record['substitutions']:
        assert text.count(before)==1
        text=text.replace(before,after)
    assert ast.dump(ast.parse(text))==ast.dump(ast.parse((D/record['target']).read_text()))
tree=ast.parse((D/'deletion.py').read_text())
frame=next(x for x in tree.body if isinstance(x,ast.ClassDef) and x.name=='ZeroFrame')
assert not any(isinstance(x,ast.Call) and isinstance(x.func,ast.Attribute)
               and x.func.attr in ('mul','inv','apply','dot') for x in ast.walk(frame))
files=list(D.rglob('*.py'))
for path in files: ast.parse(path.read_text())
print(json.dumps(dict(integration_AST_checks=len(records),syntax_files=len(files),
    numerical_trials=0,dependencies=deps,scope='static declared substitutions, not numerical validation')))
