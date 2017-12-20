mod test;

type Coordinate = (i32,i32);

fn num_of_steps(start_position : u32) -> u32
{
    //calculate coordinates and call manhattan_distance()
    panic!("");
}

fn find_nearest_odd_square()
{
    
}

fn manhattan_distance(start_position : Coordinate, destination_position : Coordinate) -> i32
{
    (start_position.0 - destination_position.0).abs() + (start_position.1 - destination_position.1).abs()
}