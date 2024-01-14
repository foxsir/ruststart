mod syntax;
mod mods;

use syntax::hash_map::run;

fn main() {
    run();

    // 使用绝对路径
    // crate::mods::one::_one_call();
}
