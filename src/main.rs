//! wip

extern crate libloading;

use anyhow::Result as Rezult;
use libloading::{Library, Symbol};
use reqwest::{
    blocking::{Client, Response},
    header::HeaderMap,
};
use std::env::var;

const EINVC: &'static str = "{\"error\":\"lambda invocation failed\"}";

macro_rules! print_runtime_info {
    () => {
        print!(
            "runtime: {} {}\nrepo: {}\nauthor: {}\n",
            option_env!("CARGO_PKG_NAME").unwrap_or_default(),
            option_env!("CARGO_PKG_VERSION").unwrap_or_default(),
            option_env!("CARGO_PKG_REPOSITORY").unwrap_or_default(),
            option_env!("CARGO_PKG_AUTHORS").unwrap_or_default()
        );
    };
}

macro_rules! create_context {
    ($headers:ident) => {
        format!(
            "{{\"function_arn\":\"{}\",\"deadline_ms\":\"{}\",\"request_id\"\
         :\"{}\",\"trace_id\":\"{}\",\"client_context\":\"{}\",\"cognito_identity\":\"{}\"}}",
            $headers["Lambda-Runtime-Invoked-Function-Arn"]
                .to_str()
                .unwrap_or_default(),
            $headers["Lambda-Runtime-Deadline-Ms"]
                .to_str()
                .unwrap_or_default(),
            $headers["Lambda-Runtime-Aws-Request-Id"]
                .to_str()
                .unwrap_or_default(),
            $headers["Lambda-Runtime-Trace-Id"]
                .to_str()
                .unwrap_or_default(),
            $headers["Lambda-Runtime-Client-Context"]
                .to_str()
                .unwrap_or_default(),
            $headers["Lambda-Runtime-Cognito-Identity"]
                .to_str()
                .unwrap_or_default()
        );
    };
}

fn api_info() -> Rezult<(String, String, String)> {
    let runtime_api: String = format!(
        "http://{}/2018-06-01/runtime",
        var("AWS_LAMBDA_RUNTIME_API")?
    );

    Ok((
        format!("{}/invocation/next", &runtime_api),
        format!("{}/invocation/error", &runtime_api),
        format!("{}/invocation/response", &runtime_api),
    ))
}

fn lib_info() -> Rezult<(String, String)> {
    let lib_lambda: Vec<String> =
        var("_HANDLER")?.split('.').map(str::to_string).collect();
    // let lib_path: String = format!("/var/task/{}.so", lib_lambda[0]);
    let lib_path: String = format!("{}.dll", lib_lambda[0]);

    Ok((lib_path, lib_lambda[1].to_string()))
}

fn main() -> Rezult<()> {
    print_runtime_info!();

    let (api_next, api_err, api_ok): (String, String, String) = api_info()?;
    let (lib_path, lambda_name): (String, String) = lib_info()?;
    let client: Client = Client::new();
    let lib: Library;
    let lambda: Symbol<unsafe fn(e: String, c: String) -> Rezult<String>>;

    unsafe {
        lib = Library::new(&lib_path)?;
        lambda = lib.get(lambda_name.as_bytes())?;
    }

    loop {
        let response: Response = client.get(&api_next).send()?;
        let headers: &HeaderMap = response.headers();
        let context: String = create_context!(headers);
        let event: String = response.text()?;

        unsafe {
            match lambda(event, context) {
                // TODO: fix returning proper http response and error bubbling
                Ok(result) => client.post(&api_ok).body(result).send()?,
                _ => client.post(&api_err).body(EINVC).send()?,
            };
        }
    }
}
