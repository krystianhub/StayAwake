mod config;
mod offset_generator;
mod power;

use crate::{config::Config, offset_generator::OffsetGenerator};
use anyhow::Result;
use dotenv::dotenv;
use mouse_rs::Mouse;
use tokio::time;
use tracing::{debug, error, info, trace};
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv().ok();

    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("INFO"))?;
    tracing_subscriber::fmt()
        .with_env_filter(filter_layer)
        .init();

    info!(concat!(
        "Initializing StayAwake (",
        env!("CARGO_PKG_VERSION"),
        ")"
    ));

    let config = envy::from_env::<Config>()?;
    debug!(?config);

    let mut offset_gen = OffsetGenerator::new(&config);

    let mouse = Mouse::new();
    let mut interval = time::interval(config.stayawake_interval);

    let get_pos_err = |err| error!(error = ?err, "Cannot get mouse position");

    info!("Initialization finished successfully");

    interval.tick().await; // Initial tick is instant

    // Create Power Manager lock
    let lock = power::lock();
    trace!(result = ?lock, "Inhibiting Power Management");

    loop {
        trace!("Loop start");

        // Grab mouse position
        let pos1 = match mouse.get_position() {
            Ok(pos) => pos,
            Err(err) => {
                get_pos_err(err);
                continue;
            }
        };

        // Wait
        trace!("Tick started");
        interval.tick().await;
        trace!("Tick completed");

        // Measure mouse position again
        let pos2 = match mouse.get_position() {
            Ok(pos) => pos,
            Err(err) => {
                get_pos_err(err);
                continue;
            }
        };

        trace!(?pos1, ?pos2, "Interval position results");

        // If position didn't change during the last interval, move the mouse now
        if pos1.x == pos2.x && pos1.y == pos2.y {
            let new_pos = offset_gen.get_random_offset_position(&pos1);

            trace!(
                "Movement not detected, moving mouse from {:?} to {:?}",
                &pos1,
                &new_pos
            );

            if let Err(err) = mouse.move_to(new_pos.x as i32, new_pos.y as i32) {
                error!(error = ?err, "Cannot move the mouse to a new position");
            }
        } else {
            trace!("Movement detected, not moving mouse");
        }
    }
}
