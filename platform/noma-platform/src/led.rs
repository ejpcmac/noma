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

//! LED abstraction.

/// A LED.
#[cfg_attr(feature = "test", mockall::automock)]
pub trait Led {
    /// Sets the LED on.
    fn on(&mut self);
    /// Sets the LED off.
    fn off(&mut self);
}

#[cfg(feature = "embedded-hal")]
impl<P> Led for P
where
    P: embedded_hal::digital::OutputPin<Error = core::convert::Infallible>,
{
    #[inline]
    fn on(&mut self) {
        self.set_high().unwrap();
    }

    #[inline]
    fn off(&mut self) {
        self.set_low().unwrap();
    }
}
