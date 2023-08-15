use std::collections::HashMap;
use std::env;
use std::process::Command;

fn main() {
    // for argument in env::args() {
    //     eprintln!("Arg: {argument}");
    // }

    // for (key, value) in env::vars() {
    //     eprintln!("Var: {key}: {value}");
    // }

    let mut args = env::args().skip(1);
    let process_name = args.next().unwrap();
    // eprintln!("Current path: {:?}", env::current_dir().unwrap());

    let full_path = env::current_dir()
        .unwrap()
        .join(process_name)
        .to_string_lossy()
        .into_owned();

    let converted_path = wsl_to_windows(&full_path);

    let ld_library_path = env::var("LD_LIBRARY_PATH").unwrap_or_default();
    // eprintln!("In LD_LIBRARY_PATH: {:?}", ld_library_path);
    let library_paths = ld_library_path
        .split(':')
        .map(wsl_to_windows)
        .collect::<Vec<_>>()
        .join(";");

    let env_vars = &[
        "CARGO_HOME",
        "CARGO_MANIFEST_DIR",
        "CARGO_PKG_NAME",
        "RUST_LOG",
    ];
    let env_vars: HashMap<String, String> =
        HashMap::from_iter(env::vars().filter(|(key, _)| env_vars.contains(&key.as_str())));
    let env_vars = env_vars
        .iter()
        .map(|(key, value)| format!("$env:{} = \"{}\"", key, value))
        .collect::<Vec<_>>()
        .join(";");

    // eprintln!("Full path: {:?}", converted_path);
    // eprintln!("Out Library Paths: {:?}", library_paths);
    // eprintln!("Env Vars: {:?}", env_vars);

    let command = format!(
        "& {{ $env:PATH = \"{};\" + $env:Path; {}; Start -FilePath {} -WorkingDirectory {} -ArgumentList \"{}\" -Wait -NoNewWindow }}",
        library_paths,
        env_vars,
        converted_path,
        wsl_to_windows(&env::current_dir().unwrap().to_string_lossy(),),
        args.map(|x| x.trim().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    // eprintln!("Command: {:?}", command);

    let mut child = Command::new("powershell.exe")
        .arg("-Command")
        .arg(command)
        .arg("-NonInteractive")
        .arg("-NoLogo")
        .spawn()
        .unwrap();
    child.wait().unwrap();
}

fn wsl_to_windows(path: &str) -> String {
    let output = Command::new("wslpath")
        .arg("-w")
        .arg(path)
        .output()
        .unwrap()
        .stdout;
    std::str::from_utf8(output.as_slice())
        .unwrap()
        .trim()
        .to_string()
}
