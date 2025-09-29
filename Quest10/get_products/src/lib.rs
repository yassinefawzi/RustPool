pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut counter: usize = 0;
    let mut product_results: Vec<usize> = Vec::new();

    if arr.len() < 2 {
        return product_results;
    };

    for _ in arr.iter() {
        let mut prod: usize = 1;
        let mut others: Vec<usize> = arr.clone();
        others.remove(counter);
        for x in others.iter() {
            prod *= *x;
        }
        product_results.push(prod);
        counter += 1;
    }
    return product_results;
}