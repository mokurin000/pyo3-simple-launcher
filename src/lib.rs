#![feature(const_option_ops, const_trait_impl)]

use std::env::{current_dir, current_exe};

use pyo3::prelude::*;

mod windows;

pub const PYTHON_SYS_PATH: &str = option_env!("PSL_PYTHON_SYS_PATH").unwrap_or(
"./python-embed-amd64;./python-embed-amd64/python314.zip;./python-embed-amd64/Lib/site-packages");

pub fn main(entry_point: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    windows::initialize();

    unsafe { std::env::set_var("PYTHONUTF8", "1") };

    let exe = current_exe()?
        .to_string_lossy()
        .into_owned()
        .replace(".exe", "");
    let mut sys_argv = vec![exe];
    sys_argv.extend(std::env::args().skip(1));

    let current_dir = current_dir()?;
    let sys_path: Vec<_> = PYTHON_SYS_PATH
        .split(";")
        .map(|subpath| current_dir.join(subpath).to_string_lossy().to_string())
        .collect();

    Python::initialize();

    Python::attach(|py| {
        // Initialize sys.argv & sys.path
        let sys = py.import("sys")?;
        let sys_exit = sys.getattr("exit")?;

        sys.setattr("argv", sys_argv)?;
        sys.setattr("path", sys_path)?;

        // Locate module and execute
        let (module, func) = entry_point.split_once(":").expect("Invalid EntryPoint!");
        let gui = py.import(module)?;
        let func = gui.getattr(func)?;

        let exit_code = func.call0()?;

        let result = sys_exit.call1((exit_code,)).unwrap_err();
        let exit_code = result
            .value(py)
            .getattr("code")?
            .extract::<i32>()
            .unwrap_or_default();
        std::process::exit(exit_code)
    })
}
