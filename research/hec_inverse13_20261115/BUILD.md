# Private msolve environment

Source v0.11.0 commitcc044551f8e202d00691990fd3fd0b1226a88ea2 from
https://github.com/algebraic-solving/msolve . Installed0.11and archived0.9.5
binaries were unusable due missingdylibs; no scientific input was run with them.
Sourcebuild is private in ignoreddeps/msolve, not a system install.

From source directory, autogen then configure:

    CC=/usr/bin/clang
    CFLAGS='-O2 -I/opt/homebrew/opt/flint/include -I/opt/homebrew/include -I/opt/homebrew/opt/libomp/include'
    CPPFLAGS='-Xpreprocessor -fopenmp'
    LDFLAGS='-L/opt/homebrew/opt/flint/lib -L/opt/homebrew/lib -L/opt/homebrew/opt/libomp/lib -lomp'

Export those variables when invoking ./configure with privateprefix, then
make-j2 and make-j2check. Current optFLINT3.5matches those headers/libraries;
rootHomebrew include/lib selects3.6with broken optinstallname, so order matters.
Installed LLVM23also fails on missingz3library; AppleClang documented fallback
used. Runtime paths/fingerprints/help and71passing tests retained. Do not replace
libraries by ABI-incompatible symlinks. Originalglobalinstallation unchanged.

Initial autogen cwd mistake, failedLLVM probe and first3.6loader failures are
retained in logs. They preceded scientific freeze/execution. A new sourcebuild
must pass tests and fingerprint before scientific use. This setup has no timing
or cryptanalytic gain claim.
