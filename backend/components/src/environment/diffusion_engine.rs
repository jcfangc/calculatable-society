// use crate::environment::coordinate::Coordinate;
// use crate::environment::coordinate_shift::CoordinateShift;
// use crate::environment::hexagon::neighbour_relation::NeighbourRelation;
// use crate::environment::hexagon::t_hexa_relational::HexaRelational;
// use crate::environment::subtance_distribution_state::SubstanceDistributionState;
// use crate::shared::property::Property;
// use crate::shared::subtance_type::SubstanceType;
// use ndarray::{Array2, Zip};
// use rayon::prelude::*;
// use std::collections::HashMap;
// use std::f64::consts::PI;
// use std::sync::{Arc, Mutex};

// pub struct DiffusionEngine<'a> {
//     pub potential: &'a Array2<f64>,
//     pub last_distribution: &'a Array2<SubstanceDistributionState>,
//     pub substance_type: SubstanceType,
//     // 使用 Arc<Mutex> 管理新的分布
//     new_distribution: Arc<Mutex<Array2<SubstanceDistributionState>>>,
// }

// impl<'a> DiffusionEngine<'a> {
//     pub fn new(
//         potential: &'a Array2<f64>,
//         last_distribution: &'a Array2<SubstanceDistributionState>,
//         substance_type: SubstanceType,
//         dimensions: (usize, usize),
//     ) -> Self {
//         Self {
//             potential,
//             last_distribution,
//             substance_type,
//             new_distribution: Arc::new(Mutex::new(
//                 Array2::from_elem(dimensions, SubstanceDistributionState::default()),
//             )),
//         }
//     }

//     pub fn new_distribution(&self) -> Arc<Mutex<Array2<SubstanceDistributionState>>> {
//         Arc::clone(&self.new_distribution)
//     }

//     // 通过锁获取对内部数据的独占访问权
//     pub fn get_new_distribution_mut(
//         &self,
//     ) -> std::sync::MutexGuard<'_, Array2<SubstanceDistributionState>> {
//         self.new_distribution
//             .lock()
//             .expect("Failed to lock new_distribution")
//     }
//     /// 计算某个单元中物质扩散的运动方向
//     fn calculate_diffusion_direction(
//         potential: &Array2<f64>,
//         center: Coordinate,
//         neighbours: &HashMap<NeighbourRelation, Coordinate>,
//     ) -> CoordinateShift {
//         // 获取中心点势能
//         let center_potential = potential[[*center.y(), *center.x()]];
//         // 各个邻居对于中心的总体影响
//         let total_gradient = neighbours
//             // 遍历邻居，获取每个邻居和中心的势差，得到最终的梯度
//             .iter()
//             // 将邻居映射为势能梯度
//             .map(|(relation, nb_coordinate)| {
//                 // 获取邻居势能，准备和中心势能做差
//                 let neighbour_potential = potential[[*nb_coordinate.y(), *nb_coordinate.x()]];
//                 // 计算势能差异，描述邻居的势能高于中心的程度
//                 let disparity = neighbour_potential - center_potential;
//                 // 梯度方向
//                 let coordinate_shift =
//                     NeighbourRelation::from_relation_to_coordinate_shift()[relation];

//                 // 模长 * 方向，得到实际梯度
//                 disparity * coordinate_shift
//             })
//             // 加总，得到各个邻居对中心的总体影响（梯度）
//             .sum::<CoordinateShift>()
//             // 每个邻居会影响各自的六个邻居，中心点这是这些邻居的邻居中的一个，提出公因式
//             * (1 / 6);

//         // 物质运动方向往往是沿着负梯度的，因此取反
//         total_gradient.reverse()
//     }

//     /// 计算本物质的流动性
//     fn calculate_fluidity(&self) -> f64 {
//         let property_params = Property::to_map()
//             .get(&Property::Fluidity)
//             .expect("Fluidity property not found");
//         property_params.calculate(&self.substance_type)
//     }

//     /// 跟据邻居方向和总体负梯度的方向差异，计算出梯度分量
//     fn calculate_partial_negative_gradient(
//         negative_gradient: CoordinateShift,
//         angle_disparity: f64,
//         neighbour_potential: f64,
//     ) -> f64 {
//         let partial_potential = (((2.0 * PI - angle_disparity) / (9.0 * PI)) * (negative_gradient.magnitude() as f64)).round();
//         (partial_potential - neighbour_potential).max(0.0)
//     }

//     fn calculate_substance_movement(
//         &self,
//         partial_potential: f64,
//         center_substance_state: &SubstanceDistributionState,
//     ) -> usize {
//         let fluidity = self.calculate_fluidity();
//         (partial_potential * fluidity).round() as usize * center_substance_state.amount()
//     }

//     fn update_state_and_neighbours(
//         last_distribution: &Array2<SubstanceDistributionState>,
//         new_distribution: &mut Array2<SubstanceDistributionState>,
//         new_state: &mut SubstanceDistributionState,
//         center: Coordinate,
//         neighbours: &HashMap<NeighbourRelation, Coordinate>,
//         negative_gradient: CoordinateShift,
//     ) {
//         let center_coords = [*center.y(), *center.x()];
//         let magnitude = negative_gradient.magnitude();

//         for (relation, nb_coordinate) in neighbours.iter() {
//             let neighbour_coords = [*nb_coordinate.y(), *nb_coordinate.x()];
//             let coordinate_shift = NeighbourRelation::from_relation_to_coordinate_shift()[relation];

//             let angle_disparity = coordinate_shift.angle_between(negative_gradient).abs();
//             let partial_potential = Self::calculate_partial_negative_gradient(
//                 negative_gradient,
//                 angle_disparity,
//                 last_distribution[neighbour_coords],
//             );

//             let center_substance_state = last_distribution[center_coords];
//             let partial_substance_movement =
//                 Self::calculate_substance_movement(partial_potential, &center_substance_state);

//             // 更新中心格点的状态
//             new_state.adjust_amount(-(partial_substance_movement as isize));

//             // 更新邻居的状态
//             new_distribution[neighbour_coords].adjust_amount(partial_substance_movement as isize);
//         }
//     }

//     pub fn apply_diffusion(&mut self) {
//         let dimensions = self.last_distribution.dim();

//         // 遍历新分布的每个点
//         Zip::indexed(&mut self.new_distribution)
//             .par_for_each(|(y, x), new_state| {
//                 let center = Coordinate::new(y, x);
//                 let neighbours = center.get_relations_map::<NeighbourRelation>(dimensions);

//                 let negative_gradient = Self::calculate_diffusion_direction(
//                     &self.potential,
//                     center,
//                     &neighbours,
//                 );

//                 // 更新当前格点和邻居状态
//                 Self::update_state_and_neighbours(
//                     &self.last_distribution,
//                     &mut self.new_distribution,
//                     new_state,
//                     center,
//                     &neighbours,
//                     negative_gradient,
//                 );
//             });
//     }
// }
