"""Static source, convention and syntax audit; runs no field operations."""
from pathlib import Path
import ast,hashlib,json
D=Path(__file__).resolve().parent
reference=D.parent/'direct-cyclic-companion/generators.py'
if not reference.exists():
    reference=D.parent/'companion-metric-pivot/dependencies/direct-cyclic-companion/generators.py'
assert reference.read_bytes()==(D/'reference_constructor.py').read_bytes()
tree=ast.parse((D/'reference_constructor.py').read_text())
expected=ast.parse('patterns=[(0,)]+[(0,d) for d in range(1,q)]+[(0,d,e) for d in range(1,q) for e in range(d+1,q)]').body[0]
actual=next(x for x in ast.walk(tree) if isinstance(x,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='patterns' for t in x.targets))
assert ast.dump(actual)==ast.dump(expected)
producer=ast.parse((D/'diagonal.py').read_text())
build=next(x for x in producer.body if isinstance(x,ast.FunctionDef) and x.name=='build')
for node in ast.walk(build):
    if isinstance(node,ast.Call):
        assert not (isinstance(node.func,ast.Name) and node.func.id in ('pow','power'))
        assert not (isinstance(node.func,ast.Attribute) and node.func.attr=='apply')
        if isinstance(node.func,ast.Name) and node.func.id=='range':
            assert not any(isinstance(x,ast.Name) and x.id=='r' for x in ast.walk(node))
files=list(D.rglob('*.py'))
for path in files: ast.parse(path.read_text())
print(json.dumps(dict(syntax_files=len(files),reference_bytes_verified=True,
    reference_pattern_AST_verified=True,producer_field_sweep_calls=0,
    numerical_trials=0,reference_sha256=hashlib.sha256(reference.read_bytes()).hexdigest(),
    scope='static source and syntax only; independent numerical checker unexecuted')))
