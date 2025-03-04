// noma - Devices for nomads.
// Copyright (C) 2025 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The firmware integrating the Noma Hello app.

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_time::Delay;

use noma_hello_app::{BlinkTask, HelloApp};
use noma_platform_devboard::{Board, Config, Led};

/// The name and version of the firmware.
const FW_INFO: &str =
    concat!(env!("CARGO_PKG_NAME"), " ", env!("VERSION_WITH_GIT"));

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Starting {}...", FW_INFO);

    let board = Board::init(Config::default());

    #[cfg(feature = "feather-nrf52840")]
    let led = board.red_led;
    #[cfg(feature = "nucleo-l476rg")]
    let led = board.ld2;

    let app = HelloApp::new(led, Delay);

    defmt::expect!(
        spawner.spawn(blink(app.blink)),
        "failed to spawn the blink task"
    );

    defmt::info!("Firmware initialised!");
}

#[embassy_executor::task]
async fn blink(mut task: BlinkTask<Led, Delay>) {
    task.run().await;
}
