# Large request variant correction

The retained maintained-Clippy failure identifies Request::Actual as at least360bytes versus72bytes for Static. Store only its Actual payload in Box<Actual>, retaining Serde derive, tag/field names, bounds and validation flow. No algorithm or campaign input changes.

References checked: https://clippy.rust-lang.org/rust-1.99.0/index.html#large_enum_variant and https://docs.rs/serde/latest/serde/trait.Deserialize.html (Box<T> implementation). Existing retained typed/file-adapter tests and full library checks will validate the corrected source; no tautological new test is added. Original passes/failure remain immutable.
