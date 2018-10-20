#[macro_use]
extern crate askama;
extern crate pandoc;

#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate bytes;

use actix_web::{http, server, App, Form};
use askama::Template;
use bytes::Bytes;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Deserialize)]
struct CustomerInfo {
    fullname: String,
    address: String,
}

#[derive(Template)]
#[template(path = "spa.md")]
struct SpaTemplate<'a> {
    fullname: &'a str,
    address: &'a str,
}

fn gen_spa(form: Form<CustomerInfo>) -> Bytes {
    let spa = SpaTemplate {
        fullname: &form.fullname,
        address: &form.address,
    };
    let markdown = spa.render().unwrap();

    // let mut pandoc = pandoc::new();
    // pandoc.set_input(pandoc::InputKind::Pipe(markdown));
    // pandoc.set_output(pandoc::OutputKind::Pipe);
    // pandoc.set_output_format(pandoc::OutputFormat::Docx, Vec::new());
    // pandoc.add_option(pandoc::PandocOption::ReferenceDocx("reference.docx".into()));
    // pandoc.set_show_cmdline(true);
    // match pandoc.execute() {
    //     Ok(_) => println!("Success"),
    //     Err(e) => println!("{}", e),
    // }

    let mut child = Command::new("/usr/bin/pandoc")
        .args(&["--reference-doc", "reference.docx", "-t", "docx"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute child");

    {
        let stdin = child.stdin.as_mut().expect("Fail to open stdin");
        stdin
            .write_all(markdown.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on child");

    Bytes::from(output.stdout)
}

fn main() {
    let sys = actix::System::new("Magnifee");

    server::new(|| App::new().resource("/gen", |r| r.method(http::Method::POST).with(gen_spa)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .start();

    println!("Starting http server: 0.0.0.0:8080");
    let _ = sys.run();
}
