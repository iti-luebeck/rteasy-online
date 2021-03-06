use super::util::slice_idx;
use crate::Error;
use anyhow::anyhow;
use rtcore::{common::BitRange, value::Value};
use std::cell::RefCell;
use std::collections::HashMap;

const REGISTER_ARRAY_PAGE_SIZE: usize = 32;

#[derive(Debug)]
pub struct RegisterArrayState {
    data: HashMap<Value, Value>,
    data_next: RefCell<Option<(Value, Value)>>,
    range: BitRange,
    len: usize,
}

impl RegisterArrayState {
    pub fn init(range: Option<BitRange>, len: usize) -> Self {
        let range = range.unwrap_or_default();
        Self { data: HashMap::new(), data_next: RefCell::new(None), len, range }
    }

    pub fn value_next(&self) -> Option<(usize, Value)> {
        let (idx, value) = self.data_next.borrow().clone()?;
        Some((usize::from_str_radix(&idx.as_bin(false), 2).unwrap(), value))
    }

    pub fn read(&self, idx: Value, range: Option<BitRange>) -> Result<Value, Error> {
        // Check idx
        if idx.size() > self.index_size() {
            return Err(anyhow!("index too big"));
        }

        // Get value by idx
        let value = self.data.get(&idx).cloned().unwrap_or_else(|| Value::zero(self.data_size()));

        // Select bits
        match range {
            Some(range) => {
                let slice_idx = slice_idx(self.range, range)?;
                Ok(value[slice_idx].to_owned())
            }
            None => Ok(value),
        }
    }

    pub fn write(&self, idx: Value, value: Value) -> Result<(), Error> {
        // Check idx and value
        if idx.size() > self.index_size() {
            return Err(anyhow!("index too big"));
        }
        if value.size() > self.data_size() {
            return Err(anyhow!("value too big"));
        }

        *self.data_next.borrow_mut() = Some((idx, value));
        Ok(())
    }

    pub fn clock(&mut self) -> Option<Value> {
        match self.data_next.get_mut().take() {
            Some((idx, value)) => {
                self.data.insert(idx.clone(), value);
                Some(idx)
            }
            None => None,
        }
    }

    pub fn page_count(&self) -> usize {
        (self.len - 1) / REGISTER_ARRAY_PAGE_SIZE + 1
    }
    pub fn page(&self, page_nr: usize) -> Vec<(usize, Value)> {
        // Check in range (1..=page_count)
        if page_nr < 1 || page_nr > self.page_count() {
            return Vec::new();
        }

        // Since we want to return the indices as usize, but the indices are stored as Values,
        // we have to calculate both idx_as_usize and idx_as_value.

        // Calc idx
        let mut idx_as_usize = (page_nr - 1) * REGISTER_ARRAY_PAGE_SIZE;
        let mut idx_as_value =
            Value::parse_bin(&format!("{:b}", idx_as_usize)).unwrap().with_size(self.index_size());

        // Get register values
        let mut result = Vec::new();
        for _ in 0..REGISTER_ARRAY_PAGE_SIZE {
            // Calc next idx
            let idx_as_usize_next = idx_as_usize.wrapping_add(1);
            let idx_as_value_next = &idx_as_value + Value::one(1);

            // Get value
            let value = self
                .data
                .get(&idx_as_value)
                .cloned()
                .unwrap_or_else(|| Value::zero(self.data_size()));
            result.push((idx_as_usize, value));

            // Update idx
            idx_as_usize = idx_as_usize_next;
            idx_as_value = idx_as_value_next;

            // Break on overflow
            if idx_as_value.is_zero() {
                break;
            }
        }
        result
    }

    pub fn index_size(&self) -> usize {
        log_2(self.len)
    }

    pub fn data_size(&self) -> usize {
        self.range.size().unwrap()
    }
}

// impl fmt::Display for RegisterArrayState {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut indexes = self.data.keys().collect::<Vec<_>>();
//         indexes.sort();
//
//         write!(f, "[\n")?;
//         for idx in indexes {
//             write!(f, "  {} = {}\n", idx.as_dec(), self.data.get(idx).unwrap().as_dec())?;
//         }
//         write!(f, "]")?;
//
//         Ok(())
//     }
// }

fn log_2(x: usize) -> usize {
    const fn num_bits<T>() -> usize {
        std::mem::size_of::<T>() * 8
    }

    if x == 0 {
        0
    } else {
        num_bits::<usize>() - x.leading_zeros() as usize - 1
    }
}
