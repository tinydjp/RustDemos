pub fn sum_list(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut flowed = false;
    for &item in arr {
        if sum.checked_add(item) == None {
            flowed = true;
            break;
        }
       sum += item;
    }
    if flowed {
        None
    } else {
        Some(sum)
    }
}