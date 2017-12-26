
#[cfg(test)]
mod bench {
    use ArtTree;
    use rand;
    use test;

    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use test::Bencher;
    use rand::Rng;

    const N: usize = 100_000;
    const N_STR_20: usize = 40_000;
    const N_STR_100: usize = 7_000;
    const N_STR_1000: usize = 1_000;
    const N_SEARCH: u64 = 100_000;

    fn new_rng() -> rand::XorShiftRng {
        rand::XorShiftRng::new_unseeded()
    }

    macro_rules! bench_num {
        ($name: ident, $ty: ident, $mapty: ident, $n: expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut rng = new_rng();

                b.iter(|| {
                    let mut t = $mapty::new();
                    for _ in 0..$n {
                        test::black_box(t.insert(rng.gen::<$ty>(), rng.gen::<$ty>()));
                    }
                })
            }
        }
    }

    bench_num!(bench_insert_art_u64, u64, ArtTree, N);
    bench_num!(bench_insert_btree_u64, u64, BTreeMap, N);
    bench_num!(bench_insert_hmap_u64, u64, HashMap, N);

    bench_num!(bench_insert_art_u32, u32, ArtTree, N);
    bench_num!(bench_insert_btree_u32, u32, BTreeMap, N);
    bench_num!(bench_insert_hmap_u32, u32, HashMap, N);

    macro_rules! bench_str {
        ($name: ident, $len: expr, $mapty: ident, $n: expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut rng = new_rng();

                b.iter(|| {
                    let mut t = $mapty::new();
                    for i in 0..$n {
                        // use Vec instead of Str to avoid utf-8 overhead
                        let mut v = Vec::with_capacity($len);
                        for ch in rng.gen_ascii_chars().take(20) {
                            v.push(ch as u8);
                        }
                        test::black_box(t.insert(v, i));
                    }
                })
            }
        }
    }

    bench_str!(bench_insert_art_20_rnd_str, 20, ArtTree, N_STR_20);
    bench_str!(bench_insert_btree_20_rnd_str, 20, BTreeMap, N_STR_20);
    bench_str!(bench_insert_hmap_20_rnd_str, 20, HashMap, N_STR_20);

    bench_str!(bench_insert_art_100_rnd_str, 100, ArtTree, N_STR_100);
    bench_str!(bench_insert_btree_100_rnd_str, 100, BTreeMap, N_STR_100);
    bench_str!(bench_insert_hmap_100_rnd_str, 100, HashMap, N_STR_100);

    bench_str!(bench_insert_art_1000_rnd_str, 1000, ArtTree, N_STR_1000);
    bench_str!(bench_insert_btree_1000_rnd_str, 1000, BTreeMap, N_STR_1000);
    bench_str!(bench_insert_hmap_1000_rnd_str, 1000, HashMap, N_STR_1000);

    macro_rules! bench_search_seq {
        ($name: ident, $mapty: ident, $n: expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut t = $mapty::new();

                for i in 0..$n {
                    t.insert(i, i);
                }

                b.iter(|| for i in 0..$n {
                    test::black_box(t.get(&i));
                })
            }
        }
    }

    bench_search_seq!(bench_search_art_seq_u64, ArtTree, N_SEARCH);
    bench_search_seq!(bench_search_btree_seq_u64, BTreeMap, N_SEARCH);
    bench_search_seq!(bench_search_hmap_seq_u64, HashMap, N_SEARCH);

    macro_rules! bench_search_rnd {
        ($name: ident, $mapty: ident, $n: expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                type InsrtType = u64;
                let mut rng = new_rng();
                let mut t = $mapty::new();

                for i in 0..$n {
                    t.insert(rng.gen::<InsrtType>(), i);
                }

                b.iter(|| for i in 0..10 * $n {
                    let k = i % $n;
                    test::black_box(t.get(&k));
                })
            }
        }
    }

    bench_search_rnd!(bench_search_art_rnd_u64, ArtTree, N_SEARCH);
    bench_search_rnd!(bench_search_btree_rnd_u64, BTreeMap, N_SEARCH);
    bench_search_rnd!(bench_search_hmap_rnd_u64, HashMap, N_SEARCH);
}
