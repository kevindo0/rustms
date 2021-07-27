#[macro_export]
macro_rules! add {
	// 处理是否单个参数通过的情况
	($a:expr) => {
		$a
	};
	// 处理是否两个参数通过的情况
	($a:expr, $b:expr) => {
		$a + $b
	};
	// 使用剩下的参数再次调用add宏
	($a:expr, $($b:tt)*) => {
		$a + add!($($b)*)
	}
}
