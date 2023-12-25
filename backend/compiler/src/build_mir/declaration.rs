use super::{expression::BuildExpr, Result};
use crate::mir::*;
use crate::{symbols::Symbols, InternalError};

pub fn build<'s>(
    declaration: ast::Declaration<'s>,
    symbols: &Symbols<'s>,
) -> Result<Declaration<'s>> {
    match declaration {
        ast::Declaration::Register(declare_register) => {
            Ok(Declaration::Register(DeclareRegister {
                registers: declare_register
                    .registers
                    .into_iter()
                    .map(|reg| Ok(Register::build(reg, symbols)?.inner))
                    .collect::<Result<_>>()?,
            }))
        }
        ast::Declaration::Bus(declare_bus) => Ok(Declaration::Bus(DeclareBus {
            buses: declare_bus
                .buses
                .into_iter()
                .map(|bus| Ok(Bus::build(bus, symbols)?.inner))
                .collect::<Result<_>>()?,
        })),
        ast::Declaration::Alias(_) => Err(InternalError(
            "Alias are not needed and should be filtered out from the list.".to_string(),
        )),
        ast::Declaration::Memory(declare_memory) => Ok(Declaration::Memory(DeclareMemory {
            memories: declare_memory
                .memories
                .into_iter()
                .map(|mem| Memory {
                    ident: mem.ident.node,
                    range: MemoryRange {
                        address_register: mem.range.address_register.node,
                        address_range: mem.range.address_range.map(|s| s.node),
                        data_register: mem.range.data_register.node,
                    },
                })
                .collect(),
        })),
        ast::Declaration::RegisterArray(declare_reg_array) => {
            Ok(Declaration::RegisterArray(DeclareRegisterArray {
                register_arrays: declare_reg_array
                    .register_arrays
                    .into_iter()
                    .map(|declare_register_array_item| DeclareRegisterArrayItem {
                        ident: declare_register_array_item.ident.node,
                        range: declare_register_array_item.range.map(|s| s.node),
                        len: declare_register_array_item.len,
                    })
                    .collect(),
                span: declare_reg_array.span,
            }))
        }
    }
}
