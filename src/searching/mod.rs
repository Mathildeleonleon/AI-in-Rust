/* auto-exports start */
mod binary_search;
mod binary_search_recursive;
mod exponential_search;
mod fibonacci_search;
mod interpolation_search;
mod jump_search;
mod kth_smallest;
mod kth_smallest_heap;
mod linear_search;
mod moore_voting;
mod quick_select;
mod saddleback_search;
mod ternary_search;
mod ternary_search_min_max;
mod ternary_search_min_max_recursive;
mod ternary_search_recursive;

pub use binary_search::binary_search;
pub use binary_search_recursive::binary_search_rec;
pub use exponential_search::exponential_search;
pub use fibonacci_search::fibonacci_search;
pub use interpolation_search::interpolation_search;
pub use jump_search::jump_search;
pub use kth_smallest::kth_smallest;
pub use kth_smallest_heap::kth_smallest_heap;
pub use linear_search::linear_search;
pub use moore_voting::moore_voting;
pub use quick_select::quick_select;
pub use saddleback_search::saddleback_search;
pub use ternary_search::ternary_search;
pub use ternary_search_min_max::{
	ternary_search_max,
	ternary_search_min
};
pub use ternary_search_min_max_recursive::{
	ternary_search_max_rec,
	ternary_search_min_rec
};
pub use ternary_search_recursive::ternary_search_rec;
/* auto-exports end */
