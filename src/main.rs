use curl::easy::Easy;
use std::{env, fs};
use std::process::Stdio;
use std::{fs::File, io::prelude::*, process::Command};

fn main() -> std::io::Result<()> {
    let alvr_download = get_current_working_dir() + "/alvr_client_oculus_quest.apk";

    let mut easy = Easy::new();
    let mut drain = Vec::new();
    easy.url(
        "https://github.com/alvr-org/ALVR/releases/latest/download/alvr_client_oculus_quest.apk",
    )
    .unwrap();
    let _redirection = easy.follow_location(true);

    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                drain.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    {
        let mut file = File::create("alvr_client_oculus_quest.apk")?;
        file.write_all(drain.as_slice())?;
    }

    let mut command_shell = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    };

    let path_to_adb = dirs::home_dir().unwrap().as_path().to_str().unwrap().to_owned() + "/Documents/platform-tools/";

    command_shell.current_dir(path_to_adb);

    let command = String::from("./adb install ") + &alvr_download;

    if cfg!(target_os = "windows") {
        command_shell.args(["/C", &command]);
    } else {
        command_shell.arg("-c").arg(command);
    }

    let mut execute = command_shell.stdout(Stdio::null()).spawn().expect("failed to execute process");
    let _result = execute.wait().unwrap();
 

    fs::remove_file("alvr_client_oculus_quest.apk")?;

    Ok(())
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().to_owned().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}
