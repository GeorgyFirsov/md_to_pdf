// This file contains application's entry point
//

mod process_markdown;
mod compose_pdf;
mod common;

extern crate clap;
extern crate regex;
extern crate base64;
extern crate comrak;
extern crate tracing;
extern crate wkhtmltopdf;


use clap::{App, load_yaml};
use tracing::{error, debug, Level};

use process_markdown::convert_markdown_to_pretty_html;
use compose_pdf::convert_html_to_pdf;
use common::{INCORRECT_INPUT_FILE, INCORRECT_OUTPUT_FILE,
             DEFAULT_MD_NAME, DEFAULT_PDF_NAME,
             MD_EXTENSION, PDF_EXTENSION, CANNOT_SET_TRACE_SUBSCRIBER};


fn main() {
    //
    // Setup trace service
    //

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect(CANNOT_SET_TRACE_SUBSCRIBER);

    //
    // Read and parse command line arguments
    //

    let yaml_cli_config = load_yaml!("../config/cli.yaml");
    let matches = App::from(yaml_cli_config).get_matches();

    let input_file = matches.value_of("input")
        .unwrap_or(DEFAULT_MD_NAME);

    let output_file = matches.value_of("output")
        .unwrap_or(DEFAULT_PDF_NAME);

    //
    // Verify their correctness
    //

    if !input_file.ends_with(MD_EXTENSION) {
        error!("incorrect input file name: {}", input_file);
        std::panic::panic_any(INCORRECT_INPUT_FILE);
    }

    if !output_file.ends_with(PDF_EXTENSION) {
        error!("incorrect output file name: {}", output_file);
        std::panic::panic_any(INCORRECT_OUTPUT_FILE);
    }

    debug!("input_file == {}", input_file);
    debug!("output_file == {}", output_file);

    //
    // Parse markdown and compose PDF
    //

    let html = convert_markdown_to_pretty_html(input_file);
    convert_html_to_pdf(output_file, html.as_str());
}
