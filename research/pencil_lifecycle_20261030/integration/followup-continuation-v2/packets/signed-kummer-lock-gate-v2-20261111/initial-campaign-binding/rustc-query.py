#!/usr/bin/python3
"""Allow the sole Cargo compiler-version metadata query; reject compilation."""
import sys,os
if sys.argv[1:] != ['-vV']:
 sys.stderr.write('lock gate permits rustc -vV only; rejected argv\n');sys.exit(64)
os.execv('/home/ec2-user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc',['rustc','-vV'])
