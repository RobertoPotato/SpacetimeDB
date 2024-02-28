// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::every_primitive_struct::EveryPrimitiveStruct;
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VecEveryPrimitiveStruct {
    pub s: Vec<EveryPrimitiveStruct>,
}

impl TableType for VecEveryPrimitiveStruct {
    const TABLE_NAME: &'static str = "VecEveryPrimitiveStruct";
    type ReducerEvent = super::ReducerEvent;
}

impl VecEveryPrimitiveStruct {
    #[allow(unused)]
    pub fn filter_by_s(s: Vec<EveryPrimitiveStruct>) -> TableIter<Self> {
        Self::filter(|row| row.s == s)
    }
}
