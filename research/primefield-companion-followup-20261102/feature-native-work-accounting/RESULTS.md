# Software qualification results

PCC-20261107-1784eb is unsigned producer evidence; independent signed verdict pending. [pcc:unverified]

validation01 failed to compile the relocated polynomial module because gcd.rs was not found. The failed generator, generation manifest and generated polynomial bytes are saved inside that attempt. No native test ran there.

validation02 passes the self-test and inherited regression; 99 source transformations reverse byte-for-byte, 113 compiler dependencies are hashed, and inherited stdout matches the saved planned-width stdout exactly. [pcc:unverified]

```
interval_meter_self_test=pass direct_lengths=5 naive_poly_products=6 eea_calls=1 fermat_calls=1 fermat_products=13 blocked_aa=pass allocator_trait=pass nesting_error_unwind_overflow=pass
```

The source-bound receipt records compiler/build flags, executable hash, command limits and raw output hashes. Compiler warnings are preserved. No width-policy cost vectors or timing data were collected. The historical full Cargo missing-fixture failure remains unresolved; this standalone validation does not replace it.
