pub fn merge_sort(array: &mut[i32]) {
	let mid = array.len()/2;
	if mid < 1 {
		return;
	}

	merge_sort(&mut array[..mid]);
	merge_sort(&mut array[mid..]);

	let mut merged_array = array.to_vec();

	merge(&array[..mid], &array[mid..], &mut merged_array);

	array.copy_from_slice(&merged_array);
}

pub fn merge(array_1: &[i32], array_2: &[i32], result_array: &mut[i32]) -> () {
	let mut array_1_index = 0;
	let mut array_2_index = 0;
	let mut result_array_index = 0;

	while array_1_index < array_1.len() && array_2_index < array_2.len() {
		if array_1[array_1_index] < array_2[array_2_index] {
			result_array[result_array_index] = array_1[array_1_index];
			array_1_index += 1;
		}
		else {
			result_array[result_array_index] = array_2[array_2_index];
			array_2_index += 1;
		}
		result_array_index += 1;
	}

	if array_1_index < array_1.len() {
		result_array[result_array_index..].copy_from_slice(&array_1[array_1_index..]);
	}
	else if array_2_index < array_2.len() {
		result_array[result_array_index..].copy_from_slice(&array_2[array_2_index..]);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

  #[test]
  fn sort_single_element_array() {
  	let mut array = [1];
  	merge_sort(&mut array);

    assert_eq!(array, [1]);
  }

  #[test]
  fn sort_two_element_array() {
  	let mut array = [2,1];
  	merge_sort(&mut array);

  	assert_eq!(array, [1,2]);
  }

  #[test]
  fn sort_array() {
  	let mut array = [1,4,3,7,23,6,4];
  	merge_sort(&mut array);

  	assert_eq!(array, [1,3,4,4,6,7,23]);
  }

  #[test]
  fn sort_empty_array() {
  	let mut array = [];
  	merge_sort(&mut array);

  	assert_eq!(array, []);
  }
}
