#![no_std]

use atomic_polyfill::{AtomicBool, Ordering};

use defmt::{error, flush};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	static PANICKED: AtomicBool = AtomicBool::new(false);

	// TODO: What `Ordering` should this use?
	if !PANICKED.swap(true, Ordering::SeqCst) {
		if let Some(location) = info.location() {
			let (file, line, column) =
				(location.file(), location.line(), location.column());
			error!(
				"A panic occured in '{}', at line {}, column {}",
				file, line, column
			);
		} else {
			error!("A panic occured at an unknown location");
		}
	}
	error!("{:#?}", defmt::Debug2Format(info));
	flush();
	loop {}
}
