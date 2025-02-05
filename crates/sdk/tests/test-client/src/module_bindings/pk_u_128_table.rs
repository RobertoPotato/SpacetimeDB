// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused, clippy::all)]
use super::pk_u_128_type::PkU128;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `pk_u128`.
///
/// Obtain a handle from the [`PkU128TableAccess::pk_u_128`] method on [`super::RemoteTables`],
/// like `ctx.db.pk_u_128()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_u_128().on_insert(...)`.
pub struct PkU128TableHandle<'ctx> {
    imp: __sdk::TableHandle<PkU128>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `pk_u128`.
///
/// Implemented for [`super::RemoteTables`].
pub trait PkU128TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`PkU128TableHandle`], which mediates access to the table `pk_u128`.
    fn pk_u_128(&self) -> PkU128TableHandle<'_>;
}

impl PkU128TableAccess for super::RemoteTables {
    fn pk_u_128(&self) -> PkU128TableHandle<'_> {
        PkU128TableHandle {
            imp: self.imp.get_table::<PkU128>("pk_u128"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct PkU128InsertCallbackId(__sdk::CallbackId);
pub struct PkU128DeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for PkU128TableHandle<'ctx> {
    type Row = PkU128;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = PkU128> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = PkU128InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkU128InsertCallbackId {
        PkU128InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: PkU128InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = PkU128DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PkU128DeleteCallbackId {
        PkU128DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: PkU128DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<PkU128>("pk_u128");
    _table.add_unique_constraint::<u128>("n", |row| &row.n);
}
pub struct PkU128UpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for PkU128TableHandle<'ctx> {
    type UpdateCallbackId = PkU128UpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> PkU128UpdateCallbackId {
        PkU128UpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: PkU128UpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<PkU128>> {
    __sdk::TableUpdate::parse_table_update_with_primary_key::<u128>(raw_updates, |row: &PkU128| &row.n).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<PkU128>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `n` unique index on the table `pk_u128`,
/// which allows point queries on the field of the same name
/// via the [`PkU128NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.pk_u_128().n().find(...)`.
pub struct PkU128NUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<PkU128, u128>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> PkU128TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `pk_u128`.
    pub fn n(&self) -> PkU128NUnique<'ctx> {
        PkU128NUnique {
            imp: self.imp.get_unique_constraint::<u128>("n"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> PkU128NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u128) -> Option<PkU128> {
        self.imp.find(col_val)
    }
}
