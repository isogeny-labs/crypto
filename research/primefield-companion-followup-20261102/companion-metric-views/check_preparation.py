"""Static exact substitution audit; executes no field arithmetic."""
from pathlib import Path
import ast
import hashlib
import json
D=Path(__file__).resolve().parent

def main():
    records=json.loads((D/'integration.json').read_text())
    deps={}
    for record in records:
        raw=(D.parent/record['source']).read_bytes()
        deps[record['source']]=hashlib.sha256(raw).hexdigest()
        text=raw.decode()
        if record['extract']:text=text[text.index(record['extract']):]
        for before,after in record['substitutions']:
            assert text.count(before)==1
            text=text.replace(before,after)
        candidate=(D/record['target']).read_text()
        assert ast.dump(ast.parse(text))==ast.dump(ast.parse(candidate))
    files=list(D.rglob('*.py'))
    for p in files:ast.parse(p.read_text())
    tree=ast.parse((D/'metric_views.py').read_text())
    # Validation is confined to the root constructor, not derivation/check methods.
    calls=[x for x in ast.walk(tree) if isinstance(x,ast.Call) and
           isinstance(x.func,ast.Name) and x.func.id=='validate_metric']
    assert len(calls)==1
    for node in tree.body:
        if isinstance(node,ast.FunctionDef) or isinstance(node,ast.ClassDef) and node.name=='MetricView':
            assert not any(isinstance(x,ast.Call) and isinstance(x.func,ast.Attribute)
                and x.func.attr in ('mul','inv','apply') for x in ast.walk(node))
    print(json.dumps(dict(integration_AST_checks=len(records),syntax_files=len(files),
        numerical_trials=0,dependencies=deps,root_validation_call_sites=1,
        view_arithmetic_call_sites=0,scope='static syntax and substitutions, not dynamic validation')))
if __name__=='__main__':main()
