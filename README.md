Mailer \n


This repository contains a Rust program that sends emails using the lettre library.

Prerequisites
Before you begin, ensure you have met the following requirements:

Rust and Cargo are installed on your system. You can download and install Rust from <a href = "https://www.rust-lang.org/tools/install">here. </a>
An email account with SMTP support.
Installation
Clone the repository:
```git clone https://github.com/FossMec/mailer.git
cd mailer```
To run the program, you need to provide your email ID and password as command-line arguments. Here's the basic usage:

```cargo run --release -- <EMAIL_ID> <PASSWORD>```
Replace <EMAIL_ID> and <PASSWORD> with your actual email ID and password.

Example

```cargo run --release -- vrntwentyone@zohomail.in passwd```

Configuration
This program uses the lettre library for sending emails. The basic configuration for sending an email is included in the code.

You can modify SMTP server in main.rs.

Add Recepients at ```utils/recepients.rs```

Adjust Mail to be sent in html format at ```utils/mail_content.rs```
