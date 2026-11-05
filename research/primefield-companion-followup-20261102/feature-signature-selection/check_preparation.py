"""Static-only checks; no producer imports, feature evaluations or sampling."""
from pathlib import Path
import ast,hashlib,json
D=Path(__file__).resolve().parent
records=json.loads((D/'source-preservation.json').read_text())
for record in records:
    copied=D/record['path']; source=D/record['source']
    assert copied.read_bytes()==source.read_bytes()
    assert hashlib.sha256(copied.read_bytes()).hexdigest()==record['sha256']
producer=ast.parse((D/'selection.py').read_text())
checker=ast.parse((D/'independent_check.py').read_text())
imports=[(x.module,tuple(a.name for a in x.names)) for x in producer.body if isinstance(x,ast.ImportFrom)]
assert imports==[('dataclasses',('dataclass',)),('itertools',('combinations',)),('reference_coefficients',('Basis','canonical'))]
assert [x.module for x in checker.body if isinstance(x,ast.ImportFrom)]==['itertools']
for tree in (producer,checker):
    for node in ast.walk(tree):
        if isinstance(node,ast.Call) and isinstance(node.func,ast.Name) and node.func.id=='range':
            assert not any((isinstance(x,ast.Name) and x.id=='r') or
                           (isinstance(x,ast.Attribute) and x.attr=='r')
                           for arg in node.args for x in ast.walk(arg))
files=list(D.rglob('*.py'))
for path in files:ast.parse(path.read_text())
print(json.dumps(dict(syntax_files=len(files),preserved_dependencies=len(records),
    selector_imports_verified=True,checker_import_separation=True,
    direct_field_range_loops=0,numerical_trials=0,
    scope='static source audit; no numerical theorem-record, stream or certificate checks')))
