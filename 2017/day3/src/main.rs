fn main() {
    
    let mut layer: i64 = 0;
    let target = 265149;
    loop {
        let x = layer * 2 +  1;

        if i64::pow(x, 2) >= target {
            break

        } else {
            layer += 1;
        }
    }
    //we now know what the layer that the raget will be in.
    //next we need to traverse to the target point
    //maybe find the difference of the corner point and the target value
    let difference = i64::pow(layer * 2 + 1,2) - target;
    
    //println!("{}",difference);

    //have to move 76 spaces, which will be along the bottom line
    let mut x = layer;
    let y = -1 * layer;

    //println!("{},{}", x,y);
    x = x - difference;

    let distance = i64::abs(0-x) + i64::abs(0-y);

    println!("{}",distance);
}

// part 2 is availavle online, https://oeis.org/A141481/b141481.txt