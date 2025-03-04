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

//! Board support crate for development boards.

#![no_std]

#[cfg(not(any(feature = "feather-nrf52840", feature = "nucleo-l476rg")))]
compile_error!(
    "No board feature is enabled. Exactly one of the following features must be enabled:

    - feather-nrf52840
    - nucleo-l476rg
    "
);

#[cfg(feature = "feather-nrf52840")]
mod feather_nrf52840;
#[cfg(feature = "nucleo-l476rg")]
mod nucleo_l476rg;

#[cfg(feature = "feather-nrf52840")]
pub use feather_nrf52840::*;
#[cfg(feature = "nucleo-l476rg")]
pub use nucleo_l476rg::*;
