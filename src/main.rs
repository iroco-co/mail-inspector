use std::fs;
use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use mail_parser::{HeaderValue, Message};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global=true)]
    file: Option<PathBuf>
}

#[derive(Args, Debug)]
struct FileArg {
    file: Option<PathBuf>
}

#[derive(Subcommand, Debug)]
enum Commands {
    CountRecipients(FileArg),
    CountAttachments(FileArg)
}


fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::CountRecipients(_cmd_args) => { count_recipients(args.file)}
        Commands::CountAttachments(_cmd_args) => { count_attachments(args.file) }
    }
}

fn count_recipients(email: Option<PathBuf>) {
    let email_content = fs::read(email.as_ref().unwrap()).expect("Failed to read email file");
    let mail = Message::parse(&email_content).expect("Failed to parse email file");
    println!("{:?} {:?}", email.as_ref().unwrap(), size(mail.to()) + size(mail.cc()));
}

fn size(addr_list: &HeaderValue) -> usize {
    match addr_list {
        HeaderValue::AddressList(addresses) => addresses.len(),
        HeaderValue::Address(_addresses) => 1,
        _ => 0,
    }
}

fn count_attachments(email: Option<PathBuf>) {
    println!("count attachments {:?}", email)
}