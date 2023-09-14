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
    DisplayRecipients(FileArg),
    DisplaySender(FileArg),
    CountRecipients(FileArg),
    CountAttachments(FileArg),
    EmailDate(FileArg)
}


fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::DisplayRecipients(_cmd_args) => {display(args.file, vec!["cc", "to", "bcc"])}
        Commands::DisplaySender(_cmd_args) => {display(args.file, vec!["from"])}
        Commands::CountRecipients(_cmd_args) => { count_recipients(args.file)}
        Commands::CountAttachments(_cmd_args) => { count_attachments(args.file) }
        Commands::EmailDate(_cmd_args) => { email_date(args.file) }
    }
}

fn display(email_path: Option<PathBuf>, fields: Vec<&str>) {
    let email_content = fs::read(email_path.as_ref().unwrap()).expect("Failed to read email file");
    let email = Message::parse(&email_content).expect("Failed to parse email file");
    for field in fields {
        match email.header(field).unwrap_or_else(|| {
            println!("no header value for {}", field);
            &HeaderValue::Empty
        }) {
            HeaderValue::AddressList(l) => {
                for addr in l {
                    println!("{:?} {:?}", email_path, addr)
                }
            },
            HeaderValue::Address(addr) => {
                println!("{:?} {:?}", email_path, addr)
            }
            HeaderValue::Empty => {}
            f => {
                println!("{:?} {:?}", email_path, f)
            }
        }
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

fn email_date(email_path: Option<PathBuf>) {
    let email_content = fs::read(email_path.as_ref().unwrap()).expect("Failed to read email file");
    let email = &Message::parse(&email_content).unwrap();
    println!("{:?} {}", email_path.as_ref().unwrap(), email.date().unwrap().to_rfc3339());
}

fn size(addr_list: &HeaderValue) -> usize {
    match addr_list {
        HeaderValue::AddressList(addresses) => addresses.len(),
        HeaderValue::Address(_addresses) => 1,
        _ => 0,
    }
}