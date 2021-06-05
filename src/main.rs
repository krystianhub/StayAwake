use std::{env, time::Duration};

use anyhow::{Context, Result};
use dotenv::dotenv;
use mouse_rs::Mouse;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use tokio::time;
use tracing::{debug, error, info, trace};

const STAYAWAKE_INTERVAL_NAME: &str = "STAYAWAKE_INTERVAL";
const DEFAULT_INTERVAL: u64 = 60;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Initializing StayAwake application");

    let mut rng = thread_rng();
    let range: Uniform<usize> = Uniform::new_inclusive(2, 5);

    let stayawake_interval: Duration = env::var(STAYAWAKE_INTERVAL_NAME)
        .map(|interval_env| interval_env.parse::<u64>().map(Duration::from_secs))
        .unwrap_or_else(|_| Ok(Duration::from_secs(DEFAULT_INTERVAL)))
        .context(format!("Cannot parse {} value", STAYAWAKE_INTERVAL_NAME))?;

    debug!(value = ?stayawake_interval, "{}", &STAYAWAKE_INTERVAL_NAME);

    let mouse = Mouse::new();
    let mut interval = time::interval(stayawake_interval);

    let get_pos_err = |err| error!(error = ?err, "Cannot get mouse position");

    info!("Initialization finished successfully");

    interval.tick().await;

    loop {
        trace!("Loop start");

        // Grab mouse position, wait for X interval
        let pos1 = match mouse.get_position() {
            Ok(pos) => pos,
            Err(err) => {
                get_pos_err(err);
                continue;
            }
        };

        trace!("Tick started");
        interval.tick().await;
        trace!("Tick completed");

        let pos2 = match mouse.get_position() {
            Ok(pos) => pos,
            Err(err) => {
                get_pos_err(err);
                continue;
            }
        };

        // If position didn't change during the last interval, move the mouse now
        if pos1.x == pos2.x && pos1.y == pos2.y {
            let x_offset = range.sample(&mut rng);
            let y_offset = range.sample(&mut rng);

            let new_x = pos1.x + x_offset;
            let new_y = pos1.y + y_offset;

            if let Err(err) = mouse.move_to(new_x as i32, new_y as i32) {
                error!(error = ?err, "Cannot move the mouse to a new position");
            }
        }
    }
}
