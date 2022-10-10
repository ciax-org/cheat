/* NOTE:
Thought code in a cheat sheet file online named 'LGR Cheat Sheet' would be nice in one module.

i.e. Let's Get Rusty Version 1.0.7 from these person(s) https://www.youtube.com/c/LetsGetRusty

Even before knowing much Rust I managed to read and output an Excel spreadsheet how I needed it.

Oh yeah #BrainFog and scrape names of all the businesses that recognise unions off `gov.uk`.

Did want de-identified Magistrate Court data but only Big Brother's 'Beautiful Ones' can see it.

Made change below to examples, but changed my mind; put them back (Regex find & replace).

- Variable & function & struct field & argument names are `_name` to prevent unused `name` warnings.

Wanted to use analyser way to find / change variable names; but I hand modified them originally.

I may have given some random letters to names that were `_` before.

Best to learn this `_` naming business separately.

cheat_copy_move_and_clone() purposely gives a lint error (comment the line back in to see it).

One or two `module` examples missing from the end as they can't go in a module; as far as I know.
*/

// Could have used this; but it masks possible errors you introduce; decided to use it anyway
// You can comment out
#![allow(unused_variables)]
// #![allow(warnings)] // Could have used this
#![allow(unreachable_patterns)] // One example has unreachable patterns; still a valid example
#![allow(dead_code)] // Silence warning for unused example functions and struct fields
#![allow(unused_assignments)] // One of these warnings

// All examples are callable - and called to avoid `unused` warnings
pub fn main() {

    // Eliminates warning so you only see warnings you introduce
    // Not actually needed now warnings are allowed
    use_stuff_to_eliminate_warning();

    // Basic Types & Variables
    cheat_basic_types_and_variables();
    cheat_tuple();
    cheat_array_slice();
    cheat_hash_map();
    cheat_struct();
    cheat_enum();
    cheat_constant();
    cheat_static_variable();
    cheat_mutability();
    cheat_shadowing();
    cheat_type_alias();
    // Control Flow
    cheat_if_and_if_let();
    cheat_loop();
    cheat_nested_loops_and_labels();
    cheat_returning_from_loops();
    cheat_while_and_while_let();
    cheat_for_loop_in();
    cheat_match();
    cheat_tuple();
    cheat_array_slice();
    cheat_hash_map();
    cheat_struct();
    cheat_enum();
    cheat_constant();
    cheat_static_variable();
    cheat_mutability();
    cheat_shadowing();
    cheat_type_alias();
    cheat_if_and_if_let();
    cheat_loop();
    cheat_nested_loops_and_labels();
    cheat_returning_from_loops();
    cheat_while_and_while_let();
    cheat_for_loop_in();
    cheat_match();
    // References, Ownership, and Borrowing
    cheat_creating_references();
    cheat_copy_move_and_clone();
    cheat_ownership_and_functions();
    // Pattern Matching
    cheat_pattern_matching_basics();
    cheat_destructuring();
    cheat_ignoring_values();
    cheat_match_guards();
    cheat_bindings();
    // Iterators
    cheat_iterators_usage();
    cheat_implementing_iterator_trait();
    // Error Handling
    cheat_throw_unrecoverable_error();
    cheat_option_enum();
    cheat_result_enum();
    cheat_question_mark_operator();
    // Combinators
    cheat_combinators_map();
    cheat_and_then();
    // Multiple error types
    cheat_define_custom_error_type();
    cheat_boxing_errors();
    // Iterating over errors
    cheat_ignore_failed_items_with_filter_map();
    cheat_fail_entire_operation_with_collect();
    cheat_collect_all_valid_values_and_failures_with_partition();
    // Generics, Traits, and Lifetimes
    cheat_using_generics();
    cheat_defining_traits();
    cheat_default_implementations_with_derive();
    cheat_trait_bounds();
    cheat_impl_trait();
    cheat_trait_objects();
    cheat_operator_overloading();
    cheat_supertraits();
    cheat_lifetimes_in_function_signatures();
    cheat_lifetimes_in_struct_definitions();
    cheat_static_lifetimes();
    // Functions, Function Pointers & Closures
    cheat_associated_functions_and_methods();
    cheat_function_pointers();
    cheat_creating_closures();
    cheat_returning_closures();
    // Closure_traits
    cheat_store_closure_in_struct();
    cheat_function_that_accepts_closure_or_function_pointer();
    // Pointers
    cheat_eferences();
    cheat_raw_pointers();
    // Smart pointers
    cheat_smart_pointers_box();
    cheat_smart_pointers_rc();
    cheat_smart_pointers_ref();
    cheat_multiple_owners_of_mutable_data();
}

// ### Ignore section below - just so examples are `whole` without errors (i.e. some plumbing)

fn use_stuff_to_eliminate_warning() {
    let a = ResultCodeResidual(NonZeroI32::new(1).unwrap());
    let e = Error {};
    let c = Connection {};
    print_type_of(&c);
}

// Application Binary Interface (ABI)
use std::num::NonZeroI32;
pub struct ResultCodeResidual(NonZeroI32);

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

struct Connection {}
impl Connection {
    pub fn connect(self) -> Result<Connection, Error> { Ok(Connection {}) }
}
struct Error {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
struct Database {
}
impl Database {
    pub fn user_exists(self, name: &str) -> bool { false}
    pub fn get_id(self, name: &str) -> u32 { 0 }
    pub fn get_user(self, id: i32) -> Option<User> {
        Some(User { username: "".to_owned(), active: false })
    }
    pub fn get_active_instance(self) -> Result<Connection, Error> { Ok(Connection {}) }
}

#[derive(Debug)]
struct Job {
    salary: u32
}

#[derive(Debug)]
struct User {
    username: String,
    active: bool
}
impl User {
    pub fn get_job(self) -> Option<Job> { Some(Job { salary: 0 }) }
}
// ### Ignore section above

// Basic Types & Variables
fn cheat_basic_types_and_variables() {
    let v: bool = true; // Boolean
    // Unsigned integers
    let v: u8 = 1; let v: u16 = 1; let v: u32 = 1; let v: u64 = 1; let v: u128 = 1;
    // Signed integers
    let v: i8 = -1; let v: i16 = -1; let v: i32 = -1; let v: i64 = -1; let v: i128 = -1;
    // Floating point numbers
    let v: f32 = -1.1; let v: f64 = -1.1;
    // Platform specific integers
    let v: usize = 1; // Unsigned integer. Same number of bits as the platform's pointer type.
    let v: isize = -1; // Signed integer. Same number of bits as the platform's pointer type.
    let v: char = '$'; // Unicode scalar value
    let v: &str = "String slice"; // String slice
    let v = std::string::String::from("Owned string"); // Owned string
}

fn cheat_tuple() {
    let coordinates = (82, 64);
    let score = ("Team A", 12);
    println!("{:?} {:?}", coordinates, score);
}

fn cheat_array_slice() {
    let array = [1, 2, 3, 4, 5];
    let array2 = [0; 3]; // [0, 0, 0]

    let slice = &array[1 .. 3]; // [2, 3]
    println!("{:?}", slice);
}

fn cheat_hash_map() {
    use std::collections::HashMap;

    let mut subs = HashMap::new();
    subs.insert(String::from("LGR"), 1);
    subs.entry(String::from("LGR")).or_insert(2);
    subs.entry("Golang Dojo".to_owned()).or_insert(3); // Insert key if it doesn't have a value

    println!("{:?}", subs);
}

fn cheat_struct() {
    let user1 = User {
        username: String::from("bogdan"),
        active: true
    };

    #[derive(Debug)]
    struct Color(i32, i32, i32); // Tuple
    let black = Color(0, 0, 0);

    println!("{:?} {:?}", user1, black);
}

fn cheat_enum() {
    enum Command {
        Quit,
        Move { x: i32, y: i32 },
        Speak(String),
        ChangeBGColor(i32, i32, i32),
    }

    let msg1 = Command::Quit;
    let msg2 = Command::Move{ x: 1, y: 2 };
    let msg3 = Command::Speak("Hi".to_owned());
    let msg4 = Command::ChangeBGColor(0, 0, 0);
}

fn cheat_constant() {
    const MAX_POINTS: u32 = 1_000_000;
}

fn cheat_static_variable() {
    static MAJOR_VERSION: u32 = 1;
    static mut COUNTER: u32 = 0;
}

fn cheat_mutability() {
    let mut x = 5;
    x = 6;
}

fn cheat_shadowing() {
    let x = 5;
    let x = x * 2;
}

fn cheat_type_alias() {
    type NanoSecond = u64;
    let x: NanoSecond = 1;
}

// Control Flow
fn cheat_if_and_if_let() {
    let num = Some(22);

    if num.is_some() {
        println!("number is: {}", num.unwrap());
    }
    if let Some(i) = num {
        println!("number is: {}", i);
    }
}

fn cheat_loop() {

    // for (count = 0; count < 5; count++)
    let mut count = 0;
    loop {
        println!("Do stuff {}", count);
        count += 1;
        if count == 2 {
            break; // Exit loop
        }
    }

    let mut count = 0;
    // while (count < 5)
    loop {
        if count == 2 {
            break; // Exit loop
        }
        println!("Do stuff {}", count);
        count += 1;
    }

    let mut count = 0;
    // do {} while (count < 5)
    loop {
        println!("Do stuff {}", count);
        count += 1;
        if count == 2 {
            break; // Exit loop
        }
    }

}

fn cheat_nested_loops_and_labels() {
    'outer: loop {
        loop {
            if 1 == 2 {
                break; // This breaks the inner loop
            }
            break 'outer; // This breaks the outer loop
        }
    }
}

fn cheat_returning_from_loops() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter;
        }
    };

    println!("{}", result);
    return result
}

fn cheat_while_and_while_let() {
    let mut n = 0;
    while n < 101 {
        n += 1;
    }

    let mut optional = Some(0); // Option type
    while let Some(i) = optional {
        println!("Some {}", i);
        optional = None;
    }
}

fn cheat_for_loop_in() {
    for n in 1..3 {
        print!("{} ", n); // 1 2
    }
    print!("\r\n");

    let names = vec!["Bogdan", "Wallace"];
    for name in names.iter() {
        println!("{}", name);
    }
}

fn cheat_match() {
    let optional = Some(0);

    match optional {
        Some(i) => println!("{}", i),
        None => println!("No value.")
    }
}

// References, Ownership, and Borrowing

/*
Ownership rules
1.	Each value in Rust has a variable that’s called its owner.
2.	There can only be one owner at a time.
3.	When the owner goes out of scope, the value will be dropped.

Borrowing rules
1.	At any given time, you can have either one mutable reference or any number of immutable references.
2.	References must always be valid.
*/
fn cheat_creating_references() {
    let s1 = String::from("hello world!");
    let s1_ref = &s1; // immutable reference

    let mut s2 = String::from("hello");
    let s2_ref = &mut s2; // mutable reference

    s2_ref.push_str(" world!");
}

fn cheat_copy_move_and_clone() {
    let x = 5; // Simple values which implement the Copy trait are copied by value
    let y = x;

    println!("{}", x); // x is still valid

    // The string is moved to s2 and s1 is invalidated
    let s1 = String::from("Let's Get Rusty!");
    let s2 = s1; // Shallow copy a.k.a move
    // println!("{}", s1); // Error: s1 is invalid - deliberate example

    let s1 = String::from("Let's Get Rusty!");
    let s2 = s1.clone(); // Deep copy
    println!("{}", s1); // Valid because s1 isn't moved
}

fn cheat_ownership_and_functions() {

    fn main() {
      let x = 5;
      takes_copy(x); // x is copied by value

      let s = String::from("Let’s Get Rusty!");
      // s is moved into the function
      takes_ownership(s);

      // return value is moved into s1
      let s1 = gives_ownership();

      let s2 = String::from("LGR");
      let s3 = takes_and_gives_back(s2);
    }

    fn takes_copy(some_integer: i32) {
      println!("{}", some_integer);
    }

    fn takes_ownership(some_string: String) {
      println!("{}", some_string);
    } // some_string goes out of scope and drop is called. The backing memory is freed.

    fn gives_ownership() -> String {
      let some_string = String::from("LGR");
      some_string
    }

    fn takes_and_gives_back(some_string: String) -> String {
      some_string
    }
}

// Pattern Matching
fn cheat_pattern_matching_basics() {
    let x = 5;

    match x {
      // matching literals
      1 => println!("one"),
      // matching multiple patterns
      2 | 3 => println!("two or three"),
      // matching ranges
      4..=9 => println!("within range"),
      // matching named variables
      x => println!("{}", x),
      // default case (ignores value)
      _ => println!("default Case")
    }
}

fn cheat_destructuring() {
    struct Point {
      x: i32,
      y: i32,
    }

    let p = Point { x: 0, y: 7 };

    match p {
      Point { x, y: 0 } => {
        println!("{}" , x);
      },
      Point { x, y } => {
        println!("{} {}" , x, y);
      },
    }

    enum Shape {
      Rectangle { width: i32, height: i32 },
      Circle(i32),
    }

    let shape = Shape::Rectangle { width: 10, height: 10 };
    let shape = Shape::Circle(10);

    match shape {
      Shape::Rectangle { width: x, height: y } => {} //...
      Shape::Circle(radius) => {} //...
    }
}

fn cheat_ignoring_values() {
    struct SemVer(i32, i32, i32);

    let version = SemVer(1, 32, 2);

    match version {
      SemVer(major, _, _) => {
        println!("{}", major);
      }
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
      (first, .., last) => {
        println!("{}, {}", first, last);
      }
    }
}

fn cheat_match_guards() {
    let num = Some(4);

    match num {
      Some(x) if x < 5 => println!("less than five: {}", x),
      Some(x) => println!("{}", x),
      None => (),
    }
}

fn cheat_bindings() {
    struct User {
      id: i32
    }

    let user = User { id: 5 };

    match user {
      User {
        id: id_variable @ 3..=7,
      } => println!("id: {}", id_variable),
      User { id: 10..=12 } => {
        println!("within range");
      },
      User { id } => println!("id: {}", id),
    }
}

// Iterators
fn cheat_iterators_usage() {
    // Methods that consume iterators
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    // Methods that produce new iterators
    let v1: Vec<i32> = vec![1, 2, 3];
    let iter = v1.iter().map(|x| x + 1);

    // Turning iterators into a collection
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

fn cheat_implementing_iterator_trait() {
    struct Counter {
      count: u32,
    }

    impl Counter {
      fn new() -> Counter {
        Counter { count: 0 }
      }
    }
    let c = Counter::new();

    impl Iterator for Counter {
      type Item = u32;

      fn next(&mut self) -> Option<Self::Item>
      {
        if self.count < 5 {
          self.count += 1;
          Some(self.count)
        } else {
          None
        }
      }
    }
}

// Error Handling
fn cheat_throw_unrecoverable_error() {
    panic!("Critical error! Exiting!");
}

fn cheat_option_enum() {

    // To make comile
    fn get_user_id(name: &str) -> Option<u32> {

        let database: Database = Database {};
        database.user_exists("");

        if database.user_exists(name) {
            return Some(database.get_id(name))
        }

        None
    }
}

fn cheat_result_enum() {
    // To make comile
    struct User {}
    struct Error { pub msg: &'static str }
    let u = User {};
    let e = Error { msg: "" };

    fn is_logged_in_as(id: u32) -> bool { false }
    fn get_user_object(id: u32) -> User { User {} }

    //fn get_user(id &str) -> bool { false }
    fn get_user(id: u32) -> Result<User, Error> {
      if is_logged_in_as(id) {
        return Ok(get_user_object(id))
      }

      Err(Error { msg: "not logged in" })
  }
}

fn cheat_question_mark_operator() {
    // To make comile
    fn get_salary(db: Database, id: i32) -> Option<u32> {
      Some(db.get_user(id)?.get_job()?.salary)
    }

    fn connect(db: Database) -> Result<Connection, Error> {
      let conn = db.get_active_instance()?.connect()?;
      Ok(conn)
    }
}

// Combinators
fn cheat_combinators_map() {
    let some_string = Some("LGR".to_owned());

    let some_len = some_string.map(|s| s.len());

    struct Error { msg: String }
    struct User { name: String }

    let string_result: Result<String, Error> = Ok("Bogdan".to_owned());

    let user_result: Result<User, Error> =
      string_result.map(|name| {
        User { name }
      });
}

fn cheat_and_then() {
    let vec = Some(vec![1, 2, 3]);
    let first_element = vec.and_then(
      |vec| vec.into_iter().next()
    );

    let string_result: Result<&'static str, _> = Ok("5");
    let number_result =
      string_result
      .and_then(|s| s.parse::<u32>());
}

// Multiple error types
fn cheat_define_custom_error_type() {
    use std::fmt;
    type Result<T> = std::result::Result<T, CustomError>;

    let r: Result<i32>;

    #[derive(Debug, Clone)]
    struct CustomError;

    impl fmt::Display for CustomError {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "custom error message")
      }
    }
}

fn cheat_boxing_errors() {
    use std::error;

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    let r: Result<i32>;
}

// Iterating over errors
fn cheat_ignore_failed_items_with_filter_map() {
    let strings = vec!["LGR", "22", "7"];
    let numbers: Vec<_> = strings
      .into_iter()
      .filter_map(|s| s.parse::<i32>().ok())
      .collect();
}

fn cheat_fail_entire_operation_with_collect() {
    let strings = vec!["LGR", "22", "7"];

    let numbers: Result<Vec<_>, _> = strings
      .into_iter()
      .map(|s| s.parse::<i32>())
      .collect();
}

fn cheat_collect_all_valid_values_and_failures_with_partition() {
    let strings = vec!["LGR", "22", "7"];

    let (numbers, errors): (Vec<_>, Vec<_>) = strings
      .into_iter()
      .map(|s| s.parse::<i32>())
      .partition(Result::is_ok);

    let numbers: Vec<_> = numbers
      .into_iter()
      .map(Result::unwrap)
      .collect();

    let errors: Vec<_> = errors
      .into_iter()
      .map(Result::unwrap_err)
      .collect();
}

// Generics, Traits, and Lifetimes
fn cheat_using_generics() {
    struct Point<T, U> {
      x: T,
      y: U,
    }

    let a = Point { x: 0, y: 0 };
    let x = a.x;
    let y = a.y;

    impl<T, U> Point<T, U> {
      fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
          x: self.x,
          y: other.y,
        }
      }
    }
}

fn cheat_defining_traits() {
    trait Animal {
      fn new(name: &'static str) -> Self;
      fn noise(&self) -> &'static str { "" }
    }

    struct Dog { name: &'static str }

    impl Dog {
      fn fetch() { } // ...
    }

    impl Animal for Dog {
      fn new(name: &'static str) -> Dog {
        Dog { name }
      }

      fn noise(&self) -> &'static str {
        "woof!"
      }
    }
}

fn cheat_default_implementations_with_derive() {
    // A tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);
}


fn cheat_trait_bounds() {
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];

      for &item in list {
        if item > largest {
          largest = item;
        }
      }

      largest
    }
}

fn cheat_impl_trait() {
    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
      let closure = move |x: i32| { x + y };
      closure
    }
}

fn cheat_trait_objects() {
    // To make compile
    pub trait Draw { }
    struct Thing { }
    impl Draw for Thing {}

    pub struct Screen {
      pub components: Vec<Box<dyn Draw>>,
    }

    // To prevent never constructed error
    let s = Screen { components: Vec::new() };
}

fn cheat_operator_overloading() {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
      x: i32,
      y: i32,
    }

    impl Add for Point {
      type Output = Point;

      fn add(self, other: Point) -> Point {
        Point {
          x: self.x + other.x,
          y: self.y + other.y,
        }
      }
    }
}

fn cheat_supertraits() {
    use std::fmt;

    trait Log: fmt::Display {
      fn log(&self) {
        let output = self.to_string();
        println!("Logging: {}", output);
      }
    }
}

fn cheat_lifetimes_in_function_signatures() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() {
        x
      } else {
        y
      }
    }
}

fn cheat_lifetimes_in_struct_definitions() {
    struct User<'a> {
      full_name: &'a str,
    }

    let u = User { full_name: "" };
}

fn cheat_static_lifetimes() {
    let s: &'static str = "Let’s Get Rusty!";
}

// Functions, Function Pointers & Closures
fn cheat_associated_functions_and_methods() {
    struct Point { x: i32, y: i32, }

    impl Point {
      // Associated function
      fn new(x: i32, y: i32) -> Point {
        Point { x, y }
      }

      // Method
      fn get_x(&self) -> i32 { self.x }
    }

    let p = Point::new(0, 0);
}

fn cheat_function_pointers() {
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
      f(arg) + f(arg)
    }
}

fn cheat_creating_closures() {
    let add_one = |num: u32| -> u32 {
      num + 1
    };
}

fn cheat_returning_closures() {
    fn add_one() -> impl Fn(i32) -> i32 {
      |x| x + 1
    }
    fn add_or_subtract(x: i32) -> Box<dyn Fn(i32) -> i32> {
      if x > 10 {
        Box::new(move |y| y + x)
      } else {
        Box::new(move |y| y - x)
      }
    }
}

// Closure_traits
/*
●	FnOnce - consumes the variables it captures from its enclosing scope.
●	FnMut - mutably borrows values from its enclosing scope.
●	Fn - immutably borrows values from its enclosing scope.
*/

fn cheat_store_closure_in_struct() {
    struct Cacher<T>
    where
      T: Fn(u32) -> u32,
    {
      calculation: T,
      value: Option<u32>,
    }

    fn calc(x: u32) -> u32 { x + 1 }
    let c = Cacher { calculation: calc, value: Some(0) };
}

fn cheat_function_that_accepts_closure_or_function_pointer() {

    fn do_twice<T>(f: T, x: i32) -> i32
      where T: Fn(i32) -> i32
    {
      f(x) + f(x)
    }
}

// Pointers
fn cheat_eferences() {
    let mut num = 5;
    let r1 = &num; // immutable reference
    let r2 = &mut num; // mutable reference
}

fn cheat_raw_pointers() {
    let mut num = 5;
    // immutable raw pointer
    let r1 = &num as *const i32;
    // mutable raw pointer
    let r2 = &mut num as *mut i32;
}

// Smart pointers
// Box<T> - for allocating values on the heap
fn cheat_smart_pointers_box() {
    let b = Box::new(5);
}

// Rc<T> - multiple ownership with reference counting
fn cheat_smart_pointers_rc() {
    use std::rc::Rc;
    let a = Rc::new(5);
    let b = Rc::clone(&a);
}

// Ref<T>, RefMut<T>, and RefCell<T> - enforce borrowing rules at runtime instead of compile time.
fn cheat_smart_pointers_ref() {
    use std::cell::RefCell;
    let num = 5;
    let r1 = RefCell::new(5);
    // Ref - immutable borrow
    let r2 = r1.borrow();
    // RefMut - mutable borrow
    let r3 = r1.borrow_mut();
    // RefMut - second mutable borrow
    let r4 = r1.borrow_mut();
}

fn cheat_multiple_owners_of_mutable_data() {
    use std::rc::Rc;
    use std::cell::RefCell;
    let x = Rc::new(RefCell::new(5));
}
