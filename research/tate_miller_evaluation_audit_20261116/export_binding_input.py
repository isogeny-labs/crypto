"""Export one immutable written-source input; Lean computes binding classification."""
from pathlib import Path
import argparse,hashlib,importlib.util,json,sys
s=Path(__file__).resolve().parent;r=s.parents[1]
a=argparse.ArgumentParser();a.add_argument('--out',type=Path,required=True);a=a.parse_args();a.out.mkdir(parents=True,exist_ok=False)
spec=importlib.util.spec_from_file_location('frozen_pcc',r/'tools/pcc/pcc.py');pcc=importlib.util.module_from_spec(spec);sys.modules[spec.name]=pcc;spec.loader.exec_module(pcc)
name='TateMillerEvaluationAudit.actual_tate_miller_evaluation_certificate';source,text=pcc.lean_find_declaration(pcc.lean_sources(s/'lean'),name)
b=source.read_bytes();h=hashlib.sha256(b).hexdigest();out=a.out/source.name;out.write_bytes(b)
x=dict(schema='statement-binding-wrapper-input/v1',cases=[dict(case_id='actual-tate-miller-evaluation-main',source_plan='research/tate_miller_evaluation_audit_20261116/sources.json',theorem=name,short_name=name.split('.')[-1],expected_decl_sha256=hashlib.sha256(text.encode()).hexdigest(),recorded_statement=text,candidates=[dict(source_path=str(source.relative_to(r)),export_path=str(out.resolve()),bytes=len(b),sha256=h,declared_source_sha256=h)])])
(a.out/'manifest.json').write_text(json.dumps(x,indent=2,ensure_ascii=False)+'\n');print(json.dumps(dict(manifest=str((a.out/'manifest.json').resolve()),source_sha256=h,recorded_characters=len(text))))
