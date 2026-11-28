# Native identity checker provenance

The v1 checker core is copied from crypto/src/cryptanalysis/identity_certificate.rs,
through check_with_extra. The existing native SHA256 interface is supplied by
this standalone crate using SHA2. The unrelated Schoenhage worked example and
its tests are omitted; the new identities have their own positive and corrupted
controls. No acceptance rule or modulus is changed.

Original source SHA256: d81ee915ad769a87d7aac5d9664631239607f4a0deb27850cdb52cea84e34270

Original local source revision: 4d0bfe24276aac6ffc4b9c56afcd70429cd8e617.
The issuer/checker is the same v1 core and uses separately seeded extra points;
this is not external-runner independent replay. The exact statement proof
pointers match ThreeLeads.lean, checked separately with Lean4.34.1.
