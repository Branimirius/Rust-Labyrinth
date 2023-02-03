use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fmt::{Display};


static mut CHECKED_NODES: Vec<Node> = vec![];
#[derive(Clone, Debug)]
struct Node {
    position: [i8; 2],
    doors: [bool; 4],
    key: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    n_left: Option<Box<Node>>,
    n_right: Option<Box<Node>>,
    n_up: Option<Box<Node>>,
    n_down: Option<Box<Node>>,
    exit: bool,
    value: i32
}

impl Default for Node {
    fn default() -> Self {
        Node {
            position: [0, 0],
            doors: [false, false, false, false],
            key: false,
            left: false,
            right: false,
            up: false,
            down: false,
            n_left: None,
            n_right: None,
            n_up: None,
            n_down: None,
            exit: false,
            value: 0
        }
    }
}


impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ position: {:?}, doors: {:?}, key: {}, left: {}, right: {}, up: {}, down: {}, n_left: {:?}, n_right: {:?}, n_up: {:?}, n_down: {:?}, exit: {}, value: {} }}", self.position, self.doors, self.key, self.left, self.right, self.up, self.down, self.n_left, self.n_right, self.n_up, self.n_down, self.exit, self.value)
    }
}

fn read_lines_from_file() -> Vec<String> {
    let path = Path::new("./maze.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

fn form_matrix_from_lines(lines: Vec<String>) -> Vec<Vec<Node>> {
    let mut matrix: Vec<Vec<Node>> = vec![vec![Node::default(); 9]; 6];
    let mut i = 0;
    let mut j = 0;
    for line in lines {
        let node = make_node_from_string(&line, [usize_to_i8(i), usize_to_i8(j)]);
        //println!("{:?}", node);
        if j < 9 {
            matrix[i][j] = node;
            j += 1;
        }
        else {
            j = 0;
            i += 1;
            matrix[i][j] = node;
            j += 1;
        }
        
    }
    matrix
    
}

fn usize_to_i8(x: usize) -> i8 {
    if x > i8::MAX as usize {
        return i8::MAX;
    }
    x as i8
}

fn make_node_from_string(string: &str, position: [i8; 2]) -> Node{
    let mut node = Node::default();
    node.position = position;
    //Walls
    if string.chars().nth(0).unwrap() == '1' {
        node.left = true;
    }
    if string.chars().nth(1).unwrap() == '1' {
        node.right = true;
    }
    if string.chars().nth(2).unwrap() == '1' {
        node.up = true;
    }
    if string.chars().nth(3).unwrap() == '1' {
        node.down = true;
    }
    //Door
    if string.chars().nth(5).unwrap() == '1' {
        node.doors[0] = true;
    }
    if string.chars().nth(6).unwrap() == '1' {
        node.doors[1] = true;
    }
    if string.chars().nth(7).unwrap() == '1' {
        node.doors[2] = true;
    }
    if string.chars().nth(8).unwrap() == '1' {
        node.doors[3] = true;
    }
    //Key
    if string.chars().nth(10).unwrap() == '1' && string.chars().nth(11).unwrap() == '1' {
        node.key = true;
    }
    //Exit
    if string.chars().nth(12).unwrap() == '1' && string.chars().nth(13).unwrap() == '1' {
        node.exit = true;
    }

    node
}

fn add_heuristics(matrix: &Vec<Vec<Node>>) -> Vec<Vec<Node>>{
    let mut ret_matrix: Vec<Vec<Node>> = matrix.clone();
    for mut node in find_exits(matrix){
        let mut queue = VecDeque::new();
        queue.push_back(node.clone());

        let mut a = 1000;
        let mut b = 0;
        //println!("Exits are: {:?}", node);
        while let Some(node) = queue.pop_front() {
            let x = node.position[0] as usize;
            let y = node.position[1] as usize;
            if x < ret_matrix.len() && y < ret_matrix[0].len() {

                if ret_matrix[x][y].value < (a - 10 * b){
                    ret_matrix[x][y].value = a - 10 * b;
                }
                
                unsafe {
                    CHECKED_NODES.push(node.clone());
                }

                if node.n_down.is_some() {
                    if !is_checked(*node.n_down.clone().unwrap()){
                        let nd = node.n_down.clone().unwrap();
                        let x = nd.position[0] as usize;
                        let y = nd.position[1] as usize;
                        if x < matrix.len() && y < matrix[0].len() {
                            queue.push_back(matrix[x][y].clone());
                        }
                    }
                }
                if node.n_left.is_some() {
                    if !is_checked(*node.n_left.clone().unwrap()){
                        let nl = node.n_left.clone().unwrap();
                        let x = nl.position[0] as usize;
                        let y = nl.position[1] as usize;
                        if x < matrix.len() && y < matrix[0].len() {
                            queue.push_back(matrix[x][y].clone());
                        }
                    }
                }
                if node.n_up.is_some() {
                    if !is_checked(*node.n_up.clone().unwrap()){
                        let nu = node.n_up.clone().unwrap();
                        let x = nu.position[0] as usize;
                        let y = nu.position[1] as usize;
                        if x < matrix.len() && y < matrix[0].len() {
                            queue.push_back(matrix[x][y].clone());
                        }
                    }
                }
                if node.n_right.is_some(){
                    if !is_checked(*node.n_right.clone().unwrap()){
                        let nr = node.n_right.clone().unwrap();
                        let x = nr.position[0] as usize;
                        let y = nr.position[1] as usize;
                        if x < matrix.len() && y < matrix[0].len() {
                            queue.push_back(matrix[x][y].clone());
                        }
                    }
                }
            }
            b += 1;
        }
    }

    ret_matrix
}

fn is_checked(node: Node) -> bool{
    unsafe{
        for n in CHECKED_NODES.clone() {
            if node.position == n.position{
                return true;
            }
        }
        return false;
    }
}

fn find_exits(matrix: &Vec<Vec<Node>>) -> Vec<Node>{
    let mut exits: Vec<Node> = vec![];
    for (i, row) in matrix.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if matrix[i][j].exit {
                exits.push(matrix[i][j].clone());
            }
        }
    }

    exits
}

fn fill_neighbours(matrix: &Vec<Vec<Node>>) -> Vec<Vec<Node>>{
    let mut ret_matrix: Vec<Vec<Node>> = matrix.iter().map(|row| row.iter().map(|node| node.clone()).collect()).collect();

    for (i, row) in matrix.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if matrix[i][j].down {
                ret_matrix[i][j].n_down = Some(Box::new(matrix[i+1][j].clone()));
            }
            if matrix[i][j].left {
                ret_matrix[i][j].n_left = Some(Box::new(matrix[i][j-1].clone()));
            }
            if matrix[i][j].up {
                ret_matrix[i][j].n_up = Some(Box::new(matrix[i-1][j].clone()));
            }
            if matrix[i][j].right {
                ret_matrix[i][j].n_right = Some(Box::new(matrix[i][j+1].clone()));
            }
            //println!("{}", ret_matrix[i][j]);
        }
    }

    ret_matrix
}

fn choose_the_path(matrix: &Vec<Vec<Node>>, start: Node) -> Vec<[i8; 2]>{
    let mut ret = vec![];
    let mut curr_position = start;

    ret.push(curr_position.position);
    while !curr_position.exit{
        let mut best = curr_position.value;
        if curr_position.down{
            if curr_position.n_down.as_ref().unwrap().value > best {
                best = curr_position.n_down.as_ref().unwrap().value;
                curr_position = matrix[curr_position.n_down.as_ref().unwrap().position[0] as usize][curr_position.n_down.as_ref().unwrap().position[1] as usize].clone()
            }
        }
        if curr_position.right{
            if curr_position.n_right.as_ref().unwrap().value > best {
                best = curr_position.n_right.as_ref().unwrap().value;
                curr_position = matrix[curr_position.n_right.as_ref().unwrap().position[0] as usize][curr_position.n_right.as_ref().unwrap().position[1] as usize].clone()
            }
        }
        if curr_position.up{
            if curr_position.n_up.as_ref().unwrap().value > best {
                best = curr_position.n_up.as_ref().unwrap().value;
                curr_position = matrix[curr_position.n_up.as_ref().unwrap().position[0] as usize][curr_position.n_up.as_ref().unwrap().position[1] as usize].clone()
            }
        }
        if curr_position.left{
            if curr_position.n_left.as_ref().unwrap().value > best {
                best = curr_position.n_left.as_ref().unwrap().value;
                curr_position = matrix[curr_position.n_left.as_ref().unwrap().position[0] as usize][curr_position.n_left.as_ref().unwrap().position[1] as usize].clone()
            }
        }
        //println!("{:?}", curr_position.position);
        ret.push(curr_position.position);
    }

    ret
}



fn print_matrix(matrix: &Vec<Vec<Node>>) {
    for row in matrix {
        for node in row {
            println!("{}", node);
        }
        println!("");
    }
}
fn print_matrix_values(matrix: &Vec<Vec<Node>>) {
    for row in matrix {
        for node in row {
            print!("{} ", node.value);
        }
        println!();
    }
}

fn print_lines(lines: Vec<String>) {
    for line in lines {
        println!("{}", line);
    }
}

fn main() {
    let mut lines = read_lines_from_file();
    print_lines(lines);
    let matrix = fill_neighbours(&form_matrix_from_lines(read_lines_from_file()));
    //print_matrix(&matrix);
    let h_matrix = fill_neighbours(&add_heuristics(&matrix));
    print_matrix_values(&h_matrix);
    println!("THE PATH: {:?}", choose_the_path(&h_matrix, h_matrix[0][0].clone()));
}