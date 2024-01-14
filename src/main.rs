mod syntax;
mod mods;

use syntax::common_collections::run;

fn main() {
    run();

    // 使用绝对路径
    // crate::mods::one::_one_call();
}
