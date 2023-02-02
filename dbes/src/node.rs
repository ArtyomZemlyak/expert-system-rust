use sj::{Value, json};
use uuid::Uuid;
use md5;
use serde_json as sj;
use serde::ser::{Serialize as SerSerialize, Serializer as SerSerializer};
use serde::de::{self, Deserialize as DeDeserialize, Deserializer as DeDeserializer, Visitor};
use serde::{Serialize, Deserialize};
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use fxhash::{FxHashMap, FxHashSet};


#[derive(Debug, PartialEq)]
pub enum Coeff {
    I32(i32),
    F32(f32),
    F64(f64),
    None
}
trait GetValue<T> {
    fn value(&self) -> T;
}
impl GetValue<i32> for Coeff {
    fn value(&self) -> i32 {
        match *self {
            Coeff::I32(t)  => t,
            _ => panic!("not i32 type!")
        }
    }
}
impl GetValue<f32> for Coeff {
    fn value(&self) -> f32 {
        match *self {
            Coeff::F32(t) => t,
            _ => panic!("not f32 type!")
        }
    }
}
impl SerSerialize for Coeff {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: SerSerializer,
    {
        match &self {
            Coeff::I32(int) => serializer.serialize_i32(*int),
            Coeff::F32(float) => serializer.serialize_f32(*float),
            Coeff::F64(float) => serializer.serialize_f64(*float),
            Coeff::None => serializer.serialize_none(),
        }
    }
}
struct CoeffVisitor;
impl<'de> Visitor<'de> for CoeffVisitor {
    type Value = Coeff;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Coeff of relation in DBES node")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> where E: de::Error, { Ok(Coeff::I32(value.try_into().unwrap())) }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> where E: de::Error, { Ok(Coeff::F64(value)) }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> where E: de::Error, { Ok(Coeff::I32(value)) }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> where E: de::Error, { Ok(Coeff::F32(value)) }

    fn visit_none<E>(self) -> Result<Self::Value, E> where E: de::Error, { Ok(Coeff::None) }

    // fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: de::Error, { Ok(Coeff::I32(1)) }
}
impl<'de> DeDeserialize<'de> for Coeff {
    fn deserialize<D>(deserializer: D) -> Result<Coeff, D::Error> where D: DeDeserializer<'de>,
    {
        deserializer.deserialize_any(CoeffVisitor)
    }
}


#[derive(Debug, PartialEq)]
pub enum NodeValue {
    Bool(bool),
    I32(i32),
    F32(f32),
    F64(f64),
    Str(String),
    None
}
impl SerSerialize for NodeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: SerSerializer,
    {
        match &self {
            NodeValue::Bool(val_bool) => serializer.serialize_bool(*val_bool),
            NodeValue::I32(int) => serializer.serialize_i32(*int),
            NodeValue::F32(float) => serializer.serialize_f32(*float),
            NodeValue::F64(float) => serializer.serialize_f64(*float),
            NodeValue::Str(val_str) => serializer.serialize_str(val_str),
            NodeValue::None => serializer.serialize_none(),
        }
    }
}
struct NodeValueVisitor;
impl<'de> Visitor<'de> for NodeValueVisitor {
    type Value = NodeValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("value of Node in DBES")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> where E: de::Error, { Ok(NodeValue::Bool(value)) }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> where E: de::Error, { Ok(NodeValue::I32(value.try_into().unwrap())) }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> where E: de::Error, { Ok(NodeValue::F64(value)) }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> where E: de::Error, { Ok(NodeValue::I32(value)) }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> where E: de::Error, { Ok(NodeValue::F32(value)) }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: de::Error, { Ok(NodeValue::Str(value)) }

    fn visit_none<E>(self) -> Result<Self::Value, E> where E: de::Error, { Ok(NodeValue::None) }

}
impl<'de> DeDeserialize<'de> for NodeValue {
    fn deserialize<D>(deserializer: D) -> Result<NodeValue, D::Error> where D: DeDeserializer<'de>,
    {
        deserializer.deserialize_any(NodeValueVisitor)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRelations {
    pub r#in: FxHashMap<String, FxHashMap<String, Coeff>>,
    pub out: FxHashMap<String, FxHashMap<String, Coeff>>
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeRelationsDir {
    In,
    Out
}
impl Hash for NodeRelationsDir {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            NodeRelationsDir::In => 1.hash(state),
            NodeRelationsDir::Out => 0.hash(state),
        };
    }
}
impl NodeRelationsDir {
    pub fn value(&self) -> String {
        match *self {
            NodeRelationsDir::In  => String::from("in"),
            NodeRelationsDir::Out => String::from("out"),
        }
    }
    pub fn value_opposite(&self) -> String {
        match *self {
            NodeRelationsDir::In  => String::from("out"),
            NodeRelationsDir::Out => String::from("in"),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRelationsSJValue {
    pub r#in: sj::Value,
    pub out: sj::Value
}



#[derive(Debug, Serialize, Deserialize)]
pub struct DBESNode {
    // hash_idx: u128,
    pub idx: String,
    pub value: NodeValue,
    pub relation: NodeRelations
}

impl PartialEq for DBESNode {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}
impl Eq for DBESNode {}
impl Hash for DBESNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}
impl std::fmt::Display for DBESNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.idx)
    }
}


impl DBESNode {

    pub fn new(value: NodeValue, relation: NodeRelations, idx: &str) -> Self {
        let idx = if idx.is_empty() {
            DBESNode::generate_idx()
        } else {String::from(idx)};

        DBESNode {
            // hash_idx: DBESNode::to_int_idx(&idx),
            idx,
            value,
            relation
        }
    }

    pub fn from_value(sj_value: sj::Value) -> Self {
        sj::from_value(sj_value).expect("cant convert from this Value to DBESNode")
    }

    pub fn from_str(sj_value: &str) -> Self {
        sj::from_str(sj_value).expect("cant convert from this Value to DBESNode")
    }

    pub fn generate_idx() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn to_idx(some_string: &str) -> String {
        format!("{:x}", md5::compute(some_string.as_bytes()))
    }

    pub fn to_int_idx(some_string: &str) -> u128 {
        let dig_md5 = md5::compute(some_string.as_bytes());
        let u128arr: [u8; 16] = dig_md5.into();
        u128::from_be_bytes(u128arr)
    }

    pub fn update_value(&mut self, value: NodeValue) {
        self.value = value;
    }

    pub fn add_relation(&mut self, other_idx: &str, rel_idx: &str, coeff: Coeff, type_rel: NodeRelationsDir) {
        let node_rels = match type_rel {
            NodeRelationsDir::In => &mut self.relation.r#in,
            NodeRelationsDir::Out => &mut self.relation.out
        };

        if node_rels.get(other_idx) == None {
            match coeff {
                Coeff::None => {
                    let mut new_hashmap = FxHashMap:: default();
                    new_hashmap.insert(String::from(rel_idx), Coeff::I32(1));
                    node_rels.insert(String::from(other_idx), new_hashmap);
                },
                _ => {
                    let mut new_hashmap = FxHashMap:: default();
                    new_hashmap.insert(String::from(rel_idx), coeff);
                    node_rels.insert(String::from(other_idx), new_hashmap);
                }
            };

        } else {
            if node_rels[other_idx].get(rel_idx) == None {
                match coeff {
                    Coeff::None => node_rels.get_mut(other_idx).unwrap().insert(String::from(rel_idx), Coeff::I32(1)),
                    _ => node_rels.get_mut(other_idx).unwrap().insert(String::from(rel_idx), coeff)
                };
            } else {
                match coeff {
                    Coeff::None => {
                        let rel_coeff = node_rels.get_mut(other_idx).unwrap().get_mut(rel_idx).expect("cant get rel coeff");
                        match rel_coeff {
                            Coeff::I32(number) => *rel_coeff = Coeff::I32(*number + 1),
                            _ => {}
                        };
                    },
                    _ => {node_rels.get_mut(other_idx).unwrap().insert(String::from(rel_idx), coeff);}
                };
            }
        }
    }

    pub fn remove_relation(&mut self, other_idx: &str, rel_idx: &str, type_rel: NodeRelationsDir) {
        let node_rels = match type_rel {
            NodeRelationsDir::In => &mut self.relation.r#in,
            NodeRelationsDir::Out => &mut self.relation.out
        };
        if node_rels.get(other_idx) != None {
            if node_rels[other_idx].get(rel_idx) != None {
                node_rels.get_mut(other_idx).unwrap().remove(rel_idx);

                if node_rels[other_idx].is_empty() {
                    node_rels.remove(other_idx);
                }
            } else {
                node_rels.remove(other_idx);
            }
        }
    }

}


#[derive(Debug, Serialize, Deserialize)]
pub struct DBESTemplate {
    pub node: DBESNode,
    pub template: sj::Value,
}

impl std::ops::Deref for DBESTemplate {
    type Target = DBESNode;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
impl std::fmt::Display for DBESTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.idx)
    }
}
impl PartialEq for DBESTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}
impl Eq for DBESTemplate {}
impl Hash for DBESTemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl DBESTemplate {

    pub fn new(value: NodeValue, relation: NodeRelations, template: sj::Value, idx: &str) -> Self {
        let idx = if idx.is_empty() {
            DBESNode::generate_idx()
        } else {String::from(idx)};

        DBESTemplate {
            node: DBESNode {
                // hash_idx: DBESNode::to_int_idx(&idx),
                idx,
                value,
                relation
            },
            template
        }
    }

    pub fn from_value(value: sj::Value) -> Self {
        sj::from_value(value).expect("cant convert from this Value to DBESNode")
    }
}



