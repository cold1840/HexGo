#![allow(dead_code)]

use std::collections::HashSet;

const MAX_NEIGHBORS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VertexId(usize);

impl VertexId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardError {
    UnknownVertex(VertexId),
    SelfLoop(VertexId),
    TooManyNeighbors(VertexId),
    DuplicateEdge(VertexId, VertexId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardGraph {
    neighbors: Vec<Vec<VertexId>>,
}

impl BoardGraph {
    pub fn from_edges(
        vertex_count: usize,
        edges: impl IntoIterator<Item = (VertexId, VertexId)>,
    ) -> Result<Self, BoardError> {
        let mut adjacency = vec![Vec::new(); vertex_count];
        let mut seen = HashSet::new();

        for (left, right) in edges {
            if left.index() >= vertex_count {
                return Err(BoardError::UnknownVertex(left));
            }
            if right.index() >= vertex_count {
                return Err(BoardError::UnknownVertex(right));
            }
            if left == right {
                return Err(BoardError::SelfLoop(left));
            }

            let edge = if left < right {
                (left, right)
            } else {
                (right, left)
            };

            if !seen.insert(edge) {
                return Err(BoardError::DuplicateEdge(edge.0, edge.1));
            }

            adjacency[left.index()].push(right);
            adjacency[right.index()].push(left);
        }

        for (index, neighbors) in adjacency.iter_mut().enumerate() {
            if neighbors.len() > MAX_NEIGHBORS {
                return Err(BoardError::TooManyNeighbors(VertexId::new(index)));
            }

            neighbors.sort_unstable();
        }

        Ok(Self {
            neighbors: adjacency,
        })
    }

    pub fn vertex_count(&self) -> usize {
        self.neighbors.len()
    }

    pub fn contains(&self, vertex: VertexId) -> bool {
        vertex.index() < self.vertex_count()
    }

    pub fn get_neighbors(&self, vertex: VertexId) -> Option<&[VertexId]> {
        self.neighbors.get(vertex.index()).map(Vec::as_slice)
    }

    pub fn are_adjacent(&self, left: VertexId, right: VertexId) -> bool {
        self.get_neighbors(left)
            .is_some_and(|neighbors| neighbors.binary_search(&right).is_ok())
    }

    pub fn vertices(&self) -> impl ExactSizeIterator<Item = VertexId> {
        (0..self.vertex_count()).map(VertexId::new)
    }
}
