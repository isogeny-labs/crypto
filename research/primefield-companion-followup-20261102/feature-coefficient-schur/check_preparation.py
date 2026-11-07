"""Static-only source audit; never imports or runs arithmetic primitives."""
from pathlib import Path
import ast, hashlib, json
D=Path(__file__).resolve().parent
reference=D.parent/'direct-cyclic-companion/generators.py'
if not reference.exists():
    reference=D.parent/'companion-metric-pivot/dependencies/direct-cyclic-companion/generators.py'
assert reference.read_bytes()==(D/'reference_constructor.py').read_bytes()
ref=ast.parse(reference.read_text())
producer=ast.parse((D/'coefficient.py').read_text())
def pattern(tree):
    return next(x for x in ast.walk(tree) if isinstance(x,ast.Assign) and
                any(isinstance(t,ast.Name) and t.id=='patterns' for t in x.targets))
assert ast.dump(pattern(ref))==ast.dump(pattern(producer))
class Names(ast.NodeTransformer):
    def visit_Name(self,node):
        if node.id=='meter':node.id='m'
        return node
    def visit_arg(self,node):
        if node.arg=='meter':node.arg='m'
        return node
p1=next(x for x in ref.body if isinstance(x,ast.FunctionDef) and x.name=='power')
p2=next(x for x in producer.body if isinstance(x,ast.FunctionDef) and x.name=='power')
assert ast.dump(p1)==ast.dump(Names().visit(p2))
checker=ast.parse((D/'independent_check.py').read_text())
imports=[x.module for x in ast.walk(checker) if isinstance(x,ast.ImportFrom)]
assert imports==['itertools'] and not any(isinstance(x,ast.Import) for x in ast.walk(checker))
for tree in (producer,checker):
    for node in ast.walk(tree):
        if isinstance(node,ast.Call) and isinstance(node.func,ast.Name) and node.func.id=='range':
            assert not any((isinstance(x,ast.Name) and x.id=='r') or
                           (isinstance(x,ast.Attribute) and x.attr=='r')
                           for arg in node.args for x in ast.walk(arg))
files=list(D.rglob('*.py'))
for path in files:ast.parse(path.read_text())
print(json.dumps(dict(syntax_files=len(files),reference_bytes_verified=True,
    pattern_AST_verified=True,binary_power_AST_verified=True,
    checker_import_separation=True,direct_field_range_loops=0,numerical_trials=0,
    scope='static source audit only; no arithmetic or runtime checks')))
