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

//! A device that says hello.

#![cfg_attr(not(test), no_std)]

mod blink;

pub use blink::BlinkTask;

use noma_platform::{led::Led, time::Timer};

/// The Hello app.
#[derive(Debug)]
pub struct HelloApp<L: Led, T: Timer> {
    /// The Blink task.
    pub blink: BlinkTask<L, T>,
}

impl<L: Led, T: Timer> HelloApp<L, T> {
    /// Instantiates a new Hello app.
    pub fn new(led: L, timer: T) -> Self {
        Self {
            blink: BlinkTask::new(led, timer),
        }
    }
}
