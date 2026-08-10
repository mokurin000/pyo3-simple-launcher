use std::ffi::c_void;

const PYTHON_DLL_PATH: &str =
    option_env!("PSL_PYTHON_DLL_PATH").unwrap_or("./python-embed-amd64/python314.dll");

windows_link::link!(
    "kernel32.dll" "system"
    fn LoadLibraryW(
        lpLibFileName: *const u16,
    ) -> *mut c_void
);

windows_link::link!(
    "kernel32.dll" "system"
    fn AttachConsole(
        dwProcessId: i32,
    ) -> i32
);

pub fn initialize() {
    let path: Vec<u16> = PYTHON_DLL_PATH.encode_utf16().collect();

    unsafe {
        AttachConsole(-1);

        let handle = LoadLibraryW(path.as_ptr());

        if handle.is_null() {
            panic!("LoadLibraryW failed");
        }
    }
}
