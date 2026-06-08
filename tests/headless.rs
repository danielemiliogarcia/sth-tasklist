//! Integration test for the headless launch path (T-6 / DV-1 follow-up).

use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn unique_temp_dir() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("shtask-headless-{}-{nanos}", std::process::id()))
}

#[test]
fn headless_launch_exits_successfully() {
    let dir = unique_temp_dir();
    fs::create_dir_all(&dir).unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_shtask"))
        .arg("--headless")
        .current_dir(&dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let status = wait_with_timeout(&mut child, Duration::from_secs(2));
    if status.is_none() {
        child.kill().ok();
        child.wait().ok();
    }
    fs::remove_dir_all(&dir).ok();

    let status = status.expect("--headless launch did not exit within 2 seconds");
    assert!(status.success(), "--headless exited with {status}");
}

fn wait_with_timeout(child: &mut Child, timeout: Duration) -> Option<ExitStatus> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            return Some(status);
        }
        if Instant::now() >= deadline {
            return None;
        }
        thread::sleep(Duration::from_millis(10));
    }
}
