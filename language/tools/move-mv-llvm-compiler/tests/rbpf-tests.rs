pub const TEST_DIR: &str = "tests/rbpf-test-cases";

datatest_stable::harness!(run_test, TEST_DIR, r".*\.move");

fn run_test(test_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(run_test_inner(test_path)?)
}

fn run_test_inner(test_path: &Path) -> anyhow::Result<()> {
    let harness_paths = get_harness_paths()?;
    let test_plan = get_test_plan(test_path)?;

    if test_plan.should_ignore() {
        eprintln!("ignoring {}", test_plan.name);
        return Ok(());
    }

    panic!()
}

#[derive(Debug)]
struct HarnessPaths {
    move_compiler: PathBuf,
    move_mv_llvm_compiler: PathBuf,
}

fn get_harness_paths() -> anyhow::Result<HarnessPaths> {
    // Cargo will tell us the location of move-mv-llvm-compiler.
    let move_mv_llvm_compiler = env!("CARGO_BIN_EXE_move-mv-llvm-compiler");
    let move_mv_llvm_compiler = PathBuf::from(move_mv_llvm_compiler);

    // We have to guess where move-compiler is
    let move_compiler = if !cfg!(windows) {
        move_mv_llvm_compiler.with_file_name("move-compiler")
    } else {
        move_mv_llvm_compiler.with_file_name("move-compiler.exe")
    };

    if !move_compiler.exists() {
        // todo: can we build move-ir-compiler automatically?

        let is_release = move_compiler.to_string_lossy().contains("release");
        let suggestion = if is_release {
            "try running `cargo build -p move-compiler --release` first"
        } else {
            "try running `cargo build -p move-compiler` first"
        };
        anyhow::bail!("move-compiler not built. {suggestion}");
    }

    Ok(HarnessPaths {
        move_compiler,
        move_mv_llvm_compiler,
    })
}

#[derive(Debug)]
struct TestPlan {
    name: String,
    /// The move file to be compiled to LLVM IR
    move_file: PathBuf,
    /// The move bytecode file, compiled from move, compiled to LLVM
    mvbc_file: PathBuf,
    /// The SBF object file compiled from mvbc
    obj_file: PathBuf,
    /// Special commands embedded in the test file as comments
    directives: Vec<TestDirective>,
}

#[derive(Debug, Eq, PartialEq)]
enum TestDirective {
    Ignore,
}

impl TestPlan {
    fn should_ignore(&self) -> bool {
        self.directives.contains(&TestDirective::Ignore)
    }
}

fn get_test_plan(test_path: &Path) -> anyhow::Result<TestPlan> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("cargo_manifest_dir");
    panic!()
}
