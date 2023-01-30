use crate::c_3::stack::par_checker;

mod c_3;

fn main() {
    let result1 = par_checker("{{( [ ] [ ] ) } ( ) }");
    let result2 = par_checker("( ( ( ) ] ) )");

    println!("{result1} {result2}");
}
