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

//! Nucleo-L476RG board support.

pub use embassy_stm32 as hal;
pub use hal::Config;

use hal::gpio::{Level, Output, Speed};

/// A LED.
pub type Led = Output<'static>;

/// Peripherals of the Nucleo-L476RG
#[expect(missing_debug_implementations, reason = "not implemented by fields")]
pub struct Board {
    /// The LD2 LED.
    pub ld2: Led,
}

impl Board {
    /// Initialises the board.
    pub fn init(config: Config) -> Self {
        let p = hal::init(config);

        Self {
            ld2: Output::new(p.PA5, Level::Low, Speed::Low),
        }
    }
}
