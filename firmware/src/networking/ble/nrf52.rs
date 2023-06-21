use defmt::{debug, error};
use embassy_futures::yield_now;
use embassy_time::Duration;
use nrf_softdevice::{raw, Softdevice};

use core::mem;

use crate::{networking::protocol::Packets, aliases::ඞ::NetConcrete};

pub async fn network_task(_packets: &Packets, _net: NetConcrete) -> ! {
	debug!("Starting BLE loop");
	loop {
		yield_now().await;
	}
}

#[embassy_executor::task]
pub async fn softdevice_task() -> ! {
	error!("softdevice task");
	embassy_time::Timer::after(Duration::from_millis(1000)).await;

	#[cfg(any(feature = "nrf-boot-s140", feature = "nrf-boot-s132"))]
	let sd = crate::networking::ble::ඞ::init_softdevice().await;
	error!("after init");

	embassy_time::Timer::after(Duration::from_millis(1000)).await;
	sd.run().await
}

pub async fn init_softdevice() -> &'static mut Softdevice {
	let config = nrf_softdevice::Config::default();
	// let config = nrf_softdevice::Config {
    //     clock: Some(raw::nrf_clock_lf_cfg_t {
    //         source: raw::NRF_CLOCK_LF_SRC_RC as u8,
    //         rc_ctiv: 16,
    //         rc_temp_ctiv: 2,
    //         accuracy: raw::NRF_CLOCK_LF_ACCURACY_250_PPM as u8,
    //     }),

    //     ..Default::default()
    // };

	error!("after config");

	embassy_time::Timer::after(Duration::from_millis(1000)).await;
	let sd = Softdevice::enable(&config);
	error!("enabled softdevice");
	sd
}
