use std::borrow::Cow;

// A clone-on-write smart pointer.

// Cow的泛型参数。包含一个生命周期 'a 和 类型 B。
// B的类型签名。可以看到B必须实现了了两个trait: ToOwned + ?Sized，还有一个生命周期 'a, 类型 B 的引用生命周期不小于 'a.
// ToOwned。这个可以克隆数据到所有者。但有人肯定会问了，为什么不直接用 Clone。这二者还是有区别的，Clone 是从 &T 到 T，但ToOwned要强大些，可以从任意借用类型构建新类型实例

// 假如有个数组里包含了一些字符串，有些字符串有统一的前缀名，有些没有，也有可能全部都有或者全部没有。
// 如果没有这个前缀名，那就插入这个前缀名。并且你不想修改输入的字符串，可能留作后用。

#[allow(dead_code)]
fn insert_prefix_clone<'a>(strs: impl IntoIterator<Item=&'a String>, prefix: &'a str) -> Vec<String> {
	strs.into_iter().filter_map(|s|
		match s.starts_with(prefix) {
			true => Some(s.clone()),
			false => Some(String::with_capacity(prefix.len() + s.len()) + prefix + s),
		}
	).collect::<Vec<String>>()
}


fn insert_prefix_cow<'a, T>(strs: T, prefix: &'a str) -> Vec<Cow<'a, String>>
where T: IntoIterator<Item=&'a String>
{
	strs.into_iter().filter_map(|s|
		match s.starts_with(prefix) {
			true => Some(Cow::Borrowed(s)),
			false => Some(Cow::Owned(String::with_capacity(prefix.len() + s.len()) + prefix + s)),
		}
	).collect::<Vec<Cow<'a, String>>>()
}


fn main() {
	let strs = vec!["row_rust".to_string(), "rust".to_string()];
	let p = "row_";
	// let fixed = insert_prefix_clone(&strs, &p);
	let fixed = insert_prefix_cow(&strs, &p);
	println!("{:?}", fixed);
	let s0 = &strs[0];
	let f0 = &*fixed[0];  // Cow实现了Deref，所以可以直接解引用。
	println!("source addr: {:?}", s0 as *const String);		// 0x7fd68bc02d80
	println!("cow addr: {:?}", f0 as *const String);     	// 0x7fd68bc02d80 地址相同

	let s1 = &strs[1]; // 第二个元素插入了前缀名
	let f1 = &*fixed[1];
	println!("source addr: {:?}", s1 as *const String); 	// 0x7fd68bc02d98
	println!("cow addr: {:?}", f1 as *const String); 		// 0x7fd68bc02e18，地址已经发生了变化
}
