; ModuleID = 'main'
source_filename = "main"

define i32 @f(i32 %0, i32 %1) {
entry:
  %"1" = alloca i32, align 4
  store i32 0, i32* %"1", align 4
  %"2" = load i32, i32* %"1", align 4
  ret i32 %"2"
  ret i32 0
}

define i32 @main() {
entry:
  %"3" = call i32 @f()
  %"4" = alloca i32, align 4
  store i32 %"3", i32* %"4", align 4
  %"5" = load i32, i32* %"4", align 4
  ret i32 %"5"
  ret i32 0
}
