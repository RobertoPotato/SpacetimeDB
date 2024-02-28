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
pub struct DeletePkU128Args {
    pub n: u128,
}

impl Reducer for DeletePkU128Args {
    const REDUCER_NAME: &'static str = "delete_pk_u128";
}

#[allow(unused)]
pub fn delete_pk_u_128(n: u128) {
    DeletePkU128Args { n }.invoke();
}

#[allow(unused)]
pub fn on_delete_pk_u_128(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u128) + Send + 'static,
) -> ReducerCallbackId<DeletePkU128Args> {
    DeletePkU128Args::on_reducer(move |__identity, __addr, __status, __args| {
        let DeletePkU128Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_delete_pk_u_128(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u128) + Send + 'static,
) -> ReducerCallbackId<DeletePkU128Args> {
    DeletePkU128Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeletePkU128Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_delete_pk_u_128(id: ReducerCallbackId<DeletePkU128Args>) {
    DeletePkU128Args::remove_on_reducer(id);
}
