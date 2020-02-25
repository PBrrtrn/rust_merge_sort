pub fn quick_sort(array: &mut[i32]) {
	_quick_sort(array, 0, array.len()-1);
}

pub fn _quick_sort(array: &mut[i32], start: usize, end: usize) {

	if end > start {
		let pivot_index = partition(array, start, end);

		_quick_sort(array, start, pivot_index);
		_quick_sort(array, pivot_index+1, end);
	}

}

pub fn partition(array: &mut[i32], start: usize, end: usize) -> usize {
	let mut i = start;
	let mut j = end;

	while i < j {
		while (array[i] < array[start]) && (i < end) { i += 1 };
		while (array[j] >= array[start]) && (j > start) { j -= 1 };

		println!("DEBUG: swapping {} with {}", array[i],array[j]);
		array.swap(i,j);
	}
	array.swap(start,i);

	i
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sort_single_element_array() {
		let mut array = [1];
		quick_sort(&mut array);

		assert_eq!(array, [1]);
	}

	#[test]
	fn sort_two_element_array() {
		let mut array = [2,1];
		quick_sort(&mut array);

		assert_eq!(array, [1,2]);
	}

	#[test]
	fn sort_array() {
		let mut array = [1,4,3,7,23,6,4];
  	quick_sort(&mut array);

  	assert_eq!(array, [1,3,4,4,6,7,23]);
	}
}