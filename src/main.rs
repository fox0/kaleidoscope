use crate::lexer::Tokenizer;

mod lexer;

fn main() {
    let t = Tokenizer::new("\
# Compute the x'th fibonacci number.
def fib(x)
  if x < 3 then
    1
  else
    fib(x-1)+fib(x-2)

# This expression will compute the 40th number.
fib(40)
");
    for i in t {
        println!("{:?}", i);
    }
}
