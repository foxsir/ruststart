mod syntax;
mod mods;

use syntax::error_handling::run;

fn main() {
    run();

    // 使用绝对路径
    // crate::mods::one::_one_call()

}
