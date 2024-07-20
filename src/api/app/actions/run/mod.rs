use std::{path::Path, sync::Arc};
use std::process::Command;

use anyhow::{bail, Result};

use crate::api::app::App;
use crate::commands::run::Args;



impl App {
  pub async fn action_run_server(self: Arc<Self>, base: &Path, args: Args) ->
  Result<()> {
    
    if let Some((path, srv)) = self.server.read().await.as_ref() {
      println!("Running server at {}", path.display());
      
      let launcher = &srv.launcher;
      
      // it has to be this janky?
      let jar_args: String = match &srv.jar {
        None => bail!("[action_run_server] unable to determine server jar!"),
        Some(j) => j,
      }.get_execution_arguments().iter().flat_map(|p| p.chars())
          .collect();
      
      // discover default execution arguments (i.e. pass these and server
      // will run)
      // argument exec
      let args = launcher.get_args(jar_args.as_str());
      
      // build server
      let mut server = Command::new("java")
          .args(args)
          .spawn()?;
      
      // TODO: fork process (see std::process io practices)
      server.wait()?;
    } else {
      bail!("[action_run_server] could not find server!");
    }
    
    Ok(())
  }
}
