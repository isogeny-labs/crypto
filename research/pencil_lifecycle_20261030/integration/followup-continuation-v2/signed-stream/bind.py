from pathlib import Path
import hashlib
root=Path("research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-stream")
files=[root/n for n in ["Cargo.toml","Cargo.lock","native-manifest/Cargo.toml","native-manifest/Cargo.lock","src/lib.rs","tests/native.rs"]]
files += [Path("src/cryptanalysis")/n for n in ["three_leads.rs","three_leads_followup.rs","three_leads_geometry_events.rs","three_leads_rho_events.rs"]]
files += [Path("research/five_leads_e2e_20261019/native")/n for n in ["Cargo.toml","lib.rs"]]
files += [Path("research/pencil_lifecycle_20261030/integration/five-followups/rank-information")/n for n in ["Cargo.toml","src/lib.rs"]]
files += [Path("research/pencil_large_prime_20261029/Cargo.toml")]
files += sorted(Path("research/pencil_large_prime_20261029/src").rglob("*.rs"))
for f in sorted(set(files)):print(hashlib.sha256(f.read_bytes()).hexdigest()+"  "+str(f))
