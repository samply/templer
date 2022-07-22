use std::{fs::read_to_string, process::{Termination, ExitCode}, collections::HashMap};

use minijinja::{Environment, context, value::Value, filters::Filter};

#[derive(thiserror::Error)]
enum Error {
    #[error("No argument supplied.\n\nUsage: templer TEMPLATEFILE [PARAM=VALUE ...]")]
    MissingArg,
    #[error("Invalid parameter.\n\nUsage: templer TEMPLATEFILE [PARAM=VALUE ...]")]
    InvalidParam,
    #[error("Unable to read your template from {0}: {1}")]
    ReadTemplateFromDisk(String, std::io::Error),
    #[error("Unable to parse template: {0}")]
    ParseTemplate(#[from] minijinja::Error),
}

impl Termination for Error {
    fn report(self) -> std::process::ExitCode {
        eprintln!("Hi! {}", self);
        ExitCode::FAILURE
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn do_render(tmpl: String, ctx: HashMap<String, String>) -> Result<String, Error> {
    let mut env = Environment::new();
    env.add_template("tmpl", &tmpl)?;
    let tmpl = env.get_template("tmpl")?;
    let render = tmpl.render(ctx)?;
    Ok(render)
}

fn parse_cli() -> Result<(String, HashMap<String, String>),Error> {
    let mut map = HashMap::new();
    let mut args = std::env::args();
    let tmpl_file = args.nth(1).ok_or(Error::MissingArg)?;
    for arg in args {
        let mut split = arg.split('=');
        let left = split.next().ok_or(Error::InvalidParam)?.to_owned();
        let right = split.next().ok_or(Error::InvalidParam)?.to_owned();
        map.insert(left, right);
    }

    Ok((tmpl_file, map))
}

fn main() -> Result<(), Error>{
    let (tmpl_file, ctx) = parse_cli()?;
    let tmpl = read_to_string(&tmpl_file)
        .map_err(|e| Error::ReadTemplateFromDisk(tmpl_file, e))?;
    let render = do_render(tmpl, ctx)?;
    println!("{render}");
    Ok(())
}
