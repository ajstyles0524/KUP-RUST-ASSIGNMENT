use text_io::read;
mod merge_sort;
mod linear_binary;
mod check_leap;

fn main() {
    println!("Assignment-2, Anand");
    println!("enter decision for apply which you want \n
    1:- Binary Search \n
    2:- LinearSearch \n
    3:- mergeSort \n
    4:- Leap \n");
    let choice: i32 = read!();

    let arr1: [i32; 5] = [10, 20, 30, 40, 50];
    let arr2: [i32; 5] = [10, 30, 60, 80, 50];
    let arr3 = [10, 30, 60, 80, 100];
    //for search
    if choice == 1 || choice == 2
    {
        println!("Enter Item which is to be searched \n");
        let search_element: i32 = read!();
        if choice == 1
        {

           linear_binary::binary_search(&arr1, 0, 4, search_element);
        }
        //for linear search
        else if choice == 2
        {
            linear_binary::lsearch(&arr2 , search_element, 0);
        }
    }
    else if choice == 3 {
       merge_sort::mergesortmain(&arr3);
    }
    else
    {
        check_leap::count_leap();
    }

}
