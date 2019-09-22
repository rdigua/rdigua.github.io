fn main(){
let mut a = "Hel";
print!("{}", a);
a = "lo";
print!("{}\n", a);
//This will print "Hello". 
use std::mem::*;
let a: &str = "";
let b: &str = "0123456789";
let c: &str = "abcdè";
print!("{} {} {}",
size_of_val(a),
size_of_val(b),
size_of_val(c));
//This program will print 0 10 6.
// use std::mem::*;
let a: &str = "";
let b: &str = "0123456789";
let c: &str = "abcdè";
print!("{} {} {}; ",
size_of_val(&a),
size_of_val(&b),
size_of_val(&c));
print!("{} {} {}\n",
size_of_val(&&a),
size_of_val(&&b),
size_of_val(&&c));
//This program in a 64-bit system will print "16 16 16; 8 8 8", while in a 32-bit system it will print "8 8 8; 4 4 4".
let mut a: String = "He".to_string();
a.push('l');
a.push('l');
a.push('o');
print!("{}\n", a);
//This will print "Hello".
let mut a: String = "Xy".to_string(); // "Xy"
a.remove(0); // "y"
a.insert(0, 'H'); // "Hy"
a.pop(); // "H"
a.push('i'); // "Hi"
print!("{}\n", a);
//This prints "Hi".
let mut s1 = "".to_string();
s1.push('e');
let mut s2 = "".to_string();
s2.push('è');
let mut s3 = "".to_string();
s3.push('€');
print!("{} {}; ", s1.capacity(), s1.len());
print!("{} {}; ", s2.capacity(), s2.len());
print!("{} {}\n", s3.capacity(), s3.len());
//This may print: "4 1; 2 2; 3 3".
let mut s = "".to_string();
for _ in 0..10 {
println!("{:?} {} {}",
s.as_ptr(), s.capacity(), s.len());
s.push('a');
}
println!("{:?} {} {}: {}\n",
s.as_ptr(), s.capacity(), s.len(), s);
//This, in a 64-bit system, may print:
}
