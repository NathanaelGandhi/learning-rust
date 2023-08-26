// using https://www.youtube.com/watch?v=BpPEoZW5IiY

mod common_programming_concepts;
mod guessing_game;

fn main() {
    println!("Hello, world!");

    // guessing_game module
    guessing_game::run_game();

    // common programming concepts
    common_programming_concepts::common_programming_concepts_main();
}
