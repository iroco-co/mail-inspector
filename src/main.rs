use std::fs;
use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use mail_parser::{HeaderValue, Message, MimeHeaders};

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

fn count_recipients(email_path: Option<PathBuf>) {
    let email_content = fs::read(email_path.as_ref().unwrap()).expect("Failed to read email file");
    let email = Message::parse(&email_content).expect("Failed to parse email file");
    println!("{:?} {:?}", email_path.as_ref().unwrap(), size(email.to()) + size(email.cc()));
}

fn count_attachments(email_path: Option<PathBuf>) {
    let email_content = fs::read(email_path.as_ref().unwrap()).expect("Failed to read email file");
    let email = &Message::parse(&email_content).unwrap();
    let mut nb = 0;
    let mut size = 0;
    print_attachments(&email_path, &email, &mut nb, &mut size);
    println!("{:?} {:?} {:?}", email_path.as_ref().unwrap(), nb, size);
}

fn print_attachments(email_path: &Option<PathBuf>, email: &Message, nb_attachments: &mut i32, total_size: &mut usize) {
    for attachment in email.attachments() {
        if !attachment.is_message() {
            println!("{:?} {:?} {:?}", email_path.as_ref().unwrap(), attachment.attachment_name().unwrap_or("Untitled"), attachment.len());
            *nb_attachments += 1;
            *total_size += attachment.len();
        } else {
            print_attachments(email_path, attachment.message().unwrap(), nb_attachments, total_size);
        }
    }
}

fn size(addr_list: &HeaderValue) -> usize {
    match addr_list {
        HeaderValue::AddressList(addresses) => addresses.len(),
        HeaderValue::Address(_addresses) => 1,
        _ => 0,
    }
}