use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "")]
    folder: String,
}

fn main() {
    let args = Args::parse();

    println!("Folder: {}", args.folder);
}