// References:
// https://github.com/sondr3/clap-man-example/blob/main/build.rs
// https://github.com/clap-rs/clap/issues/4231
// https://github.com/clap-rs/clap/discussions/3603

use clap::{Command, CommandFactory};
use clap_mangen::Man;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

include!("src/cli.rs");

// fn build_manpages(outdir: &Path) -> Result<(), Error> {
//     let file = Path::new(&outdir).join("souffle-lint.1");
//     let mut file = File::create(&file)?;
//     Man::new(Args::command()).render(&mut file)?;
//     Ok(())
// }

fn print_manpages(dir: &Path) {
    fn print(dir: &Path, app: &Command) {
        // `get_display_name()` is `Some` for all instances, except the root.
        let name = app.get_display_name().unwrap_or_else(|| app.get_name());
        let mut out = File::create(dir.join(format!("{name}.1"))).unwrap();

        Man::new(app.clone()).render(&mut out).unwrap();
        out.flush().unwrap();

        for sub in app.get_subcommands() {
            print(dir, sub);
        }
    }

    let mut app = Args::command();
    app.build();
    print(dir, &app)
}

fn main() {
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=man");

    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    // Create `target/assets/` folder.
    let out_path = PathBuf::from(outdir);
    let mut path = out_path.ancestors().nth(4).unwrap().to_owned();
    path.push("assets");
    std::fs::create_dir_all(&path).unwrap();

    print_manpages(&path);
}
