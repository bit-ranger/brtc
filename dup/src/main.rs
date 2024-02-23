use std::collections::HashMap;
use serde_json::{Map, Value};
use std::fmt::{Debug, Display, Error, Formatter};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use tokio::fs::{File};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::{fs};
use tokio::io;
use async_recursion::async_recursion;

#[derive(StructOpt)]
#[structopt(name = "dup")]
enum Dup {
    Ls {
        #[structopt(short, long)]
        path: String,
    },
}

#[derive(thiserror::Error)]
enum DupError {
    #[error("{0}")]
    Message(String),
}

impl Debug for DupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), DupError> {
    let opt = Dup::from_args();
    match opt {
        Dup::Ls { path } => run(path).await,
    }
}

async fn run(path: String) -> Result<(), DupError> {
    let mut collector = Vec::new();
    collect_files(PathBuf::from(path), &mut collector).await.unwrap();
    Ok(())
}

#[async_recursion]
async fn collect_files(path: PathBuf, collector: &mut Vec<(String, PathBuf)>) -> io::Result<()>{
    let mut read_dir = fs::read_dir(path).await?;
    loop {
        let entry = read_dir.next_entry().await?;
        if entry.is_none() {
            break;
        }
        let entry = entry.unwrap();
        if !is_dir(entry.path()).await {
            let file = File::open(entry.path()).await?;
            let mut reader = BufReader::new(file);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf).await?;
            let digest = md5::compute(buf);
            let digest = format!("{:x}", digest);
            collector.push((digest, entry.path()));
            continue;
        }  else {
            collect_files(entry.path(), collector).await;
        }
    }

    return Ok(());
}


pub async fn is_dir(path: impl AsRef<Path>) -> bool {
    fs::metadata(path).await.map(|m| m.is_dir()).unwrap_or(false)
}
