#[macro_use]
extern crate postgresql_named_parameters_derive;
extern crate postgres_from_row;
use postgresql_named_parameters::*;
use postgres_from_row::FromRow;

#[derive(FromRow)]
struct Person {
    first_name: String,
    last_name: String,
    age: i32,
    alive: bool
}

#[derive(Query)]
#[query(
    sql = "SELECT * FROM Person WHERE (first_name = @name OR last_name = @name) AND alive = @alive",
    row = Person
)]
struct GetPeople<'a> {
    alive: bool,
    name: &'a str
}