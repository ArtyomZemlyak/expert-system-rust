
pub mod net;
pub mod node;

use net::{DBESNet, CurHashSet, CurHashMap};
use node::{DBESNode, NodeRelationsSJValue, NodeRelationsDir};

use tantivy::schema::{Schema, TextOptions, TextFieldIndexing, IndexRecordOption};
use tantivy::tokenizer::{SimpleTokenizer, Tokenizer, TextAnalyzer, RemoveLongFilter, LowerCaser, Language, Stemmer};

use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use serde_json as sj;
use std::time::Instant;
use std::collections::{HashMap, HashSet};


fn main() {
    let st = Instant::now();
    let mut d = DBESNet::new();
    println!("Time up: {:?}", Instant::now() - st);

    // DBESNode.remove_relation
        // let st = Instant::now();
        // let node1 = d._net.get_mut("1e80f2bb1cf7491620f5cc7cfdb6eae1").expect("cant get node from net");
        // node1.remove_relation("4ac14d763fa720db9b2754cfcaf5b013", "depends_and", net::node::NodeRelationsDir::In);
        // let node_str = serde_json::ser::to_string_pretty(node1).unwrap();
        // let path = "./node.json"; //d._get_path("node.json", "./");
        // match File::create(path) {
        //     Ok(mut fc) => {
        //         fc.write(&node_str.as_bytes()).expect("Error while writing in file!");
        //     }
        //     Err(e) => panic!("Problem creating the file: {:?}", e),
        // };
        // println!("{:?}", Instant::now() - st);

    // DBESNode.add_relation
        // let st = Instant::now();
        // let node1 = d._net.get_mut("1e80f2bb1cf7491620f5cc7cfdb6eae1").expect("cant get node from net");
        // node1.add_relation("4ac14d763fa720db9b2754cfcaf5b013", "depends_and", net::node::Coeff::None, net::node::NodeRelationsDir::In);
        // let node_str = serde_json::ser::to_string_pretty(node1).unwrap();
        // let path = "./node.json"; //d._get_path("node.json", "./");
        // match File::create(path) {
        //     Ok(mut fc) => {
        //         fc.write(&node_str.as_bytes()).expect("Error while writing in file!");
        //     }
        //     Err(e) => panic!("Problem creating the file: {:?}", e),
        // };
        // println!("{:?}", Instant::now() - st);

    // DBESNet.find_all_out_idxs
        // let st = Instant::now();
        // let fi = d.find_all_out_idxs("ccadffd66b9ab7d65e135fa1d7a2da48");
        // println!("{:?}", Instant::now() - st);
        // println!("{:#?}", fi);

    // DBESNet.find_type_rel_idxs
        // let st = Instant::now();
        // let fi = d.find_type_rel_idxs("prob", &net::node::NodeRelationsDir::Out, None, None, None, None, None, None);
        // println!("{:?}", Instant::now() - st);
        // println!("{:#?}", fi);

    // DBESNet.find_rel_idxs
        // let st = Instant::now();
        // let fi = d.find_rel_idxs("a9234fdfc01fd9ee2d11b54b8c641033", &net::node::NodeRelationsDir::In, None, None, None, None, None, None, None);
        // println!("{:?}", Instant::now() - st);
        // println!("{:#?}", fi);

    // DBESNet.find_rel_idxs    Recursive
        // let st = Instant::now();
        // let fi = d.find_rel_idxs("a9234fdfc01fd9ee2d11b54b8c641033", &net::node::NodeRelationsDir::In, None, None, None, Some(true), None, None, None);
        // println!("{:?}", Instant::now() - st);
        // println!("{:#?}", fi);

    // DBESNet.find_rel_idxs_NORec  No Recursive - x10 faster
        // let st = Instant::now();
        // let fi = d.find_rel_idxs_NORec("a9234fdfc01fd9ee2d11b54b8c641033", &net::node::NodeRelationsDir::In, None, None, None, Some(true), None);
        // println!("{:?}", Instant::now() - st);
        // println!("{:#?}", fi);

    // DBESNet.find_mult_rel_idxs
        // let st = Instant::now();
        // let mut hash_in: CurHashSet<String> = CurHashSet::default();
        // let mut hash_out: CurHashSet<String> = CurHashSet::default();
        // hash_in.insert(String::from("e85a05901a63423764df36ba70eb2e97"));
        // hash_out.insert(String::from("74bccf98c7b7151ca9ce37cb9102d175"));
        // let mut hash_rel: CurHashMap<net::node::NodeRelationsDir, CurHashSet<String>> = CurHashMap::default();
        // hash_rel.insert(net::node::NodeRelationsDir::In, hash_in);
        // hash_rel.insert(net::node::NodeRelationsDir::Out, hash_out);
        // let fi = d.find_mult_rel_idxs(&hash_rel, None, None, None, None, None, None);
        // println!("{:?}", Instant::now() - st);
        // println!("{:#?}", fi);

    // DBESNet.find_val_idxs
        // let st = Instant::now();
        // let fi = d.find_val_idxs(net::node::NodeValue::Str(String::from("1")), true, None, None);
        // println!("{:?}", Instant::now() - st);
        // println!("{:#?}", fi);

    // DBESNet.find_rel_idxs_NORec  generic base
        // let st = Instant::now();
        // let fi = d.find_rel_idxs_NORec("0", &net::node::NodeRelationsDir::In, None, None, None, Some(true), None);
        // println!("{:?}", Instant::now() - st);


    // let mut schema_builder = Schema::builder();
    // let text_options = TextOptions::default()
    //     .set_indexing_options(
    //         TextFieldIndexing::default()
    //             .set_tokenizer("default")
    //             .set_index_option(IndexRecordOption::Basic)
    //     )
    //     .set_stored();
    // schema_builder.add_text_field("text", text_options);
    // let schema = schema_builder.build();
    // schema();

    let tokenizer = TextAnalyzer::from(SimpleTokenizer)
        // .filter(RemoveLongFilter::limit(40))
        // .filter(LowerCaser)
        .filter(Stemmer::new(Language::English));
        // .filter(Stemmer::new(Language::Russian));
    let node_str = {
        "
        {
            Начало мудрости – страх Господень; глупцы только презирают мудрость и наставление.
            ...
        Мы всё только говорим и читаем о любви, но сами мало любим.
    "};
    let st = Instant::now();
    let mut token_stream = tokenizer.token_stream(node_str);
    while let Some(token) = token_stream.next() {
        let a = 1; //println!("Token {:?}", token.text);
    }
    println!("Time tokenize: {:?}", Instant::now() - st);
}



// Time MosesTokenizer tokenize:  0.00048232078552246094
// Time NLTK tokenize:  0.009296417236328125
// Time Rust tantivy tokenize (eng) : 0.0000005 - 0.000458

