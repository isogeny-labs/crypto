"""Static integration and frame ownership audit; imports no numerical code."""
from pathlib import Path
import ast
import hashlib
import json
D = Path(__file__).resolve().parent

def main():
    old = (D.parent/'companion-border-cache/solve_border.py').read_text()
    new = (D/'solve_compact.py').read_text()
    restored = new.replace('Compact-unwind version', 'Border-cached version').replace(
        'compact frames retain border tails', 'frames retain source objects')
    for before, after in json.loads((D/'integration.json').read_text()):
        assert restored.count(after) == 1
        restored = restored.replace(after, before)
    assert ast.dump(ast.parse(old)) == ast.dump(ast.parse(restored))
    tree = ast.parse((D/'compact.py').read_text())
    cls = next(x for x in tree.body if isinstance(x, ast.ClassDef))
    slots = ast.literal_eval(cls.body[0].value)
    assert set(slots) == {'row_tail','column_tail','inverse','rhs0','row','column',
                          'meter','profile','consumed'}
    # No numerical source call may enter this implementation.
    assert not any(isinstance(x, ast.Call) and isinstance(x.func, ast.Attribute)
                   and x.func.attr in {'apply','_source','_solve'} for x in ast.walk(tree))
    files = sorted(D.rglob('*.py'))
    for p in files: ast.parse(p.read_text())
    print(json.dumps(dict(normalized_AST_equal=True, source_application_calls=0,
        frame_slots=list(slots), syntax_files=len(files), numerical_trials=0,
        reference_sha256=hashlib.sha256(old.encode()).hexdigest(),
        candidate_sha256=hashlib.sha256(new.encode()).hexdigest(),
        ownership_interpretation='slots and assignments inspected; no dynamic heap test')))

if __name__ == '__main__': main()
