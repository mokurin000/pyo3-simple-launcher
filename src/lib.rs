use std::env::current_exe;

use pyo3::exceptions::PySystemExit;
use pyo3::ffi;
use pyo3::prelude::*;

pub fn main(module: &str, func: &str) -> Result<(), Box<dyn std::error::Error>> {
    let exe = current_exe()?
        .with_extension("")
        .to_string_lossy()
        .into_owned();
    let mut sys_argv = vec![exe];
    sys_argv.extend(std::env::args().skip(1));

    unsafe {
        std::env::set_var("PYTHONUTF8", "1");
        ffi::Py_Initialize();
    }

    Python::attach(|py| {
        // Initialize sys.argv & sys.path
        let sys = py.import("sys")?;

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

        match func.call0() {
            Ok(_) => Ok(()),
            Err(e) if e.is_instance_of::<PySystemExit>(py) => {
                let code = e
                    .value(py)
                    .getattr("code")?
                    .extract::<i32>()
                    .unwrap_or_default();
                std::process::exit(code)
            }
            Err(e) => {
                e.print(py);
                std::process::exit(1)
            }
        }
    })
}
