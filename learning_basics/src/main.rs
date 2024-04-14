fn main() {
    //sorting algorithm from small to big
    let mut array = [10, 7, 3, 0, 4, 2, 9];
    
    for i in 0..array.len() - 1{
        for j in 0..array.len() - 1{
            
            if array[j] > array[j+1] {
                let temp = array[j+1];
                array[j+1] = array[j];
                array[j] = temp;         
            } 
        }
        println!("iteration {}", i + 1);
        for x in 0..array.len() {
            print!("{}  ", array[x]);
        }
        println!("");
    }

}