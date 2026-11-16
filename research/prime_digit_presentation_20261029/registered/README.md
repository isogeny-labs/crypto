# registered/: the pre-run sources, recovered and immutable

These three files are byte-for-byte what was hashed at registration or used for the counted runs. They are never edited.

| file | what it is | sha256 | where that hash is recorded |
|:--|:--|:--|:--|
| `digit_fall.rs.v0` | the engine source at registration | `d811d6f8…eb2` | `../REGISTRATION.sha256` (`digit_fall.rs`) |
| `run.sh.v0` | the launcher at registration (produced panel A) | `8726b4eb…6b4` | `../REGISTRATION.sha256` (`run.sh`) |
| `run.sh.v1` | the amended launcher (produced panels B-E) | `ab2d4ebc…3a` | `../REGISTRATION-AMENDMENT-1.sha256` (`run.sh`, recorded after the run) |

`sha256sum -c SHA256SUMS` (or `shasum -a 256 -c`) in this directory verifies all three. The amended engine source is `../digit_fall.rs` (hash in `../REGISTRATION-AMENDMENT-1.sha256`).

How the originals were recovered: `ADDENDUM-2.md` in the study directory.
