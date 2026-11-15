"""Static count of instrumented native products in the preserved fused source.

Counts syntax only on successful paths; it does not execute arithmetic,
estimate time, or count Python multiplications inside field Meter methods.
"""
from pathlib import Path
import ast
import hashlib
import json

D = Path(__file__).resolve().parent


def main():
    path = D.parent/'companion-fused/multiply.py'
    raw = path.read_bytes()
    tree = ast.parse(raw)
    apply = next(node for node in tree.body if isinstance(node, ast.FunctionDef)
                 and node.name == 'apply')
    loops = [(i, node) for i, node in enumerate(apply.body) if isinstance(node, ast.For)]
    if len(loops) != 1:
        raise ValueError('fused source loop structure changed')
    position, loop = loops[0]
    if ast.dump(loop.iter) != ast.dump(ast.parse('range(s)', mode='eval').body):
        raise ValueError('generator-width loop expected')
    def calls(nodes):
        out = {}
        for top in nodes:
            for node in ast.walk(top):
                if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == 'mul':
                    if len(node.args) != 3 or not isinstance(node.args[2], ast.Constant):
                        raise ValueError('instrumented product signature changed')
                    kind = node.args[2].value
                    out[kind] = out.get(kind, 0) + 1
        return out
    setup, column, final = calls(apply.body[:position]), calls(loop.body), calls(apply.body[position+1:])
    if (setup != {'bound': 7} or column != {'convolution': 2, 'fold': 2}
            or final != {'scale': 1}):
        raise ValueError('instrumented product schedule changed')
    print(json.dumps(dict(source_sha256=hashlib.sha256(raw).hexdigest(),
                          setup=setup, per_generator_column=column, final=final,
                          native_products_per_successful_application='4*w+8',
                          numerical_trials=0, interpretation='static syntax count, not timing')))


if __name__ == '__main__':
    main()
