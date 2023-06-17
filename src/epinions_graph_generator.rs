use std::fs::OpenOptions;
use std::io::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use std::process::exit;

use rand::Rng;
use rand::rngs::StdRng;

use std::collections::HashMap;
use std::collections::HashSet;

use crate::wrappers::Node;
use crate::wrappers::Edge;

pub struct EpinionsGraphGenerator{

    nodes_list : Vec<Node>,

    rating_relation: HashMap<Edge, Vec<f32>>,
    trust_list: HashMap<Edge, i8>,
    neighbor_list: HashMap< Node, Vec<Node> >,
}

impl EpinionsGraphGenerator{
    pub fn new()->EpinionsGraphGenerator{

        let mut rating_relation: HashMap<Edge, Vec<f32>> =  HashMap::new();
        let mut trust_list: HashMap<Edge, i8> = HashMap::new();
        let mut neighbor_list: HashMap<Node, Vec<Node>> = HashMap::new();

        //Get all nodes in the graph
        println!("Reading full Epinions database... ");

        //FETCHING RELATIONS FROM FILE
        let filename = "epinions/rating_relation.csv";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut rating_relation_consult : Vec<(Node, Node, i8)> = Vec::new();
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let list_vals: Vec<&str> = line.split(",").collect();

            let rater_id_val: u64 = match u64::from_str( list_vals[0] ) {
                Ok(id) => id,
                Err(_) =>{
                    println!("Error while parsing line: {}", line);
                    exit(1);
                }
            };

            let author_id_val: u64 = match u64::from_str( list_vals[1] ) {
                Ok(id) => id,
                Err(_) =>{
                    println!("Error while parsing line: {}", line);
                    exit(1);
                }
            };
            let rating_val: i8 = match i8::from_str( list_vals[2] ) {
                Ok(id) => id,
                Err(_) =>{
                    println!("Error while parsing line: {}", line);
                    exit(1);
                }
            };

            //There are users in EPINIOS who qualify themselves. That is not from our interest.
            if rater_id_val != author_id_val{

                rating_relation_consult.push((
                    Node::new( rater_id_val ),
                    Node::new( author_id_val ),
                    rating_val
                ));

            }

        }

        let mut raters_set : HashSet<Node> = HashSet::new();
        for (rater_id, author_id, rating) in rating_relation_consult{

            //Insert to unrepeated list
            raters_set.insert(rater_id);


            match neighbor_list.get(&rater_id){
                Some(_)=>{
                    (*neighbor_list.get_mut(&rater_id).unwrap()).push(author_id);
                }
                _ =>{
                    let mut new_list : Vec<Node> = Vec::new();
                    new_list.push(author_id);
                    neighbor_list.insert(rater_id, new_list);

                }
            };


            let e = Edge::new( rater_id, author_id );
            let rating_2 :f32 = match rating{
                5 | 6 => 1.0,
                4  => 0.75,
                3 => 0.25,
                2 | 1 => -1.0,
                _ => 0.0
            };
            match rating_relation.get(&e){
                Some(_weigh_list) => {
                    (rating_relation.get_mut(&e).unwrap()).push(rating_2);
                }
                _ => {
                    let mut new_list : Vec<f32> = Vec::new();
                    new_list.push(rating_2);
                    rating_relation.insert(e, new_list);

                }
            };

        }
        println!("Done!");

        println!("Creating trust list relation... ");

        let filename_2 = "epinions/user_rating.csv";
        let file_2 = File::open(filename_2).unwrap();
        let reader_2 = BufReader::new(file_2);

        let mut trust_relation_consult : Vec<(Node, Node, i8)> = Vec::new();
        for (_, line) in reader_2.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let list_vals: Vec<&str> = line.split(",").collect();

            let me_id_val: u64 = match u64::from_str( list_vals[0] ) {
                Ok(id) => id,
                Err(_) =>{
                    println!("Error while parsing line on me_id_val: {}", line);
                    exit(1);
                }
            };

            let other_id_val: u64 = match u64::from_str( list_vals[1] ) {
                Ok(id) => id,
                Err(_) =>{
                    println!("Error while parsing line on other_id_val: {}", line);
                    exit(1);
                }
            };
            let decision: i8 = match i8::from_str( list_vals[2] ) {
                Ok(val) => val,
                Err(_) =>{
                    println!("Error while parsing line in decision: {}", line);
                    exit(1);
                }
            };

           trust_relation_consult.push((
                Node::new( me_id_val ),
                Node::new( other_id_val ),
                decision
           ));

        }

        for (me, other, decision) in trust_relation_consult{
            let e : Edge = Edge::new(me, other);
            trust_list.insert(e, decision );
        }

        println!("Done!");

        EpinionsGraphGenerator{
            nodes_list : raters_set.into_iter().collect(),
            rating_relation : rating_relation,
            trust_list : trust_list,
            neighbor_list : neighbor_list
        }

    }

    pub fn generate_graph( &self, random_generator: &mut StdRng, number_nodes: u64 , out_file: String){

        let nodes = self.get_conected_nodes(random_generator, number_nodes);
        /*
        nodes.sort_unstable_by(
            |u,v|
            u.get_id().partial_cmp( &v.get_id()).unwrap()
        );
        println!("LISTA: {:?}", nodes);
        */
        //Generate the edges of the graph
        let mut edges : HashMap< Edge, f32> = HashMap::new();

        for i in 0..(nodes.len()-1){
            let u : Node = nodes[i];
            for j in (i+1)..nodes.len(){
                let v : Node = nodes[j];
                let e : Edge = Edge::new(u, v);
                let w : f32 = self.get_edge_weight( &e );

                if w != 0.0{
                    edges.insert(e, w );
                }

            }
        }

        let mut out_string = String::new();
        for (e,w) in edges{
            out_string = format!(
                "{}{},{},{}\n",
                out_string,
                e.get_u().get_id(),
                e.get_v().get_id(),
                w
            );
        }
        let outfile = OpenOptions::new()
                        .write(true).create_new(true)
                        .open( out_file.clone() );
        match outfile {
            Ok(mut out) =>{
                match writeln!(out, "{}",out_string.trim()){
                    Ok(_) =>{},
                    Err(_)=>{
                        println!("Error while writing output file:{}", out_file.clone());
                    }
                }
            }
            Err(_)=>{
                println!("File '{}' could not be written\n There must not exist a file with the same name in directory",out_file.clone());
                exit(1);
            }
        }

    }

    fn get_edge_weight(& self, e : &Edge)->f32{
        let w : f32 = match self.trust_list.get(e){
            Some(val) => { *val as f32 },
            _ => {
                let mut x = 0.0;
                match self.rating_relation.get(e){
                    Some(val) =>{
                        for rating in val{
                            x = x + rating;
                        }
                    },
                    _ => {}
                };

                ( 1.0 - (-1.0*x as f32).exp() )/ ( 1.0 + (-1.0*x as f32).exp() )

            }
        };

        w

    }

    fn get_conected_nodes(  &self, random_generator: &mut StdRng, number_nodes: u64  )-> Vec<Node>{

        //Get node list for building the subgraph
        let mut nodes_list : Vec<Node> = self.nodes_list.clone();
        //Sort because hashmaps do what they want
        nodes_list.sort_unstable_by(
            |u,v|
            u.get_id().partial_cmp( &v.get_id()).unwrap()
        );
        let mut graph_nodes : Vec<Node> = Vec::new();
        let mut search_list : Vec<Node> = Vec::new();

        //Get the initial index of the initial_vertex of the graph
        let initial_index : usize = random_generator.gen_range(0..nodes_list.len());
        let initial_node : Node = nodes_list.remove(initial_index);
        search_list.push(initial_node);

        let mut vertex_count = 0;
        while vertex_count < number_nodes && search_list.len()> 0{
            //Choose random vertex of conected subgraph
            let next_index : usize = random_generator.gen_range( 0..search_list.len() );
            let next_node : Node = search_list.remove(next_index);

            //Add to the graph list
            graph_nodes.push(next_node);
            vertex_count = vertex_count+1;

            //Add its neighbors to the list of posible next vertex of the subgraph
            let mut  neighbors : Vec<Node> = self.get_node_neighbors(&next_node);
            search_list.append(&mut neighbors);

        }

        if graph_nodes.len() != (number_nodes as usize) {
            self.get_conected_nodes(random_generator, number_nodes)
        }else{
            graph_nodes
        }

    }
    fn get_node_neighbors(& self, node : &Node)-> Vec<Node>{
        match self.neighbor_list.get(node){
            Some(nodes) => nodes.clone(),
            _ => Vec::new()
        }
    }
}
