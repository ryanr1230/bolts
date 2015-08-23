use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::collections::BTreeMap;

pub struct Layout {
    context: String,
    footer: String,
    header: String,
}

impl Layout {
    pub fn new(context: String, footer: String, header: String) -> Layout {
        Layout { context: context, footer: footer, header: header }
    }
}

impl ToJson for Layout {
  fn to_json(&self) -> Json {
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("context".to_string(), self.context.to_json());
    m.insert("footer".to_string(), self.footer.to_json());
    m.insert("header".to_string(), self.header.to_json());
    m.to_json()
  }
}
