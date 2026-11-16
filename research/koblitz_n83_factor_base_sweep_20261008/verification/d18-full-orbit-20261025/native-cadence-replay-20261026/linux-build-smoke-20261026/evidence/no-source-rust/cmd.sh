exec /bin/bash -lc rustc\ --version\ \>\ \"\$EPHEMERAL_OUT/rustc.txt\"\ \&\&\ aws\ --version\ \>\ \"\$EPHEMERAL_OUT/aws.txt\"\ 2\>\&1
