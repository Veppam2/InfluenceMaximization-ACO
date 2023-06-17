use std::process::exit;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hasher;
use std::hash::Hash;

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;


use crate::wrappers::Node;
use crate::wrappers::Edge;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct XYNode{

    node : Node,

    pos : (f32, f32),
    disp : (f32, f32),

}
impl XYNode{
    fn new( node : Node, x: f32, y: f32)->Self{
        XYNode{
            node: node,
            pos : (x,y),
            disp : (0.0,0.0)
        }
    }
    fn set_pos( &mut self,  pair : (f32, f32) ){
        self.pos = pair;
    }
    fn set_disp(& mut self, pair : (f32, f32) ){
        self.disp = pair;
    }
    fn get_disp(&self )->(f32,f32){
        self.disp
    }
    fn get_pos(&self )->(f32,f32){
        self.pos
    }
}
impl PartialEq for XYNode{
    fn eq( &self, other: &Self )-> bool{
        self.node == other.node
    }
}
impl Eq for XYNode{}

impl Hash for XYNode{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.node.hash(state);
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq, Eq, Hash)]
struct XYEdge{
    u : XYNode,
    v : XYNode
}
impl XYEdge{
    fn get_u(&self)-> &XYNode{
        &self.u
    }
    fn get_v(&self)-> &XYNode{
        &self.v
    }
}

pub struct Canvas{
    pixels_to_cm : f32,

    width : f32,
    height : f32,

    graph_nodes : HashSet<XYNode>,
    graph_edges : HashMap<XYEdge, f32>,
    edges_pheromones : HashMap<XYEdge, f32>,

    canvas_file : String,

}

impl Canvas{

    pub fn new()->Self{

        let width = 30.0;
        let height = 30.0;
        let pixels_to_cm = 39.37;


        /*
        start = format!("{}{}",start,
            format!(
                "
                <circle cx='5' cy='5' r='2' stroke='black' stroke-width='0.05' fill='red' />
                "
            )
        );
        */

        let nodes : HashSet<XYNode> = HashSet::new();
        let edges : HashMap<XYEdge, f32> = HashMap::new();
        let pheromones : HashMap<XYEdge, f32> = HashMap::new();
        let start : String = String::new();

        Canvas{
            pixels_to_cm: pixels_to_cm,
            width : width,
            height : height,
            graph_nodes : nodes,
            graph_edges : edges,
            edges_pheromones : pheromones,
            canvas_file : start
        }

    }

    pub fn fill_nodes_and_edges(&mut self,  adjacencies : HashMap<Edge, f32> ){

        let mut graph_nodes : HashSet<XYNode> = HashSet::new();
        let mut random_generator = StdRng::seed_from_u64(1);

        for (e,w) in adjacencies{


            let u_x_rand : f32 = random_generator.gen_range(1.0..self.width );
            let u_y_rand : f32 = random_generator.gen_range(1.0..self.height );
            let v_x_rand : f32 = random_generator.gen_range(1.0..self.width );
            let v_y_rand : f32 = random_generator.gen_range(1.0..self.height );


            let u : XYNode = XYNode::new( *e.get_u() , u_x_rand, u_y_rand );
            let v : XYNode = XYNode::new( *e.get_v() , v_x_rand, v_y_rand );

            self.graph_nodes.insert(u);
            self.graph_nodes.insert(v);



            self.graph_edges.insert( XYEdge{ u: u, v: v }, w );

        }

        //self.graph_nodes = graph_nodes.into_iter().collect();

    }

    pub fn assign_coordinates_to_nodes(&mut self){

        let mut graph_nodes : Vec<XYNode>= self.graph_nodes.clone().into_iter().collect();
        println!("INIIALMENTE");
        for v in &graph_nodes{
            println!("{:?}", v);
        }
        let area : f32 = self.width  * self.height;
        let k : f32 = (area / (self.graph_nodes.len() as f32) ).sqrt();
        let mut t: f32 = self.width;
        let cool_t : f32 = 0.95;


        for _i in 1..50{
        graph_nodes = self.graph_nodes.clone().into_iter().collect();

            //Calculate repulsive forces
            for v_index in 0..graph_nodes.len() {

                let mut v = graph_nodes[v_index];
                v.set_disp( (0.0, 0.0) );

                for u_index in 0..graph_nodes.len(){

                    if u_index != v_index {
                        let mut u = graph_nodes[u_index];

                        let v_pos = v.get_pos();
                        let u_pos = u.get_pos();
                        let delta_pos : (f32, f32) = (v_pos.0 - u_pos.0, v_pos.1 - u_pos.1 );
                        let delta_norm : f32 = (delta_pos.0.powf(2.0) + delta_pos.1.powf(2.0)).sqrt();
                        println!("v: {:?}: \n u:{:?} \nDelta NORAM: {}", v, u,delta_norm);
                        let fr : f32 = k.powf(2.0) /delta_norm;
                        println!("fr: {}", fr);
                        let delta_pos_norm : (f32, f32) = (delta_pos.0 / delta_norm, delta_pos.1 / delta_norm);

                        let v_disp = v.get_disp();
                        v.set_disp(
                            (v_disp.0+delta_pos_norm.0*fr,
                             v_disp.1+delta_pos_norm.1*fr)
                        );

                    }
                }
            }
            //Backup changed nodes:
            self.graph_nodes = HashSet::new();
            for v in &graph_nodes{
                self.graph_nodes.insert(*v);
                println!( "NODO V : {:?}", v);
            }
            println!(" 2 {:?}", self.graph_nodes);


            //Calculate attractive forces
            for (e,_) in &self.graph_edges{
                let e_u : &XYNode = e.get_u();
                let e_v : &XYNode = e.get_v();

                let mut v : XYNode = self.graph_nodes.get(e_v).unwrap().clone();
                let mut u : XYNode = self.graph_nodes.get(e_u).unwrap().clone();
;
                let v_pos : (f32,f32) = v.get_pos();
                let u_pos : (f32,f32) = u.get_pos();
                let delta_pos = (v_pos.0 -u_pos.0, v_pos.1-u_pos.1);
                let delta_norm : f32 = (delta_pos.0.powf(2.0) + delta_pos.1.powf(2.0)).sqrt();
                let fa : f32 = delta_norm.powf(2.0) /k;
                let delta_pos_norm : (f32, f32) = (delta_pos.0 / delta_norm, delta_pos.1 / delta_norm);

                let v_disp : (f32,f32) = v.get_disp();
                let u_disp : (f32,f32) = u.get_disp();

                v.set_disp(
                    (v_disp.0 - delta_pos_norm.0 * fa,
                     v_disp.1 - delta_pos_norm.1 * fa)
                );
                u.set_disp(
                    (u_disp.0 + delta_pos_norm.0 * fa,
                     u_disp.1 + delta_pos_norm.1 * fa)
                );

                //Reinsert
                self.graph_nodes.remove(e_v);
                self.graph_nodes.remove(e_u);
                self.graph_nodes.insert(v);
                self.graph_nodes.insert(u);


            }

            //Limit max dispacement to termperature t and prevent from displacement
            graph_nodes = self.graph_nodes.clone().into_iter().collect();
            for v_index in 0..graph_nodes.len(){
                let mut v = graph_nodes[v_index];
                let v_pos = v.get_pos();
                let v_disp = v.get_disp();

                let v_disp_norm : f32 = (v_disp.0.powf(2.0) + v_disp.1.powf(2.0)).sqrt();
                let v_disp_normed : (f32, f32) =(v_disp.0 / v_disp_norm, v_disp.1 / v_disp_norm);
                let min : f32 = if t < v_disp_norm { t } else { v_disp_norm };

                let pre_v_pos : (f32, f32)  = (
                    v_pos.0 + v_disp_normed.0 * min,
                    v_pos.1 + v_disp_normed.1 * min,
                );

                let w_2 = (self.width/2.0);
                let h_2 = (self.height/2.0);

                let max_x = if -1.0*w_2 > pre_v_pos.0 { -1.0*w_2 } else {pre_v_pos.0};
                let max_y = if -1.0*h_2 > pre_v_pos.1 { -1.0*h_2 } else {pre_v_pos.1};

                let v_pos_x = if w_2 < max_x { w_2 } else { max_x };
                let v_pos_y = if h_2 < max_y { h_2 } else { max_y };

                //Set values
                v.set_pos(
                  (v_pos_x, v_pos_y)
                );
                println!( "NODO F : {:?}", v);

            }
            //Backup changed nodes:
            self.graph_nodes = HashSet::new();
            for v in &graph_nodes{
                self.graph_nodes.insert(*v);
            }

            //Reduce temperature
            t = t*cool_t;

        }
    }

    pub fn generate_graph_svg(&mut self){

        //Create sgv_file
        let mut start : String = format!(
            "<?xml version='1.0' encoding='UTF-8' ?>
            <svg width='{}cm' height='{}cm'>
                <g transform='scale({})'>
            ",
            self.width*2.0, self.height*2.0, self.pixels_to_cm
        );
        self.canvas_file = format!("{}{}", self.canvas_file, start);
        //Fill with directed edges


       //Fill with circles
       for v in &self.graph_nodes{
           let v_pos : (f32, f32) = v.get_pos();
           let v_sgv = format!(
               "<circle cx='{}' cy='{}' r='1' stroke='black' stroke-width='0.05' fill='red' />",
               v_pos.0, v_pos.1
           );
           self.canvas_file = format!("{}{}", self.canvas_file, v_sgv);
           println!("x: {}, y:{}", v_pos.0, v_pos.1);
       }


        //End of SVG
        let end : String = format!(
            "   </g>
            </svg>
            "
        );
        self.canvas_file = format!("{}{}", self.canvas_file, end);

    }

    pub fn write_to_output_file( &self, file: &str  ){
        let outfile = OpenOptions::new()
                        .write(true).create_new(true)
                        .open(format!("{}.svg",file) );
        match outfile {
            Ok(mut out) =>{
                match writeln!(out, "{}",self.canvas_file){
                    Ok(_) =>{},
                    Err(_)=>{
                        println!("Error while writing output file:{}", file);
                    }
                }
            }
            Err(_)=>{
                println!("File '{}' could not be written\n There must not exist a file with the same name in directory",file);
                exit(1);
            }
        }

    }


}
