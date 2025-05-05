1.

error: failed to run `rustc` to learn about target-specific information

Caused by:
process didn't exit successfully: `/home/bushman/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc - --crate-name ___ --print=file-names --target /home/bushman/projects/studyos/x86_64-study_os.json --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg -Wwarnings` (exit status: 1)
--- stderr
error: Error loading target specification: target feature `soft-float` is incompatible with the ABI but gets enabled in target spec. Run `rustc --print target-list` for a list of built-in targets


2.

error[E0463]: can't find crate for `core`
|
= note: the `x86_64-study_os` target may not be installed
= help: consider downloading the target with `rustup target add x86_64-study_os`
= help: consider building the standard library from source with `cargo build -Zbuild-std`

For more information about this error, try `rustc --explain E0463`.
warning: `studyos` (bin "studyos") generated 1 warning
error: could not compile `studyos` (bin "testos") due to 1 previous error; 1 warning emittedCompiling studyos v0.1.0 (/home/bushman/projects/studyos)

warning: target feature 

`sse2`

 must be enabled to ensure that the ABI of the current target can be implemented correctly
|
= note: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
= note: for more information, see issue #116344 

[https://github.com/rust-lang/rust/issues/116344](https://github.com/rust-lang/rust/issues/116344)

3.

error: data-layout for target `x86_64-study_os-9146196472706914219`, `e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128`, differs from LLVM target's `x86_64-unknown-none` default layout, `e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128`
