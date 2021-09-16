mod config;

use std::process;

use bollard::Docker;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting.");

    match run().await {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e);
            process::exit(1);
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let _docker = Docker::connect_with_local_defaults()?;
    log::info!("Connected to docker daemon");

    let config = config::read_config()
        .await
        .map_err(|e| anyhow::anyhow!("Error reading config: {}", e))?;
    log::info!("Loaded config, monitoring {} repositories.", config.len());

    Ok(())
}
