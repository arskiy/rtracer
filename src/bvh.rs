use crate::aabb::AABB;
use crate::hittable::*;
use crate::ray::Ray;

use rand::prelude::*;
use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;

/*
enum BVHNode {
    Branch { left: Arc<BVH>, right: Arc<BVH> },
    Leaf(Arc<dyn Hittable>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut objs: Vec<Arc<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        fn box_compare(time0: f32, time1: f32, axis: usize) -> impl FnMut(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_box = a.bounding_box(time0, time1);
                let b_box = b.bounding_box(time0, time1);

                if let (Some(a_bbox), Some(b_bbox)) = (a_box, b_box) {
                    if a_bbox.min[axis] - b_bbox.min[axis] < 0.0 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    panic!("no bounding box in bvh constructor (box_compare).");
                }
            }
        }

        let axis = rand::thread_rng().gen_range(0..2) as usize;

        let obj_span = objs.len();

        objs.sort_unstable_by(box_compare(time0, time1, axis));

        let tree: BVHNode = match obj_span {
            0 => panic!("no objects in scene"),
            1 => BVHNode::Leaf(objs.pop().unwrap()),

            _ => {
                let right = BVH::new(objs.drain(obj_span / 2..).collect(), time0, time1);
                let left = BVH::new(objs, time0, time1);

                BVHNode::Branch { left: Arc::new(left), right: Arc::new(right) }
            }
        };

        let bbox = match &tree {
            BVHNode::Leaf(leaf) => leaf.bounding_box(time0, time1).unwrap(),
            BVHNode::Branch { left, right } =>
                AABB::surrounding_box(&left.bounding_box(time0, time1).unwrap(), &right.bounding_box(time0, time1).unwrap()),
        };

        eprintln!("bbox: {:?}", bbox);

        Self {
            tree,
            bbox,
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r.clone(), t_min, t_max) {
            match &self.tree {
                BVHNode::Leaf(leaf) => leaf.hit(&r, t_min, t_max),

                BVHNode::Branch { left, right} => {
                    let left = left.hit(&r, t_min, t_max);
                    if let Some(l) = &left { t_max = l.t };
                    let right = right.hit(&r, t_min, t_max);
                    if right.is_some() { right } else { left }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
*/

// -------------------------------------------------------------------------------------
enum BVHNode {
    Branch { left: Arc<BVH>, right: Arc<BVH> },
    Leaf(Arc<dyn Hittable>)
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB
}

impl BVH {
    pub fn new(mut hitable: Vec<Arc<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        fn box_compare(time0: f32, time1: f32, axis: usize) -> impl FnMut(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min[axis] + a.max[axis];
                    let bc = b.min[axis] + b.max[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
        }

        fn axis_range(hitable: &Vec<Arc<dyn Hittable>>, time0: f32, time1: f32, axis: usize) -> f32 {
            let (min, max) = hitable.iter().fold((f32::MAX, f32::MIN), |(bmin, bmax), hit| {
                if let Some(aabb) = hit.bounding_box(time0, time1) {
                    (bmin.min(aabb.min[axis]), bmax.max(aabb.max[axis]))
                } else {
                    (bmin, bmax)
                }
            });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f32)> = (0..3)
            .map(|a| (a, axis_range(&hitable, time0, time1, a)))
            .collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;

        hitable.sort_unstable_by(box_compare(time0, time1, axis));
        let len = hitable.len();
        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = hitable.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH { tree: BVHNode::Leaf(leaf), bbox }
                } else {
                    panic!["no bounding box in bvh node"]
                }
            },
            _ => {
                let right = BVH::new(hitable.drain(len / 2..).collect(), time0, time1);
                let left = BVH::new(hitable, time0, time1);
                let bbox = AABB::surrounding_box(&left.bbox, &right.bbox);
                BVH { tree: BVHNode::Branch { left: Arc::new(left), right: Arc::new(right) }, bbox }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(&r, t_min, t_max) {
            match &self.tree {
                BVHNode::Leaf(leaf) => leaf.hit(&r, t_min, t_max),
                BVHNode::Branch { left, right} => {
                    let left = left.hit(&r, t_min, t_max);
                    if let Some(l) = &left { t_max = l.t };
                    let right = right.hit(&r, t_min, t_max);
                    if right.is_some() { right } else { left }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

/*
pub struct BvhTree {
    nodes: Vec<BvhNode>,
    root: NodeId,
}

struct BvhNode {
    left: Option<NodeId>,
    right: Option<NodeId>,
    aabb: Option<AABB>,
    hitable: Option<Arc<dyn Hittable>>,
}

#[derive(Copy, Clone)]
pub struct NodeId {
    index: usize,
}

impl BvhTree {
    fn hit(&self, id: NodeId, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let node = &self.nodes[id.index];

        if node.aabb.is_none() || node.aabb.is_some() && node.aabb.as_ref().unwrap().hit(r.clone(), tmin, tmax) {
            match node.hitable {
                Some(ref hitable) => return hitable.hit(&r, tmin, tmax),
                None => {}
            }

            let mut hit_left: Option<HitRecord> = None;
            let mut hit_right: Option<HitRecord> = None;

            if let Some(ref left_index) = node.left {
                hit_left = self.hit(*left_index, r, tmin, tmax);
            }

            if let Some(ref right_index) = node.right {
                hit_right = self.hit(*right_index, r, tmin, tmax);
            }

            match &hit_left {
                Some(left) => match &hit_right {
                    Some(right) => {
                        if left.t < right.t {
                            return hit_left;
                        } else {
                            return hit_right;
                        }
                    }
                    None => return hit_left,
                },
                None => {}
            }

            match hit_right {
                Some(ref _right) => return hit_right,
                None => {}
            }
        }

        None
    }
}

impl Hittable for BvhTree {
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        self.nodes[self.root.index].aabb.clone()
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hit(self.root, r, t_min, t_max)
    }
}

impl BvhTree {
    pub fn new(mut l: Vec<Arc<dyn Hittable>>, t_min: f32, t_max: f32) -> BvhTree {
        let mut tree = BvhTree {
            nodes: Vec::new(),
            root: NodeId { index: 0 },
        };
        tree.root = tree.build(l, t_min, t_max);

        tree
    }

    fn build(&mut self, mut l: Vec<Arc<dyn Hittable>>, t_min: f32, t_max: f32) -> NodeId {
        let axis = rand::thread_rng().gen_range(0..3);

        match axis {
            0 => l.sort_by(|a, b| box_x_compare(a, b, t_min, t_max)),
            1 => l.sort_by(|a, b| box_y_compare(a, b, t_min, t_max)),
            2 => l.sort_by(|a, b| box_z_compare(a, b, t_min, t_max)),
            _ => panic!("Unexpected axis"),
        }

        let left: NodeId;
        let right: NodeId;

        if l.len() == 1 {
            return self.new_leaf(l[0].clone(), t_min, t_max);
        } else if l.len() == 2 {
            left = self.new_leaf(l[0].clone(), t_min, t_max);
            right = self.new_leaf(l[1].clone(), t_min, t_max);
        } else {
            let half_len = l.len() / 2;
            let (left_hitables, right_hitables) = l.split_at_mut(half_len);

            left = self.build(left_hitables.to_vec(), t_min, t_max);
            right = self.build(right_hitables.to_vec(), t_min, t_max);
        }

        if let Some(left_box) = &self.nodes[left.index].aabb {
            if let Some(right_box) = &self.nodes[right.index].aabb {
                return self.new_node(
                    AABB::surrounding_box(&left_box, &right_box),
                    Some(left),
                    Some(right),
                );
            }
        }

        panic!("No bounding box in BvhNode::build");
    }

    fn new_leaf(&mut self, hitable: Arc<dyn Hittable>, t_min: f32, t_max: f32) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left: None,
            right: None,
            aabb: hitable.bounding_box(t_min, t_max),
            hitable: Some(hitable),
        });

        return NodeId { index: next_index };
    }

    fn new_node(&mut self, aabb: AABB, left: Option<NodeId>, right: Option<NodeId>) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left,
            right,
            aabb: Some(aabb),
            hitable: None,
        });

        return NodeId { index: next_index };
    }

    fn number_hittables(&self, id: NodeId) -> usize {
        let node = &self.nodes[id.index];
        let local_hitable = if node.hitable.is_some() { 1 } else { 0 };
        let count_left = if let Some(left_index) = node.left {
            self.number_hittables(left_index)
        } else {
            0
        };
        let count_right = if let Some(right_index) = node.right {
            self.number_hittables(right_index)
        } else {
            0
        };

        local_hitable + count_left + count_right
    }
}

impl fmt::Display for BvhTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BVH with {:?} hitables and {:?} nodes",
            self.number_hittables(self.root),
            self.nodes.len()
        )
    }
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, t_min: f32, t_max: f32) -> Ordering {
    if let Some(box_left) = a.bounding_box(t_min, t_max) {
        if let Some(box_right) = b.bounding_box(t_min, t_max) {
            if let Some(cmp) = box_left.min.x.partial_cmp(&box_right.min.x) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, t_min: f32, t_max: f32) -> Ordering {
    if let Some(box_left) = a.bounding_box(t_min, t_max) {
        if let Some(box_right) = b.bounding_box(t_min, t_max) {
            if let Some(cmp) = box_left.min.y.partial_cmp(&box_right.min.y) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, t_min: f32, t_max: f32) -> Ordering {
    if let Some(box_left) = a.bounding_box(t_min, t_max) {
        if let Some(box_right) = b.bounding_box(t_min, t_max) {
            if let Some(cmp) = box_left.min.z.partial_cmp(&box_right.min.z) {
                return cmp;
            } else {
                panic!("Can't compare");
            }
        }
    }

    panic!("No bounding box in BvhNode::new");
}
*/
