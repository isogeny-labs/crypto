#set page(paper:"a4",margin:17mm)
#set text(size:9.4pt)
#text(size:16pt,weight:"bold")[Private musl compiler check]
#image("qualification.svg",width:100%)

#text("Built and installed the signed official musl 1.2.6 release under /var/tmp/pencil-musl-private-20261031/prefix using existing GCC 11.5.0. Configure/make/install completed with exit 0 under the frozen 600-second bound and two-job limit. No system packages or directories were changed.")

#text("CC path: /var/tmp/pencil-musl-private-20261031/prefix/bin/musl-gcc. This shell wrapper invokes /usr/bin/gcc with the private musl-gcc.specs. Pass it as CC_x86_64_unknown_linux_musl for a separately admitted Rust native-dependency build; no ecbench build was launched here.")

#text("A fixed static hello program compiled successfully, is an x86-64 ELF with no INTERP segment or dynamic NEEDED entry, and exited 0 with exact output pencil-musl-smoke-ok. The retained linker map names the private libc.a and CRT objects. Source, executable, specs and installed-file hashes are in status.json and installed-manifest.json. All commands, stderr, exit codes and timestamps are retained.")

#text("Source archive SHA256: d585fd3b613c66151fc3249e8ed44f77020cb5e6c1e635a616d3f9f82460512a. The detached signature verifies against the project primary key 836489290BB6B70F99FFDA0556BCDB593020450F, whose fingerprint was checked against the official releases page. Signature/key/archive receipts are retained. This establishes signature consistency with the key published over official HTTPS, not independent offline authentication of the maintainer identity; computing the archive hash alone would not establish authenticity.")

#text("Retained setup failures: initial obsolete documentation URLs returned 404 or web-tool errors; official root/releases/manual retrieval succeeded. GPG imported the public key but returned an error because gpg-agent is absent; public signature verification succeeded using --no-autostart. The tar safety gate initially rejected the safe top-level directory without a trailing slash; it was explicitly permitted, retaining the rejection. No unsafe path or symlink was accepted. GCC emitted a getcwd return-local-address warning; it remains in build.stderr. These were not silently discarded.")

#text("The official source site reports advisories affecting release 1.2.6 regcomp and iconv, plus qsort on 32-bit systems. This unmodified pinned-release build establishes the private compiler and static-hello interface; it does not qualify a patched production runtime. The fixed smoke supplies no untrusted regex or iconv input.")

#text("Final Rust linkage is a separate binding. Cargo/rustc may use the installed Rust target's self-contained CRT and libc rather than the private compiler's libc. The hello result therefore does not identify ecbench's eventual linked libc. Capture the actual final rustc/linker invocation, search paths/link map and hashes of the selected libc.a/CRT objects (including Rust sysroot self-contained objects if chosen), plus the final executable, before stating that identity. Do not infer it from CC or this musl release number.")

#text("Primary sources: official release history and key https://musl.libc.org/releases.html and https://musl.libc.org/musl.pub; maintainer announcement https://www.openwall.com/lists/musl/2026/03/20/1; build instructions from authenticated release INSTALL and official manual https://musl.libc.org/doc/1.1.24/manual.md. The older manual is explicitly marked partly outdated; release INSTALL was read before execution.\n")
