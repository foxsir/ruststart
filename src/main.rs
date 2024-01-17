mod syntax;
mod mods;
mod guess_the_number;

use syntax::error_handling::run;


fn main() {
    run();

    guess_the_number::run();


    // 使用绝对路径
    // crate::mods::one::_one_call()

}
