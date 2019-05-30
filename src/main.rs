use moduleblog;
fn main() {
    println!("{}", moduleblog::foo::bar(String::from("baz")));
    println!("{}", moduleblog::foo::max::baz(String::from("baz")));
    println!("{}", libmodule::libfoo::gimme_a_song());
}
