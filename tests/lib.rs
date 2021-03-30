#![warn(rust_2018_idioms)]

use std::{fs, path::PathBuf, str};

use lazy_static::lazy_static;
use takuzu::Grid;

lazy_static! {
    static ref GRIDS_DIR: PathBuf = {
        let output = std::process::Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .stderr(std::process::Stdio::null())
            .output()
            .expect("failed to run git command");
        assert!(output.status.success(), "git command exit status is not success");
        let stdout = &output.stdout;
        let grids_dir = str::from_utf8(stdout).expect("git command output is not valid utf8");
        let mut grids_dir = PathBuf::from(grids_dir.trim_end());
        grids_dir.push("grids");
        grids_dir
    };
}

macro_rules! test_grid {
    ($test_name:ident, $grid:expr, $out:expr) => {
        #[test]
        fn $test_name() {
            let input = fs::read_to_string(GRIDS_DIR.join($grid)).unwrap();
            let output = fs::read_to_string(GRIDS_DIR.join($out)).unwrap();
            let grid = input.parse::<Grid>().unwrap();
            let solutions = grid.solve().unwrap();
            let references = output.split("\n\n").collect::<Vec<_>>();
            assert_eq!(solutions.len(), references.len());
            for (solution, reference) in solutions.iter().zip(references) {
                assert_eq!(solution.to_string().trim_end(), reference.trim_end());
            }
        }
    };
}

test_grid!(test_grid_1, "grid1", "out1");
test_grid!(test_grid_2, "grid2", "out2");
test_grid!(test_grid_3, "grid3", "out3");
test_grid!(test_grid_4, "grid4", "out4");
