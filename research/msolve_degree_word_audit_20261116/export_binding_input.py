"""Export written-source metadata. Lean computes the binding classifications."""
from pathlib import Path
import argparse,hashlib,importlib.util,json,sys
s=Path(__file__).resolve().parent;r=s.parents[1]
a=argparse.ArgumentParser();a.add_argument('--out',type=Path,required=True);a.add_argument('--legacy',action='store_true');a=a.parse_args();a.out.mkdir(parents=True,exist_ok=False)
if a.legacy:
    plan=json.loads((r/'research/theorem_census_20261109/f4-model-sources.json').read_text())
    formal=plan['formal'];name=formal['theorem'];text=formal['statement'];expected=formal['decl_sha256']
    source=r/'research/theorem_census_20261109/lean/F4ExpectedOutputs.lean'
    planpath='research/theorem_census_20261109/f4-model-sources.json'
else:
    spec=importlib.util.spec_from_file_location('frozen_pcc',r/'tools/pcc/pcc.py');pcc=importlib.util.module_from_spec(spec);sys.modules[spec.name]=pcc;spec.loader.exec_module(pcc)
    name='MsolveDegreeWordAudit.actual_degree_word_certificate'
    found=pcc.lean_find_declaration(pcc.lean_sources(s/'lean'),name);assert found
    source,text=found;expected=hashlib.sha256(text.encode()).hexdigest();planpath=str((s/'sources.json').relative_to(r))
b=source.read_bytes();sha=hashlib.sha256(b).hexdigest();export=a.out/source.name;export.write_bytes(b)
manifest=dict(schema='statement-binding-wrapper-input/v1',cases=[dict(case_id='legacy-real-prefix'if a.legacy else'degree-word-full-main',source_plan=planpath,theorem=name,short_name=name.split('.')[-1],expected_decl_sha256=expected,recorded_statement=text,candidates=[dict(source_path=str(source.relative_to(r)),export_path=str(export.resolve()),bytes=len(b),sha256=sha,declared_source_sha256=sha)])])
(a.out/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n')
print(json.dumps(dict(manifest=str((a.out/'manifest.json').resolve()),source_sha256=sha,recorded_characters=len(text),legacy=a.legacy)))
