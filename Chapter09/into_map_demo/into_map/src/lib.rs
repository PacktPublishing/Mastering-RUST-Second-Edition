// into_map_demo/into_map/src/lib.rs

use std::collections::BTreeMap;

pub trait IntoMap {
    fn into_map(&self) -> BTreeMap<String, String>;
}
