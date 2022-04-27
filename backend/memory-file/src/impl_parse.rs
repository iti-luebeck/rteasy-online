use crate::MemoryFile;
pub use rtcore::value::Value;
use std::collections::HashMap;

pub fn parse(source: &str) -> Result<MemoryFile, ()> {
    // Split to lines
    let mut lines = source.lines().map(|line| {
        // Remove comment
        let line = match line.split_once('#') {
            Some((line, _comment)) => line,
            None => line,
        };

        // Trim
        line.trim()
    });

    // Parse header
    let header = lines.next().ok_or(())?;
    let mut parts = header.split(' ');
    let parse_fn = match parts.next() {
        Some("B") | Some("b") => Value::parse_bin,
        Some("H") | Some("h") => Value::parse_hex,
        _ => return Err(()),
    };
    let ar_size = match parts.next() {
        Some(ar_size) => ar_size.parse().map_err(|_| ())?,
        None => return Err(()),
    };
    let dr_size = match parts.next() {
        Some(dr_size) => dr_size.parse().map_err(|_| ())?,
        None => return Err(()),
    };

    // Parse data
    let mut current_address = Value::zero(ar_size);
    let mut data = HashMap::new();
    for line in lines {
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Parse as address or data
        if line.ends_with(':') {
            let mut v = parse_fn(&line[0..line.len() - 1]).map_err(|_| ())?;
            if v.size() > ar_size {
                return Err(());
            }
            v.extend_zero(ar_size);

            current_address = v;
        } else {
            let mut v = parse_fn(line).map_err(|_| ())?;
            if v.size() > dr_size {
                return Err(());
            }
            v.extend_zero(dr_size);

            data.insert(current_address.clone(), v);
            current_address = current_address + Value::one(ar_size);
        }
    }

    // Memory file
    Ok(MemoryFile { ar_size, dr_size, data })
}

// Keep this for compatibility with older versions of memory saves
pub fn parse_deprecated(source: &str) -> Result<MemoryFile, ()> {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct MemorySave {
        ar_size: usize,
        dr_size: usize,
        data: Vec<(String, String)>,
    }

    let save = serde_json::from_str::<MemorySave>(source).map_err(|_| ())?;

    MemoryFile::new(
        save.ar_size,
        save.dr_size,
        save.data
            .into_iter()
            .map(|(addr, value)| {
                let (mut addr, mut value) = (Value::parse_hex(&addr)?, Value::parse_hex(&value)?);

                if addr.size() > save.ar_size {
                    return Err(());
                }
                addr.extend_zero(save.ar_size);

                if value.size() > save.dr_size {
                    return Err(());
                }
                value.extend_zero(save.dr_size);

                Ok((addr, value))
            })
            .collect::<Result<_, _>>()?,
    )
}
