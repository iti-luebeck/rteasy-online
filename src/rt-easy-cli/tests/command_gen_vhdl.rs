use rt_easy_cli::{Command, Opt};
use std::path::PathBuf;

#[test]
fn test() {
    let vhdl_file = tempfile::NamedTempFile::new().unwrap();

    let opt = Opt {
        no_ansi: true,
        command: Command::GenVhdl {
            rt_file: file("mem.rt"),
            vhdl_file: vhdl_file.path().to_owned(),
            module_name: Some("my_mem_module".to_string()),
            memories: Some(vec!["MEM".to_string(), file("MEM.rtmem").to_str().unwrap().to_owned()]),
        },
    };
    rt_easy_cli::run(opt).unwrap();

    let rendered = std::fs::read_to_string(vhdl_file.path()).unwrap();

    assert!(rendered.contains("my_mem_module"));
    assert!(rendered.contains("\"10000001\""));
    assert!(rendered.contains("\"11010101\""));
    assert!(rendered.contains("\"00000001\""));
    assert!(rendered.contains("\"00000010\""));
    assert!(rendered.contains("\"00000100\""));
}

fn file(name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", name].iter().collect()
}
