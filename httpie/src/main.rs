use clap::{AppSettings,Clap};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "mobus sv0220@163.com")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap,Debug)]
struct SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Debug,Clap)]
struct  Get {
    url: String,
}

#[derive(Debug,Clap)]
struct Post {
    url: String,
    body: Vec<String>,
}



fn main() {

    let opts: Opts = Opts::parse();
    println!("{:?}",opts);
}

