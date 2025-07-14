//use std::env;
use std::fs;

const FILE_PATH_2D:&str = "./2d_input.txt";
const FILE_PATH_1D:&str = "./1d_input.txt";

pub enum DirectionOfMotion{
      Up,
      Down,
      Left,
      Right,
}

impl DirectionOfMotion
{
  
  pub fn direction_of_motion_to_move_values(move_direction:&DirectionOfMotion)->(i32, i32)
  {
    match *move_direction {
      DirectionOfMotion::Up    => return(  0, -1),
      DirectionOfMotion::Right => return(  1,  0),
      DirectionOfMotion::Left  => return( -1,  0),
      DirectionOfMotion::Down  => return(  0,  1),
    }
  }

}

pub struct Location
{
  pub x:usize,
  pub y:usize, 
}


pub struct TwoDimFromFile
{
  pub contents:Vec<Vec<char>>
}

impl TwoDimFromFile
{

    pub fn new()-> Self
    {
       Self{contents:read_from_2d_file()}
    }

    pub fn print(&self)
    {
      println!("Game Field");
      for s in self.contents.iter()
      {
        for v in s.iter()
          {
            print!(" {v}");
          }
        print!("\n");
      }
    }

    pub fn locate_char(game_map:&TwoDimFromFile, char_to_locate:char)->Location
    {
      let mut char_location = Location {x:0, y:0};
      
      for (index, each_row) in game_map.contents.iter().enumerate()
      {
          let result_index = each_row.iter().position(|&element| element == char_to_locate);
          
          match result_index{
              None => (),
                _  => {
                          char_location.x = result_index.unwrap();
                          char_location.y = index; 
                      },
          }
      }

      //println!("The Robot starting location is [{}, {}]",char_location.y, char_location.x);
      char_location
    }
    
}

fn read_from_2d_file()->Vec<Vec<char>>
{
  let file_string_contents = fs::read_to_string(FILE_PATH_2D).expect("Should have been able to read the file");
  let file_string_contents: Vec<&str> = file_string_contents.split('\n').collect();

  let mut game_map:Vec<Vec<char>> = Vec::new(); 

  for string_content in file_string_contents.iter()
  {
      game_map.push(string_content.chars().collect());
  }
  game_map
}

pub struct OneDimFromFile
{
  pub contents:Vec<DirectionOfMotion>
}

impl OneDimFromFile
{

    pub fn new()-> Self
    {
       Self{contents:read_from_1d_file()}
    }

    pub fn print(&self)
    {
      println!("Game instructions");
      for elements in self.contents.iter()
      {
        match elements {
        DirectionOfMotion::Up    => print!("^"),
        DirectionOfMotion::Right => print!(">"),
        DirectionOfMotion::Left  => print!("<"),
        DirectionOfMotion::Down  => print!("v"),
      }
      }
    }

}

fn read_from_1d_file()->Vec<DirectionOfMotion>
{
  let file_string_contents = fs::read_to_string(FILE_PATH_1D).expect("Should have been able to read the file");
  let file_string_contents: Vec<char> = file_string_contents.chars().collect();

  let file_content_one_direction:Vec<DirectionOfMotion> = convert_char_directions_to_enum_type(file_string_contents);

  file_content_one_direction
}

fn convert_char_directions_to_enum_type(input_list:Vec<char>)->Vec<DirectionOfMotion>
{
  let mut result_list: Vec<DirectionOfMotion> = Vec::new();
  for elements in input_list
  {
      match elements {
        '^' => result_list.push(DirectionOfMotion::Up),
        '>' => result_list.push(DirectionOfMotion::Right),
        '<' => result_list.push(DirectionOfMotion::Left),
        'v' => result_list.push(DirectionOfMotion::Down),
          _ => println!("Error in sequence"),
      }
  }

  result_list
}


