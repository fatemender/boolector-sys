use cmake::Config;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("boolector")
}

pub struct Build {
    out_dir: Option<PathBuf>,
}

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    lib: String,
}

impl Build {
    pub fn new() -> Build {
        Build {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("boolector-build")),
        }
    }

    pub fn prerequisites(&mut self) -> &mut Self {
        // TODO: Move everything to out_dir

        if !source_dir().join("deps/install/lib/liblgl.a").exists() {
            self.run_command(
                Command::new("./contrib/setup-lingeling.sh").current_dir(&source_dir()),
                "Setup Lingeling",
            );
        }

        println!(
            "cargo:rustc-link-search=native={}",
            source_dir().join("deps/install/lib").display()
        );
        println!("cargo:rustc-link-lib=static=lgl");
        println!(
            "cargo:include={}",
            source_dir().join("deps/install/include").display()
        );
        println!(
            "cargo:lib={}",
            source_dir().join("deps/install/lib").display()
        );

        if !source_dir()
            .join("deps/install/lib/libbtor2parser.a")
            .exists()
        {
            self.run_command(
                Command::new("./contrib/setup-btor2tools.sh").current_dir(&source_dir()),
                "Setup btor2tools",
            );
        }

        self
    }

    pub fn build(&mut self) -> Artifacts {
        // TODO: Unfortunately BUILDDIR is not overwriteable either implement
        // whole `Configure.sh` or find out how to overwrite `$BUILDDIR`

        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");
        let build_dir = out_dir.join("build");
        let install_dir = out_dir.join("install");

        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }

        if install_dir.exists() {
            fs::remove_dir_all(&install_dir).unwrap();
        }

        let target_dir = Config::new(source_dir()).build();

        Artifacts {
            lib_dir: target_dir.join("lib"),
            include_dir: target_dir.join("include"),
            lib: "boolector".to_string(),
        }
    }

    fn run_command(&self, command: &mut Command, desc: &str) {
        println!("running {:?}", command);
        let status = command.status().unwrap();
        if !status.success() {
            panic!(
                "
Error {}:
    Command: {:?}
    Exit status: {}
    ",
                desc, command, status
            );
        }
    }
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn lib(&self) -> &String {
        &self.lib
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        println!("cargo:rustc-link-lib=static={}", self.lib);
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}
