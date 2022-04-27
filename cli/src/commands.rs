use anyhow::{anyhow, bail, Context, Result};
use compiler::PrettyPrintError;
use memory_file::MemoryFile;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn check(file: PathBuf, ansi_colors: bool) -> Result<()> {
    let (source, file_name) = read_file(&file)?;

    let ast = match parser::parse(&source) {
        Ok(ast) => ast,
        Err(e) => bail!(parser::pretty_print_error(&e, &source, file_name, ansi_colors)),
    };
    match compiler::check(ast, &Default::default()) {
        Ok(()) => (),
        Err(e) => bail!(e.pretty_print(&source, file_name, ansi_colors)),
    };

    Ok(())
}

pub fn test(file: PathBuf, test_file: PathBuf, ansi_colors: bool) -> Result<()> {
    // Build rt file
    let program = {
        let (source, file_name) = read_file(&file)?;

        let ast = match parser::parse(&source) {
            Ok(ast) => ast,
            Err(e) => bail!(parser::pretty_print_error(&e, &source, file_name, ansi_colors)),
        };

        let backend = compiler_backend_simulator::BackendSimulator;
        match compiler::compile(&backend, (), ast, &Default::default()) {
            Ok(program) => program,
            Err(e) => bail!(e.pretty_print(&source, file_name, ansi_colors)),
        }
    };

    // Parse test file
    let unit_test = {
        let (source, file_name) = read_file(&test_file)?;

        match unit_test::parser::parse(&source) {
            Ok(unit_test) => unit_test,
            Err(e) => {
                bail!(unit_test::parser::pretty_print_error(&e, &source, file_name, ansi_colors))
            }
        }
    };

    // Run unit test
    unit_test::run(program, unit_test).context("Tests failed")?;

    Ok(())
}

pub fn gen_vhdl(
    rt_file: PathBuf,
    vhdl_file: PathBuf,
    module_name: Option<String>,
    memories: Option<Vec<String>>,
    ansi_colors: bool,
) -> Result<()> {
    // Build the vhdl
    let vhdl = {
        let (source, file_name) = read_file(&rt_file)?;

        let ast = match parser::parse(&source) {
            Ok(ast) => ast,
            Err(e) => bail!(parser::pretty_print_error(&e, &source, file_name, ansi_colors)),
        };

        let backend = compiler_backend_vhdl::BackendVhdl;
        match compiler::compile(&backend, (), ast, &Default::default()) {
            Ok(vhdl) => vhdl,
            Err(e) => bail!(e.pretty_print(&source, file_name, ansi_colors)),
        }
    };

    // Module name
    let module_name = match &module_name {
        Some(module_name) => &module_name,
        None => vhdl_file
            .file_stem()
            .with_context(|| format!("Expected file name: {}", vhdl_file.display()))?
            .to_str()
            .with_context(|| format!("Utf8 error: {}", vhdl_file.display()))?,
    };

    // Memories
    let memories = match memories {
        Some(args) => {
            let mut memories = HashMap::new();
            let mut args = args.into_iter();

            while let Some(mem_name) = args.next() {
                let mem_path = args
                    .next()
                    .with_context(|| format!("Expected file for memory {}", mem_name))?;
                let (mem_source, _) = read_file(mem_path.as_ref())?;
                let mem_file = MemoryFile::parse(&mem_source).map_err(|()| {
                    anyhow!("Failed to parsed memory {} from {}", mem_name, mem_path)
                })?;

                memories.insert(vhdl::Ident(mem_name), mem_file);
            }

            memories
        }
        None => HashMap::new(),
    };

    // Render
    let rendered = vhdl.render(module_name, memories).context("Failed to render")?;

    // Save to file
    fs::write(&vhdl_file, rendered)
        .with_context(|| format!("Failed to write to {}", vhdl_file.display()))?;

    Ok(())
}

fn read_file(file: &Path) -> Result<(String, Option<&str>)> {
    let source = fs::read_to_string(&file)
        .with_context(|| format!("Failed to read from {}", file.display()))?;
    let file_name = file.file_name().and_then(OsStr::to_str);
    Ok((source, file_name))
}
