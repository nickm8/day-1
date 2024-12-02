/// Historian Distance Calculator
/// Calculates the total distance between two lists of numbers based on their sorted pairs.

fn main() {
    let left_list = vec![3, 4, 2, 1, 3, 3];
    let right_list = vec![4, 3, 5, 3, 9, 3];
    
    // Create copies of lists to sort
    let mut sorted_left = left_list.clone();
    let mut sorted_right = right_list.clone();
    
    // Sort both lists
    sorted_left.sort();
    sorted_right.sort();
    
    let mut total_distance = 0;
    
    // Calculate distances between paired numbers
    for (i, (&left, &right)) in sorted_left.iter().zip(sorted_right.iter()).enumerate() {
        let distance = (left as i32 - right as i32).abs();
        total_distance += distance;
        
        println!("Pair {}: {} and {}, distance = {}", 
                 i + 1, left, right, distance);
    }
    
    println!("\nTotal distance: {}", total_distance);
}
