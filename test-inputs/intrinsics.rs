#![feature(link_llvm_intrinsics)]
//Credit: https://gist.github.com/hyunsik/7fa689200ee5ee6ecf3d

extern {
  #[link_name = "llvm.sqrt.f32"]
  fn sqrt(x: f32) -> f32;
}

fn main(){
  unsafe { sqrt(32.0f32); }
}
