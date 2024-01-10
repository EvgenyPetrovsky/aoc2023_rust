use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Vertex {
    name: String,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Edge {
    v1: Vertex,
    v2: Vertex,
}
pub struct Graph {
    vs: HashSet<Vertex>,
    es: HashSet<Edge>,
}

type P = Graph;

pub struct DaySolution(P);

impl Graph {
    fn count_connected_vertices(&self, start: &Vertex) -> usize {
        let start_name = start.name.clone();

        fn iterate(acc_names: HashSet<String>, es: &HashSet<Edge>) -> usize {
            let new_names: HashSet<String> = es
                .iter()
                .filter(|e| {
                    acc_names.contains(e.v1.name.as_str()) || acc_names.contains(e.v2.name.as_str())
                })
                .flat_map(|e| {
                    let v1 = e.v1.name.clone();
                    let v2 = e.v2.name.clone();
                    HashSet::from([v1, v2])
                })
                .collect();

            let new_len = new_names.len();
            if new_len == acc_names.len() {
                new_len
            } else {
                iterate(new_names, es)
            }
        }

        iterate(HashSet::from([start_name]), &self.es)
    }
}

impl DaySolution {
    // this is parsing of input file data
    fn extract_edges(line: &str) -> HashSet<Edge> {
        let re_line = Regex::new(r#"(\w+): (.*)"#).unwrap();
        let re_v2s = Regex::new(r#"(\w+)"#).unwrap();
        let (v1, v2s_str) = re_line
            .captures(line)
            .map(|c| {
                let (_, [v1, v2s]) = c.extract();
                (v1, v2s)
            })
            .unwrap();
        let v2s = re_v2s
            .captures_iter(v2s_str)
            .map(|c| c.get(0).unwrap().as_str());

        v2s.map(|v2| {
            let v1 = Vertex {
                name: String::from(v1),
            };
            let v2 = Vertex {
                name: String::from(v2),
            };
            /*
            match v1.cmp(&v2) {
                std::cmp::Ordering::Less => Edge { v1, v2 },
                _ => Edge { v1: v2, v2: v1 },
            }
            */
            Edge { v1, v2 }
        })
        .collect()
    }
    // this is parsing of input file data
    fn extract_vertices(edges: &HashSet<Edge>) -> HashSet<Vertex> {
        let v1s = edges.iter().map(|e| e.v1.clone());
        let v2s = edges.iter().map(|e| e.v2.clone());
        v1s.chain(v2s).collect()
    }
    // count all vertices which are reacheable from the start vertex in the graph
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 25;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        let es: HashSet<Edge> = text_input
            .lines()
            .flat_map(DaySolution::extract_edges)
            .collect();
        let vs = DaySolution::extract_vertices(&es);
        Graph { vs, es }
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let es = problem.es;
        es.iter().for_each(|_e| {
            //println!("{} -> {};", _e.v1.name, _e.v2.name);
            ()
        });
        /* the output was used for visualization in Graph Viz online tool using:
            digraph G {

                v1 -> v2;

            }
        ======
        3 edges that join 2 clusters of vertices are:
        tjz -> vph;
        zkt -> jhq;
        pgt -> lnr;

        ======
        2 vertices that belong to different clusters are:
        ttc, txc
        */
        println!("Initial number of connections: {}", es.len());
        let es_red: HashSet<Edge> = es
            .into_iter()
            .filter(
                |Edge { v1, v2 }| match (v1.name.as_str(), v2.name.as_str()) {
                    ("tjz", "vph") => false,
                    ("zkt", "jhq") => false,
                    ("pgt", "lnr") => false,
                    _ => true,
                },
            )
            .collect();

        println!("Reduced number of connections: {}", es_red.len());

        let g_red = Graph {
            vs: problem.vs,
            es: es_red,
        };

        let g1_count = g_red.count_connected_vertices(&Vertex {
            name: String::from("ttc"),
        });
        let g2_count = g_red.count_connected_vertices(&Vertex {
            name: String::from("txc"),
        });

        println!(
            "Total number of components: {}, 1st group: {}, 2nd group: {}",
            g_red.vs.len(),
            g1_count,
            g2_count
        );

        let answer = g1_count * g2_count;
        Some(answer)
    }

    fn solve_part_2(_problem: Self::Problem) -> Self::Answer {
        None
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
