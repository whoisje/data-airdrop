use serde_json::Value;
use serde::{Deserialize, Serialize};

///
///row data 元信息，用来说明row data中包含那些字段，类似于表头
///
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RowColumnMeta {
    name: String,
    type_of: String,
    len: u32,
    scale: u32,
    primary: bool,

}

impl RowColumnMeta {
    pub fn new(name: String, type_of: String, len: u32, scale: u32, primary: bool) -> Self {
        RowColumnMeta { name, type_of, len, scale, primary }
    }
}

impl Default for RowColumnMeta {
    fn default() -> Self {
        RowColumnMeta { name: String::new(), type_of: String::from("String"), len: 0, scale: 0, primary: false }
    }
}

impl RowColumnMeta {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_type_of(&mut self, type_of: String) {
        self.type_of = type_of;
    }
    pub fn set_len(&mut self, len: u32) {
        self.len = len;
    }
    pub fn set_scale(&mut self, scale: u32) {
        self.scale = scale;
    }
    pub fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }
}

impl RowColumnMeta {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn type_of(&self) -> &str {
        &self.type_of
    }
    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn scale(&self) -> u32 {
        self.scale
    }
    pub fn primary(&self) -> bool {
        self.primary
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RowColumn {
    column_meta: RowColumnMeta,
    value: Value,
}

impl RowColumn {
    pub fn column_meta(&self) -> &RowColumnMeta {
        &self.column_meta
    }
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl RowColumn {
    pub fn new(column_meta: RowColumnMeta, value: Value) -> Self {
        RowColumn { column_meta, value }
    }
}


