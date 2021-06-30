use anyhow::Result;
use fizz_strat::main_loop;

fn main() -> Result<()> {
    let key = dotenv::var("STACKAPPS_KEY")?;
    let webhook = dotenv::var("WEBHOOK_URL")?;

    main_loop(&key, &webhook)
}
