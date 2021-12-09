use winapi::shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

mod plugin;

#[no_mangle]
pub extern "system" fn DllMain(_instance: HMODULE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => plugin::init(),
        _ => {}
    }
    TRUE
}
