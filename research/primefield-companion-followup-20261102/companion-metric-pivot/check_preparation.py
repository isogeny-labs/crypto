"""Syntax and source-dependency audit; no numerical imports or execution."""
from pathlib import Path
import ast
import hashlib
import json
D=Path(__file__).resolve().parent

def main():
    files=list(D.rglob('*.py'))
    for p in files:ast.parse(p.read_text())
    refs=['direct-cyclic-companion/generators.py','companion-pivot-transport/pivot.py',
          'companion-rank-solver/solve.py','companion-recursive-inverse/recursive.py']
    def resolve(name):
        original=D.parent/name
        return original if original.exists() else D/'dependencies'/name
    assert (D/'dependencies'/refs[0]).read_bytes()==resolve(refs[0]).read_bytes()
    constructor=ast.parse(resolve(refs[0]).read_text())
    build=next(x for x in constructor.body if isinstance(x,ast.FunctionDef) and x.name=='build')
    # The constructor currently returns dict(...) keywords, not a literal.
    keywords=[k.arg for node in ast.walk(build) if isinstance(node,ast.Call)
              and isinstance(node.func,ast.Name) and node.func.id=='dict' for k in node.keywords]
    assert 'inverse_diagonal' in keywords
    print(json.dumps(dict(syntax_files=len(files),numerical_trials=0,
        dependencies={s:hashlib.sha256(resolve(s).read_bytes()).hexdigest() for s in refs},
        constructor_exposes_inverse_metric=True,
        scope='syntax and dependency identity; no algebraic or dynamic validation')))
if __name__=='__main__':main()
