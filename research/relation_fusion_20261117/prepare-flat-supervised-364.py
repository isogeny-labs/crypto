from pathlib import Path
import json,hashlib,datetime
s=Path('research/relation_fusion_20261117');old=s/'src/supervised_roster.rs'
t=old.read_text();t=t.replace('// Preparatory design: every existing arm is offered both launchers.\n// The frozen packed roster remains unchanged.','// New flat draft: every previous packed/generic arm is now an eligible control.\n// Existing packed/supervised source and evidence remain immutable.')
t=t.replace('const PRIMARY: &str = "packed-dag-487-native";','const PRIMARY: &str = "flat-dag-487-native";').replace('const PLANNED_CALLS: usize = 2520;','const PLANNED_CALLS: usize = 3600;')
t=t.replace('with_launchers(controls())','with_launchers(roster("AB"))')
t=t.replace('"AB" => Ok(with_launchers(roster("AB"))),','''"AB" => {
            let mut arms = supervised_controls();
            for (form, kind) in [("sparse", "flat-constructor-sparse"), ("dag", "flat-constructor-dag")] {
                for (key, kernel) in [("103", "frontier"), ("231", "packed-products"), ("487", "packed-batch")] {
                    arms.extend(with_launchers(vec![Arm {
                        name: format!("flat-{form}-{key}"), kind, kernel, worker: 2,
                    }]));
                }
            }
            Ok(arms)
        },''')
# Independent closure checks retain every original old arm/launcher as an eligible baseline.
t=t.replace('assert_eq!(all.len(), 36);','assert_eq!(all.len(), 48);').replace('            36\n','            48\n').replace('assert_eq!(supervised_controls().len(), 24);','assert_eq!(supervised_controls().len(), 36);').replace('            12\n','            18\n').replace('assert_eq!(aa.len(), 48);','assert_eq!(aa.len(), 72);').replace('assert_eq!(PRIMARY, "packed-dag-487-native");','assert_eq!(PRIMARY, "flat-dag-487-native");')
needle='''        assert_eq!(supervised_controls().len(), 36);'''
extra='''        for previous in with_launchers(roster("AB")) {
            assert!(supervised_controls().contains(&previous), "previous implementation must be eligible");
        }
        assert!(supervised_controls().iter().any(|a| a.name == "packed-dag-487-native"));
        for form in ["sparse", "dag"] {
            for kernel in ["103", "231", "487"] {
                for launcher in ["native", "wrapped"] {
                    assert!(all.iter().any(|a| a.name == format!("flat-{form}-{kernel}-{launcher}")));
                }
            }
        }
'''+needle
assert t.count(needle)==1;t=t.replace(needle,extra);p=s/'src/flat_supervised_roster.rs';assert not p.exists();p.write_text(t)
records={}
for name in ['compare','analyze','fixture']:
 p=s/f'src/bin/supervised-{name}.rs';a=p.read_text();b=a.replace('../supervised_roster.rs','../flat_supervised_roster.rs').replace('supervised-match','flat-supervised-match').replace('supervised-compare','flat-supervised-compare').replace('supervised-analyze','flat-supervised-analyze').replace('supervised-fixture','flat-supervised-fixture')
 if name=='fixture':
  # Preserve mutation-mode schema/indices; only ordinary fixture usage gains flat281 source.
  start=b.index('    if a.len() != 5 ||');prefix=b[:start];tail=b[start:]
  tail=tail.replace('if a.len() != 5 || !["neutral", "hypothetical20"].contains(&a[4].as_str())','if a.len() != 6 || !["neutral", "hypothetical20"].contains(&a[5].as_str())')
  tail=tail.replace('RAW240 PINS200 NEW_OUT neutral|hypothetical20','RAW240 PINS200 FLAT281 NEW_OUT neutral|hypothetical20').replace('let out = Path::new(&a[3]);','let flat_source = Path::new(&a[3]);\n    let out = Path::new(&a[4]);').replace('a[4] == "hypothetical20"','a[5] == "hypothetical20"').replace('"scenario":a[4]','"scenario":a[5]')
  old='''                        let from = source.join(format!("p{p}-n{n}-{base}"));'''
  new='''                        let from = if arm.kind.starts_with("flat-constructor-") {
                            let form = if arm.kind.ends_with("-dag") { "dag" } else { "sparse" };
                            let key = match arm.kernel {
                                "frontier" => "frontier103", "packed-products" => "products231",
                                "packed-batch" => "batch487", _ => return Err("unknown flat kernel".into()),
                            };
                            flat_source.join(format!("p{p}-n{n}-{form}-{key}-{}", if arm.native { "native" } else { "wrapped" }))
                        } else {
                            source.join(format!("p{p}-n{n}-{base}"))
                        };'''
  assert tail.count(old)==1;tail=tail.replace(old,new);b=prefix+tail
 q=s/f'src/bin/flat-supervised-{name}.rs';assert not q.exists();q.write_text(b);records[str(q)]={'parent':str(p),'parent_sha256':hashlib.sha256(a.encode()).hexdigest(),'sha256':hashlib.sha256(b.encode()).hexdigest()}
(s/'FLAT_MATCHED_PREPARATION_364.json').write_text(json.dumps({'schema':'flat-matched-preparation-source/v1','published_parent':'4e6364822b5cc30412f2b58a4d788bdba063d9ed','purpose':'New draft/synthetic correctness only; frozen253sources/evidence untouched. Canonical validation pending; no actual comparative timing admitted.','primary':'flat-dag-487-native','controls':36,'AA_arms':72,'AB_arms':48,'rounds':5,'cases':6,'planned_calls':3600,'all_old_18_arms_now_eligible_controls_each_with_both_launchers':True,'fixed_primes':[2147483647,9007199254740881],'fixed_n':[3,4,6],'fixed_seed':7,'fixed_graph':'LAST-star','math_and_proof_checks':'Old collector/analyzer logic copied unchanged except include/schema/usage names. Fixture adds exact savedflat281source mapping only; all reused proofs reverified.','files':records,'signed_execution_verdict':False,'performance_observations_collected':0,'15x_admitted':False},indent=2)+'\n')
p=Path('docs/session-logs/2026-11-23.md');p.write_text(p.read_text()+f'''\n## {datetime.datetime.now(datetime.timezone.utc).isoformat()} — Flat matched draft364 newsource only\n\n### What was done (commands, files, features)\nSources checked: existingpacked_roster andsupervised_roster/collector/analyzer/fixture253, qualifierflat281directorylayout, source268storagehypothesis. AddedNEWflat_supervised_roster plus3NEWbins; frozenoldfilesunchanged. Flatprimarydag487native,36controls(all18oldarms×2launchers),72AA/48AB,5rounds×6cases=3600draftcalls. Copiedproof/boundary/stat/gate logic unchanged apart frominclude/schema/usage; fixturemapsactualsavedflat281proofs alongsideold240. Noactualcomparisonrun.\n\n### Decisions made (and why)\nEverypreviouspackedimplementation becomes eligiblecontrol, includingoldprimary; selectstrongest min across36. Preserveallgenericlauncherimprovements andallflatvariants,closedoriginalseeds/instances. Syntheticfixturesmayqualifyanalysisonly, thresholdarithmeticcanneveradmit15× beforecanonicalvalidation.\n\n### Open threads (unfinished, pick up next time)\nCompile/unitqualifynewdraft, inspect alloldsourcepinsunchanged, buildnewfrozenonly-synthetictools andrunneutral/hypothetical/corruptionanalysis withfull3600proofreplays each; retainanyfailure. Archive/publishnewdraft/evidence tosamePR4989. Noactualtiming untilcanonicalaccess.\n\n### Errors (what went wrong, how it was handled)\nNonewbuild/testresultyet. Disk~748MiBfree, useexisting smallstudy target notrootmegabuild; preservefrozenartifacts.\n''')
