use handlebars::{Context, Handlebars};
use serde_json::{Map, Value};
use std::fmt::{Debug, Display, Formatter};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "tpl")]
enum Tpl {
    Csv {
        #[structopt(short, long)]
        template: String,

        #[structopt(short, long)]
        input: String,
    },
}

#[derive(thiserror::Error)]
enum TplError {
    #[error("{0}")]
    Message(String),
}

impl Debug for TplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

fn main() -> Result<(), TplError> {
    let opt = Tpl::from_args();
    match opt {
        Tpl::Csv { template, input } => run(template, input),
    }
}

fn run(template: String, input: String) -> Result<(), TplError> {
    let input_vec: Vec<(usize, &str)> = input.split(",").enumerate().collect();

    let mut map = Map::new();
    for (ii, id) in input_vec {
        map.insert(ii.to_string(), Value::String(id.to_string()));
    }

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(handlebars::no_escape);

    let ctx = Context::wraps(map).map_err(|e| TplError::Message(format!("{}", e)))?;
    let output = handlebars
        .render_template_with_context(template.as_str(), &ctx)
        .map_err(|e| TplError::Message(format!("{}", e)))?;

    println!("{}", output);
    Ok(())
}
