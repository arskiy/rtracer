use crate::aabb::AABB;
use crate::hittable::*;
use crate::ray::Ray;

use std::cmp::Ordering;
use rand::prelude::*;

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut objs: Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        fn box_compare(time0: f32, time1: f32, axis: usize) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_box = a.bounding_box(time0, time1);
                let b_box = b.bounding_box(time0, time1);

                if let (Some(a_bbox), Some(b_bbox)) = (a_box, b_box) { 
                    if a_bbox.min.at(axis) - b_bbox.min.at(axis) < 0.0 {
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

                BVHNode::Branch { left: Box::new(left), right: Box::new(right) }
            }
        };

        let bbox = match &tree {
            BVHNode::Leaf(leaf) => leaf.bounding_box(time0, time1).unwrap(),
            BVHNode::Branch { left, right } => 
                AABB::surrounding_box(left.bounding_box(time0, time1).unwrap(), right.bounding_box(time0, time1).unwrap()),
        };
        
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
