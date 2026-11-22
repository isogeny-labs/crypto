from pathlib import Path
import difflib
base=Path('/home/ec2-user/ml-dataset-resume-20261029/cli46-infrastructure-v3-draft/driver.v3.py')
out=Path(__file__).parent
s=base.read_text()
old='''        runner.command("experiment-export", [*state["validator_argv_prefix"], state["checker"], "from-rows", fixture/"functional-rows.jsonl",
            "--protocol",protocol,"--out",fixture/"experiment.export.json","--export-id","koblitz-m3-cli-functional-smoke"],120)
        bind_success_case_metadata(fixture)'''
new='''        runner.command("experiment-export", [*state["validator_argv_prefix"], Path(__file__).resolve(),
            "package-success-export", "--fixture", fixture],120)'''
assert old in s;s=s.replace(old,new)
s=s.replace('"no_dataset_commands":True})\n    return export','**dataset_command_inventory(fixture,state)})\n    return export')
s=s.replace('def bind_success_case_metadata(fixture):','def bind_success_case_metadata(fixture, value):')
s=s.replace('    output=fixture/"experiment.export.json";value=load(output)\n','')
old='''    # Preserve the canonical converter output before attaching phase evidence.
    raw=fixture/"experiment.export.unbound.json"
    with raw.open("xb") as target:
        target.write(output.read_bytes());target.flush();os.fsync(target.fileno())
    value["files"].append(diagnostic_file(fixture,raw,"raw"))
    staged=fixture/"experiment.export.bound.tmp.json";save(staged,value)
    require(sha(output)==sha(raw),"canonical export changed during evidence binding")
    os.replace(staged,output)'''
new='''    value["files"].append(diagnostic_file(fixture,fixture/"experiment.export.unbound.json","raw"))
    return value'''
assert old in s;s=s.replace(old,new)
pos=s.index('def bind_success_case_metadata(')
helper='''def dataset_command_inventory(fixture,state):
    # Count actual recorded argv, including unsuccessful or unfinished commands.
    # Missing/malformed argv cannot establish absence.
    dataset=[];unknown=[]
    for directory in sorted((fixture/"commands").iterdir()):
        if not directory.is_dir():
            unknown.append(directory.name);continue
        try:
            argv=load(directory/"argv.json")
            require(isinstance(argv,list) and argv and all(isinstance(x,str) for x in argv),"malformed argv")
        except Exception:
            unknown.append(directory.name);continue
        if len(argv)>=2 and argv[1]=="dataset":
            dataset.append(directory.name)
    return {"no_dataset_commands":not dataset if not unknown else None,
        "dataset_command_count":len(dataset),"dataset_command_directories":dataset,
        "command_inventory_complete":not unknown,"unknown_command_directories":unknown}


def pinned_export_checker(state):
    import importlib.util,importlib.metadata
    require(importlib.metadata.version("jsonschema")=="4.26.0","wrong validator dependency")
    checker=Path(state["checker"]);source=load(checker.parent.parent/"SOURCE.json")
    require(source["commit"]=="37e193bf8b3e673e0942709794f44d28e6ab66fb" and sha(checker)==state["checker_sha256"],"checker pin changed")
    for rel,digest in source["sha256"].items():require(sha(checker.parent.parent/rel)==digest,"checker source/schema changed")
    spec=importlib.util.spec_from_file_location("pinned_success_export_checker",checker)
    tool=importlib.util.module_from_spec(spec);spec.loader.exec_module(tool)
    return tool


def bind_export_instruments(value,protocol,state):
    require(len(value["arms"])==len(protocol["arms"]),"export arm count changed")
    pinned={a["id"]:a for a in protocol["arms"]}
    require(len(pinned)==len(protocol["arms"]) and {a["id"] for a in value["arms"]}==set(pinned),"export arm identities changed")
    for arm in value["arms"]:
        frozen=pinned[arm["id"]]
        require(arm["role"]==frozen["role"],"export arm role changed")
        for field,state_field in [("commit","source_commit"),("binary_sha256","binary_sha256")]:
            expected=frozen["instrument"][field]
            require(expected and expected==state[state_field],"frozen instrument differs from prepared execution")
            require(arm.get(field) in (None,expected),"converter instrument conflict")
            arm[field]=expected


def package_success_export(fixture):
    require(not (fixture/"execution-failed.json").exists(),"failed attempt cannot be packaged as successful")
    state=load(fixture/"prepared.json");protocol=load(fixture/"protocol.json")
    require(sha(fixture/"protocol.json")==state["protocol_file_sha256"],"protocol changed before packaging")
    require(sha(Path(__file__))==state["driver_sha256"],"packaging driver changed")
    require(not (fixture/"experiment.export.json").exists(),"success export already exists")
    tool=pinned_export_checker(state)
    # The pinned CLI accepts no instrument/script metadata and validates its
    # null skeleton immediately. Reuse its unchanged conversion function, bind
    # authenticated metadata in memory, then validate once before final write.
    value=tool.from_rows(fixture/"functional-rows.jsonl",protocol,"protocol.json",
        "koblitz-m3-cli-functional-smoke",{})
    save(fixture/"experiment.export.unbound.json",value)
    bind_export_instruments(value,protocol,state)
    value=bind_success_case_metadata(fixture,value)
    require(value["analysis"]["script"]["sha256"]==protocol["analysis_plan"]["script"]["sha256"],"frozen analysis script mismatch")
    findings=tool.schema_errors(protocol,"experiment.protocol/v1")+tool.schema_errors(value,"experiment.export/v1")
    if not findings:findings=tool.lint_export(value,protocol,fixture,True)
    require(not findings,"packaged export validation failed: "+repr(findings))
    save(fixture/"experiment.export.json",value)
    return {"accepted":True,"findings":[],"export_sha256":sha(fixture/"experiment.export.json")}


'''
s=s[:pos]+helper+s[pos:]
s=s.replace('    a = sub.add_parser("check-failure-export")','    a = sub.add_parser("package-success-export")\n    a.add_argument("--fixture", required=True)\n    a = sub.add_parser("check-failure-export")')
s=s.replace('    elif args.mode == "check-failure-export":','    elif args.mode == "package-success-export":\n        print(json.dumps(package_success_export(Path(args.fixture)),indent=2))\n    elif args.mode == "check-failure-export":')
(out/'driver.proposed.py').write_text(s)
(out/'packaging.patch').write_text(''.join(difflib.unified_diff(base.read_text().splitlines(True),s.splitlines(True),fromfile='a/driver.v3.py',tofile='b/driver.v3.py')))
