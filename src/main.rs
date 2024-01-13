mod syntax;
mod mods;

use syntax::enum_coin::run;

// use mods::one::_one_call;
// use mods::one::two;
use mods::one::one;

fn main() {
    run();
    one::call();
    // two::_call();

    // 使用绝对路径

    crate::mods::one::_one_call();
}
