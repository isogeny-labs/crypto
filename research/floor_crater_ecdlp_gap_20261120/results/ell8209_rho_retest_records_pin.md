# Session records pin

Raw `records.jsonl` is omitted from Git (artifacts store policy: `.jsonl` / `.gz` > 64 KiB).
Audit receipt and table remain committed; hashes below match `audit.json`.

- records.jsonl sha256: `8a5dc38c68c08bc1f2af32878f30d115cc856e778e639d3eb449c3ce528dac51`
- records.jsonl bytes: 3499639
- records.jsonl.gz sha256: `6994c527a4ccb645d42e0422ec7809b0e02eb2590199f86c88855dd744602900`
- verified_records: 768 / 768 (audit OK=True)

Rebuild: gunzip a retained local `records.jsonl.gz` and re-run `ecbench verify --dir` this session.
