use std::{
    io::{Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
    str::FromStr,
};

use build_utils::{BuildConfig, Target};
use ssh2::{Channel, Session};

pub fn deploy(conf: BuildConfig) {
    if conf.target != Target::Roborio {
        panic!("{:?} unsupported!", conf.target);
    }
    println!("\nDeploy -> Discover Roborio");

    //let tcp = TcpStream::connect(&format!("roboRIO-{}-frc.local", env!("FRC_TEAM_NUMBER")));
    let tcp = TcpStream::connect("10.11.14.2:22");
    let tcp = tcp.unwrap();

    let mut session = Session::new().expect("Failed to open SSH session");
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password("admin", "").unwrap();

    // let mut s = session.channel_session().unwrap();

    // s.exec("cd /home/lvuser; ls").unwrap();
    // s.send_eof().unwrap();
    // let mut buf = String::new();
    // s.read_to_string(&mut buf).unwrap();
    // println!("{buf}");
    // panic!();

    // let mut channel = session.channel_session().unwrap();
    // channel.write(buf)
    // session
    //     .set_host(&format!("roboRIO-{}-frc.local", env!("FRC_TEAM_NUMBER")))
    //     .expect("Failed to connect to roborio");

    // session.parse_config(None).expect("Failed to parse config");
    // session.connect().expect("Failed to connect to session");

    // let mut agent = session.agent().unwrap();

    // agent.connect().unwrap();
    // agent.list_identities().unwrap();
    // for identity in agent.identities().unwrap(){
    //     println!("{}", identity.comment());
    // }

    // println!("{:?}", session.set_blocking());
    // session.userauth_agent("admin").expect("Failed to userauth");

    println!("\nDeploy -> Kill roborio");
    request_exce_expect_empty(
        &mut session,
        "cd /home/lvuser; . /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t 2> /dev/null",
    );
    // Give the program some time to exit

    println!("\nDeploy -> Robot Command Frc Roborio");
    request_exce_expect_empty(
        &mut session,
        "cd /home/lvuser; echo '\"/home/lvuser/frcUserProgram\" ' > /home/lvuser/robotCommand",
    );
    request_exce_expect_empty(&mut session, "cd /home/lvuser; chmod +x /home/lvuser/robotCommand; chown lvuser /home/lvuser/robotCommand");

    println!("\nDeploy -> Frc Static File Deploy Roborio");
    request_exce_expect_empty(
        &mut session,
        "cd /home/lvuser; mkdir -p  @ /home/lvuser/deploy",
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
            &PathBuf::from_str("/home/lvuser/frcUserProgram").unwrap(),
        );
        if let Err(err) = res {
            if i == 29 {
                panic!("{}", err);
            } else {
                println!("Error while sending file: {err:?}, trying again");
            }
        } else {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    request_exce_expect_empty(&mut session, "cd /home/lvuser; chmod -R 777 \"/usr/local/frc/third-party/lib\" || true; chown -R lvuser:ni \"/usr/local/frc/third-party/lib\"");
    request_exce_expect_empty(&mut session, "cd /home/lvuser; ldconfig");
    request_exce_expect_empty(&mut session, "cd /home/lvuser; chmod +x \"/home/lvuser/frcUserProgram\"; chown lvuser \"/home/lvuser/frcUserProgram\"");
    request_exce_expect_empty(
        &mut session,
        "cd /home/lvuser; setcap cap_sys_nice+eip \"/home/lvuser/frcUserProgram\"",
    );
    print!("\nDeploy -> Program Start Frc Rust");
    request_exce_expect_empty(&mut session, "cd /home/lvuser; sync");
    request_exce_expect_empty(
        &mut session,
        "cd /home/lvuser; . /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t -r 2> /dev/null",
    );
}

pub fn send_file(
    session: &mut Session,
    source: impl AsRef<Path>,
    dest: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let buf = std::fs::read(source.as_ref())?;

    let mut scp = session.scp_send(dest, 0o644, buf.len() as u64, None)?;

    scp.write_all(&buf)?;
    // println!("SENT FILE: {} -> {}", buf.len(), len);
    scp.send_eof()?;
    scp.wait_eof()?;
    scp.close()?;
    scp.wait_close()?;
    Ok(())
}

pub fn request_exce_with_channel(session: &mut Session, str: &str) -> Channel {
    println!("C -> {str}");
    let mut s = session.channel_session().unwrap();
    s.exec(str).unwrap();
    s.send_eof().unwrap();
    // s.
    s
}

pub fn request_exce_expect_empty(session: &mut Session, str: &str) {
    println!("C -> {str}");
    let mut s = session.channel_session().unwrap();
    // s.open_session().unwrap();

    s.exec(str).unwrap();
    s.send_eof().unwrap();
    let mut buf = String::new();
    s.read_to_string(&mut buf).unwrap();
    if !buf.is_empty() {
        panic!("Error running command: {:?}: {:?}", buf.as_bytes(), buf);
    }
}

pub fn request_exce_with_result(session: &mut Session, str: &str) -> String {
    println!("C -> {str}");
    let mut s = session.channel_session().unwrap();

    s.exec(str).unwrap();
    s.send_eof().unwrap();
    let mut buf = String::new();
    s.read_to_string(&mut buf).unwrap();
    buf
}
