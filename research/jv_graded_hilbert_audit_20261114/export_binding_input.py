"""Export one immutable source-processing input; audit verdicts are computed by Lean."""
from pathlib import Path
import argparse, hashlib, importlib.util, json, sys

here = Path(__file__).resolve().parent
repo = here.parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--out', type=Path, required=True)
args = parser.parse_args()
args.out.mkdir(parents=True, exist_ok=False)
spec = importlib.util.spec_from_file_location('frozen_pcc', repo / 'tools/pcc/pcc.py')
pcc = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = pcc
spec.loader.exec_module(pcc)
theorem = 'JVGradedHilbertAudit.canonical_graded_hilbert_certificate'
source = here / 'lean/GradedCertificate.lean'
data = source.read_bytes()
sha = hashlib.sha256(data).hexdigest()
decl = pcc.lean_find_declaration(pcc.lean_sources(here / 'lean'), theorem)
assert decl and decl[0].resolve() == source.resolve()
statement = decl[1]
copied = args.out / source.name
copied.write_bytes(data)
manifest = dict(schema='statement-binding-wrapper-input/v1', cases=[dict(
    case_id='canonical-jv-graded-certificate', source_plan='research/jv_graded_hilbert_audit_20261114/sources.json',
    theorem=theorem, short_name=theorem.split('.')[-1],
    expected_decl_sha256=hashlib.sha256(statement.encode()).hexdigest(),
    recorded_statement=statement, candidates=[dict(
        source_path=str(source.relative_to(repo)), export_path=str(copied.resolve()),
        bytes=len(data), sha256=sha, declared_source_sha256=sha)])])
(args.out / 'manifest.json').write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + '\n')
print(json.dumps(dict(manifest=str((args.out / 'manifest.json').resolve()),
                     source_sha256=sha, recorded_characters=len(statement))))
