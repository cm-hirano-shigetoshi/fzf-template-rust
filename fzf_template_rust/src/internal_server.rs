use std::process::{Child, Command};

pub struct InternalServer {
    child: Option<Child>,
}

impl InternalServer {
    pub fn new() -> Self {
        let child = Command::new("python")
            .arg("server.py")
            .spawn()
            .expect("failed to start server.py");
        InternalServer { child: Some(child) }
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
