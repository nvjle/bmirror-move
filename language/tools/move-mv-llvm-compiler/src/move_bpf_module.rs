use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::debug_info::DICompileUnit;
use inkwell::debug_info::DebugInfoBuilder;
use inkwell::module::{Linkage, Module};
use inkwell::OptimizationLevel;
use inkwell::{targets::{TargetTriple, TargetMachine}};
use inkwell::targets::{CodeModel, RelocMode};
use inkwell::{targets::Target, targets::InitializationConfig};
use inkwell::types::{
    ArrayType, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, IntType, StringRadix,
};

use move_binary_format::{
    binary_views::BinaryIndexedView,
    control_flow_graph::{ControlFlowGraph, VMControlFlowGraph},
    file_format::{
        Ability, AbilitySet, Bytecode, CodeUnit, FieldHandleIndex, FunctionDefinition,
        FunctionDefinitionIndex, FunctionHandle, Signature, SignatureIndex, SignatureToken,
        StructDefinition, StructDefinitionIndex, StructFieldInformation, StructTypeParameter,
        TableIndex, TypeSignature, Visibility,
    },
};

use move_bytecode_source_map::source_map::SourceName;
use once_cell::sync::OnceCell;

static LLVM_INIT: OnceCell<()> = OnceCell::new();

pub struct MoveBPFModule<'a> {
    pub name: String,
    pub module: Module<'a>,
    pub builder: Builder<'a>,
    pub dibuilder: DebugInfoBuilder<'a>,
    pub di_compile_unit: DICompileUnit<'a>,
    pub(crate) context: &'a Context,
    pub(crate) opt: OptimizationLevel,
}

impl<'a> MoveBPFModule<'a> {
    fn llvm_target_triple() -> TargetTriple {
        TargetTriple::create("bpfel-unknown-unknown")
    }

    fn llvm_target_name() -> &'static str {
        "bpfel" // bpf little endian.
    }

    fn llvm_features() -> &'static str {
        "" // no additional target specific features.
    }

    pub fn get_target_machine(&self) -> Option<TargetMachine> {
        Target::initialize_bpf(&InitializationConfig::default());

        let opt = OptimizationLevel::None; // TODO: Add optimization based on command line flag.
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;
        let target = Target::from_name(MoveBPFModule::llvm_target_name()).unwrap();

        return target.create_target_machine(
            &MoveBPFModule::llvm_target_triple(),
            "v2",
            MoveBPFModule::llvm_features(),
            opt,
            reloc,
            model
        );
    }

    pub fn new(
        context: &'a Context,
        name: &str,
        filename: &str,
        opt: OptimizationLevel,
    ) -> Self {
        LLVM_INIT.get_or_init(|| {
            inkwell::targets::Target::initialize_bpf(&Default::default());
        });

        let triple = MoveBPFModule::llvm_target_triple();
        let module = context.create_module(name);

        let debug_metadata_version = context.i32_type().const_int(3, false);
        module.add_basic_value_flag(
            "Debug Info Version",
            inkwell::module::FlagBehavior::Warning,
            debug_metadata_version,
        );

        let builder = context.create_builder();
        let (dibuilder, di_compile_unit) = module.create_debug_info_builder(
            true,
            inkwell::debug_info::DWARFSourceLanguage::C,
            filename,
            ".",
            "Move",
            false,
            "",
            0,
            "",
            inkwell::debug_info::DWARFEmissionKind::Full,
            0,
            false,
            false,
            "",
            "",
        );

        module.set_triple(&triple);
        module.set_source_file_name(filename);

        MoveBPFModule {
            name: name.to_owned(),
            module,
            builder,
            dibuilder,
            di_compile_unit,
            context,
            opt,
        }
    }

    pub(crate) fn llvm_type_for_sig_tok(&self, sig_tok: &SignatureToken, _type_parameters: &[AbilitySet]) -> BasicTypeEnum<'a> {
        match sig_tok {
            SignatureToken::Bool => BasicTypeEnum::IntType(self.context.bool_type()),
            SignatureToken::U8 => BasicTypeEnum::IntType(self.context.custom_width_int_type(8)), // FIXME: The signedness
            SignatureToken::U64 => BasicTypeEnum::IntType(self.context.custom_width_int_type(64)), // FIXME: The signedness
            _ => unimplemented!("Remaining Signature tokens to be implemented"),
        }
    }
    pub fn llvm_type_for_sig_tokens(&self, sig_tokens: Vec<SignatureToken>, type_parameters: &[AbilitySet],) -> Vec<BasicTypeEnum<'a>> {
        let mut vec = Vec::new();
        for v in sig_tokens {
            vec.push(self.llvm_type_for_sig_tok(&v, type_parameters));
        }
        return vec;
    }
    pub fn llvm_constant() {
        // TODO: Return a constant value corresponding to the input type.
    }
}