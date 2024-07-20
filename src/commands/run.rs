use std::fmt::Formatter;
use std::sync::Arc;

use crate::api::app::App;



// TODO: determine/pass extra (cli) args
#[derive(clap::Args)]
pub struct Args {

}

// HACK: to get ""! Though this relevant if `mcman run` ever takes any args.
impl std::fmt::Display for Args {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Ok(())
  }
}

pub async fn run(
  app: Arc<App>,
  args: Args
) -> anyhow::Result<()> {
  let base = std::path::Path::new("./output/server");
  
  // TODO: crate::api::tools::java::JavaProcess
  
  app.action_run_server(base, args).await?;
  
  Ok(())
}