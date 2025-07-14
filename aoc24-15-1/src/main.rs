mod map_reader;

use crate::map_reader::{Location, OneDimFromFile, TwoDimFromFile, DirectionOfMotion};

const       ROBOT_CHARACTER:char = '@';
const        WALL_CHARACTER:char = '#';
const     MOVABLE_CHARACTER:char = 'O';
const EMPTY_SPACE_CHARACTER:char = '.';



fn main() {

    let mut gamemap_2d = TwoDimFromFile::new();    
    let gameinstructions_1d = OneDimFromFile::new();

    TwoDimFromFile::print(&gamemap_2d);
    OneDimFromFile::print(&gameinstructions_1d);

    let mut robot_location:map_reader::Location  = TwoDimFromFile::locate_char(&gamemap_2d,ROBOT_CHARACTER);

    let mut playable_game_map= &mut gamemap_2d;

    let game_score = play_game(playable_game_map, gameinstructions_1d, &mut robot_location);
    println!("\nScore : {}", game_score);
}



fn play_game(game_map:&mut TwoDimFromFile, game_instructions:OneDimFromFile, robot_location:&mut Location)-> i32
{
    for next_instruction in game_instructions.contents.iter()
    {
        make_moves(robot_location, next_instruction, game_map)
    }
    
    
    let score = calculate_score(game_map);
    
    return score;
}

fn look_1block_in_movement_direction(move_origin_point_x:&usize, move_origin_point_y:&usize, game_map:&TwoDimFromFile)->char
{
    return game_map.contents[*move_origin_point_y][*move_origin_point_x];
}

fn get_next_location(move_loc:&usize, move_dir:&i32)->usize
{
    match move_dir
    {
        -1 => {
                if *move_loc > std::usize::MIN 
                {
                    return *move_loc - 1 as usize;
                }
                else {
                    return *move_loc;
                }
              },
        1  => {
                if *move_loc < std::usize::MAX
                {
                    return *move_loc + 1 as usize;
                }
                else {
                    return *move_loc;
                }
              },
         _ => return *move_loc,
    }

}

fn is_movement_possible(robot_location:&Location, game_map:&TwoDimFromFile, move_x:&i32, move_y:&i32)->bool
{
    let mut move_origin_point_x = robot_location.x;
    let mut move_origin_point_y = robot_location.y;
    let mut previous_point_x = robot_location.x;
    let mut previous_point_y = robot_location.y;
    
    loop
    {
        previous_point_x = move_origin_point_x;
        previous_point_y = move_origin_point_y;
        //println!("Origin point is {}, {}", previous_point_y, previous_point_x);


        move_origin_point_x = get_next_location(&previous_point_x, move_x);
        move_origin_point_y = get_next_location(&previous_point_y, move_y);
        //println!("Next point is {}, {}", move_origin_point_y, move_origin_point_x);
        
        if previous_point_x == move_origin_point_x && move_origin_point_y == previous_point_y
        {
            // Cant Move 
            return false;
        }

        let next_block:char = look_1block_in_movement_direction(&move_origin_point_x , &move_origin_point_y, game_map);

        if next_block == EMPTY_SPACE_CHARACTER
        {
            // Movement possible
            return true;
        }
        else if next_block == WALL_CHARACTER
        {
            return false;
        }
        else  //MOVABLE_CHARACTER
        {

            continue; // keep looking
        }

    }

}

fn collect_all_movable_entities(robot_location:&Location, game_map:&TwoDimFromFile, move_x:&i32, move_y:&i32)->Vec<(Location, char)>
{
    let mut movable_object_list: Vec<(Location, char)> = Vec::new();
    movable_object_list.push( (Location{ x: robot_location.x, y: robot_location.y }, ROBOT_CHARACTER));

    let mut move_origin_point_x = robot_location.x;
    let mut move_origin_point_y = robot_location.y;
    let mut previous_point_x = robot_location.x;
    let mut previous_point_y = robot_location.y;
    
    loop
    {
        previous_point_x = move_origin_point_x;
        previous_point_y = move_origin_point_y;
        //println!("Origin point is {}, {}", previous_point_y, previous_point_x);


        move_origin_point_x = get_next_location(&previous_point_x, move_x);
        move_origin_point_y = get_next_location(&previous_point_y, move_y);
        //println!("Next point is {}, {}", move_origin_point_y, move_origin_point_x);
        
        if previous_point_x == move_origin_point_x && move_origin_point_y == previous_point_y
        {
            // Cant Move 
            break;
        }

        let next_block:char = look_1block_in_movement_direction(&move_origin_point_x , &move_origin_point_y, game_map);

        if next_block == EMPTY_SPACE_CHARACTER
        {
            break;
        }
        else if next_block == MOVABLE_CHARACTER
        {
            movable_object_list.push( (Location { x:move_origin_point_x, y:move_origin_point_y}, MOVABLE_CHARACTER));
        }
        else if next_block == WALL_CHARACTER
        {
            break;
        }

    }

    //println!("The movable locations for this move instructions are ");
    //for elements in movable_object_list.iter()
    //{
    //    println!("[{}, {}] for {}", elements.0.y, elements.0.x, elements.1);
    //}


    return movable_object_list;
}


fn perform_move(movable_object_list: Vec<(Location,char)>, robot_location:&mut Location, game_map:&mut TwoDimFromFile, move_x:&i32, move_y:&i32)
{
    for movable_object in movable_object_list.iter()
    {
        game_map.contents[movable_object.0.y][movable_object.0.x] = EMPTY_SPACE_CHARACTER;        
    }

    for movable_object in movable_object_list.iter()
    {
        let move_origin_point_x = get_next_location(&movable_object.0.x, move_x);
        let move_origin_point_y = get_next_location(&movable_object.0.y, move_y);
        game_map.contents[move_origin_point_y][move_origin_point_x] = movable_object.1; 

        if movable_object.1 == ROBOT_CHARACTER
        {
            robot_location.x = move_origin_point_x;
            robot_location.y = move_origin_point_y;
        }
    }

}

fn calculate_score(game_map:&TwoDimFromFile)->i32
{
    let mut score:i32 = 0;
      
    for(index_row, each_row) in game_map.contents.iter().enumerate()
    {
        for (index_col, element) in each_row.iter().enumerate()
        {
            if *element == MOVABLE_CHARACTER
            {
                score= score + index_row as i32*100 + index_col as i32; 
            }
        }
    }

    return score;
}




fn make_moves(robot_location:&mut Location, direction_of_motion:&DirectionOfMotion, game_map:&mut TwoDimFromFile)
{
    let (move_x, move_y) = DirectionOfMotion::direction_of_motion_to_move_values(direction_of_motion); 

    //println!("move directions are {}, {}", move_y, move_x);

    if is_movement_possible(robot_location, &game_map, &move_x, &move_y)
    {
        let mut movable_object_list: Vec<(Location, char)> = collect_all_movable_entities(robot_location, &game_map, &move_x, &move_y);
        perform_move(movable_object_list, robot_location, game_map, &move_x, &move_y);

        //println!("New map after moves should look like : ");
        //TwoDimFromFile::print(&game_map);
        //println!("New robot position at [{}, {}]", robot_location.y,robot_location.x);
    }
}

