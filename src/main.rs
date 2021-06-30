use anyhow::Result;
use fizz_strat::{logger, main_loop};

fn main() -> Result<()> {
    logger::init()?;

    let key = dotenv::var("STACKAPPS_KEY")?;
    let webhook = dotenv::var("WEBHOOK_URL")?;

    main_loop(&key, &webhook)
}
