use serde_json::{Result as SJResult, Value, json};
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::env;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use fxhash::{FxHashMap, FxHashSet};

#[path = "node.rs"] pub mod node;
use node::{DBESNode, DBESTemplate, NodeRelationsDir};


// type CurHashMap<K, V> = HashMap<K, V>;
pub type CurHashMap<K, V> = FxHashMap<K, V>;
pub type CurHashSet<K> = FxHashSet<K>;


fn open_json(path: &str, create_if_not_exist: bool) -> SJResult<Value> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => if create_if_not_exist {
                match File::create(path) {
                    Ok(mut fc) => {
                        fc.write("{}".as_bytes()).expect("Error while writing in file!");
                        match File::open(path) {
                            Ok(fr) => fr,
                            Err(e) => panic!("Problem while reading file!: {:?}", e)
                        }
                    }
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            } else {panic!("File not exist!")}
            _ => panic!("Non expected error while reading file!: {:?}", e)
        },
    };

    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("error with read to string");

    serde_json::from_str(&data)
}


enum DBESNetHashMaps {
    Net,
    Cmn,
    Tmp
}


pub(crate) struct DBESNet {
    pub CONFIG: Value,
    pub _net: CurHashMap<String, DBESNode>,
    pub _common: CurHashMap<String, Value>,
    pub _templates: CurHashMap<String, DBESTemplate>,
}


impl DBESNet {

    pub fn new() -> Self{
        let mut current_path = match env::current_dir() {
            Ok(p) => p,
            Err(e) => panic!("{}", e)
        };
        current_path.push("config.json");
        let current_path = match current_path.to_str() {
            Some(p) => p,
            _ => panic!("failed convert path to str")
        };
        let mut dbes = DBESNet {
            CONFIG: open_json(current_path, false).expect("JSON was not well-formatted"),
            _net: CurHashMap::default(),
            _common: CurHashMap::default(),
            _templates: CurHashMap::default()
        };
        dbes._init();
        dbes
    }

    fn _init(&mut self) {
        let net_path = self._get_path("net", "");
        let common_path = self._get_path("common", "");
        let templates_path = self._get_path("templates", "");

        for (path, target) in [(&net_path, DBESNetHashMaps::Net),
                                                     (&common_path, DBESNetHashMaps::Cmn),
                                                     (&templates_path, DBESNetHashMaps::Tmp)] {
            fs::create_dir_all(path).expect("error while create dirs!");
            self._read_from_folder(path, target)
        }

        let mp = self.CONFIG["save"]["common"].as_object().expect("cant take map from value");

        for (f_name, file_name_val) in mp.into_iter(){
            if !self._common.contains_key(f_name) {
                let file_name = file_name_val.as_str().expect("cant convert to string");
                let json_data = self.open_json(file_name, &common_path, true).expect("cant read from json");
                self._common.insert(String::from(f_name), json_data);
            }
        }
    }

    fn _read_from_folder(&mut self, folder_path: &str, target: DBESNetHashMaps) {
        let files_names = fs::read_dir(folder_path).expect("cant get files names in templates dir");

        let mut cntr = 0;

        for file_name in files_names {
            let file_name = file_name.expect("cant get path");
            let f_name = file_name.file_name();
            let f_name = f_name.to_str().expect("cant convert path to str");
            let f_name = DBESNet::_get_name_of_file(f_name);

            let file_path_buf = file_name.path();
            let file_path = file_path_buf.to_str().expect("cant convert path to str");

            let val = self.open_json("", file_path, false).expect("cant read json file");

            match target {
                DBESNetHashMaps::Net => {
                    let node_from_josn = DBESNode::from_value(val);
                    self._net.insert(String::from(&node_from_josn.idx), node_from_josn);
                    cntr += 1;
                },
                DBESNetHashMaps::Cmn => {
                    if self.CONFIG["save"]["common"].get(&f_name) != None {
                        self._common.insert(String::from(&f_name), val);
                        cntr += 1;
                    }
                },
                DBESNetHashMaps::Tmp => {
                    let node_from_josn = DBESTemplate::from_value(val);
                    self._templates.insert(String::from(&node_from_josn.idx), node_from_josn);
                    cntr += 1;
                }
            };
        }
    }

    pub fn _get_name_of_file(file_name: &str) -> String {
        let (_, file_name) = file_name.split_at(1);
        let (file_name,_) = file_name.split_once(".json").expect("cant convert file name");
        String::from(file_name)
    }

    pub fn _get_path(&self, file_name: &str, path_save: &str) -> String {
        let path_save = if path_save.is_empty() {
            self.CONFIG["save"]["path_save"].as_str().expect("cant convert to string")
        } else {path_save};

        let mut path = PathBuf::from(path_save);

        if !file_name.is_empty() {
            path.push(file_name);
        }

        match path.to_str() {
            Some(p) => String::from(p),
            _ => panic!("failed convert path to str")
        }
    }

    pub fn open_json(&self, file_name: &str, path_save: &str, create_if_not_exist: bool) -> SJResult<Value> {
        let path = self._get_path(file_name, path_save);
        open_json(&path, create_if_not_exist)
    }

    pub fn save_json(&self, file_name: &str, file_data: &Value, path_save: &str) {
        let path = self._get_path(file_name, path_save);
        match File::create(path) {
            Ok(mut fc) => {
                fc.write(file_data.to_string().as_bytes()).expect("Error while writing in file!");
            }
            Err(e) => panic!("Problem creating the file: {:?}", e),
        };
    }

    pub fn find_all_out_idxs(&self, node_idx: &str, filter_idxs: Option<&CurHashSet<String>>) -> CurHashSet<String> {
        let mut finded_idxs: CurHashSet<String> = CurHashSet::default();
        for node_rel_idx in self._net[node_idx].relation.out.keys() {
            match filter_idxs {
                None => {
                    finded_idxs.insert(String::from(node_idx));
                    finded_idxs.extend(self.find_all_out_idxs(node_rel_idx, filter_idxs).into_iter());
                },
                Some(fi) => {
                    if fi.contains(node_rel_idx) {
                        finded_idxs.insert(String::from(node_idx));
                        finded_idxs.extend(self.find_all_out_idxs(node_rel_idx, filter_idxs).into_iter());
                    }
                }
            }
        }
        finded_idxs
    }

    pub fn find_type_rel_idxs(&self, rel_idx: &str, dir_rel: &NodeRelationsDir, data_dict: Option<&CurHashMap<String, DBESNode>>, find_idxs: Option<&CurHashSet<String>>, filter_idxs: Option<&CurHashSet<String>>,
                              recursive: Option<bool>, use_filter_idxs_for_recursive: Option<bool>, checked_nodes: Option<&mut Value>) -> CurHashSet<String> {
        let mut finded_idxs: CurHashSet<String> = CurHashSet::default();

        let recursive = recursive.unwrap_or(false);
        let use_filter_idxs_for_recursive = match filter_idxs {
            None => false,
            _ => use_filter_idxs_for_recursive.unwrap_or(true)
        };

        let mut init_checked_nodes = json!({"true": {"in": [], "out": []}, "false": {"in": [], "out": []}});
        let mut checked_nodes: &mut Value = match checked_nodes {
            Some(cn) => {drop(init_checked_nodes); cn},
            None => &mut init_checked_nodes
        };

        let data_dict = match data_dict {
            Some(dt) => dt,
            None => &self._net
        };

        let mut gen_loop = |loop_keys: &mut dyn Iterator<Item = &String>| {
                for node_idx in loop_keys.into_iter() {
                let mut finded_ = false;

                let node_rels = match dir_rel {
                    NodeRelationsDir::In => &data_dict[node_idx].relation.r#in,
                    NodeRelationsDir::Out => &data_dict[node_idx].relation.out
                };

                for node_rel_idx in node_rels.keys() {
                    if node_rels[node_rel_idx].get(rel_idx) != None {
                        finded_ = true;
                        finded_idxs.insert(String::from(node_idx));
                        checked_nodes["true"][dir_rel.value()]
                            .as_array_mut()
                            .unwrap()
                            .push(json!(node_idx));
                        break;
                    }
                }

                if !finded_ && recursive && !node_rels.is_empty() {
                    let mut filter_idxs_: CurHashSet<String> = CurHashSet::default();

                    for rel_node_idx in node_rels.keys() {
                        if checked_nodes["true"][dir_rel.value()].get(rel_node_idx) != None {
                            finded_ = true;
                            finded_idxs.insert(String::from(node_idx));
                            checked_nodes["true"][dir_rel.value()]
                                .as_array_mut()
                                .unwrap()
                                .push(json!(node_idx));
                            filter_idxs_.clear();
                            break;

                        } else if checked_nodes["false"][dir_rel.value()].get(rel_node_idx) == None {
                            if !use_filter_idxs_for_recursive || filter_idxs.unwrap().get(rel_node_idx) != None {
                                filter_idxs_.insert(String::from(rel_node_idx));
                            }
                        }
                    }

                    if !filter_idxs_.is_empty() {
                        let finded_idxs_ = self.find_type_rel_idxs(rel_idx,
                                                                                dir_rel,
                                                                        None,
                                                                        Some(&filter_idxs_),
                                                                                filter_idxs,
                                                                        Some(true),
                                                    Some(use_filter_idxs_for_recursive),
                                                                    Some(&mut checked_nodes));

                        if !finded_idxs_.is_empty() {
                            finded_ = true;
                            finded_idxs.insert(String::from(node_idx));
                            checked_nodes["true"][dir_rel.value()]
                                .as_array_mut()
                                .unwrap()
                                .push(json!(node_idx));
                        }
                    }
                }
                if !finded_ {
                    checked_nodes["false"][dir_rel.value()]
                        .as_array_mut()
                        .unwrap()
                        .push(json!(node_idx));
                }
            }
        };

        match find_idxs {
            None => match filter_idxs {
                None => gen_loop(&mut data_dict.keys()),
                Some(fi) => gen_loop(&mut fi.iter())
            },
            Some(fi) => gen_loop(&mut fi.iter())
        };

        finded_idxs
    }

    pub fn find_rel_idxs(&self, node_rel_idx: &str, dir_rel: &node::NodeRelationsDir, data_dict: Option<&CurHashMap<String, DBESNode>>, find_idxs: Option<&CurHashSet<String>>, filter_idxs: Option<&CurHashSet<String>>,
                         recursive: Option<bool>, use_filter_idxs_for_recursive: Option<bool>, checked_nodes: Option<&mut CurHashMap<String, bool>>, root_: Option<bool>) -> CurHashSet<String> {
        let mut finded_idxs: CurHashSet<String> = CurHashSet::default();

        let recursive = recursive.unwrap_or(false);
        let use_filter_idxs_for_recursive = match filter_idxs {
            None => false,
            _ => use_filter_idxs_for_recursive.unwrap_or(true)
        };
        let root_ = root_.unwrap_or(true);

        let mut init_checked_nodes: CurHashMap<String, bool> = CurHashMap::default();
        let mut checked_nodes: &mut CurHashMap<String, bool> = match checked_nodes {
            Some(cn) => {drop(init_checked_nodes); cn},
            None => &mut init_checked_nodes
        };

        let data_dict = match data_dict {
            Some(dt) => dt,
            None => &self._net
        };

        let mut gen_loop = |loop_keys: &mut dyn Iterator<Item = &String>| {
            for node_idx in loop_keys.into_iter() {
                checked_nodes.insert(String::from(node_idx), false);

                let node_rels = match dir_rel {
                    NodeRelationsDir::In => &data_dict[node_idx].relation.r#in,
                    NodeRelationsDir::Out => &data_dict[node_idx].relation.out
                };

                if node_rels.get(node_rel_idx) != None {
                    finded_idxs.insert(String::from(node_idx));
                    checked_nodes.insert(String::from(node_idx), true);

                } else if recursive && !node_rels.is_empty() {
                    let mut filter_idxs_: CurHashSet<String> = CurHashSet::default();

                    for rel_node_idx in node_rels.keys() {
                        if !checked_nodes.contains_key(rel_node_idx) {
                            if !use_filter_idxs_for_recursive || filter_idxs.unwrap().get(rel_node_idx) != None {
                                filter_idxs_.insert(String::from(rel_node_idx));
                            }

                        } else if *checked_nodes.get(rel_node_idx).unwrap() {
                            finded_idxs.insert(String::from(node_idx));
                            checked_nodes.insert(String::from(node_idx), true);
                            filter_idxs_.clear();
                            break;
                        }
                    }
                    if !filter_idxs_.is_empty() {
                        let finded_idxs_ = self.find_rel_idxs(node_rel_idx,
                                                                            dir_rel,
                                                                None,
                                                                Some(&filter_idxs_),
                                                                            filter_idxs,
                                                                Some(true),
                                                Some(use_filter_idxs_for_recursive),
                                                            Some(&mut checked_nodes),
                                                                    Some(false));

                        if !finded_idxs_.is_empty() {
                            finded_idxs.insert(String::from(node_idx));
                            checked_nodes.insert(String::from(node_idx), true);
                        }
                    }
                }
            }
        };

        match find_idxs {
            None => match filter_idxs {
                None => gen_loop(&mut data_dict.keys()),
                Some(fi) => gen_loop(&mut fi.iter())
            },
            Some(fi) => gen_loop(&mut fi.iter())
        };

        finded_idxs
    }

    pub fn find_rel_idxs_NORec(&self, node_rel_idx: &str, dir_rel: &node::NodeRelationsDir, data_dict: Option<&CurHashMap<String, DBESNode>>, find_idxs: Option<&CurHashSet<String>>, filter_idxs: Option<&CurHashSet<String>>,
                               recursive: Option<bool>, use_filter_idxs_for_recursive: Option<bool>) -> CurHashSet<String> {
        let mut finded_idxs: CurHashSet<String> = CurHashSet::default();

        let recursive = recursive.unwrap_or(false);
        let use_filter_idxs_for_recursive = match filter_idxs {
            None => false,
            _ => use_filter_idxs_for_recursive.unwrap_or(true)
        };

        let data_dict = match data_dict {
            Some(dt) => dt,
            None => &self._net
        };

        let mut gen_loop = |loop_keys: &mut dyn Iterator<Item = &String>| {
            for node_idx in loop_keys.into_iter() {
                let node_rels = match dir_rel {
                    NodeRelationsDir::In => &data_dict[node_idx].relation.r#in,
                    NodeRelationsDir::Out => &data_dict[node_idx].relation.out
                };

                if node_rels.get(node_rel_idx) != None {
                    finded_idxs.insert(String::from(node_idx));

                    if recursive {
                        let mut current_nodes: CurHashSet<&str> = CurHashSet::default();
                        current_nodes.insert(node_idx);

                        loop {
                            let mut next_nodes: CurHashSet<&str> = CurHashSet::default();

                            for current_node in current_nodes {
                                finded_idxs.insert(String::from(current_node));

                                let node_rels_opposite = match dir_rel {
                                    NodeRelationsDir::Out => &data_dict[node_idx].relation.r#in,
                                    NodeRelationsDir::In => &data_dict[node_idx].relation.out
                                };
                                for rel_node_idx in node_rels_opposite.keys() {
                                    if finded_idxs.get(rel_node_idx) == None && (!use_filter_idxs_for_recursive || filter_idxs.unwrap().get(rel_node_idx) != None){
                                        next_nodes.insert(rel_node_idx);
                                    }
                                }
                            }

                            if !next_nodes.is_empty() {
                                current_nodes = next_nodes;
                            } else { break; }
                        }
                    }
                }
            }
        };

        match find_idxs {
            None => match filter_idxs {
                None => gen_loop(&mut data_dict.keys()),
                Some(fi) => gen_loop(&mut fi.iter())
            },
            Some(fi) => gen_loop(&mut fi.iter())
        };

        finded_idxs
    }

    pub fn find_mult_rel_idxs(&self, node_rel_idxs: &CurHashMap<node::NodeRelationsDir, CurHashSet<String>>, data_dict: Option<&CurHashMap<String, DBESNode>>, find_idxs: Option<&CurHashSet<String>>, filter_idxs: Option<&CurHashSet<String>>,
                              and_on: Option<bool>, recursive: Option<bool>, use_filter_idxs_for_recursive: Option<bool>) -> CurHashSet<String> {
        let mut finded_idxs: CurHashSet<String> = CurHashSet::default();

        let recursive = recursive.unwrap_or(false);
        let and_on = and_on.unwrap_or(true);
        let use_filter_idxs_for_recursive = match filter_idxs {
            None => false,
            _ => use_filter_idxs_for_recursive.unwrap_or(true)
        };

        let data_dict = match data_dict {
            Some(dt) => dt,
            None => &self._net
        };

        let mut gen_loop = |loop_keys: &mut dyn Iterator<Item = &String>| {
            for node_idx in loop_keys.into_iter() {
                let mut flag_add = and_on;

                for (dir_rel, set_node_rel_idxs) in node_rel_idxs.into_iter() {
                    if flag_add == !and_on { break; }

                    let node_rels = match dir_rel {
                        NodeRelationsDir::In => &data_dict[node_idx].relation.r#in,
                        NodeRelationsDir::Out => &data_dict[node_idx].relation.out
                    };

                    if !node_rels.is_empty() {
                        for node_rel_idx in set_node_rel_idxs {
                            if node_rels.get(node_rel_idx) == None {
                                if recursive {
                                    let mut filter_idxs_: CurHashSet<String> = CurHashSet::default();

                                    if use_filter_idxs_for_recursive {
                                        for rel_node_idx in node_rels.keys() {
                                            if filter_idxs.unwrap().get(rel_node_idx) != None {
                                                filter_idxs_.insert(String::from(rel_node_idx));
                                            }
                                        }

                                    } else {
                                        for rel_node_idx in node_rels.keys() {
                                            filter_idxs_.insert(String::from(rel_node_idx));
                                        }
                                    }

                                    let finded_idxs_ = self.find_rel_idxs_NORec(node_rel_idx,
                                                                                        dir_rel,
                                                                               None,
                                                                               Some(&filter_idxs_),
                                                                                        filter_idxs,
                                                                               Some(true),
                                                              Some(use_filter_idxs_for_recursive));

                                    if (finded_idxs_.is_empty() && and_on) || (!finded_idxs_.is_empty() && !and_on) {
                                        flag_add = and_on;
                                        break;
                                    }
                                } else if and_on { flag_add = false; break; }
                            } else if !and_on { flag_add = true; break; }
                        }
                    } else if and_on { flag_add = false; break; }
                }
                if flag_add {finded_idxs.insert(String::from(node_idx));}
            }
        };

        match find_idxs {
            None => match filter_idxs {
                None => gen_loop(&mut data_dict.keys()),
                Some(fi) => gen_loop(&mut fi.iter())
            },
            Some(fi) => gen_loop(&mut fi.iter())
        };

        finded_idxs
    }

    pub fn find_val_idxs(&self, value: node::NodeValue, contains: bool, data_dict: Option<&CurHashMap<String, DBESNode>>, filter_idxs: Option<&CurHashSet<String>>) -> CurHashSet<String> {
        let mut finded_idxs: CurHashSet<String> = CurHashSet::default();

        let data_dict = match data_dict {
            Some(dt) => dt,
            None => &self._net
        };

        let mut gen_loop = |loop_keys: &mut dyn Iterator<Item = &String>| {
            for node_idx in loop_keys.into_iter() {
                if value == data_dict[node_idx].value {
                    finded_idxs.insert(String::from(node_idx));

                } else if contains {
                    match &value {
                        node::NodeValue::Str(vs) => match &data_dict[node_idx].value {
                            node::NodeValue::Str(ds) => {
                                if ds.contains(vs) {
                                    finded_idxs.insert(String::from(node_idx));
                                }
                            },
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        };
        match filter_idxs {
            None => gen_loop(&mut data_dict.keys()),
            Some(fi) => gen_loop(&mut fi.iter())
        }
        finded_idxs
    }

}

