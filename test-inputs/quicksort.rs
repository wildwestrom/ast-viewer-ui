// Credit: https://gist.github.com/rust-play/0c07ebb4e388f009e2b85e91823b1bb8
// quicksort :: (Ord a) => [a] -> [a]
// quicksort [] = []
// quicksort (x:xs) =
// let smallerSorted = quicksort [a | a <- xs, a <= x]
// biggerSorted = quicksort [a | a <- xs, a > x]
// in  smallerSorted ++ [x] ++ biggerSorted

pub fn quicksort<'a, T: Ord>(xs: &[&'a T]) -> Vec<&'a T> {
	match xs {
		[] => vec![],
		[ref x, xs @ ..] => {
			let smaller = quicksort(&xs.iter().map(|a| *a).filter(|a| a <= x).collect::<Vec<_>>());

			let bigger = quicksort(&xs.iter().map(|a| *a).filter(|a| a > x).collect::<Vec<_>>());

			[smaller, vec![x], bigger].concat()
		},
	}
}

#[test]
fn it_works() {
	assert_eq!(quicksort(&[&5, &1, &3, &0, &9]), &[&0, &1, &3, &5, &9]);
}
