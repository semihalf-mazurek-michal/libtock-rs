#![no_std]

use libtock_low_level_debug::{AlertCode, LowLevelDebug};
use libtock_platform::Syscalls;
use libtock_runtime::TockSyscalls;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // Signal a panic using the LowLevelDebug capsule (if available).
    LowLevelDebug::<TockSyscalls>::print_alert_code(AlertCode::Panic);

    // Exit with a non-zero exit code to indicate failure.
    TockSyscalls::exit_terminate(1);
}
