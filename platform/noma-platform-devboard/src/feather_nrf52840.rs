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

//! Feather nRF52840 Express board.

pub use embassy_nrf as hal;
pub use hal::config::Config;

use hal::gpio::{Level, Output, OutputDrive};

/// A LED.
pub type Led = Output<'static>;

/// Peripherals of the Feather nRF52840 Express.
#[expect(missing_debug_implementations, reason = "not implemented by fields")]
pub struct Board {
    /// The blue LED.
    pub blue_led: Led,
    /// The red LED.
    pub red_led: Led,
}

impl Board {
    /// Initialises the board.
    pub fn init(config: Config) -> Self {
        let p = hal::init(config);

        Self {
            blue_led: Output::new(p.P1_10, Level::Low, OutputDrive::Standard),
            red_led: Output::new(p.P1_15, Level::Low, OutputDrive::Standard),
        }
    }
}
