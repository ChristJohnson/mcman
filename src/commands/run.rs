use std::sync::Arc;
use crate::api::app::App;


// TODO: determine/pass extra (cli) args
#[derive(clap::Args)]
pub struct Args {

}

pub async fn run(
  app: Arc<App>,
  args: crate::commands::run::Args
) -> anyhow::Result<()> {
  let base = Path::new("./output/server");
  
  // TODO: ensure java installed
  
  app.action_run_server(base).await?;
  
  Ok(())
}