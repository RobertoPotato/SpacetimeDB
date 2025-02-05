// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused, clippy::all)]
use super::one_i_64_type::OneI64;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `one_i64`.
///
/// Obtain a handle from the [`OneI64TableAccess::one_i_64`] method on [`super::RemoteTables`],
/// like `ctx.db.one_i_64()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_i_64().on_insert(...)`.
pub struct OneI64TableHandle<'ctx> {
    imp: __sdk::TableHandle<OneI64>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_i64`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneI64TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneI64TableHandle`], which mediates access to the table `one_i64`.
    fn one_i_64(&self) -> OneI64TableHandle<'_>;
}

impl OneI64TableAccess for super::RemoteTables {
    fn one_i_64(&self) -> OneI64TableHandle<'_> {
        OneI64TableHandle {
            imp: self.imp.get_table::<OneI64>("one_i64"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneI64InsertCallbackId(__sdk::CallbackId);
pub struct OneI64DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OneI64TableHandle<'ctx> {
    type Row = OneI64;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneI64> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneI64InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneI64InsertCallbackId {
        OneI64InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneI64InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneI64DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneI64DeleteCallbackId {
        OneI64DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneI64DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<OneI64>("one_i64");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<OneI64>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<OneI64>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
