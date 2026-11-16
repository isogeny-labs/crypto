"""Static reference integration checks; executes no numerical component."""
from pathlib import Path
import ast
import hashlib
import json
D=Path(__file__).resolve().parent

def main():
    old=(D.parent/'companion-compact-lift/compact.py').read_text()
    new=(D/'shared_frame.py').read_text()
    restored=old
    for before,after in json.loads((D/'integration.json').read_text()):
        assert restored.count(before)==1
        restored=restored.replace(before,after)
    assert ast.dump(ast.parse(restored))==ast.dump(ast.parse(new))
    oldsolve=(D.parent/'companion-compact-lift/solve_compact.py').read_text()
    newsolve=(D/'solve_shared.py').read_text()
    assert newsolve.count('from shared_frame import LiftFrame')==1
    assert ast.dump(ast.parse(newsolve.replace('from shared_frame import LiftFrame',
        'from compact import LiftFrame')))==ast.dump(ast.parse(oldsolve))
    files=list(D.rglob('*.py'))
    for f in files:ast.parse(f.read_text())
    pair=ast.parse((D/'pair.py').read_text())
    assert not any(isinstance(x,ast.Call) and isinstance(x.func,ast.Attribute)
        and x.func.attr in ('apply','_source','inv') for x in ast.walk(pair))
    print(json.dumps(dict(frame_integration_AST_equal=True,solver_integration_AST_equal=True,
        syntax_files=len(files),numerical_trials=0,source_calls=0,inversions=0,
        reference_frame_sha256=hashlib.sha256(old.encode()).hexdigest(),
        shared_frame_sha256=hashlib.sha256(new.encode()).hexdigest())))
if __name__=='__main__':main()
