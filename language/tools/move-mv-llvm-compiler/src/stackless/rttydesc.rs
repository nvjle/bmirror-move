//! The type descriptor accepted by runtime functions.
//!
//! Corresponds to `move_native::rt_types::MoveType`.

#![allow(unused)]

use crate::stackless::llvm;

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
        let type_descrim_ty = llcx.int8_type();
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
