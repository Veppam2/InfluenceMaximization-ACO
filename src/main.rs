extern crate dotenv;



use dotenv::dotenv;
use std::env;
use std::process::exit;


use rand::SeedableRng;
use rand::rngs::StdRng;

mod wrappers;

mod epinions_graph_generator;
use epinions_graph_generator::EpinionsGraphGenerator;

mod influence_aco;
use influence_aco::InfluenceAco;



fn print_use(){
    let s = format!(
        "   SYSTEM USE:
            $ cargo run --release graph NUMBER_OF_NODES SEED OUT_DIR
            We create a subgraph from epinions.
                -> NUMBER_OF_NODES: The desire number of nodes in the graph
                -> SEED: A number so we can replicate the created graph
                -> OUT_DIR: Dir with name of the output file.


            $ cargo run --release OPTION FILE_NAME K SEED
                -> FILE_NAME : Name of the file generated previously.
                -> K: number of desire influencers
                -> OPTION:
                        => Use 'eval' to evaluate a seed in SEED, SEED is a number.
                        => Use 'expr' to experiment with diferent seeds, in total SEED number of seeds.
        "
    );
    println!("{}",s);
}


fn main() {

    let args : Vec<String> = env::args().collect();
    let number_of_params = args.len();

    if number_of_params < 5 || number_of_params > 5 {
        print_use();
        exit(1);
    }

    let action  = args[1].clone();

    if !action.eq("eval") && !action.eq("expr") && !action.eq("graph") {
        print_use();
        exit(1);
    }

    //Read parámeters from .env file
    dotenv().ok();

    let iterations: u32 = match env::var("MAXIMUM_CYCLES") {
        Ok(val) => val.parse::<u32>().unwrap(),
        Err(_) => panic!("error en parámetro MAXIMUM_CYCLES"),
    };
    let alpha: f32 = match env::var("ALPHA") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro ALPHA"),
    };
    let beta: f32 = match env::var("BETA") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro BETA"),
    };
    let theta: f32 = match env::var("THRESHOLD_THETA") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro THRESHOLD_THETA"),
    };
    let maximum_greed_iteration: u32 = match env::var("MAXIMUM_GREED") {
        Ok(val) => val.parse::<u32>().unwrap(),
        Err(_) => panic!("error en parámetro MAXIMUM_GREED"),
    };
    let cost: f32 = match env::var("COST") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro COST"),
    };
    let revenue: f32 = match env::var("REVENUE") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro REVENUE"),
    };
    let eta: f32 = match env::var("ETA") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro ETA"),
    };
    let psi: f32 = match env::var("PSI") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro PSI"),
    };
    let gama: f32 = match env::var("GAMA") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro GAMA"),
    };
    let initial_pheromone: f32 = match env::var("INITIAL_PHEROMONE") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro INITIAL_PHEROMONE"),
    };
    let pheromone_evaporation: f32 = match env::var("PHEROMONE_EVAPORATION") {
        Ok(val) => val.parse::<f32>().unwrap(),
        Err(_) => panic!("error en parámetro PHEROMONE_EVAPORATION"),
    };

    let mut influence_heuristic = InfluenceAco::new(
        alpha,
        beta,
        theta,
        maximum_greed_iteration,
        cost,
        revenue,
        eta,
        psi,
        gama,
        initial_pheromone,
        pheromone_evaporation
    );


    if action.eq("graph"){

        let number_of_nodes : u64 =  match args[2].parse::<u64>(){
            Result::Ok(v) => v,
            _ =>{
                println!("Error in NUMBER_OF_NODES parameter");
                print_use();
                exit(1);
            }
        };

        let seed_num : u64 =  match args[3].parse::<u64>(){
            Result::Ok(v) => v,
            _ =>{
                println!("Error in SEED parameter");
                print_use();
                exit(1);
            }
        };

        let out_name  = args[4].clone();


        let gen : EpinionsGraphGenerator = EpinionsGraphGenerator::new();
        let mut seed = StdRng::seed_from_u64(seed_num);
        gen.generate_graph( &mut seed , number_of_nodes, out_name);

    } else if action.eq("eval"){
        let graph_file_name  = args[2].clone();

        let k : u32 =  match args[3].parse::<u32>(){
            Result::Ok(v) => v,
            _ =>{
                println!("Error in K parameter");
                print_use();
                exit(1);
            }
        };
        let seed_num : u64 =  match args[4].parse::<u64>(){
            Result::Ok(v) => v,
            _ =>{
                println!("Error in SEED parameter");
                print_use();
                exit(1);
            }
        };

        let seed = StdRng::seed_from_u64(seed_num);
        let number_of_ants = 5;

        println!("Finding solution...");

        let (best_path, fitness)= influence_heuristic.exe(
            graph_file_name,
            iterations,
            number_of_ants,
            k,
            seed
        );
        println!("FITNESS: {}, SEED: {}\nSOLUTION: {:?}",fitness, seed_num, best_path);

    } else if action.eq("expr"){

        let graph_file_name  = args[2].clone();

        let k : u32 =  match args[3].parse::<u32>(){
            Result::Ok(v) => v,
            _ =>{
                println!("Error in K parameter");
                print_use();
                exit(1);
            }
        };
        let iter_num : usize =  match args[4].parse::<usize>(){
            Result::Ok(v) => v,
            _ =>{
                println!("Error in SEED parameter");
                print_use();
                exit(1);
            }
        };

        let number_of_ants = 5;

        for seed_num in 1..iter_num{

            influence_heuristic = InfluenceAco::new(
                alpha,
                beta,
                theta,
                maximum_greed_iteration,
                cost,
                revenue,
                eta,
                psi,
                gama,
                initial_pheromone,
                pheromone_evaporation
            );
            let seed = StdRng::seed_from_u64(seed_num as u64);
            let (best_path, fitness)= influence_heuristic.exe(
                graph_file_name.clone(),
                iterations,
                number_of_ants,
                k,
                seed
            );
            println!("FITNESS: {}, SEED: {}\nSOLUTION: {:?}",fitness, seed_num, best_path);
        }

    }


    /*
    //Pruebas para el graficador

    let mut h : HashMap<Edge, f32> = HashMap::new();
    let uno  = Node::new(1);
    let dos = Node::new(2);
    let tres = Node::new(3);
    let cuatro = Node::new(4);
    let cinco = Node::new(5);
    let seis = Node::new(6);

    h.insert(Edge::new(uno, dos), 0.45);
    h.insert(Edge::new(uno, tres),0.75 );
    h.insert(Edge::new(dos, seis), -0.25 );
    h.insert(Edge::new(tres, cuatro), 0.48 );
    h.insert(Edge::new(tres, cinco), -0.25);
    h.insert(Edge::new(cuatro, cinco), 0.5 );
    h.insert(Edge::new(cuatro, dos), 0.31 );
    h.insert(Edge::new(cinco, seis), 0.2 );
    h.insert(Edge::new(seis, uno), -0.5);
    h.insert(Edge::new(seis, tres), 0.35 );


    let mut c : Canvas = Canvas::new();
    c.fill_nodes_and_edges(h);
    c.assign_coordinates_to_nodes();
    c.generate_graph_svg();
    c.write_to_output_file("a");

    */

}
