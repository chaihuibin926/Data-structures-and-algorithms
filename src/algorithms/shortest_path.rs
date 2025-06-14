use crate::data_structures::heap::{HasValue, MinHeap};

pub struct Graph {
    adj: Vec<Vec<Edge>>, // 邻接表
    v: usize, // 顶点个数
}

impl Graph {
    pub fn new(v: usize) -> Graph {
        let mut adj = Vec::with_capacity(v);
        for _ in 0..v {
            adj.push(Vec::new());
        }

        Graph {
            adj,
            v,
        }
    }

    pub fn add_edge(&mut self, sid: usize, tid: usize, w: usize) {
        self.adj[sid].push(Edge::new(sid, tid, w));
    }

    // 从顶点 s 到顶点 t 的最短路径
    pub fn dijkstra(&self, s: usize, t: usize) {
        // 用来还原最短路径
        let mut predecessor: Vec<usize> = Vec::with_capacity(self.v);
        // 记录s到每一个点的最短距离
        let mut vertexes: Vec<Vertex> = Vec::with_capacity(self.v);
        for i in 0..self.v {
            vertexes.push(Vertex::new(i, usize::MAX));
        }
        let mut queue = MinHeap::new(self.v);
        // 标记是否进入过队列
        let mut inqueue: Vec<bool> = Vec::with_capacity(self.v);
        
        vertexes[s].dist = 0;
        queue.push(vertexes[s].clone());
        inqueue[s] = true;

        while !queue.is_empty() {
            // 取出堆顶元素
            let min_vertex = queue.pop().unwrap();
            // 最短路径产生
            if min_vertex.id == t {
                break;
            }

            for i in 0..self.adj[min_vertex.id].len() {
                // 取出一条min_vetex相连的边
                let edge = &self.adj[min_vertex.id][i];
                let next_dist: usize = min_vertex.dist + edge.w;
                // 更新next的dist
                if next_dist < vertexes[edge.tid].dist {
                    vertexes[edge.tid].dist = next_dist;
                    predecessor[edge.tid] = min_vertex.id;
                    if inqueue[vertexes[edge.tid].id] == true {
                        // queue.update(vertexes[edge.tid].clone());
                    } else {
                        queue.push(vertexes[edge.tid].clone());
                        inqueue[edge.tid] = true;
                    }
                }
            }
        }

        println!("s: {}, t: {}, predecessor: {:?}", s, predecessor[t], predecessor);
    }
}

struct Edge {
    sid: usize, // 边的起始顶点编号
    tid: usize, // 边的终止顶点编号
    w: usize, // 权重
}

impl Edge {
    fn new(sid: usize, tid: usize, w: usize) -> Edge {
        Edge {
            sid,
            tid,
            w,
        }
    }
}

#[derive(Clone)]
struct Vertex {
    id: usize, // 顶点编号 ID
    dist: usize, // 从起始顶点到这个顶点的距离
}

impl Vertex {
    pub fn new(id: usize, dist: usize) -> Vertex {
        Vertex {
            id,
            dist
        }
    }
}

impl HasValue for Vertex {
    type Value = usize;
    fn value(&self) -> usize {
        self.dist
    }
}