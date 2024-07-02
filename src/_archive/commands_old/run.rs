use anyhow::Result;

use crate::{app::App, hot_reload::DevSession};

use super::build::BuildArgs;

#[derive(clap::Args)]
pub struct RunArgs {
    #[command(flatten)]
    build_args: BuildArgs,
    /// Test the server (stops it when it ends startup)
    #[arg(long)]
    test: bool,
}

impl RunArgs {
    pub fn create_dev_session(self, app: &mut App) -> Result<DevSession<'_>> {
        let builder = self.build_args.create_build_context(app)?;

        Ok(DevSession {
            builder,
            jar_name: None,
            hot_reload: None,
            test_mode: self.test,
        })
    }
}

pub async fn run(mut app: App, args: RunArgs) -> Result<()> {
    let dev_session = args.create_dev_session(&mut app)?;
    dev_session.start().await?;

    println!();

    Ok(())
}
