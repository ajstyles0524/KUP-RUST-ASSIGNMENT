pub fn count_leap() {
    let arr: [i32; 5] = [2016, 2020, 2000, 2100, 2200];
    let mut count = 0;
    for num in 0..5
    {
        if arr[num] % 4 == 0 && arr[num] % 100 != 0 || arr[num] % 400 == 0
        {
            count += 1;
        }
    }
    print!("{}", count);
}