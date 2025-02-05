// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused, clippy::all)]
use super::byte_struct_type::ByteStruct;
use super::one_byte_struct_type::OneByteStruct;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `one_byte_struct`.
///
/// Obtain a handle from the [`OneByteStructTableAccess::one_byte_struct`] method on [`super::RemoteTables`],
/// like `ctx.db.one_byte_struct()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.one_byte_struct().on_insert(...)`.
pub struct OneByteStructTableHandle<'ctx> {
    imp: __sdk::TableHandle<OneByteStruct>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `one_byte_struct`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OneByteStructTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OneByteStructTableHandle`], which mediates access to the table `one_byte_struct`.
    fn one_byte_struct(&self) -> OneByteStructTableHandle<'_>;
}

impl OneByteStructTableAccess for super::RemoteTables {
    fn one_byte_struct(&self) -> OneByteStructTableHandle<'_> {
        OneByteStructTableHandle {
            imp: self.imp.get_table::<OneByteStruct>("one_byte_struct"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OneByteStructInsertCallbackId(__sdk::CallbackId);
pub struct OneByteStructDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for OneByteStructTableHandle<'ctx> {
    type Row = OneByteStruct;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OneByteStruct> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OneByteStructInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneByteStructInsertCallbackId {
        OneByteStructInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OneByteStructInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OneByteStructDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OneByteStructDeleteCallbackId {
        OneByteStructDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OneByteStructDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<OneByteStruct>("one_byte_struct");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<OneByteStruct>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<OneByteStruct>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
