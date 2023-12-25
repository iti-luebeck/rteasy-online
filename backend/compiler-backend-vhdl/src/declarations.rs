use crate::gen_ident;
use compiler::mir;
use vhdl::*;

pub fn generate_declarations<'s>(mir_declarations: &[mir::Declaration<'s>]) -> Declarations {
    let mut declarations = Declarations {
        registers: Vec::new(),
        buses: Vec::new(),
        memories: Vec::new(),
        register_arrays: Vec::new(),
    };

    for declaration in mir_declarations {
        match declaration {
            mir::Declaration::Register(declaration) => {
                for register in &declaration.registers {
                    declarations.registers.push((
                        gen_ident(register.ident),
                        generate_bit_range(register.range),
                        register.kind,
                    ));
                }
            }
            mir::Declaration::Bus(declaration) => {
                for bus in &declaration.buses {
                    declarations.buses.push((
                        gen_ident(bus.ident),
                        generate_bit_range(bus.range),
                        bus.kind,
                    ));
                }
            }
            mir::Declaration::Memory(declaration) => {
                for memory in &declaration.memories {
                    let (ar_name, ar_range, ar_kind) = declarations
                        .registers
                        .iter()
                        .find(|(name, _, _)| name.0 == memory.range.address_register.0)
                        .unwrap();
                    let (dr_name, dr_range, dr_kind) = declarations
                        .registers
                        .iter()
                        .find(|(name, _, _)| name.0 == memory.range.data_register.0)
                        .unwrap();

                    declarations.memories.push((
                        gen_ident(memory.ident),
                        (ar_name.clone(), *ar_range, *ar_kind),
                        (dr_name.clone(), *dr_range, *dr_kind),
                    ));
                }
            }
            mir::Declaration::RegisterArray(declaration) => {
                for register_array in &declaration.register_arrays {
                    declarations.register_arrays.push((
                        gen_ident(register_array.ident),
                        generate_bit_range(register_array.range),
                        register_array.len,
                    ));
                }
            }
        }
    }

    declarations
}

fn generate_bit_range(range: Option<mir::BitRange>) -> BitRange {
    match range {
        Some(range) => {
            if range.is_downto() {
                BitRange::Downto(range.0, range.1)
            } else {
                BitRange::To(range.0, range.1)
            }
        }
        None => BitRange::Downto(0, 0),
    }
}
