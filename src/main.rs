use std::process::exit;
use tasklist;
use tasklist::Process;

fn main() {
    let mut tl = unsafe { tasklist::Tasklist::new() };
    let dark_souls_remastered: Process = tl.find(|it| it.pname.to_lowercase().eq("darksoulsremastered.exe")).unwrap_or_else(|| {
        eprintln!("No process for Dark Souls:Remastered has been found");
        exit(1);
    });
    println!("{} {} {}", dark_souls_remastered.get_pid(), dark_souls_remastered.get_pname(), dark_souls_remastered.get_user());
}