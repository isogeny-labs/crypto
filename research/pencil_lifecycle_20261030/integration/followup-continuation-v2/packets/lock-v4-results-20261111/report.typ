#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Offline lock gate v4: two generated locks
Native result: generated pending publication. This report retains the single completed attempt.
#image("flow.svg",width:100%)
Both offline resolutions exited zero. The driver lock contains 2952 bytes with SHA256:
#text(size:8pt)[41829631b1c402e6bec2ea0c91327d557d5160bcb71b92bd10d8eba82ecfdde3]
The standalone adapter lock contains 2999 bytes with SHA256:
#text(size:8pt)[0748a4bcf82a638dc393117a9256b828bd1a5b21f32972962abc29184d9ff6a1]
Both actual files are retained separately as small source-bindable artifacts. The native receipt reports no compiler invocation and no network access. This gate did not run the combined qualification panel.
== Timing, custody and charge
Native elapsed time was 0.8692167460103519 seconds; caller elapsed time was 1.1867481969529763 seconds. These are distinct observations. Pre-stop status was success with exit zero, and terminal ownership reports an empty owned cgroup. The full 60-second reservation is charged, bringing campaign cumulative charge to 3720 seconds and remaining allowance to 125880 seconds.
All native and caller files are archived, with file hashes and byte counts. The canonical remote archive is downloaded and hash-verified. Originals are untouched. Publication and exact lock binding remain necessary before any combined qualification attempt; this result supplies the two generated locks only.
