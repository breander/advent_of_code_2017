use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let number: i32 = args[1].parse().unwrap();
    part1(number);
    part2(number);
}

fn part1(number: i32){
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut steps_taken: i32 = 0;
    let mut dir: i32 = 0;

    let mut max_steps: i32 = 1;
    let mut directions_traveled: i32 = 0;

    for _i in 1..number{
        //right
        if dir == 0 {
            x = x + 1;
        }
        //up
        if dir == 1 {
            y = y + 1;
        }
        //left 
        if dir == 2 {
            x = x - 1;
        }
        //down
        if dir == 3 {
            y = y - 1;
        }

        //increment steps taken
        steps_taken += 1;

        //check if steps taken is the same as max steps
        if steps_taken == max_steps{
            //reset the number of steps taken
            steps_taken = 0;

            //increase directions traveled
            directions_traveled += 1;
            
            // every 2 directions we increase the number of max steps
            if directions_traveled > 1 {
                // increase max_steps by 1
                max_steps += 1;

                // reset directions traveled
                directions_traveled = 0;
            }
            
            //change the direction
            dir += 1;
            
            //check if direction has looped around
            if dir > 3{
                //set direction back to right
                dir = 0
            }
        }
    }

    println!("Part 1 distance: {}", i32::abs(x - 0) + i32::abs(y - 0));
}

fn part2(_number: i32){
    //initialize 2d array all to 0
    let mut array_2d = [[0i32; 1024]; 1024];

    let mut steps_taken: i32 = 0;
    let mut dir: i32 = 0;

    let mut max_steps: i32 = 1;
    let mut directions_traveled: i32 = 0;

    //start in center
    let mut x: usize = 512;
    let mut y: usize = 512;

    //start sum = 1
    let mut sum: i32 = 1;

    //set center to sum
    array_2d[x][y] = sum;

    //now start circling
    while sum < _number{
        //set sum to 0
        sum = 0;

        //right
        if dir == 0 {
            x = x + 1;
        }
        //up
        if dir == 1 {
            y = y + 1;
        }
        //left 
        if dir == 2 {
            x = x - 1;
        }
        //down
        if dir == 3 {
            y = y - 1;
        }

        //increment steps taken
        steps_taken += 1;

        //check if steps taken is the same as max steps
        if steps_taken == max_steps{
            //reset the number of steps taken
            steps_taken = 0;

            //increase directions traveled
            directions_traveled += 1;
            
            // every 2 directions we increase the number of max steps
            if directions_traveled > 1 {
                // increase max_steps by 1
                max_steps += 1;

                // reset directions traveled
                directions_traveled = 0;
            }
            
            //change the direction
            dir += 1;
            
            //check if direction has looped around
            if dir > 3{
                //set direction back to right
                dir = 0
            }
        }

        //calculate sum of all neighboring digits
        sum += array_2d[x+1][y]; //east
        sum += array_2d[x+1][y+1]; //north east
        sum += array_2d[x][y+1]; //north
        sum += array_2d[x-1][y+1]; //north west
        sum += array_2d[x-1][y]; //west
        sum += array_2d[x-1][y-1]; //south west
        sum += array_2d[x][y-1]; //south
        sum += array_2d[x+1][y-1]; //south east

        //set current position to sum
        array_2d[x][y] = sum;

        //println!("Sum: {}", sum);
    }
    
    //print the answer
    println!("Part 2: {}", sum);
}
