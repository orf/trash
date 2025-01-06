use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(required=true, num_args=1..)]
    paths: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    remove(args.paths)?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn remove(paths: Vec<PathBuf>) -> anyhow::Result<()> {
    trash::delete_all(args.paths)?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn remove(paths: Vec<PathBuf>) -> anyhow::Result<()> {
    use trash::{
        macos::{DeleteMethod, TrashContextExtMacos},
        TrashContext,
    };
    let mut trash_ctx = TrashContext::default();
    trash_ctx.set_delete_method(DeleteMethod::NsFileManager);
    trash_ctx.delete_all(paths)?;
    Ok(())
}
