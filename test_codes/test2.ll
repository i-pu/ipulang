; ModuleID = 'main'
source_filename = "main"

define i32 @main(i32 %0) {
entry:
  %a = alloca i32, align 4
  store i32 0, i32* %a, align 4
  %"1" = alloca i32, align 4
  store i32 2, i32* %"1", align 4
  %"2" = load i32, i32* %"1", align 4
  %"3" = load i32, i32* %a, align 4
  %"4" = add i32 %"2", %"3"
  %"5" = alloca i32, align 4
  store i32 %"4", i32* %"5", align 4
  ret i32 0
}
