//! The type descriptor accepted by runtime functions.
//!
//! Corresponds to `move_native::rt_types::MoveType`.

#![allow(unused)]

use crate::stackless::llvm;
use move_model::{ast as mast, model as mm, ty as mty};

static TD_NAME: &'static str = "__move_rt_type";
static TD_TYPE_NAME_NAME: &'static str = "__move_rt_type_name";
static TD_TYPE_INFO_NAME: &'static str = "__move_rt_type_info";
static TD_VECTOR_TYPE_INFO_NAME: &'static str = "__move_rt_type_info_vec";
static TD_STRUCT_TYPE_INFO_NAME: &'static str = "__move_rt_type_info_struct";
static TD_REFERENCE_TYPE_INFO_NAME: &'static str = "__move_rt_type_info_ref";

pub fn get_llvm_tydesc_type(llcx: &llvm::Context) -> llvm::StructType {
    match llcx.named_struct_type(TD_NAME) {
        Some(t) => t,
        None => {
            declare_llvm_tydesc_type(llcx);
            llcx.named_struct_type(TD_NAME).expect(".")
        }
    }
}

fn declare_llvm_tydesc_type(llcx: &llvm::Context) {
    let td_llty = llcx.create_opaque_named_struct(TD_NAME);
    let field_tys = {
        let type_name_ty = llcx.anonymous_struct_type(&[
            llcx.int8_type().ptr_type(),
            llcx.int64_type(),
        ]).as_any_type();
        let type_descrim_ty = llcx.int32_type();
        // This is a pointer to a statically-defined union of type infos
        let type_info_ptr_ty = llcx.int8_type().ptr_type();
        &[
            type_name_ty,
            type_descrim_ty,
            type_info_ptr_ty,
        ]
    };

    td_llty.set_struct_body(field_tys);
}

pub fn define_llvm_tydesc(
    llcx: &llvm::Context,
    llmod: &llvm::Module,
    mty: &mty::Type,
) -> llvm::Global {
    let name = global_tydesc_name(mty);
    match llmod.get_global(&name) {
        Some(g) => g,
        None => {
            let ll_tydesc_ty = get_llvm_tydesc_type(llcx);
            let ll_tydesc_ty = ll_tydesc_ty.as_any_type();
            let ll_global = llmod.add_global(
                ll_tydesc_ty,
                &name,
            );
            let ll_constant = tydesc_constant(
                llcx, llmod, mty,
            );
            todo!()
        }
    }
}

fn tydesc_constant(
    llcx: &llvm::Context,
    llmod: &llvm::Module,
    mty: &mty::Type,
) -> llvm::Constant {
    let ll_const_type_name = type_name_constant(
        llcx, llmod, mty,
    );
    let ll_const_type_descrim = {
        let ll_ty = llcx.int32_type();
        llvm::Constant::int(ll_ty, type_descrim(mty))
    };
    let ll_const_type_info_ptr = {
        let ll_global_type_info = define_type_info_global(
            llcx, llmod, mty,
        );
        ll_global_type_info.ptr()
    };
    let ll_const = llvm::Constant::struct_(&[
        ll_const_type_name,
        ll_const_type_descrim,
        ll_const_type_info_ptr,
    ]);
    ll_const
}

fn type_name_constant(
    llcx: &llvm::Context,
    llmod: &llvm::Module,
    mty: &mty::Type,
) -> llvm::Constant {
    todo!()
}

fn type_descrim(
    mty: &mty::Type,
) -> u64 {
    todo!()
}

fn define_type_info_global(
    llcx: &llvm::Context,
    llmod: &llvm::Module,
    mty: &mty::Type,
) -> llvm::Global {
    todo!()
}

fn global_tydesc_name(mty: &mty::Type) -> String {
    use mty::{PrimitiveType, Type};
    let name = match mty {
        Type::Primitive(PrimitiveType::U64) => {
            "u64"
        }
        _ => todo!()
    };

    format!("__move_rttydesc_{name}")
}
