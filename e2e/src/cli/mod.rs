use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "e2e-tests")]
pub enum Cli {
    #[structopt(name = "creator")]
    Creator {
        #[structopt(long)]
        dockerfile: PathBuf,
        #[structopt(long)]
        dockerhub_repo: String,
        #[structopt(long)]
        download_url: String,
    },

    #[structopt(name = "user")]
    User {
        #[structopt(long)]
        dockerhub_image: String,
        #[structopt(long)]
        download_url: String,
    },
}
