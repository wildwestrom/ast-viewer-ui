//Credit: https://gist.github.com/mrkishi/bffb145c3a4be6e2ca2911e7198acdb6
macro_rules! count {
	() => { 0 };
	($($half:tt $_:tt)*) => { count!($($half)*) * 2 };
	($one:tt $($half:tt $_:tt)*) => { count!($($half)*) * 2 + 1 };
}

const TOTAL: usize = count!(+++++++);

pub fn main() {
	println!("{}", TOTAL);
}
