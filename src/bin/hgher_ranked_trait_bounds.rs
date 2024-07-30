use std::fmt::Display;

trait Formatter {
    fn format<T: Display>(&self, value: T) -> String;
}

struct SimpleFormatter;

impl Formatter for SimpleFormatter {
    fn format<T: Display>(&self, value: T) -> String {
        format!("-- {} --", value)
    }
}


// zdefiniowanie 'a w ten sposób oznacza że parametr przekazywany do callbacku, oraz callback sam w sobie
// są połączone relacją, czyli parametr przekazywany do callbacku musi żyć co najmniej tak długo jak callback
// oznacza to że te relacja jest zbyt restrykcyjny, bo tak naprawdę wystarczy żeby parametr był valid w momencie wywołania callbacku
fn apply_formatter<'a, F>(formatter: F) -> impl Fn(&'a str) -> String
where
    F: Formatter,
{
    move |s| formatter.format(s)
}


// ten zapis oznacza że wystarczy żeby parametr był valid jedynie w momencie wywołania callbacku
fn apply_formatter_hr_trait<F>(formatter: F) -> impl for<'a> Fn(&'a str) -> String
where
    F: Formatter,
{
    move |s| formatter.format(s)
}


// tutaj niejawnie używany jest mechanizm 'higher-ranked trait bounds' opisany wyżej
fn apply_formatter_implicit<F>(formatter: F) -> impl Fn(&str) -> String
where
    F: Formatter,
{
    move |s| formatter.format(s)
}


fn main() {
    // https://www.youtube.com/watch?v=6fwDwJodJrg

    let format_fn = apply_formatter(SimpleFormatter);
    let format_hr_fn = apply_formatter_hr_trait(SimpleFormatter);
    let format_implicit_fn = apply_formatter_implicit(SimpleFormatter);

    let s1 = String::from("Hello, Rust 1!");
    let s2 = String::from("Hello, Rust 2!");
    let s3 = String::from("Hello, Rust 3!");

    // to nie zadziała, bo s1 żyje krócej niż format_fn (wartości są dropowane w odwrotnej kolejności niż były tworzone)
    // println!("{}", format_fn(&s1));

    // to zadziała, bo s2 jest valid w momencie wywołania callbacku
    println!("{}", format_hr_fn(&s2));

    // to też zadziałą, bo niejawnie używany jest mechanizm 'higher-ranked trait bounds'
    println!("{}", format_implicit_fn(&s3));
}
