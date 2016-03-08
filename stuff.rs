/*! Parser and AST to `SubGrid` functions.

Used to parse user initial sub-grids.

# Submission

> by adrien champion

> with nobody

> with nobody


# Report

**A few lines about what you did.**

**If you worked in a group you are required to discuss what you did, and what
the other group members did in a few lines.**


# Extra task 1

Blah blah blah

```
println!("some code to talk about")
```

# Extra task 2

Blah blah blah

```
println!("some code to talk about")
```

*/

use cfg::{ Syntax, MetaData } ;
use range::Range ;

use grid::Status ;
use frontend::SubGrid ;
use std::sync::Arc;

/// Creates the parser for user initial sub-grids.
pub fn mk_parser() -> Result<Syntax, String> {
  use cfg::* ;
  let rules = r#"

    8 comment = ["//" ..."\n"?] 
    1 player = ["player" .w? ":" .w? ?comment .w? .t!:"player_name" .w?]
    2 dead = { "0":!"alive" "_":!"alive" ".":!"alive" }
    3 alive = { "1":"alive" "*":"alive" "x":"alive" }
    4 line = [.w? .r!({alive dead}) "," .w?]
    5 try = {"0":!"alive" "_":!"alive" ".":!"alive" 
        "1":"alive" "*":"alive" "x":"alive" 
        comment "," " " "\n"}
    7 grid = ["grid" .w? "(" .w? ?comment .w?
    .$:"height" .w? "," .w? ?comment .w?
    .$:"width" .w? ")" .w? "{" .r!(try) .w? "}"]
    88 document = [
    .l(comment)
    .l(player)
    .l(comment)
    .l(grid)
    .l(comment)
    ]
  "# ;
  
  match syntax_errstr(rules) {
    Err(err) => return Err(
      format!("could not create parser:\n{}", err)
    ),
    Ok(parser) => Ok(parser),
  }
}




/// Turns an AST produced by parsing using the parser from `mk_parser` into a
/// `SubGrid`.
pub fn ast_to_sub_grid(
  id: usize, ast: & [ Range<MetaData> ]
) -> Result<SubGrid, String> {
  use cfg::MetaData::* ;

  let mut height : i32 = 0 ;
  let mut widthh : i32 = 0 ;
  let mut v = vec![]; // a vec to store all the status
  let mut name : &String = &"ASD".to_string(); // player name
  for token in ast.iter() {
    println!("{:?}", token)
    match (token.data) {
      MetaData::String(ref x,ref y) => {
        name = &(**y); // match player_name
      },
      MetaData::F64(ref x, ref y) => {
        if **x == "height" {
          height = *y as i32; // match height
        }
        else{
          widthh = *y as i32; // match widthh
        }
      },
      MetaData::Bool(ref x, ref y) => {
        // println!("Yes");
        if (*y == true){
          v.push(Status::Live(id)); // match live status
        }
        else{
          v.push(Status::Dead); // match dead status
        }
      },

      _ => {}, // match nothing
    };
  }

  let mut result = vec![]; // vec<vec<Status>>
  let mut t = vec![]; // temp variable
  let mut i = 0; // iter variable
  for status in v.iter() {
    i += 1;
    t.push(status.clone());
    if i == widthh {
      i = 0;
      result.push(t.clone()); // push a row into vec<vec<Status>>
      t.clear();
    }
  }
  let subgrid = SubGrid::mk(name.clone(), result);


  if v.len() == (widthh * height) as usize {
    Result::Ok(subgrid)
  } else {
    Result::Err("I don't care.".to_owned()) // input error
  }
}








/* |=====| TEST STUFF. |=====| */






/// Parses a string and creates the sub-grid.
#[cfg(test)]
pub fn get_grid(id: usize, to_parse: & str) -> Result<SubGrid, String> {
  use cfg::parse_errstr ;
  // Token buffer.
  let mut ast = vec![] ;
  // Creating parser.
  let parser = try!( mk_parser() ) ;
  // Parsing input string.
  try!(
    parse_errstr(& parser, to_parse, & mut ast)
  ) ;
  // Returning subgrid.
  ast_to_sub_grid(id, & ast)
}

/// Prints a grid in the input format.
///
/// For debugging in tests.
#[cfg(test)]
pub fn grid_to_str(grid: & SubGrid) -> String {
  let player = grid.player() ;
  let grid = grid.grid() ;
  let (rows, cols) = (grid.len(), grid[0].len()) ;
  let header = format!(
    "\
      player: \"{}\"\n\n\
      grid ({}, {}) {{\
    ", player, rows, cols
  ) ;
  let just_misses_footer = grid.iter().fold(
    header,
    |s, row| {
      let no_comma = row.iter().fold(
        format!("{}\n  ", s),
        |s, cell| {
          format!(
            "{}{}", s, match * cell {
              Status::Dead => "_",
              Status::Live(_) => "x",
            }
          )
        }
      ) ;
      format!("{},", no_comma)
    }
  ) ;
  format!("{}\n}}", just_misses_footer)
}

/// Checks two grids are equal, prints an error and panics otherwise.
#[cfg(test)]
pub fn check_same(
  input: & str, expected: SubGrid, to_check: Result<SubGrid, String>
) {
  match to_check {
    Ok(grid) => if expected == grid { () } else {
      println!("") ;
      println!("|===| Error on input") ;
      for line in input.lines() {
        println!("| {}", line)
      } ;
      println!("|===| expected grid") ;
      for line in grid_to_str(& expected).lines() {
        println!("| {}", line)
      } ;
      println!("|===| but got grid:") ;
      for line in grid_to_str(& grid).lines() {
        println!("| {}", line)
      } ;
      println!("|===|") ;
      println!("") ;
      panic!("test failed")
    },
    Err(e) => {
      println!("") ;
      println!("|===| Error on input") ;
      for line in input.lines() {
        println!("| {}", line)
      } ;
      println!("|===| expected grid") ;
      for line in grid_to_str(& expected).lines() {
        println!("| {}", line)
      } ;
      println!("|===| but got an error:") ;
      for line in e.lines() {
        println!("| {}", line)
      } ;
      println!("|===|") ;
      println!("") ;
      panic!("test failed")
    },
  }
}

/// Checks that parsing has failed on some input.
#[cfg(test)]
pub fn check_fails(
  input: & str, error: & str, to_check: Result<SubGrid, String>
) {
  match to_check {
    Err(_) => (),
    Ok(grid) => {
      println!("") ;
      println!("|===| Error on input") ;
      for line in input.lines() {
        println!("| {}", line)
      } ;
      println!("|===| expected parse error") ;
      for line in error.lines() {
        println!("| {}", line)
      } ;
      println!("|===| but parsing was successful:") ;
      for line in grid_to_str(& grid).lines() {
        println!("| {}", line)
      } ;
      println!("|===|") ;
      println!("") ;
      panic!("test failed")
    },
  }
}

/// DSL to create a grid.
#[cfg(test)]
macro_rules! mk_grid {
  (
    size = ( $rows:expr, $cols:expr ),
    id = ( $name:expr, $id:expr),
    grid = {
      $( $row:expr { $( $col:expr ),+ } ),*
    }
  ) => (
    {
      let mut vec = vec![
        vec![ $crate::grid::Status::Dead ; $cols ] ; $rows
      ] ;
      for (row,col) in vec![
        $( $( ($row - 1, $col - 1) ),+ ),*
      ].into_iter() {
        vec[row][col] = $crate::grid::Status::Live($id)
      } ;
      $crate::frontend::SubGrid::mk($name.to_string(), vec)
    }
  ) ;
}




#[cfg(test)]
/// Simple testcases that should work.
///
/// No comments at all.
mod task_1_correct {
  use super::* ;

  #[test]
  fn test_1() {
    let input = r#"
player: "yako"

grid (20,20) {
  ____________________,
  ____________________,
  ______xxx___________,
  _______x____________,
  _______x____________,
  ______xxx___________,
  ____________________,
  ______xxx___________,
  ______xxx___________,
  ____________________,
  ______xxx___________,
  _______x____________,
  _______x____________,
  ______xxx___________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 42 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("yako", id),
      grid = {
        3  { 7, 8, 9 },
        4  {    8    },
        5  {    8    },
        6  { 7, 8, 9 },
        8  { 7, 8, 9 },
        9  { 7, 8, 9 },
        11 { 7, 8, 9 },
        12 {    8    },
        13 {    8    },
        14 { 7, 8, 9 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_2() {
    let input = r#"
player: "agata"

grid (20,20) {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ________x___________,
  __________x_________,
  _______xx__xxx______,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 0 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("agata", id),
      grid = {
        9 { 9 },
        10 { 11 },
        11 { 8, 9, 12, 13, 14 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_3() {
    let input = r#"
player: "rika"

grid (
  20,     20
) {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ______x_x_x_________,
  ______x___x_________,
  ______x___x_________,

  ______x___x_________,
  ______x_x_x_________,
  ____________________,

  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,

  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 7 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("rika", id),
      grid = {
        7  { 7, 9, 11 },
        8  { 7,    11 },
        9  { 7,    11 },
        10 { 7,    11 },
        11 { 7, 9, 11 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_4() {
    let input = r#"
player: "adrien"

grid (20,20) {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____xxx_xxx_________,
  ____x_____x_________,
  ______x_x___________,
  _____xx_xx__________,
  ____x_____x_________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 11 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("adrien", id),
      grid = {
        8  { 5, 6, 7,   9, 10, 11 },
        9  { 5,                11 },
        10 {       7,   9         },
        11 {    6, 7,   9, 10     },
        12 { 5,                11 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

}





#[cfg(test)]
/// Simple testcases that should not work because of dimension mismatch.
///
/// No comments at all.
mod task_1_incorrect {
  use super::* ;

  #[test]
  fn test_1() {
    let input = r#"
player: "yako"

grid (17,20) {
  ____________________,
  ____________________,
  ______xxx___________,
  _______x____________,
  _______x____________,
  ______xxx___________,
  ____________________,
  ______xxx___________,
  ______xxx___________,
  ____________________,
  ______xxx___________,
  _______x____________,
  _______x____________,
  ______xxx___________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 420 ;

    check_fails(
      input, "\
        size declared is (17,20)\n\
        but actual size is (20,20)\
      ", get_grid(id, input)
    )

  }

  #[test]
  fn test_2() {
    let input = r#"
player: "yako"

grid (20,20) {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ________x___________,
  _________x_________,
  _______xx__xxx______,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 44_442 ;

    check_fails(
      input, "\
        size declared is (20,20)\n\
        but line 10 only has 19 columns\
      ", get_grid(id, input)
    )

  }

  #[test]
  fn test_3() {
    let input = r#"
player "yako"

grid (20,20) {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ________x___________,
  _________x__________,
  _______xx__xxx______,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 77 ;

    check_fails(
      input, "\
        missing `:` after `player` keyword\
      ", get_grid(id, input)
    )

  }

  #[test]
  fn test_4() {
    let input = r#"
player: "yako

grid (20,20) {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ________x___________,
  _________x__________,
  _______xx__xxx______,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 5 ;

    check_fails(
      input, "\
        missing `\"` around player name keyword\
      ", get_grid(id, input)
    )

  }

}





#[cfg(
  all(
    test,
    not( feature="task_1" ),
  )
)]
/// Simple testcases that should work.
///
/// No comments at all but some extra spaces.
mod task_2 {
  use super::* ;

  #[test]
  fn test_1() {
    let input = r#"
player: "yako"

grid (20,20) {
  ____________________,
  ____________________,
  ______xxx___________,
  _______x____________,
  _______x____________,
  ______xxx___________,
  ____________________,
  ______xxx___________,
  ______xxx___________,
  ____________
  ________,
  ______xxx___________,
  _______x____________,
  _______x________



  ____,
  ______xxx___________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,

  ____________________,
}
    "# ;

    let id = 42 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("yako", id),
      grid = {
        3  { 7, 8, 9 },
        4  {    8    },
        5  {    8    },
        6  { 7, 8, 9 },
        8  { 7, 8, 9 },
        9  { 7, 8, 9 },
        11 { 7, 8, 9 },
        12 {    8    },
        13 {    8    },
        14 { 7, 8, 9 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_2() {
    let input = r#"
player: "agata"

    grid (
  20,     20


  )

 {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ________x___________,
  __________x_________,
  _______xx__xxx______,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 1_000 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("agata", id),
      grid = {
        9 { 9 },
        10 { 11 },
        11 { 8, 9, 12, 13, 14 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_3() {
    let input = r#"
player




:    "rika"         
  
grid (
  20,     


  20
) {
  ____________________,
  __________ __________,
  ____________________,
  _ ___________________,
  _________________ ___,
  ____________________,
  ______x_x_x_________,
  ______x___x_________,
  ______x___x_________,

  ______x___x_________,
  ______x_x_x_________,
  ____________________,

  ____________________,
  ____________________,
  ____________________,
  _______ _____________,
  ____________________,

  ____________________,
  __________ __________,
  ____________________,
}
    "# ;

    let id = 7 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("rika", id),
      grid = {
        7  { 7, 9, 11 },
        8  { 7,    11 },
        9  { 7,    11 },
        10 { 7,    11 },
        11 { 7, 9, 11 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_4() {
    let input = r#"
player
:
"adrien"

grid
(
20
,
20
)
{
  ____________________
  ,
  ____________________
  ,
  ____________________
  ,
  _________
  ___________
  ,
  ____________________
  ,
  ____________________
  ,
  ____________________
  ,
  ____xxx_xxx_________
  ,   
  ____x_____x _________
  ,
  ______x_x___________
  , 
  _____  xx_xx__________
  ,
  ____x_____x_________
  ,
  ____________________
  ,
  ____________________
  ,
  ____________________
  ,
  ____________________
  ,
  ____________________
  ,
  ____________________
  ,
  ___________   _________
  ,
  ____________________
  ,



}



    "# ;

    let id = 30 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("adrien", id),
      grid = {
        8  { 5, 6, 7,   9, 10, 11 },
        9  { 5,                11 },
        10 {       7,   9         },
        11 {    6, 7,   9, 10     },
        12 { 5,                11 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

}





#[cfg(
  all(
    test,
    not( feature="task_1" ),
    not( feature="task_2" ),
  )
)]
/// Simple testcases that should work.
///
/// No comments at all.
mod task_3 {
  use super::* ;

  #[test]
  fn test_1() {
    let input = r#"
// Some comment.
player: "yako"

grid (20,20) {
  ____________________,
  ____________________,
  ______xxx___________,
  _______x____________,
  _______x____________,
  ______xxx___________,
  ____________________,
  ______xxx___________,
  ______xxx___________,
  ____________________,
  ______xxx___________,
  _______x____________,
  _______x____________,
  ______xxx___________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 17 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("yako", id),
      grid = {
        3  { 7, 8, 9 },
        4  {    8    },
        5  {    8    },
        6  { 7, 8, 9 },
        8  { 7, 8, 9 },
        9  { 7, 8, 9 },
        11 { 7, 8, 9 },
        12 {    8    },
        13 {    8    },
        14 { 7, 8, 9 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_2() {
    let input = r#"
player: "agata"
// Comment.
// Another comment.
grid (20,20) {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ________x___________,
  __________x_________,
  _______xx__xxx______,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 2 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("agata", id),
      grid = {
        9 { 9 },
        10 { 11 },
        11 { 8, 9, 12, 13, 14 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_3() {
    let input = r#"
player: "rika"

grid (
  20,     20
) {
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ______x_x_x_________,
  ______x___x_________,
  ______x___x_________,
  // Comment.
  ______x___x_________,
  ______x_x_x_________,
  ____________________,
  // Comment.
  // Comment.
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,

  ____________________,
  ____________________,
  ____________________,
}
    "# ;

    let id = 0 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("rika", id),
      grid = {
        7  { 7, 9, 11 },
        8  { 7,    11 },
        9  { 7,    11 },
        10 { 7,    11 },
        11 { 7, 9, 11 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

  #[test]
  fn test_4() {
    let input = r#"
player:
  // My name.
  "adrien"

grid (
  // Number of rows.
  20,
  // Number of cols.
  20
) {
  ____________________,
  ____________________,
  __________ // Half an empty line.
  __________ // Half an empty line.
  , // That's just a comma.
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____xxx_xxx_________,
  ____x_____x_________,
  ______x_x___________,
  _____xx_xx__________,
  ____x_____x_________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  ____________________,
  // End of grid def.
}
// At this point the file ends.
    "# ;

    let id = 7 ;

    let expected = mk_grid!(
      size = (20,20),
      id = ("adrien", id),
      grid = {
        8  { 5, 6, 7,   9, 10, 11 },
        9  { 5,                11 },
        10 {       7,   9         },
        11 {    6, 7,   9, 10     },
        12 { 5,                11 }
      }
    ) ;

    check_same(
      input, expected, get_grid(id, input)
    )

  }

}
