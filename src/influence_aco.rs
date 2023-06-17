use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use std::process::exit;

use rand::Rng;
use rand::rngs::StdRng;

use std::collections::HashMap;
use std::collections::HashSet;
use rand::seq::SliceRandom;

use crate::wrappers::Node;
use crate::wrappers::Edge;


#[derive(Debug)]
#[derive(Clone)]
struct AntPath{
    path : Vec<Node>,
    similarity: f32,
    influenced_nodes : HashMap<Node, f32>,
    fitness : f32
}

impl AntPath{

    fn new()-> AntPath{
        let p : Vec<Node> = Vec::new();
        let s : f32 = 0.0;
        let inf_n : HashMap<Node,f32> = HashMap::new();
        let f = 0.0;
        AntPath{
            path : p,
            similarity : s,
            influenced_nodes : inf_n,
            fitness : f
        }
    }
    fn get_influenced_nodes( &self )-> &HashMap<Node, f32>{
        & self.influenced_nodes
    }
    fn set_influenced_nodes( &mut self, new : HashMap<Node,f32> ){
        self.influenced_nodes = new;
    }
    fn add_to_path(&mut self, node: Node){
        self.path.push(node);
    }

    fn set_fitness( &mut self, f: f32 ){
        self.fitness = f;
    }
    fn get_fitness( & self )-> f32{
        self.fitness
    }
    fn get_similarity( & self )-> f32{
        self.similarity
    }
    fn set_similarity( &mut self, s: f32 ){
        self.similarity = s;
    }
    fn get_path( & self )-> &Vec<Node>{
        &self.path
    }
}

pub struct InfluenceAco{

    graph_edges: HashMap< Edge, f32>,
    neighbor_list: HashMap< Node, Vec<Node> >,
    //Pheromone in the search space
    pheromone_register : HashMap<Edge,f32>,

    alpha : f32,
    beta : f32,
    theta : f32,
    cost : f32,
    maximum_geed_iteration: u32,
    revenue : f32,
    eta: f32,
    psi: f32,
    gama: f32,
    initial_pheromone: f32,
    pheromone_evaporation: f32

}

impl InfluenceAco{
    pub fn new(
            alpha: f32,
            beta: f32,
            theta : f32,
            maximum_geed_iteration: u32,
            cost : f32,
            revenue : f32,
            eta: f32,
            psi: f32,
            gama: f32,
            initial_pheromone: f32,
            pheromone_evaporation: f32
    ) -> InfluenceAco{

        let pheromone : HashMap<Edge, f32> = HashMap::new();
        let graph_edges : HashMap<Edge, f32> = HashMap::new();
        let neighbor_list: HashMap<Node, Vec<Node>> = HashMap::new();

        let influence_created = InfluenceAco{
            graph_edges : graph_edges,
            neighbor_list : neighbor_list,
            pheromone_register : pheromone,
            alpha: alpha,
            beta: beta,
            theta : theta,
            cost : cost,
            maximum_geed_iteration: maximum_geed_iteration,
            revenue : revenue,
            eta: eta,
            psi: psi,
            gama: gama,
            initial_pheromone: initial_pheromone,
            pheromone_evaporation: pheromone_evaporation
        };

        influence_created
    }



    pub fn exe(&mut self, file_name : String,  iterations: u32, ants:u32, k:u32, mut random_generator:StdRng )->(Vec<Node>, f32){

        let filename_2 = file_name;
        // Open the file in read-only mode (ignoring errors).
        let file_2 = File::open(filename_2).unwrap();
        let reader_2 = BufReader::new(file_2);

        let mut nodes_set : HashSet<Node> = HashSet::new();
        for (_, line) in reader_2.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let list_vals: Vec<&str> = line.split(",").collect();

            let u_id_val: u64 = match u64::from_str( list_vals[0] ) {
                Ok(id) => id,
                Err(_) =>{
                    println!("Error while parsing line on me_id_val: {}", line);
                    exit(1);
                }
            };

            let v_id_val: u64 = match u64::from_str( list_vals[1] ) {
                Ok(id) => id,
                Err(_) =>{
                    println!("Error while parsing line on other_id_val: {}", line);
                    exit(1);
                }
            };
            let weight: f32 = match f32::from_str( list_vals[2] ) {
                Ok(val) => val,
                Err(_) =>{
                    println!("Error while parsing line in decision: {}", line);
                    exit(1);
                }
            };

            let u : Node = Node::new( u_id_val );
            let v : Node = Node::new( v_id_val );

            nodes_set.insert(u);
            nodes_set.insert(v);

            self.graph_edges.insert( Edge::new(u,v), weight );

            match self.neighbor_list.get(&u){
                Some(_)=>{
                    (*self.neighbor_list.get_mut(&u).unwrap()).push(v);
                }
                _ =>{
                    let mut new_list : Vec<Node> = Vec::new();
                    new_list.push(v);
                    self.neighbor_list.insert(u, new_list);

                }
            }

        }


        let mut nodes : Vec<Node> = nodes_set.into_iter().collect();

        if k > nodes.len() as u32{
            println!("k is bigger than the nodes in the graph");
            exit(1);
        }

        let  nodes_list: Vec<Node> = nodes.clone();
        //Sort because hashmaps do what they want
        nodes.sort_unstable_by(
            |u,v|
            u.get_id().partial_cmp( &v.get_id()).unwrap()
        );
        nodes.shuffle( &mut random_generator);

        /*
         *  Initially the similarity between nodes(u,v) is not asigned to every pair posible in G. If we want to know this value we make a query to get the needed data so we calculate this value only when needed.
         *
         *  The same goes to the pheromone value. Initialy every edge of the neighborhood is set to an initial value, when we update the value to a particular edge, we set this value in the HashMap pheromone_register, so when we want to get this value it is contained in the map or it is the initial value.
         *  */


        let mut best_path : AntPath = AntPath::new();

        for _t in 1..iterations{

            let mut ant_path : AntPath = AntPath::new();
            for _ant in 0..ants{
                nodes = nodes_list.clone();
                nodes.sort_unstable_by(
                    |u,v|
                    u.get_id().partial_cmp( &v.get_id()).unwrap()
                );
                nodes.shuffle( &mut random_generator );

                //Place ant randomly on the graph
                let initial_node = nodes.pop().unwrap();

                let (ant_path_profit, ant_influence_nodes)  = self.get_profit_value(&ant_path, &initial_node, k);
                let ant_path_similarity = self.calculate_similarity(&ant_path, &initial_node);


                ant_path.add_to_path(initial_node);
                ant_path.set_influenced_nodes(ant_influence_nodes);
                ant_path.set_similarity(ant_path_similarity);

                let mut current_path_fitness :f32 = self.evaluate_path_fitness(ant_path_profit, ant_path_similarity) ;
                ant_path.set_fitness( current_path_fitness );


                for _j in 1..k{
                    //Choose next node acording to state transition
                    let (next_index, next_profit, next_similarity, next_inf_nodes) : (usize, f32, f32, HashMap<Node, f32>) =
                        self.get_neighbor_greedy( &ant_path, &mut nodes, k, &mut random_generator);

                    let next_node = nodes.remove(next_index);

                    //Update AntPath to new node found
                    ant_path.add_to_path(next_node);
                    ant_path.set_influenced_nodes( next_inf_nodes );
                    ant_path.set_similarity( next_similarity );

                    current_path_fitness = self.evaluate_path_fitness(next_profit, next_similarity);

                    ant_path.set_fitness(current_path_fitness);


                }

                if ant_path.get_fitness() > best_path.get_fitness(){
                    best_path = ant_path.clone();
                }

                self.pheromone_update(&ant_path);

            }
        }

        (best_path.get_path().clone(), best_path.get_fitness())

    }

    fn pheromone_update( &mut self, ant_path : &AntPath){
        let path = ant_path.get_path();
        let fitness = ant_path.get_fitness();
        for i in 1..path.len(){
            let e : Edge = Edge::new( path[i-1], path[i] );

            match self.pheromone_register.get(&e){
                Some(val) =>{
                    *self.pheromone_register.get_mut(&e).unwrap() =
                        (1.0-self.pheromone_evaporation)*val + fitness;
                }
                _ => {
                    self.pheromone_register.insert(
                        e,
                        (1.0-self.pheromone_evaporation)*self.initial_pheromone + fitness
                    );
                }
            }

        }
    }

    fn evaluate_path_fitness(&mut self, path_profit : f32, similarity : f32)->f32{

        f32::powf(path_profit, self.psi) * f32::powf(1.0/similarity, self.gama)

    }


    fn get_neighbor_greedy( &mut self , ant_path: &AntPath , nodes_not_selected : &mut Vec<Node>, k:u32, random_generator:&mut StdRng  )->(usize,f32,f32, HashMap<Node, f32> ){

        //Asumes it has at least one node in path.
        let previous_nodes = ant_path.get_path();
        let last_chosen : &Node = &previous_nodes[previous_nodes.len()-1];
        let mut index: usize = random_generator.gen_range(0..nodes_not_selected.len());

        //let mut best: &Node = &nodes_not_selected[index];
        let mut best_greedy_eval = f32::NEG_INFINITY;
        let mut best_profit = 0.0;
        let mut best_similarity = 0.0;
        let mut best_inf_nodes : HashMap<Node, f32> = HashMap::new();


        for _i in 1..self.maximum_geed_iteration {


            let u_index: usize = random_generator.gen_range(0..nodes_not_selected.len());

            let u : &Node = &nodes_not_selected[u_index];
            // Get pheromone of edge
            let edge_pheromone : f32 = self.get_edge_pheromone( last_chosen, u  );
            // Get profit value
            let (profit, inf_nodes) : (f32, HashMap<Node, f32>) =  self.get_profit_value( &ant_path, u, k );
            // Calculate similarity
            let similarity : f32 = self.calculate_similarity( &ant_path ,u );

            let greedy_eval_u = edge_pheromone.powf(self.eta) *
                                profit.powf(self.psi) *
                                (1.0/similarity).powf(self.gama);

            if greedy_eval_u > best_greedy_eval{
                index = u_index;
                best_greedy_eval = greedy_eval_u;
                best_profit = profit;
                best_similarity = similarity;
                best_inf_nodes = inf_nodes;
            }

        }

        (index, best_profit, best_similarity, best_inf_nodes)

    }

    fn calculate_similarity(&mut self, ant_path: &AntPath, u: &Node)->f32{

        let mut sum : f32 = ant_path.get_similarity();

        for n in ant_path.get_path(){

            sum += self.similarity(n, u);
            sum += self.similarity(u, n);
        }

        sum += self.similarity(u,u);

        sum
    }

    fn similarity(&mut self, u:&Node, v:&Node)->f32{
        let v_fon : HashSet<Node> = self.first_order_neighbors(v);
        if v_fon.contains(u){
            return 1.0;
        }

        let v_son : HashSet<Node> = self.second_order_neighbors(v);

        let u_fon : HashSet<Node> = self.first_order_neighbors(u);
        let u_son : HashSet<Node> = self.second_order_neighbors(u);

        let fon_intersect : HashSet<_>= u_fon.intersection(&v_fon).collect();
        let fon_union : HashSet<_> = u_fon.union(&v_fon).collect();

        let son_intersect : HashSet<_>= u_son.intersection(&v_son).collect();
        let son_union : HashSet<_> = u_son.union(&v_son).collect();

        let a = self.alpha *( (fon_intersect.len() as f32)/ (fon_union.len() as f32) );
        let b = self.beta *( (son_intersect.len() as f32)/ (son_union.len() as f32) );

        a+b

    }

    fn first_order_neighbors(&mut self, w: &Node) -> HashSet<Node>{

        let mut fon : HashSet<Node> = HashSet::new();

        match self.neighbor_list.get(w){
            Some(nodes) =>{
                for n in nodes{
                    fon.insert(*n);
                }
            },
            _ => {}
        }
        fon

    }
    fn second_order_neighbors(&mut self, w: &Node) -> HashSet<Node>{

        //w's first order neighbors
        let w_fon : HashSet<Node> = self.first_order_neighbors(w);
        //w's second order neighbors
        let mut w_son : HashSet<Node> = HashSet::new();
        for neighbor in w_fon{
            let neighbor_fon : HashSet<Node> = self.first_order_neighbors(&neighbor);
            for neighbor_of_neighbor in neighbor_fon{
                w_son.insert(neighbor_of_neighbor);
            }

        }

        w_son
    }


    fn get_edge_pheromone(&mut self, i:&Node, u:&Node )-> f32{
        let e  = Edge::new( *i, *u );
        match self.pheromone_register.get(&e){
            Some(val) => *val,
            _ => self.initial_pheromone
        }
    }
    fn get_profit_value(&mut self, ant_path: &AntPath, u: &Node, k:u32)->(f32, HashMap<Node, f32> ){

        let mut influenced_nodes = ant_path.get_influenced_nodes().clone();

        let neighbor_list = self.neighbor_list.get(u);
        match neighbor_list{
            Some(n_list)=>{
                for neighbor in n_list.clone(){
                    match influenced_nodes.get(&neighbor){
                        Some(val) =>{
                            *influenced_nodes.get_mut(&neighbor).unwrap() =
                                val + self.get_vu_weigth(u, &neighbor)  ;
                        }
                        _ =>{
                            influenced_nodes.insert(
                                neighbor,
                                self.get_vu_weigth(u, &neighbor)
                            );
                        }
                    }
                }
            }
            _ => {

            }
        }


        let mut sum = 0.0;
        for (_, inf) in &influenced_nodes{
            if inf >= &self.theta {
                sum = sum + 1.0;
            }
        };

        sum = sum * self.revenue;
        sum = sum - (self.cost * (k as f32) );

        (sum,influenced_nodes)

    }

    fn get_vu_weigth(&mut self, v:&Node, u:&Node)->f32{
        let e : Edge = Edge::new( *v, *u );
        match self.graph_edges.get(&e){
            Some(w) => *w,
            _ => 0.0
        }
    }
}
