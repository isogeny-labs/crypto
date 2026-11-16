#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Exporter qualification v3: cache modes
Source successor · 2026-11-11

The materializer now binds and preserves exact modes for all copied cache files. The shared inventory covers 2,165 regular files: 2,082 mode0600, one mode0644,78 mode0700 and four mode0755. Thus82files carry executable bits. Special permission bits are excluded.

#image("flow.svg",width:100%)

Sources are checked against frozen bytes and modes. Destination descriptors receive the exact recorded mode before fsync. Source/destination descriptor and path checks surround each copy; all file modes are reconciled again at the completed staging boundary. The durable worker checks the inventory digest and2165/82receipt counts before compilation.

The source archive,4,500-file inventory,106overlays and eleven controls are unchanged. The strict lstat monitor is retained. New workspace/unit/output identities keep old attempts intact. Limits remain2CPU,16GiB and1,800build plus600execution seconds, with November11 noonUTC deadline. Ledger and release bindings are unset.

No native copy, compiler, resolver or test ran during this task. Independent source review and publication precede execution.
