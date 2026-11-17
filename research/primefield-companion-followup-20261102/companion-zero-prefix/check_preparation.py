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
files=list(D.rglob('*.py'))
for path in files: ast.parse(path.read_text())
print(json.dumps(dict(integration_AST_checks=len(records),syntax_files=len(files),
    numerical_trials=0,dependencies=deps,scope='static declared substitutions, not numerical validation')))
