use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use build_utils::{BuildConfig, Target};
use ssh::{Channel, Session, WRITE};

pub fn deploy(conf: BuildConfig) {
    if conf.target != Target::Roborio {
        panic!("{:?} unsupported!", conf.target);
    }
    println!("\nDeploy -> Discover Roborio");
    let mut session = Session::new().expect("Failed to open SSH session");
    session
        .set_host(&format!("roboRIO-{}-frc.local", env!("FRC_TEAM_NUMBER")))
        .expect("Failed to connect to roborio");

    session.parse_config(None).expect("Failed to parse config");
    session.connect().expect("Failed to connect to session");

    println!("{:?}", session.is_server_known());
    session.set_username("admin").unwrap();
    session.userauth_password("").expect("Failed to userauth");

    println!("\nDeploy -> Kill roborio");
    request_exce_expect_empty(
        &mut session,
        b"cd /home/lvuser; /usr/local/frc/bin/frcKillRobot.sh -t 2> /dev/null",
    );
    // Give the program some time to exit

    println!("\nDeploy -> Robot Command Frc Roborio");
    request_exce_expect_empty(
        &mut session,
        b"cd /home/lvuser; echo '\"/home/lvuser/frcUserProgram\" ' > /home/lvuser/robotCommand",
    );
    request_exce_expect_empty(&mut session, b"cd /home/lvuser; chmod +x /home/lvuser/robotCommand; chown lvuser /home/lvuser/robotCommand");

    println!("\nDeploy -> Frc Static File Deploy Roborio");
    request_exce_expect_empty(
        &mut session,
        b"cd /home/lvuser; mkdir -p  @ /home/lvuser/deploy",
    );

    println!("\nDeploy -> Frc Rust Roborio");

    // let location = format!("{}", std::env::var("TARGET_DIR"))
    let mut source_location =
        PathBuf::from_str(std::env::var("TARGET_DIR").unwrap().as_str()).unwrap();

    if conf.target == Target::Roborio {
        source_location.push("arm-unknown-linux-gnueabi")
    }

    source_location.push(conf.profile.as_str());

    source_location
        .push(std::env::var("TARGET_BINARY").unwrap_or_else(|_| "robot_code".to_owned()));

    //deploy robo code
    for i in 0..30 {
        let res = send_file(
            &mut session,
            &source_location,
            "/home/lvuser",
            "frcUserProgram",
        );
        if let Err(err) = res {
            if i == 19 {
                panic!("{}", err);
            } else {
                println!("Error while sending file: {err:?}, trying again");
            }
        } else {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    request_exce_expect_empty(&mut session, b"cd /home/lvuser; chmod -R 777 \"/usr/local/frc/third-party/lib\" || true; chown -R lvuser:ni \"/usr/local/frc/third-party/lib\"");
    request_exce_expect_empty(&mut session, b"cd /home/lvuser; ldconfig");
    request_exce_expect_empty(&mut session, b"cd /home/lvuser; chmod +x \"/home/lvuser/frcUserProgram\"; chown lvuser \"/home/lvuser/frcUserProgram\"");
    request_exce_expect_empty(
        &mut session,
        b"cd /home/lvuser; setcap cap_sys_nice+eip \"/home/lvuser/frcUserProgram\"",
    );
    print!("\nDeploy -> Program Start Frc Rust");
    request_exce_expect_empty(&mut session, b"cd /home/lvuser; sync");
    let mut out = request_exce_with_channel(
        &mut session,
        b"cd /home/lvuser; /usr/local/frc/bin/frcKillRobot.sh -t -r 2> /dev/null",
    );

    let mut buf = [0u8; 8];
    loop {
        let read = out.stdout().read(&mut buf).unwrap();
        if read > 0 {
            let _ = std::io::stdout().write(&buf[..read]).unwrap();
        }
    }
}

pub fn send_file(
    session: &mut Session,
    source: impl AsRef<Path>,
    dest: &str,
    file_name: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut scp = session.scp_new(WRITE, dest).unwrap();
    scp.init().unwrap();
    let buf = std::fs::read(source.as_ref()).unwrap();
    scp.push_file(file_name, buf.len(), 0o644)?;
    Ok(scp.write(&buf)?)
}

pub fn request_exce_with_channel<'a>(session: &'a mut Session, str: &[u8]) -> Channel<'a> {
    println!("C -> {}", std::str::from_utf8(str).unwrap());
    let mut s = session.channel_new().unwrap();
    s.open_session().unwrap();

    s.request_exec(str).unwrap();
    s.send_eof().unwrap();
    s
}

pub fn request_exce_expect_empty(session: &mut Session, str: &[u8]) {
    println!("C -> {}", std::str::from_utf8(str).unwrap());
    let mut s = session.channel_new().unwrap();
    s.open_session().unwrap();

    s.request_exec(str).unwrap();
    s.send_eof().unwrap();
    let mut buf = Vec::new();
    s.stdout().read_to_end(&mut buf).unwrap();
    let str = std::str::from_utf8(&buf).unwrap();
    if !str.is_empty() {
        panic!("Error running command: {str}");
    }
}

pub fn request_exce_with_result(session: &mut Session, str: &[u8]) -> String {
    println!("C -> {}", std::str::from_utf8(str).unwrap());
    let mut s = session.channel_new().unwrap();
    s.open_session().unwrap();

    s.request_exec(str).unwrap();
    s.send_eof().unwrap();
    let mut buf = Vec::new();
    s.stdout().read_to_end(&mut buf).unwrap();
    std::string::String::from_utf8(buf).unwrap()
}
