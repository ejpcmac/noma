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

//! A task that blinks a LED.

use noma_platform::{led::Led, time::Timer};

/// The Blink task.
#[derive(Debug)]
pub struct BlinkTask<L: Led, T: Timer> {
    /// The LED to blink.
    led: L,
    /// The timer.
    timer: T,
}

impl<L: Led, T: Timer> BlinkTask<L, T> {
    /// Creates a new Blink task.
    pub(super) fn new(led: L, timer: T) -> Self {
        Self { led, timer }
    }

    /// Runs the Blink task.
    pub async fn run(&mut self) {
        defmt::info!("Starting the blink task");

        #[cfg_attr(
            test,
            expect(clippy::never_loop, reason = "not looping in tests")
        )]
        #[cfg_attr(
            not(test),
            expect(clippy::infinite_loop, reason = "the task runs forever")
        )]
        loop {
            self.led.on();
            defmt::debug!("LED on");
            self.timer.after_ms(500).await;

            self.led.off();
            defmt::debug!("LED off");
            self.timer.after_ms(500).await;

            #[cfg(test)]
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use mockall::{Sequence, predicate::eq};

    use noma_platform::{led::MockLed, time::MockTimer};

    use super::*;

    #[pollster::test]
    async fn blinks_the_led() {
        let mut seq = Sequence::new();
        let mut led = MockLed::new();
        let mut timer = MockTimer::new();

        led.expect_on()
            .once()
            .in_sequence(&mut seq)
            .return_const(());

        timer
            .expect_after_ms()
            .once()
            .with(eq(500))
            .in_sequence(&mut seq)
            .return_const(());

        led.expect_off()
            .once()
            .in_sequence(&mut seq)
            .return_const(());

        timer
            .expect_after_ms()
            .once()
            .with(eq(500))
            .in_sequence(&mut seq)
            .return_const(());

        BlinkTask::new(led, timer).run().await;
    }
}
