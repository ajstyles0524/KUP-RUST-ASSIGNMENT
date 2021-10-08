/// Return String in sorted form
pub fn sort_char(input: &str) -> String {
    let mut s: Vec<char> = input.chars().collect();
    s.sort_unstable();
    let j: String = s.into_iter().collect();
    return j;
}
pub fn rotate_check(string1: &str, string2: &str, track_index: usize)-> bool{
    let len1=string1.len();
    let len2=string2.len();
    let sorted_input1 = sort_char(string1);
    let sorted_input2 = sort_char(string2);

    if len1 != len2 {
        return false;
    }
    if sorted_input1.chars().nth(track_index) != sorted_input2.chars().nth(track_index) {
        return false;
    }
    if track_index > len1 || track_index > len2 {
        return true;
    }
    else {
        rotate_check(&sorted_input1,&sorted_input2, track_index + 1)
    }
}




