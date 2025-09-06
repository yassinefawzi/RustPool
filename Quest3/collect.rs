pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for _ in 0..len{
        for j in 0..len-1{
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}
