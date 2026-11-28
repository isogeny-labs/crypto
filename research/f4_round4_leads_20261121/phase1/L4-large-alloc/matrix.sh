cd /tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/round4/L4-large-alloc
B=/tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/base/jv_quintic
A="--variant weierstrass --sizes 271 --seeds 1 --residuals 3 --constructed 2 --max-degree 24"
JE=LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2
R=RAYON_NUM_THREADS=1
python3 ru.py g_trim $R MALLOC_TRIM_THRESHOLD_=17179869184 MALLOC_TOP_PAD_=268435456 -- $B $A
python3 ru.py g_mmap $R MALLOC_MMAP_THRESHOLD_=33554432 -- $B $A
python3 ru.py g_all $R MALLOC_MMAP_THRESHOLD_=33554432 MALLOC_TRIM_THRESHOLD_=17179869184 MALLOC_TOP_PAD_=268435456 MALLOC_ARENA_MAX=1 -- $B $A
python3 ru.py g_arena1 $R MALLOC_ARENA_MAX=1 -- $B $A
python3 ru.py je_nodecay $R $JE MALLOC_CONF=retain:true,dirty_decay_ms:-1,muzzy_decay_ms:-1 -- $B $A
python3 ru.py je_thp $R $JE MALLOC_CONF=thp:always,metadata_thp:always -- $B $A
python3 ru.py je_nodecay_ovs $R $JE MALLOC_CONF=retain:true,dirty_decay_ms:-1,muzzy_decay_ms:-1,oversize_threshold:0 -- $B $A
python3 ru.py je_stats $R $JE MALLOC_CONF=stats_print:true -- $B $A
