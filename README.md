# ArtTree
Implementation of Adaptive Radix Tree in Rust

Implemented data structue from this paper: http://15721.courses.cs.cmu.edu/spring2016/papers/leis-icde2013.pdf. Structure can efficiently handle both large and small keys (and values).

Implemented several benchmarks (you can replay it with `cargo bench`):
```
test bench::bench::bench_insert_art_len_1000_rnd_string   ... bench:  13,311,152 ns/iter (+/- 1,114,654)
test bench::bench::bench_insert_art_len_100_rnd_string    ... bench:  11,815,418 ns/iter (+/- 733,268)
test bench::bench::bench_insert_art_len_20_rnd_string     ... bench:  22,631,417 ns/iter (+/- 724,474)
test bench::bench::bench_insert_art_u32                   ... bench:  17,645,349 ns/iter (+/- 1,502,178)
test bench::bench::bench_insert_art_u64                   ... bench:  19,319,125 ns/iter (+/- 2,012,933)
test bench::bench::bench_insert_btree_len_1000_rnd_string ... bench:  13,250,122 ns/iter (+/- 1,170,126)
test bench::bench::bench_insert_btree_len_100_rnd_string  ... bench:  13,019,667 ns/iter (+/- 1,726,351)
test bench::bench::bench_insert_btree_len_20_rnd_string   ... bench:  28,261,226 ns/iter (+/- 3,604,350)
test bench::bench::bench_insert_btree_u32                 ... bench:  21,007,001 ns/iter (+/- 8,948,690)
test bench::bench::bench_insert_btree_u64                 ... bench:  18,143,172 ns/iter (+/- 3,885,773)
test bench::bench::bench_insert_hmap_len_1000_rnd_string  ... bench:  13,390,068 ns/iter (+/- 1,130,891)
test bench::bench::bench_insert_hmap_len_100_rnd_string   ... bench:  12,033,142 ns/iter (+/- 1,906,299)
test bench::bench::bench_insert_hmap_len_20_rnd_string    ... bench:  21,099,683 ns/iter (+/- 2,102,632)
test bench::bench::bench_insert_hmap_u32                  ... bench:  10,004,462 ns/iter (+/- 1,104,645)
test bench::bench::bench_insert_hmap_u64                  ... bench:  11,384,752 ns/iter (+/- 1,595,347)
test bench::bench::bench_search_art_rnd_u64               ... bench:  59,151,228 ns/iter (+/- 6,332,539)
test bench::bench::bench_search_art_seq_u64               ... bench:   4,276,859 ns/iter (+/- 1,995,596)
test bench::bench::bench_search_btree_rnd_u64             ... bench:  15,406,040 ns/iter (+/- 1,254,874)
test bench::bench::bench_search_btree_seq_u64             ... bench:  54,306,120 ns/iter (+/- 2,914,241)
test bench::bench::bench_search_hmap_rnd_u64              ... bench:  43,172,266 ns/iter (+/- 4,653,746)
test bench::bench::bench_search_hmap_seq_u64              ... bench:  52,610,392 ns/iter (+/- 21,480,607)
```

TODO:
1) Use SIMD where possible (`ArtNode16`)
2) Implement iterators
3) Match `BTreeMap` API as close as possible
4) Optimize
5) Open to debate, but `ArtKey` trait might be implemented as `impl<T: Serde> ArtKey for T`
6) Implement `ArtKey` for most common types
