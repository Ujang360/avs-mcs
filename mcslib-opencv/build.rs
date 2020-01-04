use std::process::Command;

fn main() {
    if !cfg!(target_os = "linux") {
        panic!("This library only support linux system!");
    }

    Command::new("sh")
        .arg("-c")
        .arg("check-opencv.sh")
        .output()
        .expect("Failure installing OpenCV!");
}
