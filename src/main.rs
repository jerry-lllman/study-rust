// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好";
//     let e&nglish = "World, hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter() {
//         println!("{}", &region)
//     }
// }
// fn main() {
//     greet_world()
// }


// https://course.rs/first-try/hello-world.html#rust-%E8%AF%AD%E8%A8%80%E5%88%9D%E5%8D%B0%E8%B1%A1
fn main() {
  let penguin_data = "\
  common name,length (cm)
  Little penguin,33
  Yellow-eyed penguin,65
  Fiordland penguin,60
  Invalid,data
  ";

  let records = penguin_data.lines();

  for (i, record) in records.enumerate() {
    if i == 0 || record.trim().len() == 0 {
      continue;
    }

    // 声明一个 fields 变量，类型是 Vec
    // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
    // <_> 表示 Vec 中的元素类型由编译器自行推断
    let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
    // 只有在debug模式下生效
    if cfg!(debug_assertions) {
      // 输出到标准错误输出
      eprintln!("debug: {:?} -> {:?}", record, fields)
    }

    let name = fields[0];

    if let Ok(length) = fields[1].parse::<f32>() {
      // 输出到标准输出
      println!("{}, {}cm", name, length)
    }
  }
}