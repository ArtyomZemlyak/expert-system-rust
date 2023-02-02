#[path = "../src/node.rs"] pub mod node;
#[path = "../src/net.rs"] pub mod net;


use net::{DBESNet, CurHashSet, CurHashMap};
use node::{DBESNode, NodeRelationsSJValue, NodeRelationsDir};

use std::convert::TryInto;
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use serde_json as sj;
use std::time::Instant;

use std::collections::{HashMap, HashSet};
use slab::Slab;
use serde_json::{Value, json};
use std::hash::BuildHasherDefault;
use seahash::SeaHasher;
use fxhash::{FxHashMap, FxHashSet};
use ahash::AHashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};


use my_macro::big_int_match;


/// A builder for default Sea hashers.
pub type SeaBuildHasher = BuildHasherDefault<SeaHasher>;

/// A `HashMap` using a default Sea hasher.
pub type SeaHashMap<K, V> = HashMap<K, V, SeaBuildHasher>;

/// A `HashSet` using a default Sea hasher.
pub type SeaHashSet<V> = HashSet<V, SeaBuildHasher>;



fn test_my_macro(upper_count: usize) {
    for i in 0..upper_count {
        big_int_match!(1000);
    }
}



fn dbes_up() {
    let mut d = DBESNet::new();
}

fn dbesnode_remove_relation(d: &mut DBESNet) {
    let node1 = d._net.get_mut("1e80f2bb1cf7491620f5cc7cfdb6eae1").expect("cant get node from net");
    node1.remove_relation("4ac14d763fa720db9b2754cfcaf5b013", "depends_and", net::node::NodeRelationsDir::In);
    let node_str = serde_json::ser::to_string_pretty(node1).unwrap();
    let path = "./node.json"; //d._get_path("node.json", "./");
    match File::create(path) {
        Ok(mut fc) => {
            fc.write(&node_str.as_bytes()).expect("Error while writing in file!");
        }
        Err(e) => panic!("Problem creating the file: {:?}", e),
    };
}

fn dbesnode_add_relation(d: &mut DBESNet) {
    let node1 = d._net.get_mut("1e80f2bb1cf7491620f5cc7cfdb6eae1").expect("cant get node from net");
    node1.add_relation("4ac14d763fa720db9b2754cfcaf5b013", "depends_and", net::node::Coeff::None, net::node::NodeRelationsDir::In);
    let node_str = serde_json::ser::to_string_pretty(node1).unwrap();
    let path = "./node.json"; //d._get_path("node.json", "./");
    match File::create(path) {
        Ok(mut fc) => {
            fc.write(&node_str.as_bytes()).expect("Error while writing in file!");
        }
        Err(e) => panic!("Problem creating the file: {:?}", e),
    };
}


fn func_match_i32(upper_count: usize) {
    for i in 0..upper_count {
        match i {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => 7,
            8 => 8,
            9 => 9,
            10 => 10,
            11 => 11,
            12 => 12,
            13 => 13,
            14 => 14,
            15 => 15,
            16 => 16,
            17 => 17,
            18..=200 => 18,
            201..=400 => 201,
            _ => 202
        };
    }
}

fn func_match_str(str_w: &str, upper_count: usize) {
    for i in 0..upper_count {
        match str_w {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "11" => 11,
            "12" => 12,
            "13" => 13,
            "14" => 14,
            "15" => 15,
            "16" => 16,
            "17" => 17,
            "200" => 200,
            "300" => 300,
            _ => 202
        };
    }
}

fn func_match_i32_hashset(str_w: &HashSet<usize>) {
    for i in str_w {
        match i {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => 7,
            8 => 8,
            9 => 9,
            10 => 10,
            11 => 11,
            12 => 12,
            13 => 13,
            14 => 14,
            15 => 15,
            16 => 16,
            17 => 17,
            18..=200 => 18,
            201..=400 => 201,
            _ => 202
        };
    }
}


fn func_hashmap(str_w: &HashMap<usize, usize>, upper_count: usize) {
    for i in 0..upper_count {
        str_w.get(&i);
    }
}

fn func_fx_hashmap(str_w: &FxHashMap<usize, usize>, upper_count: usize) {
    for i in 0..upper_count {
        str_w.get(&i);
    }
}

fn func_sea_hashmap(str_w: &SeaHashMap<usize, usize>, upper_count: usize) {
    for i in 0..upper_count {
        str_w.get(&i);
    }
}

fn func_a_hashmap(str_w: &AHashMap<usize, usize>, upper_count: usize) {
    for i in 0..upper_count {
        str_w.get(&i);
    }
}

fn func_slab(str_w: &Slab<usize>, upper_count: usize) {
    for i in 0..upper_count {
        str_w.get(i);
    }
}

fn func_sj_val(str_w: &Value, upper_count: usize) {
    for i in 0..upper_count {
        str_w.get(i);
    }
}

fn func_hashmap_init(upper_count: usize) {
    let mut hashmap: HashMap<usize, usize> = HashMap::with_capacity(upper_count);
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
}

fn func_fx_hashmap_init(upper_count: usize) {
    let mut hashmap: FxHashMap<usize, usize> = FxHashMap::default();
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
}

fn func_a_hashmap_init(upper_count: usize) {
    let mut hashmap: AHashMap<usize, usize> = AHashMap::default();
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
}

fn func_sea_hashmap_init(upper_count: usize) {
    let mut hashmap: SeaHashMap<usize, usize> = SeaHashMap::default();
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
}

fn func_slab_init(upper_count: usize) {
    let mut slab: Slab<usize> = Slab::with_capacity(upper_count);
    for i in 0..upper_count {
        slab.insert(i);
    }
}

fn func_sj_val_init(upper_count: usize) {
    let mut sj_val: Value = json!({});
    for i in 0..upper_count {
        sj_val[i.to_string()] = json!(i);
    }
}

fn func_hashmap_full(upper_count: usize) {
    let mut hashmap: HashMap<usize, usize> = HashMap::with_capacity(upper_count);
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
    for i in 0..upper_count {
        hashmap.get(&i);
    }
    drop(hashmap);
}

fn func_fx_hashmap_full(upper_count: usize) {
    let mut hashmap: FxHashMap<usize, usize> = FxHashMap::default();
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
    for i in 0..upper_count {
        hashmap.get(&i);
    }
    drop(hashmap);
}

fn func_a_hashmap_full(upper_count: usize) {
    let mut hashmap: AHashMap<usize, usize> = AHashMap::default();
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
    for i in 0..upper_count {
        hashmap.get(&i);
    }
    drop(hashmap);
}

fn func_sea_hashmap_full(upper_count: usize) {
    let mut hashmap: SeaHashMap<usize, usize> = SeaHashMap::default();
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
    for i in 0..upper_count {
        hashmap.get(&i);
    }
    drop(hashmap);
}

fn func_slab_full(upper_count: usize) {
    let mut slab: Slab<usize> = Slab::with_capacity(upper_count);
    for i in 0..upper_count {
        slab.insert(i);
    }
    for i in 0..upper_count {
        slab.get(i);
    }
    drop(slab);
}

fn func_sj_val_full(upper_count: usize) {
    let mut sj_val: Value = json!({});
    for i in 0..upper_count {
        sj_val[i.to_string()] = json!(i);
    }
    for i in 0..upper_count {
        sj_val.get(i);
    }
    drop(sj_val);
}

fn func_sj_val_str(sj_val: &Value, str_w: &HashSet<String>) {
    for i in str_w {
        sj_val.get(i);
    }
}

fn func_sj_val_from_str_to_val(string: &str) {
    for i in 0..1000 {
        let sj_val = sj::from_str::<Value>(string);
    }
}

fn func_sj_val_from_str_to_val_to_struct(string: &str) {
    for i in 0..1000 {
        let sj_val = sj::from_str::<Value>(string).unwrap();
        let mynode = sj::from_value::<DBESNode>(sj_val);
    }
}

fn func_sj_val_from_str_to_struct(string: &str) {
    for i in 0..1000 {
        let mynode = sj::from_str::<DBESNode>(string);
    }
}

fn func_sj_val_deep(sj_val: &Value) {
    for i in 0..1000 {
        let a = sj_val["1"]["2"]["3"]["4"].get("5");
    }
}

struct StructDeep0 {a: StructDeep1}
struct StructDeep1 {b: StructDeep2}
struct StructDeep2 {c: StructDeep3}
struct StructDeep3 {d: StructDeep4}
struct StructDeep4 {e: i32}

fn func_struct_deep(struct_deep: &StructDeep0) {
    for i in 0..1000 {
        let a = struct_deep.a.b.c.d.e;
    }
}

fn func_struct_fx_hashmap_deep(hashmap_deep: &FxHashMap<String, FxHashMap<String, FxHashMap<String, FxHashMap<String, FxHashMap<String, i32>>>>>) {
    for i in 0..1000 {
        let a = hashmap_deep["1"]["2"]["3"]["4"].get("5");
    }
}



// ------------------------------------------------------------------------------------------------
// БЗ Иркутска тест функций
// ------------------------------------------------------------------------------------------------
fn dbes_irkutsk_find_val_idxs(d: &DBESNet) {
    // let mut important_nodes: FxHashMap<&str, $str> = FxHashMap::default();
    let mut important_nodes: FxHashMap<&str, String> = FxHashMap::default();
    important_nodes.insert("Словарь", d.find_val_idxs(net::node::NodeValue::Str("Словарь".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("id", d.find_val_idxs(net::node::NodeValue::Str("id".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("eid", d.find_val_idxs(net::node::NodeValue::Str("eid".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Услуга", d.find_val_idxs(net::node::NodeValue::Str("Услуга".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Тег услуги", d.find_val_idxs(net::node::NodeValue::Str("Тег услуги".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Параметр тега", d.find_val_idxs(net::node::NodeValue::Str("Параметр тега".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Параметр услуги", d.find_val_idxs(net::node::NodeValue::Str("Параметр услуги".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("SERVICE",  d.find_val_idxs(net::node::NodeValue::Str("SERVICE".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("CONSULTATION", d.find_val_idxs(net::node::NodeValue::Str("CONSULTATION".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("DEPERSONALIZED_CONSULTATION", d.find_val_idxs(net::node::NodeValue::Str("DEPERSONALIZED_CONSULTATION".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Портал госуслуг", d.find_val_idxs(net::node::NodeValue::Str("Портал госуслуг".to_string()), false, None, None).iter().next().cloned().unwrap());
}



// _______________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________
// _______________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________
// _______________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________
fn criterion_benchmark(c: &mut Criterion) {
    // ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    // DBESNet
    // ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    c.bench_function("DBESNet-init", |b| b.iter(|| dbes_up()));

    let mut d = DBESNet::new();
    c.bench_function("DBESNet-find_all_out_idxs",
                     |b| b.iter(|| d.find_all_out_idxs(black_box("ccadffd66b9ab7d65e135fa1d7a2da48"), black_box(None))));

    c.bench_function("DBESNet-find_type_rel_idxs",
                     |b| b.iter(|| d.find_type_rel_idxs(black_box("prob"),
                                                                            black_box(&net::node::NodeRelationsDir::Out),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None))));

    c.bench_function("DBESNet-find_rel_idxs",
                     |b| b.iter(|| d.find_rel_idxs(black_box("a9234fdfc01fd9ee2d11b54b8c641033"),
                                                                        black_box(&net::node::NodeRelationsDir::In),
                                                                        black_box(None),
                                                                        black_box(None),
                                                                        black_box(None),
                                                                        black_box(None),
                                                                        black_box(None),
                                                                        black_box(None),
                                                                        black_box(None))));

    c.bench_function("DBESNet-find_rel_idxs-Rec",
    |b| b.iter(|| d.find_rel_idxs(black_box("a9234fdfc01fd9ee2d11b54b8c641033"),
                                                        black_box(&net::node::NodeRelationsDir::In),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(Some(true)),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(None))));

    c.bench_function("DBESNet-find_rel_idxs_NORec-Rec",
    |b| b.iter(|| d.find_rel_idxs_NORec(black_box("a9234fdfc01fd9ee2d11b54b8c641033"),
                                                            black_box(&net::node::NodeRelationsDir::In),
                                                            black_box(None),
                                                            black_box(None),
                                                            black_box(None),
                                                            black_box(Some(true)),
                                                            black_box(None))));

    c.bench_function("DBESNet-find_rel_idxs_NORec-noRec",
    |b| b.iter(|| d.find_rel_idxs_NORec(black_box("a9234fdfc01fd9ee2d11b54b8c641033"),
                                                            black_box(&net::node::NodeRelationsDir::In),
                                                            black_box(None),
                                                            black_box(None),
                                                            black_box(None),
                                                            black_box(Some(false)),
                                                            black_box(None))));

    let mut hash_in: CurHashSet<String> = CurHashSet::default();
    let mut hash_out: CurHashSet<String> = CurHashSet::default();
    hash_in.insert(String::from("adc51d28318a7219b48182d6703e0053"));
    hash_out.insert(String::from("b078683f8f26833b94ef3d30c4264f16"));
    let mut hash_rel: CurHashMap<net::node::NodeRelationsDir, CurHashSet<String>> = CurHashMap::default();
    hash_rel.insert(net::node::NodeRelationsDir::In, hash_in);
    hash_rel.insert(net::node::NodeRelationsDir::Out, hash_out);
    c.bench_function("DBESNet-find_mult_rel_idxs",
                     |b| b.iter(|| d.find_mult_rel_idxs(black_box(&hash_rel),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None))));

    c.bench_function("DBESNet-find_val_idxs",
                    |b| b.iter(|| d.find_val_idxs(black_box(net::node::NodeValue::Str(String::from("услуга"))),
                                                                        black_box(true),
                                                                        black_box(None),
                                                                        black_box(None))));





    // ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    // DBESNode
    // ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    c.bench_function("DBESNode_remove_relation", |b| b.iter(|| dbesnode_remove_relation(black_box(&mut d))));
    c.bench_function("DBESNode_add_relation", |b| b.iter(|| dbesnode_add_relation(black_box(&mut d))));





    // ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    // Match, hashmaps, hashsets, hash
    // ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    let upper_count:usize = 1000;
    c.bench_function("func_match_i32", |b| b.iter(|| func_match_i32(black_box(upper_count))));
    c.bench_function("func_match_str", |b| b.iter(|| func_match_str(black_box("10"), black_box(upper_count))));

    let mut hashset_i32: HashSet<usize> = HashSet::with_capacity(upper_count);
    for i in 0..upper_count {
        hashset_i32.insert(i);
    }
    c.bench_function("func_match_i32_hashset", |b| b.iter(|| func_match_i32_hashset(black_box(&hashset_i32))));

    let mut hashmap: HashMap<usize, usize> = HashMap::with_capacity(upper_count);
    for i in 0..upper_count {
        hashmap.insert(i, i);
    }
    c.bench_function("func_hashmap", |b| b.iter(|| func_hashmap(black_box(&hashmap), black_box(upper_count))));

    let mut slab: Slab<usize> = Slab::with_capacity(upper_count);
    for i in 0..upper_count {
        slab.insert(i);
    }
    c.bench_function("func_slab", |b| b.iter(|| func_slab(black_box(&slab), black_box(upper_count))));

    let mut sj_val: Value = json!(hashmap);
    c.bench_function("func_sj_val", |b| b.iter(|| func_sj_val(black_box(&sj_val), black_box(upper_count))));

    c.bench_function("func_hashmap_init", |b| b.iter(|| func_hashmap_init(black_box(upper_count))));
    c.bench_function("func_slab_init", |b| b.iter(|| func_slab_init(black_box(upper_count))));
    c.bench_function("func_sj_val_init", |b| b.iter(|| func_sj_val_init(black_box(upper_count))));
    c.bench_function("func_hashmap_full", |b| b.iter(|| func_hashmap_full(black_box(upper_count))));
    c.bench_function("func_slab_full", |b| b.iter(|| func_slab_full(black_box(upper_count))));
    c.bench_function("func_sj_val_full", |b| b.iter(|| func_sj_val_full(black_box(upper_count))));

    let mut hashset_str: HashSet<String> = HashSet::with_capacity(upper_count);
    for i in 0..upper_count {
        hashset_str.insert(i.to_string());
    }
    let mut sj_val_str: Value = json!({});
    for i in 0..upper_count {
        sj_val_str[i.to_string()] = json!(i);
    }
    c.bench_function("func_sj_val_str", |b| b.iter(|| func_sj_val_str(black_box(&sj_val_str), black_box(&hashset_str))));

    c.bench_function("test_my_macro", |b| b.iter(|| test_my_macro(black_box(1000))));

    let mut fx_map: FxHashMap<usize, usize> = FxHashMap::default();
    for i in 0..upper_count {
        fx_map.insert(i, i);
    }
    c.bench_function("func_fx_hashmap", |b| b.iter(|| func_fx_hashmap(black_box(&fx_map), black_box(upper_count))));
    c.bench_function("func_fx_hashmap_init", |b| b.iter(|| func_fx_hashmap_init(black_box(upper_count))));
    c.bench_function("func_fx_hashmap_full", |b| b.iter(|| func_fx_hashmap_full(black_box(upper_count))));

    let mut sea_map: SeaHashMap<usize, usize> = SeaHashMap::default();
    for i in 0..upper_count {
        sea_map.insert(i, i);
    }
    c.bench_function("func_sea_hashmap", |b| b.iter(|| func_sea_hashmap(black_box(&sea_map), black_box(upper_count))));
    c.bench_function("func_sea_hashmap_init", |b| b.iter(|| func_sea_hashmap_init(black_box(upper_count))));
    c.bench_function("func_sea_hashmap_full", |b| b.iter(|| func_sea_hashmap_full(black_box(upper_count))));

    let mut a_map: AHashMap<usize, usize> = AHashMap::default();
    for i in 0..upper_count {
        a_map.insert(i, i);
    }
    c.bench_function("func_a_hashmap", |b| b.iter(|| func_a_hashmap(black_box(&a_map), black_box(upper_count))));
    c.bench_function("func_a_hashmap_init", |b| b.iter(|| func_a_hashmap_init(black_box(upper_count))));
    c.bench_function("func_a_hashmap_full", |b| b.iter(|| func_a_hashmap_full(black_box(upper_count))));

    let node_str = {
    "
    {
        \"hash_idx\": \"242482701536651308181230180303272102656\",
        \"idx\": \"de64c253f9f18ebddb14ee5ba9ccdc4d\",
        \"relation\": {
            \"in\": {
                \"068d7daf641f5c47b8cb9b8a49a4fd79\": {
                    \"struct\": 1
                },
                \"13bdf15ecf4ffa5c0b08b9f2065b0323\": {
                    \"value\": 1
                },
                \"1b305303b98e70ef60d7442a8015c09b\": {
                    \"struct\": 1
                },
                \"2627813f07f05743e21f899a5094ca71\": {
                    \"depends_and\": 1
                },
                \"27c65864510ad060db720da588e93f19\": {
                    \"prob\": 0.5
                },
                \"286bd7db5bbece3ca188200367814523\": {
                    \"depends_and\": 1
                },
                \"2ddc275c72a878be6b3f079434a75882\": {
                    \"prob\": 0.5
                },
                \"421ec96bf88796db26d3d357216a4d47\": {
                    \"struct\": 1
                },
                \"442abc23e0bdea83de5bc83ea6d56650\": {
                    \"prob\": 1
                },
                \"46cd8d0d1ab9668779130e701432da85\": {
                    \"value\": 1
                },
                \"4dcb01854733e6f9b75fe7cebf994b6e\": {
                    \"struct\": 1
                },
                \"58c09b897f1a936dfb3d56791996b399\": {
                    \"depends_and\": 1
                },
                \"5d46706cb02f75e663d6ecbe608d8f98\": {
                    \"depends_and\": 1
                },
                \"703f08aa3d2c45d9383297e83ef66926\": {
                    \"prob\": 0.9
                },
                \"71035d8ffd84627ecd41741ce9bf3a53\": {
                    \"depends_and\": 1
                },
                \"711cf8c6b4f244fdb0d8ba9e83947b9c\": {
                    \"struct\": 1
                },
                \"75a72f44c238739941a74ddb2381c841\": {
                    \"value\": 1
                },
                \"7689cbf50a0aa2567e40ff82fb8af233\": {
                    \"struct\": 1
                },
                \"7941326b81b63cd678122ed00976f0cb\": {
                    \"struct\": 1
                },
                \"7b2c790a82f610a5b9c44556f3ecfbac\": {
                    \"struct\": 1
                },
                \"7e1fe99ff337ad3ed5909894b8bbbe13\": {
                    \"struct\": 1
                },
                \"885ec41e608dcd47d983a7332af33016\": {
                    \"struct\": 1
                },
                \"8c0033804863f6f953143cfd0e062691\": {
                    \"struct\": 1
                },
                \"90403bd16760aff52001ca55633a36e0\": {
                    \"struct\": 1
                },
                \"90b53b1033ad8d566bddea2200db939e\": {
                    \"prob\": 0.5
                },
                \"a138931a38789644453af670ba4fa010\": {
                    \"struct\": 1
                },
                \"a1f6774950994f9c270bfaf869012a27\": {
                    \"struct\": 1
                },
                \"a50c4ecdefca78b46b1cb88547b9bd84\": {
                    \"struct\": 1
                },
                \"a699b233f6d373633050ffea65cfe193\": {
                    \"depends_and\": 1
                },
                \"a8021f346ed1cb94fc13c83de5435c94\": {
                    \"value\": 1
                },
                \"a9234fdfc01fd9ee2d11b54b8c641033\": {
                    \"struct\": 1
                },
                \"afa147b75638bac1e765fa21bac98a66\": {
                    \"depends_and\": 1
                },
                \"b40d12d5b1c2072397a5f76f7ab0acfc\": {
                    \"struct\": 1
                },
                \"bc2d9ebf0a10cb7b79239c01fa26f1d5\": {
                    \"value\": 1
                },
                \"c252387142bcf63a4d9bd94f2e7b32a3\": {
                    \"depends_and\": 1
                },
                \"c252cb47ce3b95f6cfef0cf658bf5b00\": {
                    \"prob\": 0.9
                },
                \"c33efbfabe1dfd2a8bc21557a6fc8815\": {
                    \"depends_and\": 1
                },
                \"c65a9dc651fc59d7152e88eb42c01bbd\": {
                    \"depends_and\": 1
                },
                \"c743eb8a422a829246e7633647d2aaa8\": {
                    \"value\": 1
                },
                \"ca250b2232fa3db6f3c74a882d914e0d\": {
                    \"struct\": 1
                },
                \"cf421fb4b8ca58c2ec310bde49105f44\": {
                    \"struct\": 1
                },
                \"d57db13dbd46bdadc6c52596adab8855\": {
                    \"struct\": 1
                },
                \"d70009fe2e135ef45fb81d72f6f7299c\": {
                    \"struct\": 1
                },
                \"e1384d8ddfca68a4056f29814a08c744\": {
                    \"prob\": 0.5
                },
                \"e358c81865e08da29e928154c6219074\": {
                    \"depends_and\": 1
                },
                \"e6efa6b30666c70593fed16ef79693ea\": {
                    \"prob\": 0.7
                },
                \"ef127aa03d2c2ab08a59cf00d4fa1fdc\": {
                    \"depends_and\": 1
                },
                \"f0367cd98f5d100222757caa939de70a\": {
                    \"prob\": 0.5
                },
                \"f3052ac1020369117070f8d2e82eb2b9\": {
                    \"depends_and\": 1
                },
                \"f3226382c50f09a53f4fcbfb935c8ca6\": {
                    \"depends_and\": 1
                },
                \"fb7759ce97dfd89d1f594434a62582b8\": {
                    \"struct\": 1
                },
                \"fca875b6299f23e5c3c590872e48845b\": {
                    \"depends_and\": 1
                }
            },
            \"out\": {
                \"1e946a2ddfd1fa31a52c327635fa80cc\": {
                    \"depends_and\": 1
                },
                \"25625a59036357f2bda9e0c87eff695d\": {
                    \"depends_and\": 1
                },
                \"3052068dd00713a3a778b6150268ccf1\": {
                    \"depends_and\": 1
                },
                \"43a624ef314aa4170ecc82257db56270\": {
                    \"depends_and\": 1
                },
                \"4bb052b9123db520b39208a3379b51d6\": {
                    \"depends_and\": 1
                },
                \"77fa919c84ef4dd6d2b273b9ff400341\": {
                    \"depends_and\": 1
                },
                \"89ad638936172f2a45f89a634703d0d6\": {
                    \"depends_and\": 1
                },
                \"d31aaf4ea96a08e27c00b9edd1422096\": {
                    \"depends_and\": 1
                },
                \"e352ec4b943940bb20e629045411b872\": {
                    \"depends_and\": 1
                },
                \"ed8c301d58f12f462a7b92692bd2c25b\": {
                    \"depends_and\": 1
                }
            }
        },
        \"value\": \"Государственная регистрация права собственности на земельный участок или договора аренды земельного участка\"
    }"
    };
    c.bench_function("func_sj_val_from_str_to_val",
    |b| b.iter(|| func_sj_val_from_str_to_val(black_box(node_str))));
    c.bench_function("func_sj_val_from_str_to_val_to_struct",
    |b| b.iter(|| func_sj_val_from_str_to_val_to_struct(black_box(node_str))));
    c.bench_function("func_sj_val_from_str_to_struct",
    |b| b.iter(|| func_sj_val_from_str_to_struct(black_box(node_str))));

    let sj_val = json!({"1": {"2": {"3": {"4": {"5": 1}}}}});
    c.bench_function("func_sj_val_deep",
    |b| b.iter(|| func_sj_val_deep(black_box(&sj_val))));

    let sd4 = StructDeep4 {e: 1};
    let sd3 = StructDeep3 {d: sd4};
    let sd2 = StructDeep2 {c: sd3};
    let sd1 = StructDeep1 {b: sd2};
    let sd0 = StructDeep0 {a: sd1};
    c.bench_function("func_struct_deep",
    |b| b.iter(|| func_struct_deep(black_box(&sd0))));

    let mut hashmap_deep0: FxHashMap<String, FxHashMap<String, FxHashMap<String, FxHashMap<String, FxHashMap<String, i32>>>>> = FxHashMap::default();
    let mut hashmap_deep1: FxHashMap<String, FxHashMap<String, FxHashMap<String, FxHashMap<String, i32>>>> = FxHashMap::default();
    let mut hashmap_deep2: FxHashMap<String, FxHashMap<String, FxHashMap<String, i32>>> = FxHashMap::default();
    let mut hashmap_deep3: FxHashMap<String, FxHashMap<String, i32>> = FxHashMap::default();
    let mut hashmap_deep4: FxHashMap<String, i32> = FxHashMap::default();
    hashmap_deep4.insert(String::from("5"), 1);
    hashmap_deep3.insert(String::from("4"), hashmap_deep4);
    hashmap_deep2.insert(String::from("3"), hashmap_deep3);
    hashmap_deep1.insert(String::from("2"), hashmap_deep2);
    hashmap_deep0.insert(String::from("1"), hashmap_deep1);
    c.bench_function("func_struct_fx_hashmap_deep",
    |b| b.iter(|| func_struct_fx_hashmap_deep(black_box(&hashmap_deep0))));





    // ------------------------------------------------------------------------------------------------
    // БЗ Иркутска тест функций
    // ------------------------------------------------------------------------------------------------
    let mut important_nodes: FxHashMap<&str, String> = FxHashMap::default();
    important_nodes.insert("Словарь", d.find_val_idxs(net::node::NodeValue::Str("Словарь".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("id", d.find_val_idxs(net::node::NodeValue::Str("id".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("eid", d.find_val_idxs(net::node::NodeValue::Str("eid".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Услуга", d.find_val_idxs(net::node::NodeValue::Str("Услуга".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Тег услуги", d.find_val_idxs(net::node::NodeValue::Str("Тег услуги".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Параметр тега", d.find_val_idxs(net::node::NodeValue::Str("Параметр тега".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Параметр услуги", d.find_val_idxs(net::node::NodeValue::Str("Параметр услуги".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("SERVICE",  d.find_val_idxs(net::node::NodeValue::Str("SERVICE".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("CONSULTATION", d.find_val_idxs(net::node::NodeValue::Str("CONSULTATION".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("DEPERSONALIZED_CONSULTATION", d.find_val_idxs(net::node::NodeValue::Str("DEPERSONALIZED_CONSULTATION".to_string()), false, None, None).iter().next().cloned().unwrap());
    important_nodes.insert("Портал госуслуг", d.find_val_idxs(net::node::NodeValue::Str("Портал госуслуг".to_string()), false, None, None).iter().next().cloned().unwrap());

    c.bench_function("dbes_irkutsk_find_val_idxs",
    |b| b.iter(|| dbes_irkutsk_find_val_idxs(black_box(&d))));

    c.bench_function("dbes_irkutsk_find_all_out_idxs",
    |b| b.iter(|| d.find_all_out_idxs(black_box(&important_nodes["SERVICE"]), black_box(None))));

    c.bench_function("dbes_irkutsk_find_type_rel_idxs",
    |b| b.iter(|| d.find_type_rel_idxs(black_box("prob"), black_box(&net::node::NodeRelationsDir::Out),
                                                            black_box(None),
                                                            black_box(None),
                                                            black_box(None),
                                                            black_box(None),
                                                            black_box(None),
                                                            black_box(None))));

    c.bench_function("dbes_irkutsk_find_rel_idxs_1",
    |b| b.iter(|| d.find_rel_idxs(black_box(&important_nodes["Услуга"]),
                                                        black_box(&net::node::NodeRelationsDir::In),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(Some(false)),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(None))));

    c.bench_function("dbes_irkutsk_find_rel_idxs_2",
    |b| b.iter(|| d.find_rel_idxs(black_box(&important_nodes["eid"]),
                                                        black_box(&net::node::NodeRelationsDir::In),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(Some(true)),
                                                        black_box(None),
                                                        black_box(None),
                                                        black_box(None))));

    c.bench_function("dbes_irkutsk_find_rel_idxs_NORec",
    |b| b.iter(|| d.find_rel_idxs_NORec(black_box(&important_nodes["eid"]),
                                                                black_box(&net::node::NodeRelationsDir::In),
                                                                black_box(None),
                                                                black_box(None),
                                                                black_box(None),
                                                                black_box(Some(true)),
                                                                black_box(None))));

    let mut hash_in: CurHashSet<String> = CurHashSet::default();
    let mut hash_out: CurHashSet<String> = CurHashSet::default();
    hash_in.insert(String::from(&important_nodes["id"]));
    hash_out.insert(String::from(&important_nodes["Портал госуслуг"]));
    let mut hash_rel: CurHashMap<net::node::NodeRelationsDir, CurHashSet<String>> = CurHashMap::default();
    hash_rel.insert(net::node::NodeRelationsDir::In, hash_in);
    hash_rel.insert(net::node::NodeRelationsDir::Out, hash_out);
    c.bench_function("dbes_irkutsk_find_mult_rel_idxs",
                        |b| b.iter(|| d.find_mult_rel_idxs(black_box(&hash_rel),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None),
                                                                            black_box(None))));

    c.bench_function("dbes_irkutsk_find_val_idxs_2",
    |b| b.iter(|| d.find_val_idxs(black_box(net::node::NodeValue::Str("а".to_string())), black_box(true), black_box(None), black_box(None))));

    c.bench_function("dbes_irkutsk_find_rel_idxs_NORec_2",
    |b| b.iter(|| d.find_rel_idxs_NORec(black_box(&important_nodes["Услуга"]),
                                                                black_box(&net::node::NodeRelationsDir::In),
                                                                black_box(None),
                                                                black_box(None),
                                                                black_box(None),
                                                                black_box(Some(true)),
                                                                black_box(None))));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

