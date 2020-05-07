//! Defines database & queries for name resolution.
use std::sync::Arc;

use hir_expand::{db::AstDatabase, HirFileId};
use ra_db::{salsa, CrateId, SourceDatabase, Upcast};
use ra_prof::profile;
use ra_syntax::SmolStr;

use crate::{
    adt::{EnumData, StructData},
    attr::Attrs,
    body::{scope::ExprScopes, Body, BodySourceMap},
    data::{ConstData, FunctionData, ImplData, TraitData, TypeAliasData},
    generics::GenericParams,
    lang_item::{LangItemTarget, LangItems},
    nameres::{raw::RawItems, CrateDefMap},
    AttrDefId, ConstId, ConstLoc, DefWithBodyId, EnumId, EnumLoc, FunctionId, FunctionLoc,
    GenericDefId, ImplId, ImplLoc, ModuleId, StaticId, StaticLoc, StructId, StructLoc, TraitId,
    TraitLoc, TypeAliasId, TypeAliasLoc, UnionId, UnionLoc,
};

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: SourceDatabase {
    #[salsa::interned]
    fn intern_function(&self, loc: FunctionLoc) -> FunctionId;
    #[salsa::interned]
    fn intern_struct(&self, loc: StructLoc) -> StructId;
    #[salsa::interned]
    fn intern_union(&self, loc: UnionLoc) -> UnionId;
    #[salsa::interned]
    fn intern_enum(&self, loc: EnumLoc) -> EnumId;
    #[salsa::interned]
    fn intern_const(&self, loc: ConstLoc) -> ConstId;
    #[salsa::interned]
    fn intern_static(&self, loc: StaticLoc) -> StaticId;
    #[salsa::interned]
    fn intern_trait(&self, loc: TraitLoc) -> TraitId;
    #[salsa::interned]
    fn intern_type_alias(&self, loc: TypeAliasLoc) -> TypeAliasId;
    #[salsa::interned]
    fn intern_impl(&self, loc: ImplLoc) -> ImplId;
}

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + AstDatabase + Upcast<dyn AstDatabase> {
    #[salsa::invoke(RawItems::raw_items_query)]
    fn raw_items(&self, file_id: HirFileId) -> Arc<RawItems>;

    #[salsa::invoke(crate_def_map_wait)]
    #[salsa::transparent]
    fn crate_def_map(&self, krate: CrateId) -> Arc<CrateDefMap>;

    #[salsa::invoke(CrateDefMap::crate_def_map_query)]
    fn crate_def_map_query(&self, krate: CrateId) -> Arc<CrateDefMap>;

    #[salsa::invoke(StructData::struct_data_query)]
    fn struct_data(&self, id: StructId) -> Arc<StructData>;
    #[salsa::invoke(StructData::union_data_query)]
    fn union_data(&self, id: UnionId) -> Arc<StructData>;

    #[salsa::invoke(EnumData::enum_data_query)]
    fn enum_data(&self, e: EnumId) -> Arc<EnumData>;

    #[salsa::invoke(ImplData::impl_data_query)]
    fn impl_data(&self, e: ImplId) -> Arc<ImplData>;

    #[salsa::invoke(TraitData::trait_data_query)]
    fn trait_data(&self, e: TraitId) -> Arc<TraitData>;

    #[salsa::invoke(TypeAliasData::type_alias_data_query)]
    fn type_alias_data(&self, e: TypeAliasId) -> Arc<TypeAliasData>;

    #[salsa::invoke(FunctionData::fn_data_query)]
    fn function_data(&self, func: FunctionId) -> Arc<FunctionData>;

    #[salsa::invoke(ConstData::const_data_query)]
    fn const_data(&self, konst: ConstId) -> Arc<ConstData>;

    #[salsa::invoke(ConstData::static_data_query)]
    fn static_data(&self, konst: StaticId) -> Arc<ConstData>;

    #[salsa::invoke(Body::body_with_source_map_query)]
    fn body_with_source_map(&self, def: DefWithBodyId) -> (Arc<Body>, Arc<BodySourceMap>);

    #[salsa::invoke(Body::body_query)]
    fn body(&self, def: DefWithBodyId) -> Arc<Body>;

    #[salsa::invoke(ExprScopes::expr_scopes_query)]
    fn expr_scopes(&self, def: DefWithBodyId) -> Arc<ExprScopes>;

    #[salsa::invoke(GenericParams::generic_params_query)]
    fn generic_params(&self, def: GenericDefId) -> Arc<GenericParams>;

    #[salsa::invoke(Attrs::attrs_query)]
    fn attrs(&self, def: AttrDefId) -> Attrs;

    #[salsa::invoke(LangItems::module_lang_items_query)]
    fn module_lang_items(&self, module: ModuleId) -> Option<Arc<LangItems>>;

    #[salsa::invoke(LangItems::crate_lang_items_query)]
    fn crate_lang_items(&self, krate: CrateId) -> Arc<LangItems>;

    #[salsa::invoke(LangItems::lang_item_query)]
    fn lang_item(&self, start_crate: CrateId, item: SmolStr) -> Option<LangItemTarget>;
}

fn crate_def_map_wait(db: &impl DefDatabase, krate: CrateId) -> Arc<CrateDefMap> {
    let _p = profile("crate_def_map:wait");
    db.crate_def_map_query(krate)
}
