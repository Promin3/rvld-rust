use std::vec;
use std::env;
use std::process;
use std::rc::Rc;
use std::cell::RefCell;

use crate::linker::elf_file::ElfFile;
use crate::linker::machine_type::get_machinetype_from_contents;
use crate::linker::machine_type::MACHINE_TYPE_NONE;
use crate::linker::machine_type::MACHINE_TYPE_RV64;
use crate::linker::{context::Context, utils::fatal};
mod linker;


pub fn parse_args(ctx: &mut Context) -> Vec<String>{
   let mut args:Vec<String> = env::args().skip(1).collect();

   // 输出链接器参数
   println!("{:#?}", args);
   
   //生成短选项和长选项
   let dishes = |name: &str| -> Vec<String>{
      if name.len() == 1{
         return vec!["-".to_string() + name];
      }else{
         return vec!["-".to_string() + name, "--".to_string() + name]
      }
   };

   let arg = Rc::new(RefCell::new(String::new()));
   // 读取带参数的选项
   let read_arg = |name : &str, args: &mut Vec<String>| -> bool{
      for opt in dishes(name){

         // "-o", "out/tests/hello/out"
         if args[0] == opt{
            if args.len() == 1{
               fatal(format!("option -{}: argument missing", name).as_str());
            }

            *arg.borrow_mut() = args[1].clone();
            args.drain(0..2);
            return true;
         }

         // -plugin-opt=
         let mut prefix = opt;
         if name.len() > 1{
            prefix += "=";
         }

         // -plugin-opt=/usr/lib/gc....
         // -L/usr/lib/gcc-cross/ri...
         // -lgcc
         if args[0].starts_with(&prefix){
            *arg.borrow_mut() = args[0][prefix.len()..].to_string();
            args.drain(0..1);
            return true;
         }
      }
      false
   };


   let read_flag = |name: &str, args: &mut Vec<String>| -> bool{
      for opt in dishes(name){
         if args[0] == opt{
            args.drain(0..1);
            return true;
         }
      }
      false
   };

   let mut remaining: Vec<String> = vec![];

   while !args.is_empty(){
      if read_flag("help", &mut args){
         println!("usage: {} [options] file...", env::args().next().unwrap());
         process::exit(0);
      }else if read_arg("o", &mut args) || read_arg("output", &mut args){
         ctx.args.output = arg.borrow().clone();
      }else if read_flag("v", &mut args) || read_flag("version", &mut args) {
         println!("rvld {}", arg.borrow());
         process::exit(0);
      }else if read_arg("m", &mut args){
         if arg.borrow().as_str() == "elf64lriscv" {
            ctx.args.emulation = MACHINE_TYPE_RV64;
         }else {
             fatal(format!("unknown -m argument: {}", arg.borrow()).as_str());
         }
      }else if read_arg("L", &mut args){
         ctx.args.library_paths.push(arg.borrow().clone());
      }else if read_arg("l", &mut args){
         remaining.push(format!("-l{}", arg.borrow().clone()));
      }else if read_arg("sysroot", &mut args)
            || read_flag("static", &mut args)
            || read_arg("plugin", &mut args)
            || read_arg("plugin-opt", &mut args)
            || read_flag("as-needed", &mut args)
            || read_flag("start-group", &mut args)
            || read_flag("end-group", &mut args)
            || read_arg("hash-style", &mut args)
            || read_arg("build-id", &mut args)
            || read_flag("s", &mut args)
            || read_flag("no-relax", &mut args)
            || read_arg("z", &mut args)
      {
         // ignore
      }else{
         if args[0].starts_with("-"){
            fatal(format!("unknown command line option: {}",args[0]).as_str())
         }
         remaining.push(args[0].clone());
         args.drain(0..1);
      }  
   }
   remaining
}


fn main() {

   let mut ctx = Context::new();
   let remaining = parse_args(&mut ctx);

   print!("{:#?}", remaining);
   
   if ctx.args.emulation == MACHINE_TYPE_NONE{
      for filename in remaining{
         // 跳过
         // "--start-group"
         // "-lgcc"                  # GCC 运行时库
         // "-lgcc_eh"               # GCC 异常处理库
         // "-lc"                    # C 标准库
         // "--end-group"
         if filename.starts_with("-"){
            continue; 
         }
         let file = ElfFile::new(filename);
         ctx.args.emulation = get_machinetype_from_contents(&file.contents);

         if ctx.args.emulation != MACHINE_TYPE_NONE{
            break;
         }
      }
   }

   if ctx.args.emulation != MACHINE_TYPE_RV64{
      fatal("unknown emulation type");
   }

}
