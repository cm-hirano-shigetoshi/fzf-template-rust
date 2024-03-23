use std::process::{Child, Command};

pub struct InternalServer {
    child: Option<Child>,
}

impl InternalServer {
    pub fn new() -> Self {
        InternalServer { child: None }
    }

    pub fn start(&mut self) {
        if self.child.is_none() {
            let child = Command::new("python")
                .arg("server.py")
                .spawn()
                .expect("failed to start server.py");
        } else {
            println!("Server is already running.");
        }
    }
    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            child.kill().expect("Command failed to kill the process");
            child.wait().expect("Command failed to wait for process");
        }
    }
}

impl Drop for InternalServer {
    fn drop(&mut self) {
        self.stop();
    }
}
