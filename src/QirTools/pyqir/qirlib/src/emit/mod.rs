use inkwell::values::BasicValueEnum;

use self::constants::Constants;
use self::intrinsics::Intrinsics;
use self::runtime_library::RuntimeLibrary;
use self::types::Types;

use super::interop::SemanticModel;
use std::collections::BTreeMap;
use std::path::Path;

pub mod constants;
mod intrinsics;
mod qir;
mod runtime_library;
pub mod types;

pub struct Emitter {}
impl Emitter {
    pub fn write(model: &SemanticModel, file_name: &str) {
        let ctx = inkwell::context::Context::create();
        let context = Context::new(&ctx, model.name.as_str());

        let entrypoint = qir::get_entry_function(&context);
        let entry = context.context.append_basic_block(entrypoint, "entry");
        context.builder.position_at_end(entry);

        let qubits = Emitter::write_qubits(&model, &context);

        let registers = Emitter::write_registers(&model, &context);

        let _ = Emitter::write_instructions(&model, &context, &qubits);

        Emitter::free_qubits(&context, &qubits);
        let output = registers.get("results").unwrap();
        context.builder.build_return(Some(output));

        context.emit_ir(file_name);
    }

    fn free_qubits<'ctx>(context: &Context<'ctx>, qubits: &BTreeMap<String, BasicValueEnum<'ctx>>) {
        for (_, value) in qubits.iter() {
            qir::qubits::emit_release(context, value);
        }
    }

    fn write_qubits<'ctx>(
        model: &SemanticModel,
        context: &Context<'ctx>,
    ) -> BTreeMap<String, BasicValueEnum<'ctx>> {
        let mut qubits = BTreeMap::new();
        for reg in model.qubits.iter() {
            let indexed_name = format!("{}{}", &reg.name[..], reg.index);
            let value = qir::qubits::emit_allocate(&context, indexed_name.as_str());
            qubits.insert(indexed_name, value);
        }
        qubits
    }

    fn write_registers<'ctx>(
        model: &SemanticModel,
        context: &Context<'ctx>,
    ) -> BTreeMap<String, BasicValueEnum<'ctx>> {
        let mut registers = BTreeMap::new();
        let number_of_registers = model.registers.len() as u64;
        if number_of_registers > 0 {
            let results =
                qir::array1d::emit_array_allocate1d(&context, 8, number_of_registers, "results");
            registers.insert(String::from("results"), results);
            let mut sub_results = vec![];
            for reg in model.registers.iter() {
                let sub_result =
                    qir::array1d::emit_array_1d(context, reg.name.as_str(), reg.size.clone());
                sub_results.push(sub_result);
                registers.insert(reg.name.clone(), sub_result);
            }
            qir::array1d::set_elements(&context, &results, sub_results, "results");
            registers
        } else {
            let results = qir::array1d::emit_empty_result_array_allocate1d(&context, "results");
            registers.insert(String::from("results"), results);
            registers
        }
    }

    fn write_instructions<'ctx>(
        model: &SemanticModel,
        context: &Context<'ctx>,
        qubits: &BTreeMap<String, BasicValueEnum<'ctx>>,
    ) {
        for inst in model.instructions.iter() {
            qir::instructions::emit(context, inst, qubits);
        }
    }
}

pub struct Context<'ctx> {
    pub(crate) context: &'ctx inkwell::context::Context,
    pub(crate) module: inkwell::module::Module<'ctx>,
    pub(crate) builder: inkwell::builder::Builder<'ctx>,
    pub(crate) types: Types<'ctx>,
    pub(crate) runtime_library: RuntimeLibrary<'ctx>,
    pub(crate) intrinsics: Intrinsics<'ctx>,
    pub(crate) constants: Constants<'ctx>,
}
impl<'ctx> Context<'ctx> {
    pub fn new(context: &'ctx inkwell::context::Context, name: &'ctx str) -> Self {
        let builder = context.create_builder();

        let module = qir::load_module_from_bitcode_file(&context, name);

        let types = Types::new(&context, &module);
        let runtime_library = RuntimeLibrary::new(&module);
        let intrinsics = Intrinsics::new(&module);
        let constants = Constants::new(&module, &types);
        Context {
            builder: builder,
            module: module,
            types: types,
            context: context,
            runtime_library: runtime_library,
            intrinsics: intrinsics,
            constants: constants,
        }
    }

    pub fn add_boilerplate(&self) {
        let void_type = self.context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let fn_val = self.module.add_function("my_fn", fn_type, None);
        let basic_block = self.context.append_basic_block(fn_val, "entry");
        self.builder.position_at_end(basic_block);
        self.builder.build_return(None);
    }

    pub fn emit_bitcode(&self, file_path: &str) {
        let bitcode_path = Path::new(file_path);
        self.module.write_bitcode_to_path(&bitcode_path);
    }

    pub fn emit_ir(&self, file_path: &str) {
        let ir_path = Path::new(file_path);
        if let Err(_) = self.module.print_to_file(ir_path) {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interop::{ClassicalRegister, Controlled, Instruction, QuantumRegister, Single};

    use super::*;
    #[test]
    fn bernstein_vazirani() {
        let name = String::from("Bernstein-Vazirani circuit");
        let mut model = SemanticModel::new(name);
        model.add_reg(QuantumRegister::new(String::from("input_"), 0).as_register());
        model.add_reg(QuantumRegister::new(String::from("input_"), 1).as_register());
        model.add_reg(QuantumRegister::new(String::from("input_"), 2).as_register());
        model.add_reg(QuantumRegister::new(String::from("input_"), 3).as_register());
        model.add_reg(QuantumRegister::new(String::from("input_"), 4).as_register());
        model.add_reg(QuantumRegister::new(String::from("target_"), 0).as_register());
        model.add_reg(ClassicalRegister::new(String::from("output_"), 5).as_register());

        model.add_inst(Instruction::X(Single::new(String::from("target_0"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_0"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_1"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_2"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_3"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_4"))));
        model.add_inst(Instruction::H(Single::new(String::from("target_0"))));

        // random chosen
        model.add_inst(Instruction::Cx(Controlled::new(
            String::from("input_2"),
            String::from("target_0"),
        )));
        model.add_inst(Instruction::Cx(Controlled::new(
            String::from("input_2"),
            String::from("target_0"),
        )));
        model.add_inst(Instruction::H(Single::new(String::from("input_0"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_1"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_2"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_3"))));
        model.add_inst(Instruction::H(Single::new(String::from("input_4"))));
        model.add_inst(Instruction::M {
            qubit: String::from("input_0"),
            target: String::from("output_0"),
        });
        model.add_inst(Instruction::M {
            qubit: String::from("input_1"),
            target: String::from("output_1"),
        });
        model.add_inst(Instruction::M {
            qubit: String::from("input_2"),
            target: String::from("output_2"),
        });
        model.add_inst(Instruction::M {
            qubit: String::from("input_3"),
            target: String::from("output_3"),
        });
        model.add_inst(Instruction::M {
            qubit: String::from("input_4"),
            target: String::from("output_4"),
        });
        model.add_inst(Instruction::Reset(Single::new(String::from("input_0"))));
        model.add_inst(Instruction::Reset(Single::new(String::from("input_1"))));
        model.add_inst(Instruction::Reset(Single::new(String::from("input_2"))));
        model.add_inst(Instruction::Reset(Single::new(String::from("input_3"))));
        model.add_inst(Instruction::Reset(Single::new(String::from("input_4"))));
        Emitter::write(&model, "BernsteinVazirani.ll");
    }
    #[test]
    fn bell_measure() {
        let name = String::from("Bell circuit");
        let mut model = SemanticModel::new(name);
        model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
        model.add_reg(QuantumRegister::new(String::from("qr"), 1).as_register());
        model.add_reg(ClassicalRegister::new(String::from("qc"), 2).as_register());

        model.add_inst(Instruction::H(Single::new(String::from("qr0"))));
        model.add_inst(Instruction::Cx(Controlled::new(String::from("qr0"), String::from("qr1"))));
        model.add_inst(Instruction::M {
            qubit: String::from("qr0"),
            target: String::from("qc0"),
        });
        model.add_inst(Instruction::M {
            qubit: String::from("qr1"),
            target: String::from("qc1"),
        });
        Emitter::write(&model, "bell_measure.ll");
    }

    #[test]
    fn bell_no_measure() {
        let name = String::from("Bell circuit");
        let mut model = SemanticModel::new(name);
        model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
        model.add_reg(QuantumRegister::new(String::from("qr"), 1).as_register());
        model.add_reg(ClassicalRegister::new(String::from("qc"), 2).as_register());

        model.add_inst(Instruction::H(Single::new(String::from("qr0"))));
        model.add_inst(Instruction::Cx(Controlled::new(String::from("qr0"), String::from("qr1"))));
        Emitter::write(&model, "bell_no_measure.ll");
    }
}