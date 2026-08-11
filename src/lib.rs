#![feature(const_option_ops, const_trait_impl)]

use std::env::current_exe;

use pyo3::ffi;
use pyo3::prelude::*;

mod windows;

pub fn main(module: &str, func: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    windows::link_python();

    let exe = current_exe()?
        .to_string_lossy()
        .into_owned()
        .replace(".com", ".exe")
        .replace(".scr", ".exe");
    let mut sys_argv = vec![exe];
    sys_argv.extend(std::env::args().skip(1));

    unsafe {
        std::env::set_var("PYTHONUTF8", "1");
        ffi::Py_Initialize();
    }

    Python::attach(|py| {
        // Initialize sys.argv & sys.path
        let sys = py.import("sys")?;
        let sys_exit = sys.getattr("exit")?;

        sys.setattr("argv", sys_argv)?;

        #[cfg(debug_assertions)]
        {
            let sys_path = sys.getattr("path")?.extract::<Vec<String>>()?;
            eprintln!("sys.path = {sys_path:#?}");

            let sys_flags = sys
                .getattr("flags")?
                .call_method0("__str__")?
                .extract::<String>()?;
            eprintln!("sys.flags = {sys_flags:#?}");
        }

        // Locate module and execute
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
