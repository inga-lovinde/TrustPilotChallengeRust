@echo off
set RUSTFLAGS=-Zinstrument-coverage 
set LLVM_PROFILE_FILE=tests-%m.profraw 
cargo clean
cargo test --tests 
cargo profdata -- merge -sparse tests-*.profraw -o tests.profdata
cargo cov -- report --use-color --ignore-filename-regex="(registry|toolchains)" --instr-profile=tests.profdata target\debug\deps\trustpilot_challenge_rust.exe target\debug\deps\trustpilot_challenge_rust-16a5f5b3dcaa6625.exe target\debug\deps\trustpilot_challenge_rust-ad09af9137a4ac83.exe target\debug\deps\hash_computer_test-e97eaa049af7d579.exe target\debug\deps\solver_tests-5c0a8c7757425eff.exe | more
cargo cov -- show --use-color --ignore-filename-regex="(registry|toolchains)" --instr-profile=tests.profdata target\debug\deps\trustpilot_challenge_rust.exe target\debug\deps\trustpilot_challenge_rust-16a5f5b3dcaa6625.exe target\debug\deps\trustpilot_challenge_rust-ad09af9137a4ac83.exe target\debug\deps\hash_computer_test-e97eaa049af7d579.exe target\debug\deps\solver_tests-5c0a8c7757425eff.exe --show-instantiations --show-line-counts-or-regions --Xdemangler=rustfilt | more
cargo clean
del *.profraw
del *.profdata
