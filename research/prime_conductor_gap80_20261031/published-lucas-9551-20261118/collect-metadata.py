"""Collect native proof receipts and compare metadata; no research arithmetic."""
import hashlib,json,pathlib,shutil,sys
s=pathlib.Path(__file__).resolve().parent;tmp=pathlib.Path(sys.argv[1]);b=s/'attempt-b'
for ext in ('stdout','stderr','exit'):
    shutil.copyfile(tmp/f'staged-qualification.{ext}',b/f'qualification.{ext}')
a=json.loads((s/'attempt-a/arithmetic.json').read_text())
rows=[json.loads(line) for line in (s/'attempt-a/ring.jsonl').read_text().splitlines()]
assert all(rows[-1][key]==a[key] for key in ('trace','frobenius_conductor','conductor_bits'))
assert json.loads((b/'arithmetic.json').read_text())==a
assert (s/'attempt-a/certificate.exit').read_text().strip()=='124'
qual_exit=int((b/'qualification.exit').read_text())
cert=b/'conductor.cert';replay=b/'replay.stdout'
passed=qual_exit==0
if passed:
    q=json.loads((b/'qualification.stdout').read_text())
    assert q['qualification_passed'] and cert.exists()
    assert int((b/'certificate.exit').read_text())==0 and not (b/'certificate.stderr').read_bytes()
    assert int((b/'replay.exit').read_text())==0 and not (b/'replay.stderr').read_bytes()
    assert all(json.loads(replay.read_text())[key] for key in ('subject_matches','certificate_replay_accept','changed_subject_reject'))
summary={'schema':'fixed-lucas-qualification/v1','fixed_published_index':9551,'field':'GF(2^9551)','conductor_bits':a['conductor_bits'],'decimal_digits':len(a['frobenius_conductor']),'source_endomorphism_conductor':1,'source_geometric_discriminant':-7,'native_oracles_match':True,'attempt_a_certificate_exit':124,'attempt_a_qualification_exit':1,'attempt_b_qualification_exit':qual_exit,'certificate_attempt_exit':int((b/'certificate.exit').read_text()),'certificate_stderr_empty':not (b/'certificate.stderr').read_bytes(),'certificate_generated':passed,'certificate_replayed':passed,'changed_subject_reject':passed,'certificate_bytes':cert.stat().st_size if cert.exists() else None,'certificate_sha256':hashlib.sha256(cert.read_bytes()).hexdigest() if cert.exists() else None,'prime_status':'PARI complete ECPP certificate accepted and replayed' if passed else 'Complete certificate qualification not established within the frozen attempts','partial_certificates':[{'path':p.name,'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'status':'partial only; not an independent primality proof'} for p in sorted(b.glob('partial-*.cert'))],'certificate_verifier':'PARI/GP 2.17.2 primecertisvalid, separate process on same development host' if passed else None,'independent_signed_execution':False,'lower_endpoint':None,'kernel':None,'isogeny':None,'point_images':None,'endpoint_timings':None,'benchmark_session_executed':False,'world_record_claim':None,'original_80_bit_crossing_complete':False}
(s/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(s/'conductor.txt').write_text(a['frobenius_conductor']+'\n')
print(json.dumps({key:summary[key] for key in ('conductor_bits','decimal_digits','certificate_generated','certificate_replayed','attempt_b_qualification_exit','certificate_bytes')}))
