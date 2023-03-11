use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();

   
    let old_count_result = posts.count().first::<i64>(connection);
    let old_count = old_count_result.unwrap();
    println!("old count {}",old_count);


    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}