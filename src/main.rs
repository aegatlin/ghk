use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
    dot_git_hooks_clear();
    dot_ghk();
}

fn dot_ghk() {
    let mut dot_git_hooks = env::current_dir().unwrap();
    dot_git_hooks.push(".git/hooks");
    let mut path = env::current_dir().unwrap();
    path.push(".ghk");
    fs::read_dir(path)
        .expect("should read dir")
        .enumerate()
        .for_each(|(u, de)| {
            let d = de.expect("should be a dir entry");
            let p = d.path();
            let f = d.file_name();
            let mut link = dot_git_hooks.clone();
            link.push(f);
            symlink_files(p, link);
        })
}

fn dot_git_hooks_clear() {
    let mut path = env::current_dir().unwrap();
    path.push(".git/hooks");
    fs::read_dir(path)
        .expect("should read dir")
        .enumerate()
        .for_each(|(u, de)| {
            let p = de.expect("should be a dir entry").path();
            remove_file(p);
        })
}

fn symlink_files(source: PathBuf, link: PathBuf) {
    let output = Command::new("ln")
        .arg("-s")
        .arg(source)
        .arg(link)
        .output()
        .expect("symlink should work");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn remove_file(p: PathBuf) {
    let output = Command::new("rm")
        .arg(p)
        .output()
        .expect("remove should work");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn remove_git_hooks() {
    let output = Command::new("rm")
        .arg("./.git/hooks/*")
        .output()
        .expect("remove command should work");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
