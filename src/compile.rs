mod parameters;

fn compile(file_name_truncated: String, template_source: String) -> Option<String> {
    println!("Compiling template: {}", file_name_truncated);
    print!("    building parameters index ");
    let params: &[Parameters::Parameter] = Parameters::build_parameters_index();
    println!("OK"); // or FAIL
}
