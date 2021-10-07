// linear search using recursion amd immutable variables
pub fn lsearch(arr:&[i32], search_element: i32, index: i32)
{
    if arr.len() == index as usize
    {
        println!("Number is not present in the given array");
        return;
    }
    if arr[index as usize] == search_element
    {
        println!("\n Item {} found at index {}\n", search_element  , index);
        return ;
    }
    lsearch(arr, search_element, index+1);
}

// binary search using recursion and immutable variables

pub fn binary_search(arr: &[i32], starting_index: i32, ending_index: i32 , search_element: i32)
{
    if ending_index > starting_index
    {
        //for find element
        let mid_index = starting_index + (ending_index - 1) / 2;

        if arr[mid_index as usize] == search_element
        {
            println!("\n Item {} found at index {}\n", search_element, mid_index);
            return;
        } else if arr[mid_index as usize] < search_element {
            binary_search(arr, mid_index + 1, ending_index, search_element)
        } else {
            binary_search(arr, starting_index, mid_index - 1, search_element)
        }
    }
}