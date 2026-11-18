# Native PARI raw records

The exact 264 GP scripts and their stdout/stderr are packaged in raw.tar.xz. RAW_ARCHIVE.sha256 checks the archive; RAW_MEMBERS.sha256 checks every extracted member.

```sh
sha256sum -c RAW_ARCHIVE.sha256
tar -xJf raw.tar.xz
sha256sum -c RAW_MEMBERS.sha256
```

GNU tar, sorted names, fixed 2026-10-21 00:00:00Z mtime, uid/gid 0 and normalized permissions; xz -6 -T1. The original member contents are unchanged. The ignored raw/ tree is only a local extraction.
