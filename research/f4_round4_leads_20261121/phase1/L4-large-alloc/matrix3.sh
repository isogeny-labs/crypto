cd /tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/round4/L4-large-alloc
B=/tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/base/jv_quintic
A="--variant weierstrass --sizes 271 --seeds 1 --residuals 3 --constructed 2 --max-degree 24"
JE=LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2
R=RAYON_NUM_THREADS=1
python3 ru.py je_decay60s $R $JE MALLOC_CONF=dirty_decay_ms:60000,muzzy_decay_ms:60000 -- $B $A
python3 ru.py je_fit10_nodecay $R $JE MALLOC_CONF=lg_extent_max_active_fit:10,dirty_decay_ms:-1,muzzy_decay_ms:-1 -- $B $A
python3 ru.py je_hpa $R $JE MALLOC_CONF=hpa:true -- $B $A
C="--exp ccov --sizes 53 --seeds 1 --residuals 100 --constructed 4 --oracle-base 0"
python3 ru.py cov_new3 $R -- /tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/new3/jv_cover $C
python3 ru.py cov_new3_nodecay $R _RJEM_MALLOC_CONF=dirty_decay_ms:-1,muzzy_decay_ms:-1 -- /tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/new3/jv_cover $C
python3 ru.py cov_base_glibc $R -- /tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/base/jv_cover $C
python3 ru.py cov_base_glibc_trim $R MALLOC_ARENA_MAX=1 -- /tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/base/jv_cover $C
