# mail-inspector


## Why?

For a Life Cycle Assessment about mail we needed to parse a lot of emails and make statistics:

* number of recipients (to, cc)
* number and size of attachments
* print the date in a standard format (rfc3339)

We first used python mail module, but it was too slow (days).

## How to use it?

```shell
./mail-inspector 
Usage: mail-inspector [FILE] <COMMAND>

Commands:
  display-recipients  
  display-sender      
  count-recipients    
  count-attachments   
  email-date          
  help                Print this message or the help of the given subcommand(s)

Arguments:
  [FILE]  

Options:
  -h, --help     Print help
  -V, --version  Print version

```

For example to dump attachments for several email files in a folder:

```shell
for email in myfolder/*.eml ; do ./mail-inspector count-attachments $email; done
"myfolder/attachment.eml" "data_processing.drawio" 1591
"myfolder/attachment.eml" 1 1591
"myfolder/mail_2_attachments.eml" "data_processing.drawio" 1591
"myfolder/mail_2_attachments.eml" "content.md" 2816
"myfolder/mail_2_attachments.eml" 2 4407
"myfolder/mail_embbedded.eml" "data_processing.drawio" 1591
"myfolder/mail_embbedded.eml" 1 1591
"myfolder/mail.eml" 0 0
```

And to show number of recipients:

```shell
for file in myfolder/*.eml ; do ./mail-inspector count-recipients $file; done
"myfolder/attachment.eml" 1
"myfolder/mail_2_attachments.eml" 1
"myfolder/mail_embbedded.eml" 1
"myfolder/mail.eml" 15
```
