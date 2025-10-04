// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::HashMap;

use crate::collections::BranchVec;
use crate::common::{Branch, Label};
use crate::introspection::{Step, Trace};
use crate::options::Options;

type NodeId = usize;

#[derive(Default)]
pub struct ForwardEdges {
    nodes: HashMap<Label, NodeId>,
    num_completely_visited: usize,
}

impl ForwardEdges {
    fn is_completely_visited(&self) -> bool {
        self.num_completely_visited == self.nodes.len().max(1)
    }
}

#[derive(Clone, Copy)]
pub struct BackwardEdge {
    node_id: NodeId,
    branch: Branch,
}

pub struct Node {
    parent: Option<BackwardEdge>,
    nexts: BranchVec<ForwardEdges>,
    is_final: bool,
    label: Label,
}

impl Node {
    pub fn new(label: Label, parent_edge: Option<BackwardEdge>) -> Self {
        Self {
            parent: parent_edge,
            nexts: Default::default(),
            is_final: false,
            label,
        }
    }

    pub fn is_completely_visited(&self) -> bool {
        self.is_final
            || self.nexts[Branch::Activate].is_completely_visited()
                && self.nexts[Branch::Skip].is_completely_visited()
    }
}

pub struct Tree {
    options: Options,
    nodes: Vec<Node>,
    roots: ForwardEdges,
    current_edge: Option<BackwardEdge>,
    non_determinism_witnessed: bool,
}

pub enum ExecutionStatus {
    Continue,
    Stop,
}

impl Tree {
    pub fn new(options: Options) -> Self {
        Self {
            options,
            nodes: Default::default(),
            roots: Default::default(),
            current_edge: None,
            non_determinism_witnessed: false,
        }
    }

    pub fn start(&mut self) {
        self.current_edge = None;
    }

    fn advance(&mut self, label: Label) -> NodeId {
        let new_node_id = self.nodes.len();

        let parent_nexts = if let Some(current_edge) = self.current_edge {
            &mut self.nodes[current_edge.node_id].nexts[current_edge.branch]
        } else {
            &mut self.roots
        };

        if let Some(current_node_id) = parent_nexts.nodes.get(&label) {
            *current_node_id
        } else {
            if parent_nexts.nodes.len() >= 1 {
                self.non_determinism_witnessed = true;
            }
            parent_nexts.nodes.insert(label, new_node_id);
            self.nodes.push(Node::new(label, self.current_edge));
            new_node_id
        }
    }

    pub fn finalize(&mut self, label: Label) -> ExecutionStatus {
        self.advance(label);

        let mut current_edge = self.current_edge;
        loop {
            if let Some(edge) = current_edge {
                let parent_node = &mut self.nodes[edge.node_id];
                let parent_nexts = &mut parent_node.nexts[edge.branch];
                assert!(parent_nexts.num_completely_visited <= parent_nexts.nodes.len());
                parent_nexts.num_completely_visited += 1;
                if parent_node.is_completely_visited() {
                    current_edge = parent_node.parent;
                } else {
                    return ExecutionStatus::Continue;
                }
            } else {
                self.roots.num_completely_visited += 1;
                assert!(self.roots.num_completely_visited <= self.roots.nodes.len());
                if self.roots.is_completely_visited() {
                    return ExecutionStatus::Stop;
                } else {
                    return ExecutionStatus::Continue;
                }
            };
        }
    }

    pub fn visit(&mut self, label: Label) -> Branch {
        let current_node_id = self.advance(label);

        let branches = match self.options.branch_preference {
            Branch::Activate => &[Branch::Activate, Branch::Skip],
            Branch::Skip => &[Branch::Skip, Branch::Activate],
        };

        for branch in *branches {
            let current_node = &mut self.nodes[current_node_id];
            let current_node_next = &mut current_node.nexts[branch];
            if !current_node_next.is_completely_visited() {
                self.current_edge = Some(BackwardEdge {
                    node_id: current_node_id,
                    branch,
                });
                return branch;
            }
        }

        unreachable!();
    }

    pub fn generate_current_trace(&self) -> Trace {
        self.generate_trace(self.current_edge.as_ref())
    }

    pub fn generate_trace(&self, final_edge: Option<&BackwardEdge>) -> Trace {
        let mut path = vec![];

        let mut current_edge = final_edge;
        while let Some(edge) = current_edge {
            let node = &self.nodes[edge.node_id];
            match node.label {
                Label::Failpoint(name) => {
                    path.push(Step {
                        name,
                        branch: edge.branch,
                    });
                }
                Label::Finished => {
                    unreachable!();
                }
            }
            current_edge = node.parent.as_ref();
        }

        path.reverse();
        Trace { path }
    }

    pub fn generate_all_traces(&self) -> Vec<Trace> {
        let mut res = vec![];
        for node in &self.nodes {
            if node.label == Label::Finished {
                res.push(self.generate_trace(node.parent.as_ref()));
            }
        }
        res
    }
}
