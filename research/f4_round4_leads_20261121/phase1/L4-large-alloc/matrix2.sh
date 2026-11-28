cd /tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/round4/L4-large-alloc
B=/tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/base/jv_quintic
A="--variant weierstrass --sizes 271 --seeds 1 --residuals 3 --constructed 2 --max-degree 24"
JE=LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2
R=RAYON_NUM_THREADS=1
python3 ru.py je_n1 $R $JE MALLOC_CONF=narenas:1 -- $B $A
python3 ru.py je_n1_nodecay $R $JE MALLOC_CONF=narenas:1,dirty_decay_ms:-1,muzzy_decay_ms:-1 -- $B $A
python3 ru.py je_notcache_nodecay $R $JE MALLOC_CONF=tcache:false,dirty_decay_ms:-1,muzzy_decay_ms:-1 -- $B $A
python3 ru.py je_nodecay_stats $R $JE MALLOC_CONF=dirty_decay_ms:-1,muzzy_decay_ms:-1,stats_print:true -- $B $A
python3 ru.py g_t4 RAYON_NUM_THREADS=4 -- $B $A
python3 ru.py g_t4_trim RAYON_NUM_THREADS=4 MALLOC_TRIM_THRESHOLD_=17179869184 MALLOC_TOP_PAD_=268435456 -- $B $A
python3 ru.py je_t4 RAYON_NUM_THREADS=4 $JE -- $B $A
python3 ru.py je_t4_thp RAYON_NUM_THREADS=4 $JE MALLOC_CONF=thp:always,metadata_thp:always -- $B $A
