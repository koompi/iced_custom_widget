
use std::mem;
use std::fs::File;
use std::io::{
	prelude::*, BufRead, Result, BufReader, Lines
};
use std::path::Path;

#[derive(Debug)]
struct Test {
	is_testable: bool,
	example: Example,
}

impl HasTest for Test {
	fn is_valid(&self) -> bool {
		self.is_testable
	}
}

struct DebitData {
	pub card_number: String,
	pub amount: f32,
}

#[allow(dead_code)]
enum Payment {
	Cash(f32),
	CreditCard(String, f32),
	DebitCard(DebitData),
	Crypto { account_id: String, amount: f32 },
}

#[derive(Debug)]
struct Point<T> {
	x: T,
	y: T,
}

#[allow(dead_code)]
enum Status<T> {
	Success(T),
	Warning(T),
	Error(T)
}

enum Step {
	Requirement,
	Analyze,
	Design,
	Implement,
	Test,
	Deploy
}

#[derive(Debug)]
pub struct Example {
    pub a_bool: bool,
    pub a_i32: i32,
    pub a_f32: f32,
    pub a_string: String,
}

#[allow(dead_code)]
impl Example {
    pub fn new(is_new: bool) -> Self {
        Self {
            a_bool: is_new,
            a_i32: 127,
            a_f32: 12.27,
            a_string: "example".to_string(),
        }
    }

    pub fn is_smaller(&self, compare_to: i32) -> bool {
        self.a_i32 < compare_to
    }

    pub fn increase(&mut self) {
        self.a_i32 += 1;
        self.a_f32 += 1.;
        self.a_string += " ...";
    }
}

pub trait HasTest {
    fn is_valid(&self) -> bool;
}


fn is_odd(n: u32) -> bool {
	n % 2 == 1
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path> {
	if let Ok(file) = File::open(filename) {
		Ok(BufReader::new(file).lines())
	} 
	else {
		panic!("File can not open")
	}
}

fn apply<F: FnOnce()>(f: F) {
	f();
}

fn applu_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
	f(3)
}

fn get_age() -> Option<u8> {
	Some(21)
}

#[allow(dead_code)]
fn dump_func<T>(a: T, b: T) -> T
where T: std::ops::Add<Output=T> + std::fmt::Debug{
	println!("A: {:?}, B: {:?}", a, b);	
	a + b
}

#[allow(dead_code)]
fn generic_fn<T>(a: T, b: T) -> T
where
	T: HasTest + std::ops::Add<Output = T> + std::fmt::Debug,
{
	println!("A:{:?}, B:{:?}", a, b);
	a + b
}

#[allow(dead_code)]
fn process_payment(payment: &Payment) {
	match payment {
		Payment::Cash(amount) => println!("Paying with cash.......In amount of {}", amount),
		Payment::CreditCard(_, amount) => {
			println!("Paying with credit card.......Amount:{}", amount)
		}
		Payment::DebitCard(data) => println!(
			"Paying with debit card......CN:{}, amount:{}",
			data.card_number, data.amount
		),
		Payment::Crypto { account_id, amount } => println!(
			"Paying with crypto........ACID:{}, amount:{}",
			account_id, amount
		),
	}
}

#[allow(dead_code)]
fn printif_has_test(obj: &dyn HasTest) {
	if obj.is_valid() {
		println!("Test it!!!!");
	}
}

fn greeting(name: &str) {
	println!("HEY, {}. How is going?", name);
}

#[allow(dead_code)]
fn sum(a: i8, b: i8) -> i8 {
	a + b
}

