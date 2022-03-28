#![deny(rust_2018_idioms)]

mod impl_parse;

use std::collections::HashMap;
use std::fmt;

pub use rtcore::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryFile {
    ar_size: usize,
    dr_size: usize,
    data: HashMap<Value, Value>,
}

impl MemoryFile {
    /// All keys in `data` should have `Value::size() <= ar_size`.
    ///
    /// All values in `data` should have `Value::size() <= dr_size`.
    pub fn new(ar_size: usize, dr_size: usize, data: HashMap<Value, Value>) -> Result<Self, ()> {
        // Check and extend zero
        let data = data
            .into_iter()
            .map(|(mut addr, mut value)| {
                if addr.size() > ar_size {
                    return Err(());
                }
                addr.extend_zero(ar_size);

                if value.size() > dr_size {
                    return Err(());
                }
                value.extend_zero(dr_size);

                Ok((addr, value))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { ar_size, dr_size, data })
    }

    pub fn empty(ar_size: usize, dr_size: usize) -> Self {
        Self { ar_size, dr_size, data: HashMap::new() }
    }

    pub fn ar_size(&self) -> usize {
        self.ar_size
    }

    pub fn dr_size(&self) -> usize {
        self.dr_size
    }

    /// All keys are guaranteed to have `Value::size() == ar_size`.
    ///
    /// All values are guaranteed to have `Value::size() == dr_size`.
    pub fn data(&self) -> &HashMap<Value, Value> {
        &self.data
    }

    /// All keys are guaranteed to have `Value::size() == ar_size`.
    ///
    /// All values are guaranteed to have `Value::size() == dr_size`.
    pub fn into_data(self) -> HashMap<Value, Value> {
        self.data
    }
}

impl MemoryFile {
    pub fn parse(source: &str) -> Result<Self, ()> {
        self::impl_parse::parse(source)
            .or_else(|e| self::impl_parse::parse_deprecated(source).map_err(|_| e))
    }
}

impl fmt::Display for MemoryFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write header
        writeln!(f, "H {} {}", self.ar_size, self.dr_size)?;
        if !self.data.is_empty() {
            write!(f, "\n")?;
        }

        // Sort data (by address ASC)
        let mut data = self.data.iter().collect::<Vec<_>>();
        data.sort_by(|a, b| a.0.cmp(b.0));

        // Write data
        let mut current_address = Value::zero(self.ar_size);
        for (address, value) in data {
            if *address != current_address {
                writeln!(f, "\n{}:", address.as_hex())?;
            }
            writeln!(f, "{}", value.as_hex())?;

            current_address = address + Value::one(self.ar_size);
        }

        Ok(())
    }
}
