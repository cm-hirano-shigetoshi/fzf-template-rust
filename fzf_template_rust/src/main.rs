mod fzf;
mod internal_server;
mod utils;
use fzf::Fzf;
use internal_server::InternalServer;
use std::env;

fn main() {
    // ポートを確定させる
    env::set_var("FZF_PORT", utils::find_free_port().unwrap().to_string());
    env::set_var("SERVER_PORT", utils::find_free_port().unwrap().to_string());

    // serverを起動する
    InternalServer::new();

    // fzfのプロセスを開始
    let mut fzf = Fzf { proc: None };
    let stdout = fzf.start();
    print!("{}", stdout.unwrap_or_else(|_err| String::from("")))
}