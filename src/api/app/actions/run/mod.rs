use std::{path::Path, sync::Arc};
use std::process::{Command, Stdio};
use anyhow::Result;

use crate::api::app::App;

impl App {
  pub async fn action_run_server(self: Arc<Self>, base: &Path) -> Result<()> {
    
    if let Some(jar) = self.server.read().await.as_ref().map(|(_, server)| {
      server.jar.clone()
    }).flatten() {
      println!("Running server jar");
      // determine type of server
      let server_type = &jar.server_type;
      
      // discover default execution arguments (i.e. pass these and server
      // will run)
      let args = server_type.get_execution_arguments();
      
      // TODO: concatenate
      
      // build server
      let mut server = Command::new("java")
          .args(args)
          .stdin(Stdio::inherit())
          .stdout(Stdio::inherit())
          .stderr(Stdio::inherit())
          .spawn()?;
      
      // TODO: fork process (see std::process io practices)
      server.wait()?;
      
      todo!("execute server jar");
      
    }
    
    todo!();
  }
}
