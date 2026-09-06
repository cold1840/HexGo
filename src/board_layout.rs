use std::collections::{BTreeMap, BTreeSet};

use crate::game::board::{BoardGraph, VertexId};

const SQRT_3: f32 = 1.732_050_8;
const COMPACT_ROW_LENGTHS: [usize; 9] = [4, 3, 4, 3, 4, 3, 4, 3, 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LatticePoint {
    x: i32,
    y: i32,
}

impl LatticePoint {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn position(self) -> [f32; 2] {
        [self.x as f32 * 0.5, -(self.y as f32) * SQRT_3 * 0.5]
    }
}

/// A board topology and its stable presentation coordinates.
#[derive(Debug, Clone)]
pub struct BoardDefinition {
    graph: BoardGraph,
    positions: Vec<[f32; 2]>,
    edges: Vec<(VertexId, VertexId)>,
}

impl BoardDefinition {
    /// Builds the provisional board shown in the Phase 2 reference image.
    pub fn compact() -> Self {
        let mut points = BTreeSet::new();
        let mut lattice_edges = BTreeSet::new();

        for (row, cell_count) in COMPACT_ROW_LENGTHS.into_iter().enumerate() {
            let center_x = if row % 2 == 0 { 0 } else { 3 };

            for column in 0..cell_count {
                let center_x = center_x + (column as i32 * 6);
                let center_y = row as i32;
                let corners = [
                    LatticePoint::new(center_x + 2, center_y),
                    LatticePoint::new(center_x + 1, center_y + 1),
                    LatticePoint::new(center_x - 1, center_y + 1),
                    LatticePoint::new(center_x - 2, center_y),
                    LatticePoint::new(center_x - 1, center_y - 1),
                    LatticePoint::new(center_x + 1, center_y - 1),
                ];

                points.extend(corners);
                for index in 0..corners.len() {
                    let left = corners[index];
                    let right = corners[(index + 1) % corners.len()];
                    lattice_edges.insert(if left < right {
                        (left, right)
                    } else {
                        (right, left)
                    });
                }
            }
        }

        let ordered_points: Vec<_> = points.into_iter().collect();
        let ids: BTreeMap<_, _> = ordered_points
            .iter()
            .copied()
            .enumerate()
            .map(|(index, point)| (point, VertexId::new(index)))
            .collect();
        let edges: Vec<_> = lattice_edges
            .into_iter()
            .map(|(left, right)| (ids[&left], ids[&right]))
            .collect();
        let graph = BoardGraph::from_edges(ordered_points.len(), edges.iter().copied())
            .expect("the compact board preset must always form a valid graph");

        let mut positions: Vec<_> = ordered_points
            .into_iter()
            .map(LatticePoint::position)
            .collect();
        center_positions(&mut positions);

        Self {
            graph,
            positions,
            edges,
        }
    }

    pub fn graph(&self) -> &BoardGraph {
        &self.graph
    }

    pub fn positions(&self) -> &[[f32; 2]] {
        &self.positions
    }

    pub fn position(&self, vertex: VertexId) -> Option<[f32; 2]> {
        self.positions.get(vertex.index()).copied()
    }

    pub fn edges(&self) -> &[(VertexId, VertexId)] {
        &self.edges
    }

    pub fn bounds(&self) -> ([f32; 2], [f32; 2]) {
        self.positions.iter().fold(
            ([f32::INFINITY; 2], [f32::NEG_INFINITY; 2]),
            |(mut min, mut max), position| {
                min[0] = min[0].min(position[0]);
                min[1] = min[1].min(position[1]);
                max[0] = max[0].max(position[0]);
                max[1] = max[1].max(position[1]);
                (min, max)
            },
        )
    }
}

fn center_positions(positions: &mut [[f32; 2]]) {
    let (min, max) = positions.iter().fold(
        ([f32::INFINITY; 2], [f32::NEG_INFINITY; 2]),
        |(mut min, mut max), position| {
            min[0] = min[0].min(position[0]);
            min[1] = min[1].min(position[1]);
            max[0] = max[0].max(position[0]);
            max[1] = max[1].max(position[1]);
            (min, max)
        },
    );
    let center = [(min[0] + max[0]) * 0.5, (min[1] + max[1]) * 0.5];

    for position in positions {
        position[0] -= center[0];
        position[1] -= center[1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_board_has_expected_topology() {
        let definition = BoardDefinition::compact();

        assert_eq!(definition.graph().vertex_count(), 88);
        assert_eq!(definition.positions().len(), 88);
        assert_eq!(definition.edges().len(), 119);
        assert!(definition.graph().vertices().all(|vertex| matches!(
            definition.graph().get_neighbors(vertex).unwrap().len(),
            2 | 3
        )));
    }

    #[test]
    fn coordinates_and_graph_ids_stay_in_lockstep() {
        let definition = BoardDefinition::compact();
        let unique_positions: BTreeSet<_> = definition
            .positions()
            .iter()
            .map(|position| (position[0].to_bits(), position[1].to_bits()))
            .collect();

        assert_eq!(unique_positions.len(), definition.graph().vertex_count());
        assert!(definition.edges().iter().all(|&(left, right)| {
            definition.position(left).is_some()
                && definition.position(right).is_some()
                && definition.graph().are_adjacent(left, right)
        }));
    }

    #[test]
    fn compact_board_is_centered() {
        let definition = BoardDefinition::compact();
        let (min, max) = definition.bounds();

        assert!((min[0] + max[0]).abs() < f32::EPSILON);
        assert!((min[1] + max[1]).abs() < f32::EPSILON);
    }
}
