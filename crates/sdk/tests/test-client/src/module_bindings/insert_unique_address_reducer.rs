// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
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
pub struct InsertUniqueAddressArgs {
    pub a: Address,
    pub data: i32,
}

impl Reducer for InsertUniqueAddressArgs {
    const REDUCER_NAME: &'static str = "insert_unique_address";
}

#[allow(unused)]
pub fn insert_unique_address(a: Address, data: i32) {
    InsertUniqueAddressArgs { a, data }.invoke();
}

#[allow(unused)]
pub fn on_insert_unique_address(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Address, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertUniqueAddressArgs> {
    InsertUniqueAddressArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertUniqueAddressArgs { a, data } = __args;
        __callback(__identity, __addr, __status, a, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_unique_address(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Address, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertUniqueAddressArgs> {
    InsertUniqueAddressArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertUniqueAddressArgs { a, data } = __args;
        __callback(__identity, __addr, __status, a, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_unique_address(id: ReducerCallbackId<InsertUniqueAddressArgs>) {
    InsertUniqueAddressArgs::remove_on_reducer(id);
}
